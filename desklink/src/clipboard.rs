//! Clipboard watcher: notices local copies and applies copies from the other
//! computer. Runs on its own thread because clipboard APIs block.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{anyhow, Result};

/// Largest picture we send (encoded PNG).
pub const MAX_IMAGE_BYTES: usize = 16 * 1024 * 1024;
/// Largest text we send.
pub const MAX_TEXT_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq)]
pub enum ClipContent {
    Text(String),
    Image { width: u32, height: u32, png: Vec<u8> },
}

impl ClipContent {
    fn digest(&self) -> u64 {
        let mut h = DefaultHasher::new();
        match self {
            ClipContent::Text(t) => {
                0u8.hash(&mut h);
                t.hash(&mut h);
            }
            ClipContent::Image { width, height, png } => {
                1u8.hash(&mut h);
                width.hash(&mut h);
                height.hash(&mut h);
                png.hash(&mut h);
            }
        }
        h.finish()
    }
}

pub trait ClipboardBackend {
    /// A number that changes whenever the clipboard changes, if the OS has one.
    fn change_marker(&mut self) -> Option<u64>;
    /// The current contents come from a password manager (marked concealed).
    fn is_concealed(&mut self) -> bool {
        false
    }
    fn read(&mut self) -> Option<ClipContent>;
    fn write(&mut self, content: &ClipContent) -> Result<()>;
}

pub type BackendFactory = Box<dyn FnOnce() -> Option<Box<dyn ClipboardBackend>> + Send>;

/// Starts the watcher thread. Local changes are passed to `on_change`;
/// contents from the other computer arrive on `incoming`.
pub fn spawn(
    factory: BackendFactory,
    enabled: Arc<AtomicBool>,
    incoming: Receiver<ClipContent>,
    on_change: impl Fn(ClipContent) + Send + 'static,
) {
    std::thread::Builder::new()
        .name("clipboard".into())
        .spawn(move || {
            let mut backend = match factory() {
                Some(b) => b,
                None => {
                    log::warn!("clipboard unavailable");
                    // Keep draining so senders don't error out.
                    while incoming.recv().is_ok() {}
                    return;
                }
            };
            let mut last_marker = backend.change_marker();
            let mut last_digest: Option<u64> = if last_marker.is_none() {
                backend.read().map(|c| c.digest())
            } else {
                None
            };
            loop {
                match incoming.recv_timeout(Duration::from_millis(300)) {
                    Ok(content) => {
                        let digest = content.digest();
                        if let Err(e) = backend.write(&content) {
                            log::warn!("setting clipboard failed: {e:#}");
                        }
                        last_digest = Some(digest);
                        last_marker = backend.change_marker();
                        continue;
                    }
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => return,
                }
                let marker = backend.change_marker();
                if marker.is_some() {
                    if marker == last_marker {
                        continue;
                    }
                    last_marker = marker;
                }
                if !enabled.load(Ordering::Relaxed) {
                    continue;
                }
                if backend.is_concealed() {
                    log::info!("skipping a concealed (password) clipboard entry");
                    continue;
                }
                if let Some(content) = backend.read() {
                    let digest = content.digest();
                    if Some(digest) != last_digest {
                        last_digest = Some(digest);
                        on_change(content);
                    }
                }
            }
        })
        .expect("spawning clipboard thread");
}

// ---------------------------------------------------------------------------
// In-memory clipboard for tests.

#[derive(Clone, Default)]
pub struct MockClipboard {
    pub inner: Arc<Mutex<(u64, Option<ClipContent>)>>,
}

impl MockClipboard {
    pub fn set(&self, c: ClipContent) {
        let mut g = self.inner.lock().unwrap();
        g.0 += 1;
        g.1 = Some(c);
    }
    pub fn get(&self) -> Option<ClipContent> {
        self.inner.lock().unwrap().1.clone()
    }
}

impl ClipboardBackend for MockClipboard {
    fn change_marker(&mut self) -> Option<u64> {
        Some(self.inner.lock().unwrap().0)
    }
    fn read(&mut self) -> Option<ClipContent> {
        self.get()
    }
    fn write(&mut self, content: &ClipContent) -> Result<()> {
        self.set(content.clone());
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// The real clipboard.

pub struct SystemClipboard {
    cb: arboard::Clipboard,
}

impl SystemClipboard {
    pub fn new() -> Option<SystemClipboard> {
        match arboard::Clipboard::new() {
            Ok(cb) => Some(SystemClipboard { cb }),
            Err(e) => {
                log::warn!("cannot open clipboard: {e}");
                None
            }
        }
    }
}

pub fn system_factory() -> BackendFactory {
    Box::new(|| SystemClipboard::new().map(|c| Box::new(c) as Box<dyn ClipboardBackend>))
}

fn encode_png(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut out, width, height);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.set_compression(png::Compression::Fast);
        let mut w = enc.write_header()?;
        w.write_image_data(rgba)?;
    }
    Ok(out)
}

