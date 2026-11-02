//! In-memory platform used by tests and on unsupported systems.

use std::sync::Mutex;

use crate::geometry::Rect;
use crate::proto::Os;

use super::{Inject, InputStatus, Platform};

pub struct MockPlatform {
    pub os: Os,
    pub screens: Mutex<Vec<Rect>>,
    pub injected: Mutex<Vec<Inject>>,
    pub grabbed: Mutex<bool>,
    pub cursor: Mutex<(f64, f64)>,
    status: InputStatus,
}

impl MockPlatform {
    pub fn new(os: Os, screens: Vec<Rect>) -> MockPlatform {
        MockPlatform {
            os,
            screens: Mutex::new(screens),
            injected: Mutex::new(Vec::new()),
            grabbed: Mutex::new(false),
            cursor: Mutex::new((100.0, 100.0)),
            status: InputStatus::Ready,
        }
    }

    pub fn unsupported() -> MockPlatform {
        let mut p = MockPlatform::new(Os::current(), vec![Rect::new(0.0, 0.0, 1920.0, 1080.0)]);
        p.status = InputStatus::Unsupported;
        p
    }

    pub fn take_injected(&self) -> Vec<Inject> {
        std::mem::take(&mut *self.injected.lock().unwrap())
    }

    pub fn is_grabbed(&self) -> bool {
        *self.grabbed.lock().unwrap()
    }
}

impl Platform for MockPlatform {
    fn os(&self) -> Os {
        self.os
    }
    fn screens(&self) -> Vec<Rect> {
        self.screens.lock().unwrap().clone()
    }
    fn cursor_pos(&self) -> (f64, f64) {
        *self.cursor.lock().unwrap()
    }
    fn set_grab(&self, grab: bool, _park: (f64, f64)) {
        *self.grabbed.lock().unwrap() = grab;
    }
    fn warp(&self, x: f64, y: f64) {
        *self.cursor.lock().unwrap() = (x, y);
    }
    fn inject(&self, ev: Inject) {
        if let Inject::MoveTo { x, y, .. } = ev {
            *self.cursor.lock().unwrap() = (x, y);
        }
        if self.status == InputStatus::Ready {
            self.injected.lock().unwrap().push(ev);
        }
    }
    fn input_status(&self) -> InputStatus {
        self.status.clone()
    }
}
