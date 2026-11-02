//! The engine: keeps the connection to the other computer, moves the mouse
//! and keyboard across the screen edge, syncs the clipboard and moves files.
//!
//! It runs as a single task that owns all state; network, input hooks,
//! clipboard and the UI talk to it through channels.

pub mod conn;
pub mod files;
pub mod state;

use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::clipboard::{self, BackendFactory, ClipContent, MAX_IMAGE_BYTES};
use crate::config::{self, Config, PairedPeer, Settings};
use crate::discovery::{self, Seen};
use crate::geometry::{Desktop, Side};
use crate::keys;
use crate::platform::{CapEvent, Inject, Platform};
use crate::proto::{Beacon, BlobKind, Button, Hello, Msg, Os, BEACON_MAGIC, PROTOCOL_VERSION};
use crate::transport::{self, Channel};

use conn::{AuthCtx, AuthError, Authed};
use files::IncomingFile;
pub use state::*;

pub const DEFAULT_PORT: u16 = 47810;
pub const DEFAULT_DISCOVERY_PORT: u16 = 47811;

const PING_EVERY: Duration = Duration::from_secs(2);
const DEAD_AFTER: Duration = Duration::from_secs(8);
const NEARBY_FOR: Duration = Duration::from_secs(8);
const RETRY_EVERY: Duration = Duration::from_secs(4);

pub struct EngineOptions {
    pub config_dir: PathBuf,
    pub listen_port: u16,
    /// `None` turns discovery off (tests).
    pub discovery_port: Option<u16>,
    /// Overrides the configured download folder (tests).
    pub download_dir: Option<PathBuf>,
}

impl Default for EngineOptions {
    fn default() -> Self {
        EngineOptions {
            config_dir: config::default_config_dir(),
            listen_port: DEFAULT_PORT,
            discovery_port: Some(DEFAULT_DISCOVERY_PORT),
            download_dir: None,
        }
    }
}

#[derive(Clone)]
pub struct EngineHandle {
    pub cmd: UnboundedSender<Command>,
    pub shared: Arc<Shared>,
}

impl EngineHandle {
    pub fn send(&self, c: Command) {
        let _ = self.cmd.send(c);
    }
}

/// Internal events delivered to the engine task.
pub enum Ev {
    Seen(Seen),
    Incoming(TcpStream, SocketAddr),
    DialFailed { addr: String, peer: Option<String>, err: String, user: bool },
    PairPrompt(PairPrompt),
    PairCodeWaiter { conn: u64, tx: oneshot::Sender<String> },
    AuthFailed { conn: u64, err: AuthError, user: bool, peer: Option<String> },
    Authed(Box<Authed>),
    Net { conn: u64, msg: Msg },
    Closed { conn: u64, err: Option<String> },
    Clip(ClipContent),
}

struct Session {
    conn: u64,
    peer: Hello,
    addr: SocketAddr,
    /// The computer with the smaller id initiated this connection.
    canonical: bool,
    hi: UnboundedSender<Msg>,
    lo: mpsc::Sender<Msg>,
    last_rx: Instant,
    last_ping: Instant,
    tasks: Vec<JoinHandle<()>>,
}

impl Drop for Session {
    fn drop(&mut self) {
        for t in &self.tasks {
            t.abort();
        }
    }
}

impl Session {
    fn send(&self, m: Msg) {
        let _ = self.hi.send(m);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    /// The pointer is on this computer.
    Local,
    /// This computer's mouse and keyboard drive the other computer.
    Controlling { exit_point: (f64, f64) },
}

/// The other computer is driving this one.
struct Remote {
    pos: (f64, f64),
    /// Edge through which the pointer entered (and leaves again).
    side: Side,
    keys: HashSet<u16>,
    buttons: HashSet<Button>,
}

struct Discovered {
    beacon: Beacon,
    addr: SocketAddr,
    seen: Instant,
}

enum IncomingBlob {
    File { file: IncomingFile, transfer: u64, name: String },
    Image { width: u32, height: u32, size: u64, data: Vec<u8> },
}

pub struct Engine {
    opts: EngineOptions,
    cfg: Config,
    platform: Arc<dyn Platform>,
    shared: Arc<Shared>,
    ev_tx: UnboundedSender<Ev>,
    listen_port: u16,
    beacon_name: Arc<Mutex<String>>,
    clip_tx: std::sync::mpsc::Sender<ClipContent>,
    clip_enabled: Arc<AtomicBool>,

    next_conn: u64,
    session: Option<Session>,
    auth_tasks: HashMap<u64, JoinHandle<()>>,
    code_waiters: HashMap<u64, oneshot::Sender<String>>,
    dialing: HashSet<String>,
    last_dial: HashMap<String, Instant>,
    disconnected_since: Instant,
    discovered: HashMap<String, Discovered>,

    desktop: Desktop,
    desktop_at: Instant,
    mode: Mode,
    remote: Option<Remote>,
    last_pos: (f64, f64),
    armed: bool,
    edge_since: Option<Instant>,
    local_buttons: HashSet<Button>,
    sent_keys: HashMap<u16, u16>,
    shift_l: bool,
    shift_r: bool,
    move_rest: (f64, f64),