fn decode_png(data: &[u8]) -> Result<(u32, u32, Vec<u8>)> {
    let mut dec = png::Decoder::new(std::io::Cursor::new(data));
    dec.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = dec.read_info()?;
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    buf.truncate(info.buffer_size());
    let (w, h) = (info.width, info.height);
    let rgba = match info.color_type {
        png::ColorType::Rgba => buf,
        png::ColorType::Rgb => buf
            .chunks_exact(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect(),
        png::ColorType::GrayscaleAlpha => buf
            .chunks_exact(2)
            .flat_map(|p| [p[0], p[0], p[0], p[1]])
            .collect(),
        png::ColorType::Grayscale => buf.iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::Indexed => return Err(anyhow!("indexed PNG not expanded")),
    };
    Ok((w, h, rgba))
}

impl ClipboardBackend for SystemClipboard {
    fn change_marker(&mut self) -> Option<u64> {
        os::change_count()
    }

    fn is_concealed(&mut self) -> bool {
        os::is_concealed()
    }

    fn read(&mut self) -> Option<ClipContent> {
        if let Ok(text) = self.cb.get_text() {
            if !text.is_empty() {
                if text.len() > MAX_TEXT_BYTES {
                    log::info!("clipboard text too large to share");
                    return None;
                }
                return Some(ClipContent::Text(text));
            }
        }
        // Without a change counter we'd decode the picture every poll.
        os::change_count()?;
        let img = self.cb.get_image().ok()?;
        let (w, h) = (img.width as u32, img.height as u32);
        if w == 0 || h == 0 || img.bytes.len() != (w as usize) * (h as usize) * 4 {
            return None;
        }
        match encode_png(w, h, &img.bytes) {
            Ok(png) if png.len() <= MAX_IMAGE_BYTES => Some(ClipContent::Image { width: w, height: h, png }),
            Ok(_) => {
                log::info!("clipboard picture too large to share");
                None
            }
            Err(e) => {
                log::warn!("encoding clipboard picture: {e:#}");
                None
            }
        }
    }

    fn write(&mut self, content: &ClipContent) -> Result<()> {
        match content {
            ClipContent::Text(t) => self.cb.set_text(t.clone())?,
            ClipContent::Image { png, .. } => {
                let (w, h, rgba) = decode_png(png)?;
                self.cb.set_image(arboard::ImageData {
                    width: w as usize,
                    height: h as usize,
                    bytes: rgba.into(),
                })?;
            }
        }
        Ok(())
    }
}

#[cfg(target_os = "macos")]
mod os {
    use objc2_app_kit::NSPasteboard;

    pub fn change_count() -> Option<u64> {
        let pb = NSPasteboard::generalPasteboard();
        Some(pb.changeCount() as u64)
    }

    pub fn is_concealed() -> bool {
        let pb = NSPasteboard::generalPasteboard();
        let Some(types) = pb.types() else { return false };
        types.iter().any(|t| {
            let s = t.to_string();
            s == "org.nspasteboard.ConcealedType" || s == "org.nspasteboard.TransientType"
        })
    }
}

#[cfg(target_os = "windows")]
mod os {
    use windows_sys::Win32::System::DataExchange::{
        GetClipboardSequenceNumber, IsClipboardFormatAvailable, RegisterClipboardFormatW,
    };

    pub fn change_count() -> Option<u64> {
        // SAFETY: no arguments; always safe to call.
        Some(unsafe { GetClipboardSequenceNumber() } as u64)
    }

    pub fn is_concealed() -> bool {
        for name in [
            "ExcludeClipboardContentFromMonitorProcessing",
            "Clipboard Viewer Ignore",
        ] {
            let wide: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
            // SAFETY: `wide` is a NUL-terminated UTF-16 string.
            let fmt = unsafe { RegisterClipboardFormatW(wide.as_ptr()) };
            // SAFETY: plain query on a registered format id.
            if fmt != 0 && unsafe { IsClipboardFormatAvailable(fmt) } != 0 {
                return true;
            }
        }
        false
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
mod os {
    pub fn change_count() -> Option<u64> {
        None
    }
    pub fn is_concealed() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn png_roundtrip() {
        let rgba: Vec<u8> = (0..4 * 3 * 2).map(|i| i as u8).collect();
        let png = encode_png(3, 2, &rgba).unwrap();
        let (w, h, back) = decode_png(&png).unwrap();
        assert_eq!((w, h), (3, 2));
        assert_eq!(back, rgba);
    }

    #[test]
    fn watcher_reports_and_applies() {
        let mock = MockClipboard::default();
        let m2 = mock.clone();
        let (tx_in, rx_in) = std::sync::mpsc::channel();
        let (tx_out, rx_out) = std::sync::mpsc::channel();
        spawn(
            Box::new(move || Some(Box::new(m2) as Box<dyn ClipboardBackend>)),
            Arc::new(AtomicBool::new(true)),
            rx_in,
            move |c| tx_out.send(c).unwrap(),
        );
        // What was on the clipboard before start is not sent; wait for start.
        std::thread::sleep(Duration::from_millis(100));
        mock.set(ClipContent::Text("hello".into()));
        let got = rx_out.recv_timeout(Duration::from_secs(3)).unwrap();
        assert_eq!(got, ClipContent::Text("hello".into()));
        // Content from the other computer is applied and not echoed back.
        tx_in.send(ClipContent::Text("from peer".into())).unwrap();
        std::thread::sleep(Duration::from_millis(800));
        assert_eq!(mock.get(), Some(ClipContent::Text("from peer".into())));
        assert!(rx_out.try_recv().is_err());
    }
}
