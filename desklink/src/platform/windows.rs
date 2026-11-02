//! Windows: low-level mouse/keyboard hooks for capture, SendInput for
//! injection.

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use tokio::sync::mpsc::UnboundedSender;
use windows_sys::Win32::Foundation::{BOOL, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{EnumDisplayMonitors, GetMonitorInfoW, HDC, HMONITOR, MONITORINFO};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::System::Power::{SetThreadExecutionState, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED};
use windows_sys::Win32::System::Threading::{GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_TIME_CRITICAL};
use windows_sys::Win32::UI::HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    MapVirtualKeyW, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, INPUT_MOUSE, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY,
    KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, MAPVK_VK_TO_VSC, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_HWHEEL,
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_MOVE,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_VIRTUALDESK, MOUSEEVENTF_WHEEL, MOUSEEVENTF_XDOWN,
    MOUSEEVENTF_XUP, MOUSEINPUT,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, CreateCursor, DispatchMessageW, GetCursorPos, GetMessageW, GetSystemMetrics, SetCursorPos,
    SetSystemCursor, SetWindowsHookExW, SystemParametersInfoW, TranslateMessage, HC_ACTION, KBDLLHOOKSTRUCT,
    LLKHF_EXTENDED, MSG, MSLLHOOKSTRUCT, OCR_APPSTARTING, OCR_CROSS, OCR_HAND, OCR_HELP, OCR_IBEAM, OCR_NO,
    OCR_NORMAL, OCR_SIZEALL, OCR_SIZENESW, OCR_SIZENS, OCR_SIZENWSE, OCR_SIZEWE, OCR_UP, OCR_WAIT,
    SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN, SM_YVIRTUALSCREEN, SPI_SETCURSORS, WH_KEYBOARD_LL,
    WH_MOUSE_LL, WM_KEYDOWN, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEHWHEEL,
    WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN, WM_XBUTTONDOWN, WM_XBUTTONUP,
};

use super::{CapEvent, Inject, InputStatus, KeyFilter, Platform};
use crate::geometry::{Desktop, Rect};
use crate::keys;
use crate::proto::{Button, Os};

/// Marks input we inject ourselves so the hooks ignore it.
const MAGIC: usize = 0x444C_4E4B; // "DLNK"

struct HookState {
    tx: UnboundedSender<CapEvent>,
    grab: AtomicBool,
    park_x: AtomicI32,
    park_y: AtomicI32,
    last: Mutex<(i32, i32)>,
    filter: Mutex<KeyFilter>,
    status: Mutex<InputStatus>,
}

static STATE: OnceLock<HookState> = OnceLock::new();
static CURSOR_HIDDEN: AtomicBool = AtomicBool::new(false);

pub struct WinPlatform {
    injected_buttons: Mutex<u8>,
}

impl WinPlatform {
    pub fn start(tx: UnboundedSender<CapEvent>) -> Arc<dyn Platform> {
        // Physical pixels everywhere, so hook, cursor and monitor coordinates
        // agree on high-DPI screens.
        // SAFETY: plain Win32 call; fails harmlessly if already set.
        unsafe {
            SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        }
        // A previous run that crashed while the pointer was away could have
        // left the cursor hidden.
        restore_cursors();
        let _ = STATE.set(HookState {
            tx,
            grab: AtomicBool::new(false),
            park_x: AtomicI32::new(0),
            park_y: AtomicI32::new(0),
            last: Mutex::new(cursor_point()),
            filter: Mutex::new(KeyFilter::default()),
            status: Mutex::new(InputStatus::Ready),
        });
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            restore_cursors();
            prev_hook(info);
        }));
        std::thread::Builder::new()
            .name("input-hooks".into())
            .spawn(hook_thread)
            .expect("spawning hook thread");
        Arc::new(WinPlatform {
            injected_buttons: Mutex::new(0),
        })
    }
}

fn cursor_point() -> (i32, i32) {
    let mut p = POINT { x: 0, y: 0 };
    // SAFETY: p is a valid out pointer.
    unsafe {
        GetCursorPos(&mut p);
    }
    (p.x, p.y)
}