    blobs: HashMap<u64, IncomingBlob>,
    bg_tasks: Vec<JoinHandle<()>>,
    quit: bool,
}

fn is_lower(a: &str, b: &str) -> bool {
    a < b
}

impl Engine {
    /// Starts the engine on the current tokio runtime.
    pub async fn spawn(
        opts: EngineOptions,
        platform: Arc<dyn Platform>,
        cap_rx: UnboundedReceiver<CapEvent>,
        clip: BackendFactory,
    ) -> Result<EngineHandle> {
        let cfg = Config::load_or_create(&opts.config_dir)
            .with_context(|| format!("설정을 읽을 수 없습니다 ({})", opts.config_dir.display()))?;
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let (ev_tx, ev_rx) = mpsc::unbounded_channel();

        let download_dir = opts
            .download_dir
            .clone()
            .or_else(|| cfg.settings.download_dir.clone())
            .unwrap_or_else(config::default_download_dir);
        let shared = Arc::new(Shared::new(UiState {
            my_id: cfg.device_id.clone(),
            fingerprint: transport::fingerprint(&cfg.public_key),
            os: platform.os(),
            listen_port: None,
            listen_error: None,
            settings: cfg.settings.clone(),
            peers: cfg.peers.clone(),
            nearby: Vec::new(),
            conn: ConnStatus::Idle,
            pairing: None,
            notice: None,
            controlling: false,
            controlled: false,
            transfers: Vec::new(),
            input: platform.input_status(),
            download_dir,
            log: Default::default(),
            quit: false,
        }));

        // Listener.
        let listener = match TcpListener::bind(("0.0.0.0", opts.listen_port)).await {
            Ok(l) => Some(l),
            Err(e) => {
                let msg = if e.kind() == std::io::ErrorKind::AddrInUse {
                    format!("포트 {}을(를) 다른 프로그램이 쓰고 있습니다. DeskLink가 이미 실행 중인지 확인하세요.", opts.listen_port)
                } else {
                    format!("연결을 받을 수 없습니다: {e}")
                };
                shared.update(|s| s.listen_error = Some(msg.clone()));
                shared.log(msg);
                None
            }
        };
        let listen_port = listener
            .as_ref()
            .and_then(|l| l.local_addr().ok())
            .map(|a| a.port())
            .unwrap_or(opts.listen_port);
        shared.update(|s| s.listen_port = listener.as_ref().map(|_| listen_port));
        let mut bg_tasks = Vec::new();
        if let Some(listener) = listener {
            let tx = ev_tx.clone();
            bg_tasks.push(tokio::spawn(async move {
                loop {
                    match listener.accept().await {
                        Ok((s, a)) => {
                            if tx.send(Ev::Incoming(s, a)).is_err() {
                                return;
                            }
                        }
                        Err(e) => {
                            log::warn!("accept: {e}");
                            tokio::time::sleep(Duration::from_millis(200)).await;
                        }
                    }
                }
            }));
        }

        // Discovery.
        let beacon_name = Arc::new(Mutex::new(cfg.settings.device_name.clone()));
        if let Some(dport) = opts.discovery_port {
            let (seen_tx, mut seen_rx) = mpsc::unbounded_channel();
            let name = beacon_name.clone();
            let id = cfg.device_id.clone();
            let os = platform.os();
            let shared2 = shared.clone();
            bg_tasks.push(tokio::spawn(async move {
                let make = move || Beacon {
                    magic: BEACON_MAGIC,
                    version: PROTOCOL_VERSION,
                    id: id.clone(),
                    name: name.lock().unwrap().clone(),
                    os,
                    port: listen_port,
                };
                if let Err(e) = discovery::run(dport, make, seen_tx).await {
                    shared2.log(format!("주변 컴퓨터 찾기를 시작하지 못했습니다: {e}"));
                }
            }));
            let tx = ev_tx.clone();
            bg_tasks.push(tokio::spawn(async move {
                while let Some(s) = seen_rx.recv().await {
                    if tx.send(Ev::Seen(s)).is_err() {
                        return;
                    }
                }
            }));
        }

        // Clipboard.
        let (clip_tx, clip_rx) = std::sync::mpsc::channel();
        let clip_enabled = Arc::new(AtomicBool::new(cfg.settings.share_clipboard));
        {
            let tx = ev_tx.clone();
            clipboard::spawn(clip, clip_enabled.clone(), clip_rx, move |c| {
                let _ = tx.send(Ev::Clip(c));
            });
        }

        let desktop = Desktop::new(platform.screens());
        let last_pos = platform.cursor_pos();
        let engine = Engine {
            opts,
            cfg,
            platform,
            shared: shared.clone(),
            ev_tx,
            listen_port,
            beacon_name,
            clip_tx,
            clip_enabled,
            next_conn: 1,
            session: None,
            auth_tasks: HashMap::new(),
            code_waiters: HashMap::new(),
            dialing: HashSet::new(),
            last_dial: HashMap::new(),
            disconnected_since: Instant::now(),
            discovered: HashMap::new(),
            desktop,
            desktop_at: Instant::now(),
            mode: Mode::Local,
            remote: None,
            last_pos,
            armed: true,
            edge_since: None,
            local_buttons: HashSet::new(),
            sent_keys: HashMap::new(),
            shift_l: false,
            shift_r: false,
            move_rest: (0.0, 0.0),
            blobs: HashMap::new(),
            bg_tasks,
            quit: false,
        };
        tokio::spawn(engine.run(cmd_rx, cap_rx, ev_rx));
        shared.log(format!("DeskLink 시작 (포트 {listen_port})"));
        Ok(EngineHandle { cmd: cmd_tx, shared })
    }

    async fn run(
        mut self,
        mut cmd_rx: UnboundedReceiver<Command>,
        mut cap_rx: UnboundedReceiver<CapEvent>,
        mut ev_rx: UnboundedReceiver<Ev>,
    ) {
        let mut tick = tokio::time::interval(Duration::from_millis(500));
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        let mut cmd_open = true;
        let mut cap_open = true;
        loop {
            tokio::select! {
                biased;
                c = cap_rx.recv(), if cap_open => match c {
                    Some(c) => self.on_cap(c),
                    None => cap_open = false,
                },
                e = ev_rx.recv() => match e {
                    Some(e) => self.on_event(e),
                    None => break,
                },
                c = cmd_rx.recv(), if cmd_open => match c {
                    Some(c) => self.on_command(c),
                    None => cmd_open = false,
                },
                _ = tick.tick() => self.on_tick(),
            }
            if self.quit {
                break;
            }
        }
        self.close_session(None);
        for t in self.bg_tasks.drain(..) {
            t.abort();
        }
        for (_, t) in self.auth_tasks.drain() {
            t.abort();
        }
        self.platform.shutdown();
        self.shared.update(|s| s.quit = true);
    }

