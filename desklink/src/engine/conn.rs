//! Per-connection work: authentication / pairing, and the reader and writer
//! tasks of an established session.

use std::net::SocketAddr;
use std::time::Duration;

use anyhow::{anyhow, bail, Result};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use tokio::time::timeout;

use crate::config::PairedPeer;
use crate::proto::{Hello, Msg, PROTOCOL_VERSION};
use crate::transport::{self, Channel, FrameReader, FrameWriter};

use super::state::PairPrompt;
use super::Ev;

pub const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
const CODE_ENTRY_TIMEOUT: Duration = Duration::from_secs(180);

#[derive(Clone)]
pub struct AuthCtx {
    pub me: Hello,
    pub private_key: Vec<u8>,
    pub peers: Vec<PairedPeer>,
}

pub struct Authed {
    pub conn: u64,
    pub hello: Hello,
    pub channel: Channel,
    pub addr: SocketAddr,
    pub remote_key: String,
    pub newly_paired: bool,
}

/// Why authentication failed, in words for the user.
#[derive(Debug)]
pub enum AuthError {
    /// The other side doesn't know us (or we don't know it) and nobody asked
    /// to pair.
    NotPaired,
    WrongCode,
    Other(anyhow::Error),
}

impl From<anyhow::Error> for AuthError {
    fn from(e: anyhow::Error) -> Self {
        AuthError::Other(e)
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::NotPaired => write!(f, "페어링되지 않은 컴퓨터입니다"),
            AuthError::WrongCode => write!(f, "코드가 맞지 않습니다"),
            AuthError::Other(e) => write!(f, "{e:#}"),
        }
    }
}

async fn read_timed(r: &mut FrameReader, t: Duration) -> Result<Msg> {
    timeout(t, r.read_msg())
        .await
        .map_err(|_| anyhow!("상대 컴퓨터가 응답하지 않습니다"))?
}

/// Exchanges hellos and, if needed and allowed, pairs with a 6-digit code.
///
/// `user` is true when this computer's user started the connection; only
/// then may an unknown computer be paired (the initiator types the code, the
/// responder shows it).
pub async fn authenticate(
    ctx: AuthCtx,
    mut ch: Channel,
    conn: u64,
    user: bool,
    ev: UnboundedSender<Ev>,
) -> Result<(Hello, Channel, bool), AuthError> {
    let mut hello = ctx.me.clone();
    hello.want_pair = user && ch.initiator;
    ch.writer.send(&Msg::Hello(hello)).await?;
    let peer = match read_timed(&mut ch.reader, HANDSHAKE_TIMEOUT).await? {
        Msg::Hello(h) => h,
        other => return Err(anyhow!("unexpected message {other:?}").into()),
    };
    if peer.version != PROTOCOL_VERSION {
        return Err(anyhow!(
            "DeskLink 버전이 다릅니다 (이 컴퓨터 {}, 상대 {}). 두 컴퓨터를 같은 버전으로 업데이트하세요",
            PROTOCOL_VERSION,
            peer.version
        )
        .into());
    }
    if peer.id == ctx.me.id {
        return Err(anyhow!("자기 자신에게는 연결할 수 없습니다").into());
    }
    let key_hex = hex::encode(&ch.remote_static);
    let known = ctx
        .peers
        .iter()
        .any(|p| p.id == peer.id && p.public_key == key_hex);
    ch.writer.send(&Msg::Known(known)).await?;
    let their_known = match read_timed(&mut ch.reader, HANDSHAKE_TIMEOUT).await? {
        Msg::Known(k) => k,
        other => return Err(anyhow!("unexpected message {other:?}").into()),
    };
    if known && their_known {
        return Ok((peer, ch, false));
    }

    // ---- pairing
    let show_code = !ch.initiator && peer.want_pair;
    let enter_code = ch.initiator && user;
    if !show_code && !enter_code {
        return Err(AuthError::NotPaired);
    }
    let hh = ch.handshake_hash.clone();
    let mut inbound: Option<Vec<u8>> = None;
    let code = if show_code {
        let code = transport::new_pair_code();
        let _ = ev.send(Ev::PairPrompt(PairPrompt::ShowCode {
            conn,
            peer: peer.name.clone(),
            code: code.clone(),
        }));
        code
    } else {
        let (tx, mut rx) = oneshot::channel::<String>();
        let _ = ev.send(Ev::PairCodeWaiter { conn, tx });
        let _ = ev.send(Ev::PairPrompt(PairPrompt::EnterCode {
            conn,
            peer: peer.name.clone(),
        }));
        let deadline = tokio::time::sleep(CODE_ENTRY_TIMEOUT);
        tokio::pin!(deadline);
        // While the user types, also watch the connection so a cancel on the
        // other side closes the prompt here.
        loop {
            tokio::select! {
                r = &mut rx => match r {
                    Ok(code) => break transport::normalize_code(&code),
                    Err(_) => return Err(anyhow!("페어링을 취소했습니다").into()),
                },
                // The other side sends its SPAKE message right away and then
                // nothing until it gets ours, so this read can be dropped
                // safely when the code arrives; it ends early only if the
                // other side cancels or disconnects.
                m = ch.reader.read_msg() => match m? {
                    Msg::PairSpake(x) if inbound.is_none() => inbound = Some(x),
                    Msg::PairFailed => return Err(anyhow!("상대 컴퓨터에서 페어링을 취소했습니다").into()),
                    other => return Err(anyhow!("unexpected message {other:?}").into()),
                },
                _ = &mut deadline => return Err(anyhow!("코드 입력 시간이 지났습니다").into()),
            }
        }
    };
    let (state, out) = transport::spake_start(&code, &hh);
    ch.writer.send(&Msg::PairSpake(out)).await?;
    let inbound = match inbound {
        Some(m) => m,
        None => match read_timed(&mut ch.reader, CODE_ENTRY_TIMEOUT + Duration::from_secs(20)).await? {
            Msg::PairSpake(m) => m,
            Msg::PairFailed => return Err(anyhow!("상대 컴퓨터에서 페어링을 취소했습니다").into()),
            other => return Err(anyhow!("unexpected message {other:?}").into()),
        },
    };
    let key = transport::spake_finish(state, &inbound)?;
    ch.writer
        .send(&Msg::PairConfirm(transport::confirm_tag(&key, &hh, ch.initiator)))
        .await?;
    let tag = match read_timed(&mut ch.reader, HANDSHAKE_TIMEOUT).await {
        Ok(Msg::PairConfirm(t)) => t,
        Ok(Msg::PairFailed) | Err(_) => return Err(AuthError::WrongCode),
        Ok(other) => return Err(anyhow!("unexpected message {other:?}").into()),
    };
    if !transport::verify_tag(&key, &hh, !ch.initiator, &tag) {
        let _ = ch.writer.send(&Msg::PairFailed).await;
        return Err(AuthError::WrongCode);
    }
    Ok((peer, ch, true))
}

