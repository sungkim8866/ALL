//! State shared with the user interface, and commands from it.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::config::{PairedPeer, Settings};
use crate::platform::InputStatus;
use crate::proto::Os;

#[derive(Clone, Debug)]
pub enum Command {
    /// Connect (and pair if needed) to "ip", "ip:port" or "host:port".
    ConnectAddr(String),
    /// Connect (and pair if needed) to a computer seen on the network.
    ConnectPeer(String),
    Disconnect,
    /// The 6-digit code the user typed for the current pairing.
    SubmitCode(String),
    CancelPairing,
    UpdateSettings(Settings),
    /// Forget a paired computer.
    Forget(String),
    SendFiles(Vec<PathBuf>),
    RequestPermissions,
    ClearFinishedTransfers,
    Shutdown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConnStatus {
    Idle,
    Connecting(String),
    Connected {
        id: String,
        name: String,
        os: Os,
        addr: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum PairPrompt {
    /// Show this code; the other computer types it.
    ShowCode { conn: u64, peer: String, code: String },
    /// Ask the user for the code shown on the other computer.
    EnterCode { conn: u64, peer: String },
}

impl PairPrompt {
    pub fn conn(&self) -> u64 {
        match self {
            PairPrompt::ShowCode { conn, .. } | PairPrompt::EnterCode { conn, .. } => *conn,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Nearby {
    pub id: String,
    pub name: String,
    pub os: Os,
    pub addr: String,
    pub paired: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TransferState {
    Active,
    Done,
    Failed(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Transfer {
    pub id: u64,
    pub name: String,
    pub size: u64,
    pub done: u64,
    pub outgoing: bool,
    pub state: TransferState,
    /// Where a received file was saved.
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct UiState {
    pub my_id: String,
    pub fingerprint: String,
    pub os: Os,
    pub listen_port: Option<u16>,
    pub listen_error: Option<String>,
    pub settings: Settings,
    pub peers: Vec<PairedPeer>,
    pub nearby: Vec<Nearby>,
    pub conn: ConnStatus,
    pub pairing: Option<PairPrompt>,
    /// Last pairing / connection problem to show to the user.
    pub notice: Option<String>,
    pub controlling: bool,
    pub controlled: bool,
    pub transfers: Vec<Transfer>,
    pub input: InputStatus,
    pub download_dir: PathBuf,
    pub log: VecDeque<String>,
    pub quit: bool,
}

pub struct Shared {
    state: Mutex<UiState>,
    repaint: Mutex<Option<Box<dyn Fn() + Send + Sync>>>,
}

impl Shared {
    pub fn new(state: UiState) -> Shared {
        Shared {
            state: Mutex::new(state),
            repaint: Mutex::new(None),
        }
    }

    /// Called whenever the state changes (the UI uses it to redraw).
    pub fn set_repaint(&self, f: impl Fn() + Send + Sync + 'static) {
        *self.repaint.lock().unwrap() = Some(Box::new(f));
    }

    pub fn snapshot(&self) -> UiState {
        self.state.lock().unwrap().clone()
    }

    pub fn update<R>(&self, f: impl FnOnce(&mut UiState) -> R) -> R {
        let r = f(&mut self.state.lock().unwrap());
        if let Some(cb) = self.repaint.lock().unwrap().as_ref() {
            cb();
        }
        r
    }

    pub fn log(&self, line: impl Into<String>) {
        let line = line.into();
        log::info!("{line}");
        self.update(|s| {
            s.log.push_back(line);
            while s.log.len() > 200 {
                s.log.pop_front();
            }
        });
    }
}