    fn log(&self, line: impl Into<String>) {
        self.shared.log(line);
    }

    fn settings(&self) -> &Settings {
        &self.cfg.settings
    }

    fn save_config(&self) {
        if let Err(e) = self.cfg.save(&self.opts.config_dir) {
            self.log(format!("설정 저장 실패: {e:#}"));
        }
        let peers = self.cfg.peers.clone();
        let settings = self.cfg.settings.clone();
        self.shared.update(|s| {
            s.peers = peers;
            s.settings = settings;
        });
    }

    fn download_dir(&self) -> PathBuf {
        self.opts
            .download_dir
            .clone()
            .or_else(|| self.cfg.settings.download_dir.clone())
            .unwrap_or_else(config::default_download_dir)
    }

    // ------------------------------------------------------------------
    // Commands from the UI

    fn on_command(&mut self, c: Command) {
        match c {
            Command::ConnectAddr(text) => self.dial_text(text),
            Command::ConnectPeer(id) => {
                let addr = self
                    .discovered
                    .get(&id)
                    .map(|d| d.addr.to_string())
                    .or_else(|| self.cfg.peer(&id).and_then(|p| p.last_addr.clone()));
                match addr {
                    Some(a) => {
                        if self.session.as_ref().is_some_and(|s| s.peer.id != id) {
                            self.close_session(Some("다른 컴퓨터로 전환".into()));
                        }
                        self.dial(a, Some(id), true);
                    }
                    None => self.notice("그 컴퓨터의 주소를 모릅니다. 같은 네트워크에 있는지 확인하세요."),
                }
            }
            Command::Disconnect => {
                // Don't reconnect right away.
                if let Some(s) = &self.session {
                    self.last_dial.insert(s.peer.id.clone(), Instant::now() + Duration::from_secs(60));
                }
                self.close_session(Some("연결을 끊었습니다".into()));
            }
            Command::SubmitCode(code) => {
                if let Some(PairPrompt::EnterCode { conn, .. }) = self.shared.snapshot().pairing {
                    if let Some(tx) = self.code_waiters.remove(&conn) {
                        let _ = tx.send(code);
                    }
                }
            }
            Command::CancelPairing => {
                if let Some(p) = self.shared.snapshot().pairing {
                    let conn = p.conn();
                    self.code_waiters.remove(&conn);
                    if let Some(t) = self.auth_tasks.remove(&conn) {
                        t.abort();
                    }
                    self.shared.update(|s| {
                        s.pairing = None;
                        if matches!(s.conn, ConnStatus::Connecting(_)) {
                            s.conn = ConnStatus::Idle;
                        }
                    });
                }
            }
            Command::UpdateSettings(new) => self.update_settings(new),
            Command::Forget(id) => {
                self.cfg.peers.retain(|p| p.id != id);
                if self.session.as_ref().is_some_and(|s| s.peer.id == id) {
                    self.close_session(Some("페어링을 삭제했습니다".into()));
                }
                self.save_config();
                self.refresh_nearby();
            }
            Command::SendFiles(paths) => self.send_files(paths),
            Command::RequestPermissions => {
                self.platform.request_permissions();
            }
            Command::ClearFinishedTransfers => self.shared.update(|s| {
                s.transfers.retain(|t| t.state == TransferState::Active);
            }),
            Command::Shutdown => self.quit = true,
        }
    }

    fn notice(&self, text: impl Into<String>) {
        let text = text.into();
        self.log(text.clone());
        self.shared.update(|s| s.notice = Some(text));
    }

    fn update_settings(&mut self, mut new: Settings) {
        new.pointer_speed = new.pointer_speed.clamp(0.2, 5.0);
        new.switch_delay_ms = new.switch_delay_ms.min(2000);
        if new.device_name.trim().is_empty() {
            new.device_name = self.cfg.settings.device_name.clone();
        }
        let stop_sharing = !new.share_input && self.cfg.settings.share_input;
        let side_changed = new.peer_side != self.cfg.settings.peer_side;
        self.clip_enabled.store(new.share_clipboard, Ordering::Relaxed);
        *self.beacon_name.lock().unwrap() = new.device_name.clone();
        let dir_changed = new.download_dir != self.cfg.settings.download_dir;
        self.cfg.settings = new;
        if (stop_sharing || side_changed) && matches!(self.mode, Mode::Controlling { .. }) {
            self.take_back(true);
        }
        self.save_config();
        if dir_changed {
            let d = self.download_dir();
            self.shared.update(|s| s.download_dir = d);
        }
    }

    // ------------------------------------------------------------------
    // Connecting

    fn dial_text(&mut self, text: String) {
        let text = text.trim().to_string();
        if text.is_empty() {
            return;
        }
        let has_port = text.parse::<SocketAddr>().is_ok()
            || (text.matches(':').count() == 1
                && text.rsplit(':').next().is_some_and(|p| p.parse::<u16>().is_ok()));
        let with_port = if has_port { text } else { format!("{text}:{DEFAULT_PORT}") };
        if let Some(s) = &self.session {
            if s.addr.to_string() != with_port {
                self.close_session(Some("다른 컴퓨터로 전환".into()));
            }
        }
        self.dial(with_port, None, true);
    }

