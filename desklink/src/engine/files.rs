//! File transfers: sending files to the other computer and saving files it
//! sends.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc;

use crate::proto::{BlobKind, Msg};

use super::state::{Shared, Transfer, TransferState};

pub const CHUNK: usize = 32 * 1024;

/// Makes a received file name safe to create: no paths, no reserved names.
pub fn sanitize_name(name: &str) -> String {
    let base = name
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or("")
        .trim();
    let mut out: String = base
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect();
    while out.ends_with('.') || out.ends_with(' ') {
        out.pop();
    }
    while out.starts_with('.') {
        out.remove(0);
    }
    if out.is_empty() {
        out = "file".into();
    }
    let stem = out.split('.').next().unwrap_or("").to_ascii_uppercase();
    let reserved = matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || ((stem.starts_with("COM") || stem.starts_with("LPT"))
            && stem.len() == 4
            && stem.as_bytes()[3].is_ascii_digit());
    if reserved {
        out.insert(0, '_');
    }
    if out.chars().count() > 200 {
        let ext: String = Path::new(&out)
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();
        let keep: String = out.chars().take(200 - ext.chars().count().min(50)).collect();
        out = format!("{keep}{}", ext.chars().take(50).collect::<String>());
    }
    out
}

/// `dir/name`, or `dir/name (2).ext` etc. if that exists.
pub fn unique_path(dir: &Path, name: &str) -> PathBuf {
    let first = dir.join(name);
    if !first.exists() {
        return first;
    }
    let p = Path::new(name);
    let stem = p.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let ext = p
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    for i in 2.. {
        let cand = dir.join(format!("{stem} ({i}){ext}"));
        if !cand.exists() {
            return cand;
        }
    }
    unreachable!()
}

/// Writes one incoming file on its own thread.
pub struct IncomingFile {
    tx: Option<std::sync::mpsc::Sender<Vec<u8>>>,
    pub size: u64,
    pub received: u64,
    done: Option<std::thread::JoinHandle<Result<PathBuf>>>,
}

impl IncomingFile {
    pub fn create(dir: &Path, name: &str, size: u64) -> Result<IncomingFile> {
        std::fs::create_dir_all(dir).with_context(|| format!("폴더를 만들 수 없습니다: {}", dir.display()))?;
        let final_path = unique_path(dir, name);
        let part = final_path.with_file_name(format!(
            ".{}.desklink-part",
            final_path.file_name().unwrap_or_default().to_string_lossy()
        ));
        let mut file = std::fs::File::create(&part)
            .with_context(|| format!("파일을 만들 수 없습니다: {}", part.display()))?;
        let (tx, rx) = std::sync::mpsc::channel::<Vec<u8>>();
        let done = std::thread::Builder::new()
            .name("file-writer".into())
            .spawn(move || -> Result<PathBuf> {
                let mut finished = false;
                let result = (|| -> Result<()> {
                    while let Ok(chunk) = rx.recv() {
                        if chunk.is_empty() {
                            finished = true;
                            break;
                        }
                        file.write_all(&chunk)?;
                    }
                    file.flush()?;
                    Ok(())
                })();
                drop(file);
                if result.is_err() || !finished {
                    let _ = std::fs::remove_file(&part);
                    result?;
                    anyhow::bail!("전송이 중단되었습니다");
                }
                // Don't clobber a file that appeared meanwhile.
                let target = if final_path.exists() {
                    unique_path(
                        final_path.parent().unwrap_or(Path::new(".")),
                        &final_path.file_name().unwrap_or_default().to_string_lossy(),
                    )
                } else {
                    final_path
                };
                std::fs::rename(&part, &target)?;
                Ok(target)
            })?;
        Ok(IncomingFile {
            tx: Some(tx),
            size,
            received: 0,
            done: Some(done),
        })
    }

    /// Returns false if the data is more than announced.
    pub fn write(&mut self, data: Vec<u8>) -> bool {
        if data.is_empty() {
            return true;
        }
        self.received += data.len() as u64;
        if self.received > self.size {
            return false;
        }
        if let Some(tx) = &self.tx {
            let _ = tx.send(data);
        }
        true
    }

    /// Completes the file; returns where it was saved.
    pub fn finish(mut self) -> Result<PathBuf> {
        if self.received != self.size {
            anyhow::bail!("파일 크기가 맞지 않습니다");
        }
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(Vec::new());
        }
        match self.done.take() {
            Some(h) => h.join().map_err(|_| anyhow::anyhow!("writer panicked"))?,
            None => anyhow::bail!("already finished"),
        }
    }
}

impl Drop for IncomingFile {
    fn drop(&mut self) {
        // Dropping the sender without the end marker deletes the partial file.
        self.tx.take();
    }
}