pub async fn reader_task(mut r: FrameReader, conn: u64, ev: UnboundedSender<Ev>) {
    loop {
        match r.read_msg().await {
            Ok(msg) => {
                if ev.send(Ev::Net { conn, msg }).is_err() {
                    return;
                }
            }
            Err(e) => {
                let _ = ev.send(Ev::Closed {
                    conn,
                    err: Some(format!("{e:#}")),
                });
                return;
            }
        }
    }
}

/// Sends queued messages. Input and control messages (`hi`) always go before
/// bulk data (`lo`), so a big file never makes the pointer lag.
pub async fn writer_task(
    mut w: FrameWriter,
    mut hi: UnboundedReceiver<Msg>,
    mut lo: mpsc::Receiver<Msg>,
) -> Result<()> {
    let mut batch: Vec<Msg> = Vec::new();
    let mut lo_open = true;
    loop {
        batch.clear();
        tokio::select! {
            biased;
            m = hi.recv() => match m {
                Some(m) => batch.push(m),
                None => return Ok(()),
            },
            m = lo.recv(), if lo_open => match m {
                Some(m) => batch.push(m),
                None => { lo_open = false; continue; }
            },
        }
        while batch.len() < 512 {
            match hi.try_recv() {
                Ok(m) => batch.push(m),
                Err(_) => break,
            }
        }
        coalesce_moves(&mut batch);
        if batch.is_empty() {
            bail!("nothing to send");
        }
        w.send_batch(&batch).await?;
    }
}

/// Merges consecutive pointer moves into one.
fn coalesce_moves(batch: &mut Vec<Msg>) {
    let mut out: Vec<Msg> = Vec::with_capacity(batch.len());
    for m in batch.drain(..) {
        if let (Some(Msg::Move { dx: ax, dy: ay }), Msg::Move { dx, dy }) = (out.last_mut(), &m) {
            *ax += *dx;
            *ay += *dy;
            continue;
        }
        out.push(m);
    }
    *batch = out;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coalesce() {
        let mut b = vec![
            Msg::Move { dx: 1.0, dy: 1.0 },
            Msg::Move { dx: 2.0, dy: -1.0 },
            Msg::Key { code: 4, down: true },
            Msg::Move { dx: 1.0, dy: 0.0 },
        ];
        coalesce_moves(&mut b);
        assert_eq!(
            b,
            vec![
                Msg::Move { dx: 3.0, dy: 0.0 },
                Msg::Key { code: 4, down: true },
                Msg::Move { dx: 1.0, dy: 0.0 },
            ]
        );
    }
}