fn hook_thread() {
    // SAFETY: standard hook installation and message loop on this thread.
    unsafe {
        SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_TIME_CRITICAL);
        let hinst = GetModuleHandleW(std::ptr::null());
        let mouse = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_proc), hinst, 0);
        let kbd = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_proc), hinst, 0);
        if mouse.is_null() || kbd.is_null() {
            let err = std::io::Error::last_os_error();
            if let Some(st) = STATE.get() {
                *st.status.lock().unwrap() = InputStatus::Failed(format!("입력 후크를 설치하지 못했습니다: {err}"));
            }
            return;
        }
        let mut msg: MSG = std::mem::zeroed();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn hiword(v: u32) -> u16 {
    (v >> 16) as u16
}

unsafe extern "system" fn mouse_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        if let Some(st) = STATE.get() {
            // SAFETY: for WH_MOUSE_LL, lparam points to an MSLLHOOKSTRUCT.
            let info = &*(lparam as *const MSLLHOOKSTRUCT);
            if info.dwExtraInfo != MAGIC {
                let grab = st.grab.load(Ordering::Relaxed);
                let msg = wparam as u32;
                let button = |b: Button, down: bool| CapEvent::Button { button: b, down };
                let ev = match msg {
                    WM_MOUSEMOVE => {
                        let (x, y) = (info.pt.x, info.pt.y);
                        if grab {
                            let px = st.park_x.load(Ordering::Relaxed);
                            let py = st.park_y.load(Ordering::Relaxed);
                            let (dx, dy) = (x - px, y - py);
                            if dx != 0 || dy != 0 {
                                let _ = st.tx.send(CapEvent::Move {
                                    x: x as f64,
                                    y: y as f64,
                                    dx: dx as f64,
                                    dy: dy as f64,
                                });
                            }
                            // Swallowed: the cursor stays parked.
                            return 1;
                        }
                        let mut last = st.last.lock().unwrap();
                        let (dx, dy) = (x - last.0, y - last.1);
                        *last = (x, y);
                        Some(CapEvent::Move {
                            x: x as f64,
                            y: y as f64,
                            dx: dx as f64,
                            dy: dy as f64,
                        })
                    }
                    WM_LBUTTONDOWN => Some(button(Button::Left, true)),
                    WM_LBUTTONUP => Some(button(Button::Left, false)),
                    WM_RBUTTONDOWN => Some(button(Button::Right, true)),
                    WM_RBUTTONUP => Some(button(Button::Right, false)),
                    WM_MBUTTONDOWN => Some(button(Button::Middle, true)),
                    WM_MBUTTONUP => Some(button(Button::Middle, false)),
                    WM_XBUTTONDOWN | WM_XBUTTONUP => {
                        let b = if hiword(info.mouseData) == 1 { Button::Back } else { Button::Forward };
                        Some(button(b, msg == WM_XBUTTONDOWN))
                    }
                    WM_MOUSEWHEEL => Some(CapEvent::Wheel {
                        dx: 0,
                        dy: hiword(info.mouseData) as i16 as i32,
                    }),
                    WM_MOUSEHWHEEL => Some(CapEvent::Wheel {
                        dx: hiword(info.mouseData) as i16 as i32,
                        dy: 0,
                    }),
                    _ => None,
                };
                if let Some(ev) = ev {
                    let _ = st.tx.send(ev);
                }
                if grab {
                    return 1;
                }
            }
        }
    }
    CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam)
}

unsafe extern "system" fn keyboard_proc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION as i32 {
        if let Some(st) = STATE.get() {
            // SAFETY: for WH_KEYBOARD_LL, lparam points to a KBDLLHOOKSTRUCT.
            let info = &*(lparam as *const KBDLLHOOKSTRUCT);
            if info.dwExtraInfo != MAGIC {
                let grab = st.grab.load(Ordering::Relaxed);
                let msg = wparam as u32;
                let down = msg == WM_KEYDOWN || msg == WM_SYSKEYDOWN;
                let extended = info.flags & LLKHF_EXTENDED != 0;
                match keys::from_windows(info.vkCode as u16, info.scanCode as u16, extended) {
                    Some(hid) => {
                        let d = st.filter.lock().unwrap().on_key(hid, down, grab);
                        if d.forward {
                            let _ = st.tx.send(CapEvent::Key { code: hid, down });
                        }
                        if d.suppress {
                            return 1;
                        }
                    }
                    None => {
                        if grab {
                            return 1;
                        }
                    }
                }
            }
        }
    }
    CallNextHookEx(std::ptr::null_mut(), code, wparam, lparam)
}

