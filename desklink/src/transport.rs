//! Encrypted, authenticated channel between two computers.
//!
//! * Noise XX (X25519, ChaCha20-Poly1305, BLAKE2s) over TCP. Each computer has
//!   a long-term static key; after pairing each side remembers the other's
//!   public key and refuses anything else.
//! * Pairing uses SPAKE2 with a 6-digit code shown on one computer and typed
//!   on the other. The SPAKE2 identity is bound to the Noise handshake hash, so
//!   a man in the middle (who necessarily has two different handshakes) cannot
//!   complete it, and the code cannot be brute-forced offline.

use std::sync::Arc;

use anyhow::{anyhow, bail, Context, Result};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use snow::StatelessTransportState;
use spake2::{Ed25519Group, Identity, Password, Spake2};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

use crate::config::NOISE_PARAMS;
use crate::proto::Msg;

const NOISE_MAX: usize = 65535;
const TAG_LEN: usize = 16;
/// Plaintext bytes per Noise frame (1 flag byte + data).
const PIECE: usize = 60_000;
/// Largest reassembled message we accept.
const MAX_MESSAGE: usize = 48 * 1024 * 1024;
const PROLOGUE: &[u8] = b"DeskLink v1";

pub struct Channel {
    pub reader: FrameReader,
    pub writer: FrameWriter,
    pub remote_static: Vec<u8>,
    pub handshake_hash: Vec<u8>,
    pub initiator: bool,
}

pub struct FrameReader {
    rd: OwnedReadHalf,
    st: Arc<StatelessTransportState>,
    nonce: u64,
    buf: Vec<u8>,
}

pub struct FrameWriter {
    wr: OwnedWriteHalf,
    st: Arc<StatelessTransportState>,
    nonce: u64,
    out: Vec<u8>,
    scratch: Vec<u8>,
}

async fn read_raw(rd: &mut (impl AsyncReadExt + Unpin), buf: &mut Vec<u8>) -> Result<()> {
    let len = rd.read_u16().await? as usize;
    buf.resize(len, 0);
    rd.read_exact(buf).await?;
    Ok(())
}

async fn write_raw(wr: &mut (impl AsyncWriteExt + Unpin), data: &[u8]) -> Result<()> {
    let mut out = Vec::with_capacity(data.len() + 2);
    out.extend_from_slice(&(data.len() as u16).to_be_bytes());
    out.extend_from_slice(data);
    wr.write_all(&out).await?;
    Ok(())
}

/// Runs the Noise XX handshake on a fresh TCP connection.
pub async fn handshake(stream: TcpStream, private_key: &[u8], initiator: bool) -> Result<Channel> {
    stream.set_nodelay(true).ok();
    let (mut rd, mut wr) = stream.into_split();
    let builder = snow::Builder::new(NOISE_PARAMS.parse()?)
        .local_private_key(private_key)
        .prologue(PROLOGUE);
    let mut hs = if initiator {
        builder.build_initiator()?
    } else {
        builder.build_responder()?
    };
    let mut msg = vec![0u8; NOISE_MAX];
    let mut payload = vec![0u8; NOISE_MAX];
    let mut buf = Vec::new();
    while !hs.is_handshake_finished() {
        if hs.is_my_turn() {
            let n = hs.write_message(&[], &mut msg)?;
            write_raw(&mut wr, &msg[..n]).await?;
        } else {
            read_raw(&mut rd, &mut buf).await?;
            hs.read_message(&buf, &mut payload)
                .map_err(|e| anyhow!("handshake failed: {e:?}"))?;
        }
    }
    let remote_static = hs
        .get_remote_static()
        .ok_or_else(|| anyhow!("peer sent no static key"))?
        .to_vec();
    let handshake_hash = hs.get_handshake_hash().to_vec();
    let st = Arc::new(hs.into_stateless_transport_mode()?);
    Ok(Channel {
        reader: FrameReader {
            rd,
            st: st.clone(),
            nonce: 0,
            buf: Vec::new(),
        },
        writer: FrameWriter {
            wr,
            st,
            nonce: 0,
            out: Vec::new(),
            scratch: vec![0u8; NOISE_MAX],
        },
        remote_static,
        handshake_hash,
        initiator,
    })
}

impl FrameReader {
    pub async fn read_msg(&mut self) -> Result<Msg> {
        let mut assembled: Vec<u8> = Vec::new();
        let mut plain = vec![0u8; NOISE_MAX];
        loop {
            read_raw(&mut self.rd, &mut self.buf).await?;
            if self.buf.len() < TAG_LEN + 1 {
                bail!("short frame");
            }
            let n = self
                .st
                .read_message(self.nonce, &self.buf, &mut plain)
                .map_err(|e| anyhow!("decrypt failed: {e:?}"))?;
            self.nonce += 1;
            if n == 0 {
                bail!("empty frame");
            }
            let more = plain[0] == 1;
            if assembled.len() + n - 1 > MAX_MESSAGE {
                bail!("message too large");
            }
            assembled.extend_from_slice(&plain[1..n]);
            if !more {
                return Msg::decode(&assembled).context("decoding message");
            }
        }
    }
}

impl FrameWriter {
    fn push(&mut self, msg: &Msg) -> Result<()> {
        let bytes = msg.encode();
        let mut chunks = bytes.chunks(PIECE).peekable();
        if bytes.is_empty() {
            bail!("empty message");
        }
        let mut piece = Vec::with_capacity(PIECE + 1);
        while let Some(chunk) = chunks.next() {
            piece.clear();
            piece.push(if chunks.peek().is_some() { 1 } else { 0 });
            piece.extend_from_slice(chunk);
            let n = self
                .st
                .write_message(self.nonce, &piece, &mut self.scratch)
                .map_err(|e| anyhow!("encrypt failed: {e:?}"))?;
            self.nonce += 1;
            self.out.extend_from_slice(&(n as u16).to_be_bytes());
            self.out.extend_from_slice(&self.scratch[..n]);
        }
        Ok(())
    }