    fn dial(&mut self, addr: String, peer: Option<String>, user: bool) {
        if self.dialing.contains(&addr) {
            return;
        }
        self.dialing.insert(addr.clone());
        if let Some(p) = &peer {
            self.last_dial.insert(p.clone(), Instant::now());
        }
        if user {
            let a = addr.clone();
            self.shared.update(|s| {
                s.notice = None;
                if s.conn == ConnStatus::Idle {
                    s.conn = ConnStatus::Connecting(a);
                }
            });
        }
        let conn = self.next_conn;
        self.next_conn += 1;
        let ctx = self.auth_ctx();
        let ev = self.ev_tx.clone();
        let task = tokio::spawn(async move {
            let stream = async {
                let mut last_err = anyhow::anyhow!("주소를 찾을 수 없습니다");
                for sa in tokio::net::lookup_host(&addr).await? {
                    match tokio::time::timeout(Duration::from_secs(4), TcpStream::connect(sa)).await {
                        Ok(Ok(s)) => return Ok((s, sa)),
                        Ok(Err(e)) => last_err = e.into(),
                        Err(_) => last_err = anyhow::anyhow!("시간 초과"),
                    }
                }
                Err(last_err)
            }
            .await;
            let (stream, sa) = match stream {
                Ok(v) => v,
                Err(e) => {
                    let _ = ev.send(Ev::DialFailed {
                        addr,
                        peer,
                        err: format!("{e:#}"),
                        user,
                    });
                    return;
                }
            };
            let _ = ev.send(Ev::DialFailed {
                addr: addr.clone(),
                peer: None,
                err: String::new(),
                user: false,
            }); // clears the "dialing" mark
            run_auth(ctx, stream, sa, conn, true, user, peer, ev).await;
        });
        self.auth_tasks.insert(conn, task);
    }

    fn auth_ctx(&self) -> AuthCtx {
        AuthCtx {
            me: Hello {
                version: PROTOCOL_VERSION,
                id: self.cfg.device_id.clone(),
                name: self.cfg.settings.device_name.clone(),
                os: self.platform.os(),
                port: self.listen_port,
                want_pair: false,
            },
            private_key: self.cfg.private_key_bytes(),
            peers: self.cfg.peers.clone(),
        }
    }

    fn on_incoming(&mut self, stream: TcpStream, addr: SocketAddr) {
        if self.auth_tasks.len() > 8 {
            return; // too many half-open connections
        }
        let conn = self.next_conn;
        self.next_conn += 1;
        let ctx = self.auth_ctx();
        let ev = self.ev_tx.clone();
        let task = tokio::spawn(run_auth(ctx, stream, addr, conn, false, false, None, ev));
        self.auth_tasks.insert(conn, task);
    }

    fn on_authed(&mut self, a: Authed) {
        self.auth_tasks.remove(&a.conn);
        self.code_waiters.remove(&a.conn);
        self.shared.update(|s| {
            if s.pairing.as_ref().is_some_and(|p| p.conn() == a.conn) {
                s.pairing = None;
            }
        });
        let peer_addr = SocketAddr::new(a.addr.ip(), a.hello.port).to_string();
        if a.newly_paired {
            self.cfg.upsert_peer(PairedPeer {
                id: a.hello.id.clone(),
                name: a.hello.name.clone(),
                os: a.hello.os,
                public_key: a.remote_key.clone(),
                last_addr: Some(peer_addr.clone()),
            });
            self.save_config();
            self.log(format!("{}와(과) 페어링했습니다", a.hello.name));
        } else if let Some(p) = self.cfg.peers.iter_mut().find(|p| p.id == a.hello.id) {
            if p.name != a.hello.name || p.os != a.hello.os || p.last_addr.as_deref() != Some(&peer_addr) {
                p.name = a.hello.name.clone();
                p.os = a.hello.os;
                p.last_addr = Some(peer_addr.clone());
                self.save_config();
            }
        }

        let initiator_id = if a.channel.initiator { &self.cfg.device_id } else { &a.hello.id };
        let responder_id = if a.channel.initiator { &a.hello.id } else { &self.cfg.device_id };
        let canonical = is_lower(initiator_id, responder_id);
        if let Some(s) = &self.session {
            if s.peer.id != a.hello.id {
                self.log(format!(
                    "{}의 연결을 거절했습니다 (이미 {}와(과) 연결됨)",
                    a.hello.name, s.peer.name
                ));
                return;
            }
            if s.canonical && !canonical {
                return; // keep the existing (preferred) connection
            }
            // Replace the old connection quietly.
            self.session = None;
        }
        self.start_session(a, canonical);
    }

    fn start_session(&mut self, a: Authed, canonical: bool) {
        let Authed {
            conn,
            hello,
            channel,
            addr,
            ..
        } = a;
        let Channel { reader, writer, .. } = channel;
        let (hi_tx, hi_rx) = mpsc::unbounded_channel();
        let (lo_tx, lo_rx) = mpsc::channel(64);
        let ev = self.ev_tx.clone();
        let reader_task = tokio::spawn(conn::reader_task(reader, conn, ev.clone()));
        let writer_task = tokio::spawn(async move {
            let err = conn::writer_task(writer, hi_rx, lo_rx).await.err();
            let _ = ev.send(Ev::Closed {
                conn,
                err: err.map(|e| format!("{e:#}")),
            });
        });
        let status = ConnStatus::Connected {
            id: hello.id.clone(),
            name: hello.name.clone(),
            os: hello.os,
            addr: addr.ip().to_string(),
        };
        self.log(format!("{} ({}) 연결됨", hello.name, hello.os.label()));
        self.session = Some(Session {
            conn,
            peer: hello,
            addr,
            canonical,
            hi: hi_tx,
            lo: lo_tx,
            last_rx: Instant::now(),
            last_ping: Instant::now(),
            tasks: vec![reader_task, writer_task],
        });
        self.shared.update(|s| {
            s.conn = status;
            s.notice = None;
        });
        self.refresh_nearby();
    }