fn restore_cursors() {
    // SAFETY: reloads the user's cursor scheme from the registry.
    unsafe {
        SystemParametersInfoW(SPI_SETCURSORS, 0, std::ptr::null_mut(), 0);
    }
    CURSOR_HIDDEN.store(false, Ordering::SeqCst);
}

/// Replaces every system cursor with a transparent one while the pointer is
/// on the other computer.
fn hide_cursors() {
    if CURSOR_HIDDEN.swap(true, Ordering::SeqCst) {
        return;
    }
    let and_mask = [0xFFu8; 32 * 32 / 8];
    let xor_mask = [0u8; 32 * 32 / 8];
    // SAFETY: masks are the right size for a 32x32 monochrome cursor;
    // SetSystemCursor takes ownership of each created cursor.
    unsafe {
        let hinst = GetModuleHandleW(std::ptr::null());
        for id in [
            OCR_NORMAL,
            OCR_IBEAM,
            OCR_WAIT,
            OCR_CROSS,
            OCR_UP,
            OCR_SIZENWSE,
            OCR_SIZENESW,
            OCR_SIZEWE,
            OCR_SIZENS,
            OCR_SIZEALL,
            OCR_NO,
            OCR_HAND,
            OCR_APPSTARTING,
            OCR_HELP,
        ] {
            let cur = CreateCursor(
                hinst,
                0,
                0,
                32,
                32,
                and_mask.as_ptr() as *const _,
                xor_mask.as_ptr() as *const _,
            );
            if !cur.is_null() {
                SetSystemCursor(cur, id);
            }
        }
    }
}

unsafe extern "system" fn monitor_cb(hmon: HMONITOR, _hdc: HDC, _rc: *mut RECT, data: LPARAM) -> BOOL {
    // SAFETY: data is the &mut Vec<Rect> passed to EnumDisplayMonitors.
    let out = &mut *(data as *mut Vec<Rect>);
    let mut mi: MONITORINFO = std::mem::zeroed();
    mi.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
    if GetMonitorInfoW(hmon, &mut mi) != 0 {
        let r = mi.rcMonitor;
        out.push(Rect::new(
            r.left as f64,
            r.top as f64,
            (r.right - r.left) as f64,
            (r.bottom - r.top) as f64,
        ));
    }
    1
}

fn send_inputs(inputs: &[INPUT]) {
    if inputs.is_empty() {
        return;
    }
    // SAFETY: inputs is a valid slice of INPUT structures.
    unsafe {
        SendInput(inputs.len() as u32, inputs.as_ptr(), std::mem::size_of::<INPUT>() as i32);
    }
}

fn mouse_input(dx: i32, dy: i32, data: u32, flags: u32) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx,
                dy,
                mouseData: data,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: MAGIC,
            },
        },
    }
}

fn key_input(vk: u16, scan: u16, flags: u32) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: scan,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: MAGIC,
            },
        },
    }
}

impl Platform for WinPlatform {
    fn os(&self) -> Os {
        Os::Windows
    }

    fn screens(&self) -> Vec<Rect> {
        let mut out: Vec<Rect> = Vec::new();
        // SAFETY: the callback only writes into `out`, which outlives the call.
        unsafe {
            EnumDisplayMonitors(
                std::ptr::null_mut(),
                std::ptr::null(),
                Some(monitor_cb),
                &mut out as *mut Vec<Rect> as LPARAM,
            );
        }
        out
    }

    fn cursor_pos(&self) -> (f64, f64) {
        let (x, y) = cursor_point();
        (x as f64, y as f64)
    }

    fn set_grab(&self, grab: bool, park: (f64, f64)) {
        let Some(st) = STATE.get() else { return };
        if grab {
            // Park in the middle of the screen so every movement shows up as
            // an offset from the park point (edges would clamp it).
            let desktop = Desktop::new(self.screens());
            let (cx, cy) = desktop.center_of_screen_at(park.0, park.1);
            st.park_x.store(cx as i32, Ordering::Relaxed);
            st.park_y.store(cy as i32, Ordering::Relaxed);
            // SAFETY: plain Win32 call.
            unsafe {
                SetCursorPos(cx as i32, cy as i32);
            }
            st.grab.store(true, Ordering::SeqCst);
            hide_cursors();
        } else {
            st.grab.store(false, Ordering::SeqCst);
            restore_cursors();
        }
    }

