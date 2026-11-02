//! Operating-system input: capturing the local mouse and keyboard, injecting
//! input that comes from the other computer, and screen geometry.

use std::collections::HashSet;
use std::sync::Arc;

use tokio::sync::mpsc::UnboundedSender;

use crate::geometry::Rect;
use crate::proto::{Button, Os};

#[cfg(target_os = "macos")]
pub mod mac;
#[cfg(target_os = "windows")]
pub mod windows;
pub mod mock;

/// Something the local user did.
#[derive(Clone, Debug, PartialEq)]
pub enum CapEvent {
    /// The pointer moved. `x`, `y` is the new global position (meaningful when
    /// not grabbed); `dx`, `dy` is the relative motion (used while grabbed).
    Move { x: f64, y: f64, dx: f64, dy: f64 },
    Button { button: Button, down: bool },
    /// 120 units per notch; positive `dy` = scroll up, positive `dx` = right.
    Wheel { dx: i32, dy: i32 },
    /// Only reported while grabbed. `code` is a HID usage (see `keys`).
    Key { code: u16, down: bool },
}

/// Input to synthesize on this computer.
#[derive(Clone, Debug, PartialEq)]
pub enum Inject {
    MoveTo { x: f64, y: f64, dx: f64, dy: f64 },
    Button { button: Button, down: bool, x: f64, y: f64 },
    Wheel { dx: i32, dy: i32 },
    Key { code: u16, down: bool },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputStatus {
    Ready,
    /// The OS has not granted the permissions needed (macOS Accessibility).
    NeedsPermission,
    /// Starting the input hooks failed for another reason.
    Failed(String),
    /// No input support on this platform (development builds).
    Unsupported,
}

pub trait Platform: Send + Sync {
    fn os(&self) -> Os;
    /// Display rectangles in global cursor coordinates.
    fn screens(&self) -> Vec<Rect>;
    fn cursor_pos(&self) -> (f64, f64);
    /// While grabbed, local input is hidden from this computer and reported
    /// as `CapEvent`s; the cursor is parked (and hidden) near `park`.
    fn set_grab(&self, grab: bool, park: (f64, f64));
    /// Moves the local cursor (used when the pointer comes back).
    fn warp(&self, x: f64, y: f64);
    fn inject(&self, ev: Inject);
    fn input_status(&self) -> InputStatus;
    /// Asks the OS for the input permissions (shows the system prompt).
    fn request_permissions(&self) {}
    /// Wakes the display if it's asleep (called when input arrives).
    fn wake_display(&self) {}
    /// Undo anything that must not outlive the process (hidden cursor…).
    fn shutdown(&self) {}
}

/// Creates the platform backend for this OS. Captured events go to `tx`.
pub fn create(tx: UnboundedSender<CapEvent>) -> Arc<dyn Platform> {
    #[cfg(target_os = "macos")]
    {
        mac::MacPlatform::start(tx)
    }
    #[cfg(target_os = "windows")]
    {
        windows::WinPlatform::start(tx)
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        drop(tx);
        Arc::new(mock::MockPlatform::unsupported())
    }
}

/// Decides what happens to a key event in the input hooks.
///
/// Keys that were already held when the grab started must have their release
/// delivered locally, otherwise this computer would think they're stuck.
#[derive(Default)]
pub struct KeyFilter {
    local_down: HashSet<u16>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyDecision {
    pub suppress: bool,
    pub forward: bool,
}

impl KeyFilter {
    pub fn on_key(&mut self, code: u16, down: bool, grabbing: bool) -> KeyDecision {
        if !grabbing {
            if down {
                self.local_down.insert(code);
            } else {
                self.local_down.remove(&code);
            }
            return KeyDecision {
                suppress: false,
                forward: false,
            };
        }
        if self.local_down.contains(&code) {
            if !down {
                self.local_down.remove(&code);
                return KeyDecision {
                    suppress: false,
                    forward: false,
                };
            }
            // Auto-repeat of a key held before the grab: swallow it.
            return KeyDecision {
                suppress: true,
                forward: false,
            };
        }
        KeyDecision {
            suppress: true,
            forward: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn held_key_released_locally() {
        let mut f = KeyFilter::default();
        f.on_key(1, true, false); // Shift held locally
        // Grab starts; repeat is swallowed, release goes to the local OS.
        assert_eq!(f.on_key(1, true, true), KeyDecision { suppress: true, forward: false });
        assert_eq!(f.on_key(1, false, true), KeyDecision { suppress: false, forward: false });
        // A fresh press while grabbed is forwarded.
        assert_eq!(f.on_key(1, true, true), KeyDecision { suppress: true, forward: true });
        assert_eq!(f.on_key(1, false, true), KeyDecision { suppress: true, forward: true });
    }
}
