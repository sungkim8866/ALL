//! Messages exchanged between two DeskLink computers (inside the encrypted
//! channel).

use serde::{Deserialize, Serialize};

use crate::geometry::Side;

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Os {
    Mac,
    Windows,
    Linux,
    Other,
}

impl Os {
    pub fn current() -> Os {
        if cfg!(target_os = "macos") {
            Os::Mac
        } else if cfg!(target_os = "windows") {
            Os::Windows
        } else if cfg!(target_os = "linux") {
            Os::Linux
        } else {
            Os::Other
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Os::Mac => "Mac",
            Os::Windows => "Windows",
            Os::Linux => "Linux",
            Os::Other => "?",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Button {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hello {
    pub version: u32,
    pub id: String,
    pub name: String,
    pub os: Os,
    /// TCP port the sender listens on (to reconnect later).
    pub port: u16,
    /// The sender's user explicitly asked to pair with this computer.
    pub want_pair: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BlobKind {
    File { name: String },
    ClipImage { width: u32, height: u32 },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Msg {
    Hello(Hello),
    /// Whether the sender already trusts the other side's key.
    Known(bool),
    PairSpake(Vec<u8>),
    PairConfirm(Vec<u8>),
    PairFailed,
    Ping,
    Pong,

    /// The cursor enters the receiver's desktop through `side` at fraction `pos`.
    Enter { side: Side, pos: f64 },
    /// The cursor left the receiver's desktop back toward the controller.
    Leave { pos: f64 },
    /// The controller took its cursor back (hotkey, sharing turned off…).
    Release,
    Move { dx: f32, dy: f32 },
    Button { button: Button, down: bool },
    /// Wheel units: 120 = one notch. Positive dy scrolls up, positive dx right.
    Wheel { dx: i32, dy: i32 },
    Key { code: u16, down: bool },

    ClipText(String),
    BlobStart { id: u64, kind: BlobKind, size: u64 },
    BlobChunk { id: u64, data: Vec<u8> },
    BlobEnd { id: u64 },
    BlobAbort { id: u64 },
}

impl Msg {
    pub fn encode(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("message serialization cannot fail")
    }

    pub fn decode(bytes: &[u8]) -> anyhow::Result<Msg> {
        Ok(postcard::from_bytes(bytes)?)
    }

    /// Bulk messages go through the low-priority queue so input stays snappy.
    pub fn is_bulk(&self) -> bool {
        matches!(
            self,
            Msg::ClipText(_) | Msg::BlobStart { .. } | Msg::BlobChunk { .. } | Msg::BlobEnd { .. }
        )
    }
}

/// UDP discovery beacon (sent in the clear; it only announces presence).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Beacon {
    pub magic: [u8; 4],
    pub version: u32,
    pub id: String,
    pub name: String,
    pub os: Os,
    pub port: u16,
}

pub const BEACON_MAGIC: [u8; 4] = *b"DLK1";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let msgs = vec![
            Msg::Enter { side: Side::Left, pos: 0.25 },
            Msg::Move { dx: -3.5, dy: 2.0 },
            Msg::Key { code: 0x04, down: true },
            Msg::ClipText("안녕하세요".into()),
            Msg::BlobStart {
                id: 7,
                kind: BlobKind::File { name: "사진.png".into() },
                size: 1234,
            },
        ];
        for m in msgs {
            assert_eq!(Msg::decode(&m.encode()).unwrap(), m);
        }
    }
}