    fn warp(&self, x: f64, y: f64) {
        // SAFETY: plain Win32 call.
        unsafe {
            SetCursorPos(x.round() as i32, y.round() as i32);
        }
        if let Some(st) = STATE.get() {
            *st.last.lock().unwrap() = (x.round() as i32, y.round() as i32);
        }
    }

    fn inject(&self, ev: Inject) {
        match ev {
            Inject::MoveTo { x, y, .. } => {
                // SAFETY: plain Win32 calls.
                let (vx, vy, vw, vh) = unsafe {
                    (
                        GetSystemMetrics(SM_XVIRTUALSCREEN),
                        GetSystemMetrics(SM_YVIRTUALSCREEN),
                        GetSystemMetrics(SM_CXVIRTUALSCREEN),
                        GetSystemMetrics(SM_CYVIRTUALSCREEN),
                    )
                };
                let nx = ((x - vx as f64) * 65535.0 / ((vw - 1).max(1) as f64)).round() as i32;
                let ny = ((y - vy as f64) * 65535.0 / ((vh - 1).max(1) as f64)).round() as i32;
                send_inputs(&[mouse_input(
                    nx,
                    ny,
                    0,
                    MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK,
                )]);
            }
            Inject::Button { button, down, .. } => {
                let (flags, data) = match (button, down) {
                    (Button::Left, true) => (MOUSEEVENTF_LEFTDOWN, 0),
                    (Button::Left, false) => (MOUSEEVENTF_LEFTUP, 0),
                    (Button::Right, true) => (MOUSEEVENTF_RIGHTDOWN, 0),
                    (Button::Right, false) => (MOUSEEVENTF_RIGHTUP, 0),
                    (Button::Middle, true) => (MOUSEEVENTF_MIDDLEDOWN, 0),
                    (Button::Middle, false) => (MOUSEEVENTF_MIDDLEUP, 0),
                    (Button::Back, true) => (MOUSEEVENTF_XDOWN, 1),
                    (Button::Back, false) => (MOUSEEVENTF_XUP, 1),
                    (Button::Forward, true) => (MOUSEEVENTF_XDOWN, 2),
                    (Button::Forward, false) => (MOUSEEVENTF_XUP, 2),
                };
                let bit = 1u8 << (button as u8);
                let mut held = self.injected_buttons.lock().unwrap();
                if down {
                    *held |= bit;
                } else {
                    *held &= !bit;
                }
                send_inputs(&[mouse_input(0, 0, data, flags)]);
            }
            Inject::Wheel { dx, dy } => {
                let mut v = Vec::new();
                if dy != 0 {
                    v.push(mouse_input(0, 0, dy as u32, MOUSEEVENTF_WHEEL));
                }
                if dx != 0 {
                    v.push(mouse_input(0, 0, dx as u32, MOUSEEVENTF_HWHEEL));
                }
                send_inputs(&v);
            }
            Inject::Key { code, down } => {
                let Some(k) = keys::to_windows(code) else { return };
                let mut flags = if down { 0 } else { KEYEVENTF_KEYUP };
                if k.extended {
                    flags |= KEYEVENTF_EXTENDEDKEY;
                }
                let input = if k.vk != 0 {
                    // SAFETY: plain Win32 call.
                    let scan = unsafe { MapVirtualKeyW(k.vk as u32, MAPVK_VK_TO_VSC) } as u16;
                    key_input(k.vk, scan, flags)
                } else {
                    key_input(0, k.scan, flags | KEYEVENTF_SCANCODE)
                };
                send_inputs(&[input]);
            }
        }
    }

    fn input_status(&self) -> InputStatus {
        STATE
            .get()
            .map(|s| s.status.lock().unwrap().clone())
            .unwrap_or(InputStatus::Failed("not started".into()))
    }

    fn wake_display(&self) {
        // SAFETY: plain Win32 call.
        unsafe {
            SetThreadExecutionState(ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED);
        }
    }

    fn shutdown(&self) {
        if let Some(st) = STATE.get() {
            st.grab.store(false, Ordering::SeqCst);
        }
        restore_cursors();
    }
}