fn set_transfer(shared: &Shared, id: u64, f: impl FnOnce(&mut Transfer)) {
    shared.update(|s| {
        if let Some(t) = s.transfers.iter_mut().find(|t| t.id == id) {
            f(t);
        }
    });
}

/// Sends one file. Progress shows up in the shared state.
pub async fn send_file(path: PathBuf, id: u64, lo: mpsc::Sender<Msg>, shared: Arc<Shared>) {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "file".into());
    let result: Result<()> = async {
        let mut f = tokio::fs::File::open(&path).await?;
        let size = f.metadata().await?.len();
        set_transfer(&shared, id, |t| t.size = size);
        lo.send(Msg::BlobStart {
            id,
            kind: BlobKind::File { name: name.clone() },
            size,
        })
        .await
        .map_err(|_| anyhow::anyhow!("연결이 끊겼습니다"))?;
        let mut buf = vec![0u8; CHUNK];
        let mut sent: u64 = 0;
        let mut reported: u64 = 0;
        loop {
            let n = match f.read(&mut buf).await {
                Ok(n) => n,
                Err(e) => {
                    let _ = lo.send(Msg::BlobAbort { id }).await;
                    return Err(e.into());
                }
            };
            if n == 0 {
                break;
            }
            if sent + n as u64 > size {
                let _ = lo.send(Msg::BlobAbort { id }).await;
                anyhow::bail!("파일이 보내는 중에 바뀌었습니다");
            }
            lo.send(Msg::BlobChunk {
                id,
                data: buf[..n].to_vec(),
            })
            .await
            .map_err(|_| anyhow::anyhow!("연결이 끊겼습니다"))?;
            sent += n as u64;
            if sent - reported >= 512 * 1024 {
                reported = sent;
                set_transfer(&shared, id, |t| t.done = sent);
            }
        }
        if sent != size {
            let _ = lo.send(Msg::BlobAbort { id }).await;
            anyhow::bail!("파일이 보내는 중에 바뀌었습니다");
        }
        lo.send(Msg::BlobEnd { id })
            .await
            .map_err(|_| anyhow::anyhow!("연결이 끊겼습니다"))?;
        set_transfer(&shared, id, |t| t.done = sent);
        Ok(())
    }
    .await;
    match result {
        Ok(()) => {
            set_transfer(&shared, id, |t| t.state = TransferState::Done);
            shared.log(format!("보냄: {name}"));
        }
        Err(e) => {
            set_transfer(&shared, id, |t| t.state = TransferState::Failed(format!("{e:#}")));
            shared.log(format!("보내기 실패: {name} ({e:#})"));
        }
    }
}

/// Sends a clipboard picture as a blob.
pub async fn send_image(width: u32, height: u32, png: Vec<u8>, id: u64, lo: mpsc::Sender<Msg>) {
    let size = png.len() as u64;
    if lo
        .send(Msg::BlobStart {
            id,
            kind: BlobKind::ClipImage { width, height },
            size,
        })
        .await
        .is_err()
    {
        return;
    }
    for chunk in png.chunks(CHUNK) {
        if lo
            .send(Msg::BlobChunk {
                id,
                data: chunk.to_vec(),
            })
            .await
            .is_err()
        {
            return;
        }
    }
    let _ = lo.send(Msg::BlobEnd { id }).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(sanitize_name("../../etc/passwd"), "passwd");
        assert_eq!(sanitize_name("C:\\Windows\\evil.exe"), "evil.exe");
        assert_eq!(sanitize_name("a<b>:c?.txt"), "a_b__c_.txt");
        assert_eq!(sanitize_name("CON.txt"), "_CON.txt");
        assert_eq!(sanitize_name("..."), "file");
        assert_eq!(sanitize_name(".hidden"), "hidden");
        assert_eq!(sanitize_name("사진 1.png"), "사진 1.png");
        assert!(sanitize_name(&"x".repeat(500)).chars().count() <= 250);
    }

    #[test]
    fn incoming_file() {
        let dir = tempfile::tempdir().unwrap();
        let mut f = IncomingFile::create(dir.path(), "a.txt", 6).unwrap();
        assert!(f.write(b"abc".to_vec()));
        assert!(f.write(b"def".to_vec()));
        let p = f.finish().unwrap();
        assert_eq!(std::fs::read(&p).unwrap(), b"abcdef");
        // Second file with the same name gets a new name.
        let mut f = IncomingFile::create(dir.path(), "a.txt", 1).unwrap();
        f.write(b"x".to_vec());
        let p2 = f.finish().unwrap();
        assert_eq!(p2.file_name().unwrap(), "a (2).txt");
        // Aborted transfer leaves nothing behind.
        let mut f = IncomingFile::create(dir.path(), "b.txt", 10).unwrap();
        f.write(b"x".to_vec());
        drop(f);
        std::thread::sleep(std::time::Duration::from_millis(200));
        let names: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
            .collect();
        assert_eq!(names.len(), 2, "{names:?}");
    }
}
