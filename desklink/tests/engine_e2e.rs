//! Two engines with simulated input talk over real TCP on localhost.

use std::sync::Arc;
use std::time::{Duration, Instant};

use desklink::clipboard::{ClipContent, ClipboardBackend, MockClipboard};
use desklink::engine::{Command, ConnStatus, Engine, EngineHandle, EngineOptions, PairPrompt, TransferState, UiState};
use desklink::geometry::{Rect, Side};
use desklink::keys;
use desklink::platform::mock::MockPlatform;
use desklink::platform::{CapEvent, Inject};
use desklink::proto::Os;
use tempfile::TempDir;
use tokio::sync::mpsc::UnboundedSender;

struct Node {
    h: EngineHandle,
    plat: Arc<MockPlatform>,
    cap: UnboundedSender<CapEvent>,
    clip: MockClipboard,
    cfg_dir: TempDir,
    dl_dir: TempDir,
}

async fn node_in(os: Os, w: f64, h: f64, cfg_dir: TempDir, dl_dir: TempDir) -> Node {
    let plat = Arc::new(MockPlatform::new(os, vec![Rect::new(0.0, 0.0, w, h)]));
    let (cap, cap_rx) = tokio::sync::mpsc::unbounded_channel();
    let clip = MockClipboard::default();
    let c2 = clip.clone();
    let h = Engine::spawn(
        EngineOptions {
            config_dir: cfg_dir.path().to_path_buf(),
            listen_port: 0,
            discovery_port: None,
            download_dir: Some(dl_dir.path().to_path_buf()),
        },
        plat.clone(),
        cap_rx,
        Box::new(move || Some(Box::new(c2) as Box<dyn ClipboardBackend>)),
    )
    .await
    .unwrap();
    Node {
        h,
        plat,
        cap,
        clip,
        cfg_dir,
        dl_dir,
    }
}

async fn node(os: Os, w: f64, h: f64) -> Node {
    node_in(os, w, h, tempfile::tempdir().unwrap(), tempfile::tempdir().unwrap()).await
}

impl Node {
    fn state(&self) -> UiState {
        self.h.shared.snapshot()
    }
    fn port(&self) -> u16 {
        self.state().listen_port.unwrap()
    }
    fn set(&self, f: impl FnOnce(&mut desklink::config::Settings)) {
        let mut s = self.state().settings;
        f(&mut s);
        self.h.send(Command::UpdateSettings(s));
    }
    fn cap(&self, e: CapEvent) {
        self.cap.send(e).unwrap();
    }
}

