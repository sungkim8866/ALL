// No console window on Windows.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use desklink::engine::{Command, Engine, EngineOptions};
use desklink::{clipboard, platform, ui};

struct FileLogger {
    file: Mutex<Option<std::fs::File>>,
}

impl log::Log for FileLogger {
    fn enabled(&self, m: &log::Metadata) -> bool {
        m.level() <= log::Level::Info || m.target().starts_with("desklink")
    }
    fn log(&self, r: &log::Record) {
        if !self.enabled(r.metadata()) {
            return;
        }
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let line = format!("{secs} {:<5} {}: {}\n", r.level(), r.target(), r.args());
        if let Some(f) = self.file.lock().unwrap().as_mut() {
            let _ = f.write_all(line.as_bytes());
        }
        if cfg!(debug_assertions) {
            eprint!("{line}");
        }
    }
    fn flush(&self) {}
}

fn init_logging(dir: &Path) {
    let _ = std::fs::create_dir_all(dir);
    let path = dir.join("desklink.log");
    // Keep the log small: start over when it grows past 2 MB.
    if std::fs::metadata(&path).map(|m| m.len() > 2_000_000).unwrap_or(false) {
        let _ = std::fs::rename(&path, dir.join("desklink.old.log"));
    }
    let file = std::fs::OpenOptions::new().create(true).append(true).open(&path).ok();
    let logger = Box::leak(Box::new(FileLogger { file: Mutex::new(file) }));
    let _ = log::set_logger(logger);
    log::set_max_level(log::LevelFilter::Debug);
}

fn main() {
    let mut opts = EngineOptions::default();
    let mut headless = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--headless" => headless = true,
            "--config-dir" => {
                if let Some(d) = args.next() {
                    opts.config_dir = PathBuf::from(d);
                }
            }
            "--port" => {
                if let Some(p) = args.next().and_then(|p| p.parse().ok()) {
                    opts.listen_port = p;
                }
            }
            "--version" => {
                println!("DeskLink {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => {}
        }
    }
    init_logging(&opts.config_dir);
    log::info!("DeskLink {} starting", env!("CARGO_PKG_VERSION"));

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("desklink")
        .enable_all()
        .build()
        .expect("tokio runtime");

    let (cap_tx, cap_rx) = tokio::sync::mpsc::unbounded_channel();
    let plat = platform::create(cap_tx);
    let handle = match rt.block_on(Engine::spawn(opts, plat.clone(), cap_rx, clipboard::system_factory())) {
        Ok(h) => h,
        Err(e) => {
            log::error!("start failed: {e:#}");
            eprintln!("DeskLink could not start: {e:#}");
            std::process::exit(1);
        }
    };

    if headless {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        });
        return;
    }

    if let Err(e) = ui::run(handle.clone(), plat.clone()) {
        log::error!("window failed: {e}");
    }
    handle.send(Command::Shutdown);
    plat.shutdown();
    // Give the engine a moment to say goodbye to the other computer.
    let deadline = std::time::Instant::now() + Duration::from_millis(800);
    while std::time::Instant::now() < deadline && !handle.shared.snapshot().quit {
        std::thread::sleep(Duration::from_millis(20));
    }
    log::info!("DeskLink stopped");
}