    fn close_session(&mut self, reason: Option<String>) {
        let Some(s) = self.session.take() else { return };
        if matches!(self.mode, Mode::Controlling { .. }) {
            self.take_back(false);
        }
        self.release_remote();
        self.blobs.clear();
        self.disconnected_since = Instant::now();
        let name = s.peer.name.clone();
        drop(s);
        self.shared.update(|st| {
            st.conn = ConnStatus::Idle;
            st.controlling = false;
            st.controlled = false;
            for t in st.transfers.iter_mut() {
                if t.state == TransferState::Active {
                    t.state = TransferState::Failed("연결이 끊겼습니다".into());
                }
            }
        });
        match reason {
            Some(r) => self.log(format!("{name}: {r}")),
            None => self.log(format!("{name} 연결 끊김")),
        }
    }

    // ------------------------------------------------------------------
    // Internal events

    fn on_event(&mut self, e: Ev) {
        match e {
            Ev::Seen(seen) => {
                if seen.beacon.id == self.cfg.device_id {
                    return;
                }
                let new = !self.discovered.contains_key(&seen.beacon.id);
                self.discovered.insert(
                    seen.beacon.id.clone(),
                    Discovered {
                        beacon: seen.beacon,
                        addr: seen.addr,
                        seen: Instant::now(),
                    },
                );
                if new {
                    self.refresh_nearby();
                }
            }
            Ev::Incoming(s, a) => self.on_incoming(s, a),
            Ev::DialFailed { addr, peer, err, user } => {
                self.dialing.remove(&addr);
                if err.is_empty() {
                    return;
                }
                if let Some(p) = peer.as_ref() {
                    self.last_dial.insert(p.clone(), Instant::now());
                }
                if user {
                    self.shared.update(|s| {
                        if s.conn == ConnStatus::Connecting(addr.clone()) {
                            s.conn = ConnStatus::Idle;
                        }
                    });
                    self.notice(format!("{addr}에 연결할 수 없습니다: {err}"));
                } else {
                    log::debug!("auto-connect {addr}: {err}");
                }
            }
            Ev::PairPrompt(p) => {
                self.shared.update(|s| {
                    s.pairing = Some(p);
                    s.notice = None;
                });
            }
            Ev::PairCodeWaiter { conn, tx } => {
                self.code_waiters.insert(conn, tx);
            }
            Ev::AuthFailed { conn, err, user, peer } => {
                self.auth_tasks.remove(&conn);
                self.code_waiters.remove(&conn);
                let had_prompt = self.shared.update(|s| {
                    let had = s.pairing.as_ref().is_some_and(|p| p.conn() == conn);
                    if had {
                        s.pairing = None;
                    }
                    if user && matches!(s.conn, ConnStatus::Connecting(_)) {
                        s.conn = ConnStatus::Idle;
                    }
                    had
                });
                let paired_peer = peer.as_ref().and_then(|id| self.cfg.peer(id)).map(|p| p.name.clone());
                match (&err, paired_peer) {
                    (AuthError::NotPaired, Some(name)) => {
                        if let Some(id) = &peer {
                            // Try much less often until the user re-pairs.
                            self.last_dial.insert(id.clone(), Instant::now() + Duration::from_secs(30));
                        }
                        self.notice(format!(
                            "{name}에서 이 컴퓨터의 페어링이 삭제되었습니다. 목록에서 '페어링'을 눌러 다시 연결하세요."
                        ));
                    }
                    (AuthError::NotPaired, None) => {
                        if user {
                            self.notice("페어링되지 않은 컴퓨터입니다");
                        }
                    }
                    _ if user || had_prompt => self.notice(format!("연결 실패: {err}")),
                    _ => log::debug!("auth failed: {err}"),
                }
            }
            Ev::Authed(a) => self.on_authed(*a),
            Ev::Net { conn, msg } => {
                if self.session.as_ref().is_some_and(|s| s.conn == conn) {
                    if let Some(s) = self.session.as_mut() {
                        s.last_rx = Instant::now();
                    }
                    self.on_net(msg);
                }
            }
            Ev::Closed { conn, err } => {
                if self.session.as_ref().is_some_and(|s| s.conn == conn) {
                    self.close_session(err.map(|e| format!("연결 끊김 ({e})")));
                }
            }
            Ev::Clip(c) => self.on_local_clip(c),
        }
    }

    fn on_tick(&mut self) {
        let now = Instant::now();
        // Keepalive.
        let mut dead = false;
        if let Some(s) = self.session.as_mut() {
            if now.duration_since(s.last_rx) > DEAD_AFTER {
                dead = true;
            } else if now.duration_since(s.last_ping) >= PING_EVERY {
                s.last_ping = now;
                s.send(Msg::Ping);
            }
        }
        if dead {
            self.close_session(Some("응답이 없어 연결을 끊었습니다".into()));
        }
        // Screens can change (monitor plugged in, resolution…).
        if now.duration_since(self.desktop_at) > Duration::from_secs(2) {
            self.desktop = Desktop::new(self.platform.screens());
            self.desktop_at = now;
        }
        // Forget computers we haven't heard from.
        let before = self.discovered.len();
        self.discovered.retain(|_, d| now.duration_since(d.seen) < NEARBY_FOR);
        if before != self.discovered.len() {
            self.refresh_nearby();
        }
        // Permissions may be granted while we run.
        let status = self.platform.input_status();
        if self.shared.snapshot().input != status {
            self.shared.update(|s| s.input = status);
        }
        self.auto_connect(now);
    }