async fn wait<T>(what: &str, mut f: impl FnMut() -> Option<T>) -> T {
    let start = Instant::now();
    loop {
        if let Some(v) = f() {
            return v;
        }
        if start.elapsed() > Duration::from_secs(20) {
            panic!("timed out waiting for {what}");
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

async fn pair(a: &Node, b: &Node) {
    a.h.send(Command::ConnectAddr(format!("127.0.0.1:{}", b.port())));
    let code = wait("code shown on B", || match b.state().pairing {
        Some(PairPrompt::ShowCode { code, .. }) => Some(code),
        _ => None,
    })
    .await;
    wait("code prompt on A", || match a.state().pairing {
        Some(PairPrompt::EnterCode { .. }) => Some(()),
        _ => None,
    })
    .await;
    a.h.send(Command::SubmitCode(format!("{} {}", &code[..3], &code[3..])));
    wait("both connected", || {
        let (sa, sb) = (a.state(), b.state());
        (matches!(sa.conn, ConnStatus::Connected { .. }) && matches!(sb.conn, ConnStatus::Connected { .. })).then_some(())
    })
    .await;
}

fn moves(injected: &[Inject]) -> Vec<(f64, f64)> {
    injected
        .iter()
        .filter_map(|e| match e {
            Inject::MoveTo { x, y, .. } => Some((*x, *y)),
            _ => None,
        })
        .collect()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn pair_control_clipboard_files() {
    let mac = node(Os::Mac, 1440.0, 900.0).await;
    let win = node(Os::Windows, 1920.0, 1080.0).await;
    // The PC sits to the right of the Mac.
    mac.set(|s| {
        s.peer_side = Side::Right;
        s.caps_lock_as_hangul = true;
        s.swap_ctrl_cmd = true;
    });
    win.set(|s| s.peer_side = Side::Left);

    pair(&mac, &win).await;
    let (ms, ws) = (mac.state(), win.state());
    assert_eq!(ms.peers.len(), 1);
    assert_eq!(ws.peers.len(), 1);
    assert_eq!(ms.peers[0].id, ws.my_id);
    assert_eq!(ws.peers[0].id, ms.my_id);
    assert!(ms.pairing.is_none() && ws.pairing.is_none());

    // ---- Pointer crosses the Mac's right edge.
    mac.cap(CapEvent::Move { x: 1200.0, y: 450.0, dx: 0.0, dy: 0.0 });
    mac.cap(CapEvent::Move { x: 1439.0, y: 450.0, dx: 5.0, dy: 0.0 });
    wait("mac grabs", || mac.plat.is_grabbed().then_some(())).await;
    wait("win shows pointer at its left edge", || {
        let m = moves(&win.plat.injected.lock().unwrap());
        (m.last() == Some(&(1.0, 540.0))).then_some(())
    })
    .await;
    assert!(win.state().controlled);
    assert!(mac.state().controlling);
    win.plat.take_injected();

    mac.cap(CapEvent::Move { x: 0.0, y: 0.0, dx: 10.0, dy: -4.0 });
    wait("pointer moves on win", || {
        let m = moves(&win.plat.injected.lock().unwrap());
        (m.last() == Some(&(11.0, 536.0))).then_some(())
    })
    .await;

    // ---- Cmd+C on the Mac keyboard is Ctrl+C on the PC; Caps Lock is 한/영.
    win.plat.take_injected();
    for (code, down) in [
        (keys::LEFT_META, true),
        (keys::C, true),
        (keys::C, false),
        (keys::LEFT_META, false),
        (keys::CAPS_LOCK, true),
        (keys::CAPS_LOCK, false),
    ] {
        mac.cap(CapEvent::Key { code, down });
    }
    mac.cap(CapEvent::Button { button: desklink::proto::Button::Left, down: true });
    mac.cap(CapEvent::Button { button: desklink::proto::Button::Left, down: false });
    mac.cap(CapEvent::Wheel { dx: 0, dy: 120 });
    let got = wait("keys on win", || {
        let inj = win.plat.injected.lock().unwrap().clone();
        (inj.len() >= 9).then_some(inj)
    })
    .await;
    assert_eq!(
        got,
        vec![
            Inject::Key { code: keys::LEFT_CTRL, down: true },
            Inject::Key { code: keys::C, down: true },
            Inject::Key { code: keys::C, down: false },
            Inject::Key { code: keys::LEFT_CTRL, down: false },
            Inject::Key { code: keys::LANG1, down: true },
            Inject::Key { code: keys::LANG1, down: false },
            Inject::Button { button: desklink::proto::Button::Left, down: true, x: 11.0, y: 536.0 },
            Inject::Button { button: desklink::proto::Button::Left, down: false, x: 11.0, y: 536.0 },
            Inject::Wheel { dx: 0, dy: 120 },
        ]
    );

    // ---- Moving back left past the PC's left edge returns the pointer.
    mac.cap(CapEvent::Move { x: 0.0, y: 0.0, dx: -50.0, dy: 0.0 });
    wait("mac releases grab", || (!mac.plat.is_grabbed()).then_some(())).await;
    let cursor = *mac.plat.cursor.lock().unwrap();
    assert_eq!(cursor.0, 1437.0, "pointer is back just inside the Mac's right edge");
    assert!((cursor.1 - 536.0 / 1080.0 * 900.0).abs() < 1.5, "{cursor:?}");
    wait("win no longer controlled", || (!win.state().controlled).then_some(())).await;

    // Not re-armed until the pointer moves away from the edge.
    mac.cap(CapEvent::Move { x: 1439.0, y: 450.0, dx: 2.0, dy: 0.0 });
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(!mac.plat.is_grabbed());
    mac.cap(CapEvent::Move { x: 1400.0, y: 450.0, dx: -39.0, dy: 0.0 });
    mac.cap(CapEvent::Move { x: 1439.0, y: 450.0, dx: 39.0, dy: 0.0 });
    wait("mac grabs again", || mac.plat.is_grabbed().then_some(())).await;

    // ---- Both Shift keys take the pointer back; held keys are released.
    mac.cap(CapEvent::Key { code: keys::A, down: true });
    mac.cap(CapEvent::Key { code: keys::LEFT_SHIFT, down: true });
    mac.cap(CapEvent::Key { code: keys::RIGHT_SHIFT, down: true });
    wait("hotkey releases grab", || (!mac.plat.is_grabbed()).then_some(())).await;
    wait("win released", || (!win.state().controlled).then_some(())).await;
    let inj = win.plat.take_injected();
    assert!(inj.contains(&Inject::Key { code: keys::A, down: false }), "{inj:?}");
    assert!(inj.contains(&Inject::Key { code: keys::LEFT_SHIFT, down: false }), "{inj:?}");

    // ---- Clipboard text, both directions, and a picture.
    mac.clip.set(ClipContent::Text("안녕하세요 DeskLink".into()));
    wait("text on win", || (win.clip.get() == Some(ClipContent::Text("안녕하세요 DeskLink".into()))).then_some(())).await;
    win.clip.set(ClipContent::Text("from windows".into()));
    wait("text on mac", || (mac.clip.get() == Some(ClipContent::Text("from windows".into()))).then_some(())).await;
    let png: Vec<u8> = (0..300_000u32).map(|i| (i % 251) as u8).collect();
    let img = ClipContent::Image { width: 10, height: 20, png };
    win.clip.set(img.clone());
    wait("picture on mac", || (mac.clip.get() == Some(img.clone())).then_some(())).await;

    // ---- Send a file.
    let src_dir = tempfile::tempdir().unwrap();
    let src = src_dir.path().join("보고서 final.bin");
    let data: Vec<u8> = (0..1_000_003u32).map(|i| (i * 7 % 256) as u8).collect();
    std::fs::write(&src, &data).unwrap();
    mac.h.send(Command::SendFiles(vec![src.clone()]));
    let saved = wait("file saved on win", || {
        win.state()
            .transfers
            .iter()
            .find(|t| t.state == TransferState::Done)
            .and_then(|t| t.path.clone())
    })
    .await;
    assert_eq!(saved.parent().unwrap(), win.dl_dir.path());
    assert_eq!(saved.file_name().unwrap().to_string_lossy(), "보고서 final.bin");
    assert_eq!(std::fs::read(&saved).unwrap(), data);
    wait("sender done", || {
        mac.state()
            .transfers
            .iter()
            .any(|t| t.outgoing && t.state == TransferState::Done && t.done == data.len() as u64)
            .then_some(())
    })
    .await;

    // ---- Disconnect, then reconnect without pairing again.
    mac.h.send(Command::Disconnect);
    wait("both idle", || {
        (mac.state().conn == ConnStatus::Idle && win.state().conn == ConnStatus::Idle).then_some(())
    })
    .await;
    let win_id = win.state().my_id;
    mac.h.send(Command::ConnectPeer(win_id));
    wait("reconnected", || {
        (matches!(mac.state().conn, ConnStatus::Connected { .. }) && matches!(win.state().conn, ConnStatus::Connected { .. })).then_some(())
    })
    .await;
    assert!(mac.state().pairing.is_none() && win.state().pairing.is_none());

    // ---- Restart the Mac side: it reconnects to the PC by itself.
    let Node { h, cfg_dir, dl_dir, .. } = mac;
    h.send(Command::Shutdown);
    wait("mac stopped", || h.shared.snapshot().quit.then_some(())).await;
    wait("win notices", || (win.state().conn == ConnStatus::Idle).then_some(())).await;
    let mac2 = node_in(Os::Mac, 1440.0, 900.0, cfg_dir, dl_dir).await;
    wait("auto reconnect after restart", || {
        (matches!(mac2.state().conn, ConnStatus::Connected { .. }) && matches!(win.state().conn, ConnStatus::Connected { .. })).then_some(())
    })
    .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn wrong_code_is_rejected() {
    let a = node(Os::Mac, 1440.0, 900.0).await;
    let b = node(Os::Windows, 1920.0, 1080.0).await;
    a.h.send(Command::ConnectAddr(format!("127.0.0.1:{}", b.port())));
    let code = wait("code", || match b.state().pairing {
        Some(PairPrompt::ShowCode { code, .. }) => Some(code),
        _ => None,
    })
    .await;
    wait("prompt", || a.state().pairing.is_some().then_some(())).await;
    let wrong = if code == "000000" { "111111" } else { "000000" };
    a.h.send(Command::SubmitCode(wrong.into()));
    wait("failure on A", || {
        let s = a.state();
        (s.pairing.is_none() && s.notice.as_deref().is_some_and(|n| n.contains("코드가 맞지 않습니다"))).then_some(())
    })
    .await;
    wait("failure on B", || {
        let s = b.state();
        (s.pairing.is_none() && s.notice.is_some()).then_some(())
    })
    .await;
    assert!(a.state().peers.is_empty());
    assert!(b.state().peers.is_empty());
    assert_eq!(a.state().conn, ConnStatus::Idle);
    assert_eq!(b.state().conn, ConnStatus::Idle);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn cancel_on_code_screen() {
    let a = node(Os::Mac, 1440.0, 900.0).await;
    let b = node(Os::Windows, 1920.0, 1080.0).await;
    a.h.send(Command::ConnectAddr(format!("127.0.0.1:{}", b.port())));
    wait("code", || matches!(b.state().pairing, Some(PairPrompt::ShowCode { .. })).then_some(())).await;
    wait("prompt", || a.state().pairing.is_some().then_some(())).await;
    // The computer showing the code cancels; the other one closes its prompt.
    b.h.send(Command::CancelPairing);
    wait("prompt closed on A", || a.state().pairing.is_none().then_some(())).await;
    assert!(a.state().peers.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn no_crossing_while_dragging_or_disconnected() {
    let a = node(Os::Mac, 1440.0, 900.0).await;
    a.set(|s| s.peer_side = Side::Right);
    tokio::time::sleep(Duration::from_millis(100)).await;
    // Not connected: nothing happens at the edge.
    a.cap(CapEvent::Move { x: 1439.0, y: 450.0, dx: 5.0, dy: 0.0 });
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(!a.plat.is_grabbed());

    let b = node(Os::Windows, 1920.0, 1080.0).await;
    pair(&a, &b).await;
    a.cap(CapEvent::Move { x: 1000.0, y: 450.0, dx: 0.0, dy: 0.0 });
    a.cap(CapEvent::Button { button: desklink::proto::Button::Left, down: true });
    a.cap(CapEvent::Move { x: 1439.0, y: 450.0, dx: 5.0, dy: 0.0 });
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert!(!a.plat.is_grabbed(), "dragging a window to the edge must not cross");
    a.cap(CapEvent::Button { button: desklink::proto::Button::Left, down: false });
    a.cap(CapEvent::Move { x: 1439.0, y: 450.0, dx: 5.0, dy: 0.0 });
    wait("crosses after release", || a.plat.is_grabbed().then_some(())).await;

    // Losing the connection gives the pointer back immediately.
    let Node { h, .. } = b;
    h.send(Command::Shutdown);
    wait("ungrabbed on disconnect", || (!a.plat.is_grabbed()).then_some(())).await;
}