    /// Encrypts and sends several messages with a single write.
    pub async fn send_batch(&mut self, msgs: &[Msg]) -> Result<()> {
        self.out.clear();
        for m in msgs {
            self.push(m)?;
        }
        self.wr.write_all(&self.out).await?;
        Ok(())
    }

    pub async fn send(&mut self, msg: &Msg) -> Result<()> {
        self.send_batch(std::slice::from_ref(msg)).await
    }
}

// ---------------------------------------------------------------------------
// Pairing

pub fn new_pair_code() -> String {
    use rand::Rng;
    format!("{:06}", rand::thread_rng().gen_range(0..1_000_000u32))
}

/// Normalizes what the user typed ("123 456", "123-456") to six digits.
pub fn normalize_code(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_digit()).collect()
}

pub fn spake_start(code: &str, handshake_hash: &[u8]) -> (Spake2<Ed25519Group>, Vec<u8>) {
    let mut ident = b"DeskLink pairing ".to_vec();
    ident.extend_from_slice(handshake_hash);
    Spake2::<Ed25519Group>::start_symmetric(&Password::new(code.as_bytes()), &Identity::new(&ident))
}

pub fn spake_finish(state: Spake2<Ed25519Group>, inbound: &[u8]) -> Result<Vec<u8>> {
    state.finish(inbound).map_err(|e| anyhow!("pairing failed: {e:?}"))
}

fn confirm_mac(key: &[u8], handshake_hash: &[u8], from_initiator: bool) -> Hmac<Sha256> {
    let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key).expect("hmac accepts any key size");
    mac.update(b"DeskLink confirm ");
    mac.update(if from_initiator { b"I" } else { b"R" });
    mac.update(handshake_hash);
    mac
}

pub fn confirm_tag(key: &[u8], handshake_hash: &[u8], from_initiator: bool) -> Vec<u8> {
    confirm_mac(key, handshake_hash, from_initiator)
        .finalize()
        .into_bytes()
        .to_vec()
}

pub fn verify_tag(key: &[u8], handshake_hash: &[u8], from_initiator: bool, tag: &[u8]) -> bool {
    confirm_mac(key, handshake_hash, from_initiator)
        .verify_slice(tag)
        .is_ok()
}

/// Short human-readable fingerprint of a public key.
pub fn fingerprint(public_key_hex: &str) -> String {
    let bytes = hex::decode(public_key_hex).unwrap_or_default();
    let digest = Sha256::digest(&bytes);
    let h = hex::encode(&digest[..6]).to_uppercase();
    format!("{}-{}-{}", &h[0..4], &h[4..8], &h[8..12])
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    fn keypair() -> snow::Keypair {
        snow::Builder::new(NOISE_PARAMS.parse().unwrap())
            .generate_keypair()
            .unwrap()
    }

    #[tokio::test]
    async fn handshake_and_big_messages() {
        let a = keypair();
        let b = keypair();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let b_priv = b.private.clone();
        let server = tokio::spawn(async move {
            let (s, _) = listener.accept().await.unwrap();
            let mut ch = handshake(s, &b_priv, false).await.unwrap();
            let m = ch.reader.read_msg().await.unwrap();
            ch.writer.send(&m).await.unwrap();
            (ch.remote_static, ch.handshake_hash)
        });
        let s = TcpStream::connect(addr).await.unwrap();
        let mut ch = handshake(s, &a.private, true).await.unwrap();
        assert_eq!(ch.remote_static, b.public);
        let big = Msg::ClipText("가".repeat(200_000));
        ch.writer.send(&big).await.unwrap();
        let echo = ch.reader.read_msg().await.unwrap();
        assert_eq!(echo, big);
        let (server_saw, server_hh) = server.await.unwrap();
        assert_eq!(server_saw, a.public);
        assert_eq!(server_hh, ch.handshake_hash);
    }

    #[test]
    fn spake_pairing() {
        let hh = [7u8; 32];
        let (sa, ma) = spake_start("123456", &hh);
        let (sb, mb) = spake_start("123456", &hh);
        let ka = spake_finish(sa, &mb).unwrap();
        let kb = spake_finish(sb, &ma).unwrap();
        assert_eq!(ka, kb);
        let tag = confirm_tag(&ka, &hh, true);
        assert!(verify_tag(&kb, &hh, true, &tag));
        assert!(!verify_tag(&kb, &hh, false, &tag));

        // Wrong code → different keys.
        let (sa, ma) = spake_start("123456", &hh);
        let (sb, mb) = spake_start("654321", &hh);
        let ka = spake_finish(sa, &mb).unwrap();
        let kb = spake_finish(sb, &ma).unwrap();
        assert_ne!(ka, kb);

        // Different handshake (MITM) → different keys.
        let (sa, ma) = spake_start("123456", &[1u8; 32]);
        let (sb, mb) = spake_start("123456", &[2u8; 32]);
        assert_ne!(spake_finish(sa, &mb).unwrap(), spake_finish(sb, &ma).unwrap());
    }

    #[test]
    fn code_normalization() {
        assert_eq!(normalize_code(" 123-456 "), "123456");
        assert_eq!(new_pair_code().len(), 6);
    }
}