    fn auto_connect(&mut self, now: Instant) {
        if self.session.is_some() {
            return;
        }
        let me = self.cfg.device_id.clone();
        let mut targets = Vec::new();
        for p in &self.cfg.peers {
            if let Some(t) = self.last_dial.get(&p.id) {
                if *t > now || now.duration_since(*t) < RETRY_EVERY {
                    continue;
                }
            }
            let seen = self.discovered.get(&p.id);
            // The computer with the smaller id dials; the other one waits a
            // little so both don't dial at the same moment.
            if !is_lower(&me, &p.id) && now.duration_since(self.disconnected_since) < Duration::from_secs(6) {
                continue;
            }
            let addr = seen
                .map(|d| d.addr.to_string())
                .or_else(|| p.last_addr.clone());
            if let Some(a) = addr {
                targets.push((a, p.id.clone()));
            }
        }
        for (a, id) in targets {
            self.dial(a, Some(id), false);
        }
    }

    fn refresh_nearby(&self) {
        let mut list: Vec<Nearby> = self
            .discovered
            .values()
            .map(|d| Nearby {
                id: d.beacon.id.clone(),
                name: d.beacon.name.clone(),
                os: d.beacon.os,
                addr: d.addr.ip().to_string(),
                paired: self.cfg.peer(&d.beacon.id).is_some(),
            })
            .collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        self.shared.update(|s| s.nearby = list);
    }

    // ------------------------------------------------------------------
    // Local input

    fn can_cross(&self) -> bool {
        self.session.is_some()
            && self.settings().share_input
            && self.local_buttons.is_empty()
            && self.remote.is_none()
    }

    fn on_cap(&mut self, ev: CapEvent) {
        match self.mode {
            Mode::Local => self.on_cap_local(ev),
            Mode::Controlling { .. } => self.on_cap_controlling(ev),
        }
    }

    fn on_cap_local(&mut self, ev: CapEvent) {
        match ev {
            CapEvent::Move { x, y, .. } => {
                let p = self.desktop.clamp(self.last_pos, (x, y));
                self.last_pos = p;
                let side = self.settings().peer_side;
                let hit = self.desktop.edge_hit(side, p.0, p.1);
                if !self.armed {
                    if hit.is_none() {
                        self.armed = true;
                    }
                    return;
                }
                let Some(frac) = hit else {
                    self.edge_since = None;
                    return;
                };
                if !self.can_cross() {
                    self.edge_since = None;
                    return;
                }
                let delay = Duration::from_millis(self.settings().switch_delay_ms as u64);
                let now = Instant::now();
                let since = *self.edge_since.get_or_insert(now);
                if delay.is_zero() || now.duration_since(since) >= delay {
                    self.enter_controlling(frac, p);
                }
            }
            CapEvent::Button { button, down } => {
                if down {
                    self.local_buttons.insert(button);
                } else {
                    self.local_buttons.remove(&button);
                }
            }
            _ => {}
        }
    }

    fn enter_controlling(&mut self, frac: f64, at: (f64, f64)) {
        let Some(s) = &self.session else { return };
        let side = self.cfg.settings.peer_side;
        s.send(Msg::Enter {
            side: side.opposite(),
            pos: frac,
        });
        // Park the (hidden) cursor mid-screen: at the edge it could reveal
        // the Dock or a hot corner while it's away.
        let park = self.desktop.center_of_screen_at(at.0, at.1);
        self.platform.set_grab(true, park);
        self.mode = Mode::Controlling { exit_point: at };
        self.edge_since = None;
        self.sent_keys.clear();
        self.shift_l = false;
        self.shift_r = false;
        self.move_rest = (0.0, 0.0);
        self.shared.update(|st| st.controlling = true);
    }

    /// Ends control of the other computer and puts the pointer back here.
    /// `tell_peer`: send Release (we took it back ourselves).
    fn take_back(&mut self, tell_peer: bool) {
        let Mode::Controlling { exit_point } = self.mode else { return };
        if tell_peer {
            if let Some(s) = &self.session {
                s.send(Msg::Release);
            }
        }
        self.platform.set_grab(false, exit_point);
        let side = self.cfg.settings.peer_side;
        // Back where it left, a little inside the edge.
        let p = match self.desktop.edge_hit(side, exit_point.0, exit_point.1) {
            Some(f) => self.desktop.entry_point(side, f, 2.0),
            None => exit_point,
        };
        self.platform.warp(p.0, p.1);
        self.last_pos = p;
        self.armed = false;
        self.mode = Mode::Local;
        self.sent_keys.clear();
        self.shared.update(|st| st.controlling = false);
    }

    fn on_peer_leave(&mut self, frac: f64) {
        if !matches!(self.mode, Mode::Controlling { .. }) {
            return;
        }
        self.platform.set_grab(false, self.last_pos);
        let side = self.cfg.settings.peer_side;
        let p = self.desktop.entry_point(side, frac, 2.0);
        self.platform.warp(p.0, p.1);
        self.last_pos = p;
        self.armed = false;
        self.mode = Mode::Local;
        self.sent_keys.clear();
        self.shared.update(|st| st.controlling = false);
    }

    fn map_key(&self, code: u16) -> u16 {
        let peer_os = self.session.as_ref().map(|s| s.peer.os).unwrap_or(Os::Other);
        let mut c = code;
        if c == keys::CAPS_LOCK && self.settings().caps_lock_as_hangul {
            c = keys::LANG1;
        }
        if self.settings().swap_ctrl_cmd && peer_os != self.platform.os() && (peer_os == Os::Mac || self.platform.os() == Os::Mac) {
            c = keys::swap_ctrl_meta(c);
        }
        c
    }

