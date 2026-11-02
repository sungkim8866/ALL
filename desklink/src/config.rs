//! Persistent configuration: this computer's identity and keys, the paired
//! computer(s) and user settings.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::geometry::Side;
use crate::proto::Os;

pub const NOISE_PARAMS: &str = "Noise_XX_25519_ChaChaPoly_BLAKE2s";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub device_name: String,
    /// Where the other computer sits relative to this one.
    pub peer_side: Side,
    /// Move the pointer across the screen edge to control the other computer.
    pub share_input: bool,
    pub share_clipboard: bool,
    /// When the other computer runs a different OS, swap Ctrl and Cmd/Win so
    /// shortcuts (copy, paste, undo…) keep working with the same fingers.
    pub swap_ctrl_cmd: bool,
    /// Caps Lock on this keyboard acts as the 한/영 key on the other computer.
    pub caps_lock_as_hangul: bool,
    /// How long the pointer must push against the edge before it crosses.
    pub switch_delay_ms: u32,
    /// Speed multiplier for the pointer on the other computer.
    pub pointer_speed: f32,
    /// Closing the window keeps DeskLink running in the background.
    pub keep_running_on_close: bool,
    /// Received files go here (defaults to Downloads/DeskLink).
    pub download_dir: Option<PathBuf>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            device_name: default_device_name(),
            peer_side: if Os::current() == Os::Mac {
                Side::Left
            } else {
                Side::Right
            },
            share_input: true,
            share_clipboard: true,
            swap_ctrl_cmd: true,
            caps_lock_as_hangul: Os::current() == Os::Mac,
            switch_delay_ms: 0,
            pointer_speed: 1.0,
            keep_running_on_close: true,
            download_dir: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairedPeer {
    pub id: String,
    pub name: String,
    pub os: Os,
    /// Noise static public key (hex).
    pub public_key: String,
    /// Last address we reached it at ("ip:port").
    pub last_addr: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub device_id: String,
    pub private_key: String,
    pub public_key: String,
    #[serde(default)]
    pub settings: Settings,
    #[serde(default)]
    pub peers: Vec<PairedPeer>,
}

impl Config {
    pub fn path(dir: &Path) -> PathBuf {
        dir.join("config.json")
    }

    /// Loads the configuration, creating a fresh identity on first run.
    pub fn load_or_create(dir: &Path) -> Result<Config> {
        let path = Self::path(dir);
        if let Ok(text) = std::fs::read_to_string(&path) {
            match serde_json::from_str::<Config>(&text) {
                Ok(cfg) if !cfg.private_key.is_empty() => return Ok(cfg),
                Ok(_) => log::warn!("config has no key; creating a new identity"),
                Err(e) => {
                    log::warn!("config unreadable ({e}); keeping a backup and starting fresh");
                    let _ = std::fs::rename(&path, path.with_extension("json.bad"));
                }
            }
        }
        let builder = snow::Builder::new(NOISE_PARAMS.parse().expect("valid noise params"));
        let kp = builder.generate_keypair().context("generating key pair")?;
        let mut id_bytes = [0u8; 8];
        rand::Rng::fill(&mut rand::thread_rng(), &mut id_bytes);
        let cfg = Config {
            device_id: hex::encode(id_bytes),
            private_key: hex::encode(kp.private),
            public_key: hex::encode(kp.public),
            settings: Settings::default(),
            peers: Vec::new(),
        };
        cfg.save(dir)?;
        Ok(cfg)
    }

    pub fn save(&self, dir: &Path) -> Result<()> {
        std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
        let path = Self::path(dir);
        let tmp = path.with_extension("json.tmp");
        let text = serde_json::to_string_pretty(self)?;
        std::fs::write(&tmp, text).with_context(|| format!("writing {}", tmp.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o600));
        }
        std::fs::rename(&tmp, &path).with_context(|| format!("replacing {}", path.display()))?;
        Ok(())
    }

    pub fn private_key_bytes(&self) -> Vec<u8> {
        hex::decode(&self.private_key).unwrap_or_default()
    }

    pub fn peer(&self, id: &str) -> Option<&PairedPeer> {
        self.peers.iter().find(|p| p.id == id)
    }

    pub fn upsert_peer(&mut self, peer: PairedPeer) {
        self.peers.retain(|p| p.id != peer.id);
        self.peers.push(peer);
    }
}

pub fn default_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("DeskLink")
}

pub fn default_download_dir() -> PathBuf {
    dirs::download_dir()
        .or_else(|| dirs::home_dir().map(|h| h.join("Downloads")))
        .unwrap_or_else(|| PathBuf::from("."))
        .join("DeskLink")
}

fn default_device_name() -> String {
    let from_env = std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .ok()
        .filter(|s| !s.trim().is_empty());
    if let Some(name) = from_env {
        return name;
    }
    #[cfg(unix)]
    {
        let mut buf = [0u8; 256];
        // SAFETY: buf is valid for writes of buf.len() bytes.
        let rc = unsafe { libc_gethostname(buf.as_mut_ptr(), buf.len()) };
        if rc == 0 {
            let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
            let name = String::from_utf8_lossy(&buf[..end]).to_string();
            let name = name.trim_end_matches(".local").to_string();
            if !name.is_empty() {
                return name;
            }
        }
    }
    match Os::current() {
        Os::Mac => "Mac".into(),
        Os::Windows => "Windows PC".into(),
        _ => "Computer".into(),
    }
}

#[cfg(unix)]
extern "C" {
    #[link_name = "gethostname"]
    fn libc_gethostname(name: *mut u8, len: usize) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_reload() {
        let dir = tempfile::tempdir().unwrap();
        let mut a = Config::load_or_create(dir.path()).unwrap();
        assert_eq!(a.private_key_bytes().len(), 32);
        a.settings.peer_side = Side::Top;
        a.save(dir.path()).unwrap();
        let b = Config::load_or_create(dir.path()).unwrap();
        assert_eq!(a.device_id, b.device_id);
        assert_eq!(b.settings.peer_side, Side::Top);
    }
}