    fn on_cap_controlling(&mut self, ev: CapEvent) {
        let Some(s) = &self.session else {
            self.take_back(false);
            return;
        };
        match ev {
            CapEvent::Move { dx, dy, .. } => {
                let speed = self.cfg.settings.pointer_speed as f64;
                let fx = dx * speed + self.move_rest.0;
                let fy = dy * speed + self.move_rest.1;
                // Send whole units; keep the remainder for next time.
                let (sx, sy) = (fx.trunc(), fy.trunc());
                self.move_rest = (fx - sx, fy - sy);
                if sx != 0.0 || sy != 0.0 {
                    s.send(Msg::Move {
                        dx: sx as f32,
                        dy: sy as f32,
                    });
                }
            }
            CapEvent::Button { button, down } => {
                if down {
                    self.local_buttons.insert(button);
                } else {
                    self.local_buttons.remove(&button);
                }
                s.send(Msg::Button { button, down });
            }
            CapEvent::Wheel { dx, dy } => s.send(Msg::Wheel { dx, dy }),
            CapEvent::Key { code, down } => {
                if code == keys::LEFT_SHIFT {
                    self.shift_l = down;
                }
                if code == keys::RIGHT_SHIFT {
                    self.shift_r = down;
                }
                // Both Shift keys together: take the pointer back.
                if self.shift_l && self.shift_r && down {
                    self.take_back(true);
                    return;
                }
                // Releases use the mapping chosen at press time.
                let mapped = if down {
                    let m = self.map_key(code);
                    self.sent_keys.insert(code, m);
                    m
                } else {
                    self.sent_keys.remove(&code).unwrap_or_else(|| self.map_key(code))
                };
                s.send(Msg::Key { code: mapped, down });
            }
        }
    }

    // ------------------------------------------------------------------
    // Messages from the other computer

    fn send(&self, m: Msg) {
        if let Some(s) = &self.session {
            s.send(m);
        }
    }

    fn release_remote(&mut self) {
        let Some(r) = self.remote.take() else { return };
        for code in r.keys {
            self.platform.inject(Inject::Key { code, down: false });
        }
        for button in r.buttons {
            self.platform.inject(Inject::Button {
                button,
                down: false,
                x: r.pos.0,
                y: r.pos.1,
            });
        }
        self.shared.update(|s| s.controlled = false);
    }

    fn on_net(&mut self, msg: Msg) {
        match msg {
            Msg::Ping => self.send(Msg::Pong),
            Msg::Pong => {}
            Msg::Enter { side, pos } => {
                if matches!(self.mode, Mode::Controlling { .. }) {
                    // Both pointers crossed at once; this side yields.
                    self.platform.set_grab(false, self.last_pos);
                    self.mode = Mode::Local;
                    self.shared.update(|st| st.controlling = false);
                }
                self.release_remote();
                self.desktop = Desktop::new(self.platform.screens());
                self.desktop_at = Instant::now();
                let p = self.desktop.entry_point(side, pos, 1.0);
                self.platform.wake_display();
                self.platform.inject(Inject::MoveTo {
                    x: p.0,
                    y: p.1,
                    dx: 0.0,
                    dy: 0.0,
                });
                self.remote = Some(Remote {
                    pos: p,
                    side,
                    keys: HashSet::new(),
                    buttons: HashSet::new(),
                });
                self.shared.update(|st| st.controlled = true);
            }
            Msg::Move { dx, dy } => {
                let Some(r) = self.remote.as_mut() else { return };
                let (np, exit) = self.desktop.step(r.pos, dx as f64, dy as f64, r.side);
                if let Some(f) = exit {
                    if r.buttons.is_empty() {
                        // The local pointer stays at the edge; it must move
                        // away before it can cross by itself.
                        self.last_pos = np;
                        self.armed = false;
                        self.release_remote();
                        self.send(Msg::Leave { pos: f });
                        return;
                    }
                }
                if np != r.pos {
                    r.pos = np;
                    self.platform.inject(Inject::MoveTo {
                        x: np.0,
                        y: np.1,
                        dx: dx as f64,
                        dy: dy as f64,
                    });
                }
            }
            Msg::Button { button, down } => {
                let Some(r) = self.remote.as_mut() else { return };
                if down {
                    r.buttons.insert(button);
                } else if !r.buttons.remove(&button) {
                    return;
                }
                let (x, y) = r.pos;
                self.platform.inject(Inject::Button { button, down, x, y });
            }
            Msg::Wheel { dx, dy } => {
                if self.remote.is_some() {
                    self.platform.inject(Inject::Wheel { dx, dy });
                }
            }
            Msg::Key { code, down } => {
                let Some(r) = self.remote.as_mut() else { return };
                if down {
                    r.keys.insert(code);
                } else if !r.keys.remove(&code) {
                    return;
                }
                self.platform.inject(Inject::Key { code, down });
            }
            Msg::Leave { pos } => self.on_peer_leave(pos),
            Msg::Release => self.release_remote(),
            Msg::ClipText(t) => {
                if self.settings().share_clipboard {
                    let _ = self.clip_tx.send(ClipContent::Text(t));
                }
            }
            Msg::BlobStart { id, kind, size } => self.on_blob_start(id, kind, size),
            Msg::BlobChunk { id, data } => self.on_blob_chunk(id, data),
            Msg::BlobEnd { id } => self.on_blob_end(id),
            Msg::BlobAbort { id } => {
                if let Some(IncomingBlob::File { transfer, name, .. }) = self.blobs.remove(&id) {
                    self.set_transfer(transfer, |t| t.state = TransferState::Failed("상대가 취소했습니다".into()));
                    self.log(format!("받기 취소됨: {name}"));
                }
            }
            // Handshake messages are not expected any more.
            Msg::Hello(_) | Msg::Known(_) | Msg::PairSpake(_) | Msg::PairConfirm(_) | Msg::PairFailed => {}
        }
    }

    fn set_transfer(&self, id: u64, f: impl FnOnce(&mut Transfer)) {
        self.shared.update(|s| {
            if let Some(t) = s.transfers.iter_mut().find(|t| t.id == id) {
                f(t);
            }
        });
    }

    fn on_blob_start(&mut self, id: u64, kind: BlobKind, size: u64) {
        match kind {
            BlobKind::ClipImage { width, height } => {
                if size as usize > MAX_IMAGE_BYTES || !self.settings().share_clipboard {
                    return;
                }
                self.blobs.insert(
                    id,
                    IncomingBlob::Image {
                        width,
                        height,
                        size,
                        data: Vec::with_capacity(size as usize),
                    },
                );
            }
            BlobKind::File { name } => {
                let name = files::sanitize_name(&name);
                let dir = self.download_dir();
                let transfer = rand::random::<u64>();
                self.shared.update(|s| {
                    s.transfers.push(Transfer {
                        id: transfer,
                        name: name.clone(),
                        size,
                        done: 0,
                        outgoing: false,
                        state: TransferState::Active,
                        path: None,
                    })
                });
                match IncomingFile::create(&dir, &name, size) {
                    Ok(file) => {
                        self.blobs.insert(id, IncomingBlob::File { file, transfer, name });
                    }
                    Err(e) => {
                        self.set_transfer(transfer, |t| t.state = TransferState::Failed(format!("{e:#}")));
                        self.log(format!("받기 실패: {name} ({e:#})"));
                    }
                }
            }
        }
    }

    fn on_blob_chunk(&mut self, id: u64, data: Vec<u8>) {
        let mut bad = false;
        let mut progress = None;
        match self.blobs.get_mut(&id) {
            Some(IncomingBlob::Image { size, data: buf, .. }) => {
                if buf.len() + data.len() > *size as usize {
                    bad = true;
                } else {
                    buf.extend_from_slice(&data);
                }
            }
            Some(IncomingBlob::File { file, transfer, .. }) => {
                let before = file.received;
                if !file.write(data) {
                    bad = true;
                } else if file.received / (512 * 1024) != before / (512 * 1024) {
                    progress = Some((*transfer, file.received));
                }
            }
            None => {}
        }
        if let Some((t, n)) = progress {
            self.set_transfer(t, |x| x.done = n);
        }
        if bad {
            if let Some(IncomingBlob::File { transfer, .. }) = self.blobs.remove(&id) {
                self.set_transfer(transfer, |t| t.state = TransferState::Failed("잘못된 데이터".into()));
            }
        }
    }

    fn on_blob_end(&mut self, id: u64) {
        match self.blobs.remove(&id) {
            Some(IncomingBlob::Image { width, height, data, size }) => {
                if data.len() as u64 == size && self.settings().share_clipboard {
                    let _ = self.clip_tx.send(ClipContent::Image {
                        width,
                        height,
                        png: data,
                    });
                }
            }
            Some(IncomingBlob::File { file, transfer, name }) => {
                let size = file.size;
                match file.finish() {
                    Ok(path) => {
                        self.set_transfer(transfer, |t| {
                            t.done = size;
                            t.state = TransferState::Done;
                            t.path = Some(path.clone());
                        });
                        self.log(format!("받음: {}", path.display()));
                    }
                    Err(e) => {
                        self.set_transfer(transfer, |t| t.state = TransferState::Failed(format!("{e:#}")));
                        self.log(format!("받기 실패: {name} ({e:#})"));
                    }
                }
            }
            None => {}
        }
    }

    // ------------------------------------------------------------------
    // Clipboard and files going out

    fn on_local_clip(&mut self, c: ClipContent) {
        if !self.settings().share_clipboard {
            return;
        }
        let Some(s) = &self.session else { return };
        let lo = s.lo.clone();
        match c {
            ClipContent::Text(t) => {
                tokio::spawn(async move {
                    let _ = lo.send(Msg::ClipText(t)).await;
                });
            }
            ClipContent::Image { width, height, png } => {
                let id = rand::random::<u64>();
                tokio::spawn(files::send_image(width, height, png, id, lo));
            }
        }
    }

    fn send_files(&mut self, paths: Vec<PathBuf>) {
        let Some(s) = &self.session else {
            self.notice("연결된 컴퓨터가 없습니다");
            return;
        };
        for path in paths {
            if path.is_dir() {
                self.log(format!("폴더는 아직 보낼 수 없습니다: {}", path.display()));
                continue;
            }
            let id = rand::random::<u64>();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            self.shared.update(|st| {
                st.transfers.push(Transfer {
                    id,
                    name,
                    size: 0,
                    done: 0,
                    outgoing: true,
                    state: TransferState::Active,
                    path: Some(path.clone()),
                })
            });
            tokio::spawn(files::send_file(path, id, s.lo.clone(), self.shared.clone()));
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn run_auth(
    ctx: AuthCtx,
    stream: TcpStream,
    addr: SocketAddr,
    conn: u64,
    initiator: bool,
    user: bool,
    peer: Option<String>,
    ev: UnboundedSender<Ev>,
) {
    let key = ctx.private_key.clone();
    let ch = match tokio::time::timeout(conn::HANDSHAKE_TIMEOUT, transport::handshake(stream, &key, initiator)).await {
        Ok(Ok(ch)) => ch,
        Ok(Err(e)) => {
            let _ = ev.send(Ev::AuthFailed {
                conn,
                err: AuthError::Other(e),
                user,
                peer,
            });
            return;
        }
        Err(_) => {
            let _ = ev.send(Ev::AuthFailed {
                conn,
                err: AuthError::Other(anyhow::anyhow!("암호화 연결 시간 초과")),
                user,
                peer,
            });
            return;
        }
    };
    let remote_key = hex::encode(&ch.remote_static);
    match conn::authenticate(ctx, ch, conn, user, ev.clone()).await {
        Ok((hello, channel, newly_paired)) => {
            let _ = ev.send(Ev::Authed(Box::new(Authed {
                conn,
                hello,
                channel,
                addr,
                remote_key,
                newly_paired,
            })));
        }
        Err(err) => {
            let _ = ev.send(Ev::AuthFailed { conn, err, user, peer });
        }
    }
}
