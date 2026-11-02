//! macOS: a Quartz event tap captures (and, while the pointer is on the other
//! computer, swallows) local input; CGEventPost injects input from the other
//! computer. Needs the Accessibility permission.

#![allow(non_upper_case_globals, non_snake_case)]

use std::collections::HashSet;
use std::ffi::{c_char, c_void, CStr};
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use tokio::sync::mpsc::UnboundedSender;

use super::{CapEvent, Inject, InputStatus, KeyFilter, Platform};
use crate::geometry::Rect;
use crate::keys;
use crate::proto::{Button, Os};

// ---------------------------------------------------------------------------
// FFI

type CFTypeRef = *const c_void;
type CFStringRef = *const c_void;
type CFDictionaryRef = *const c_void;
type CFArrayRef = *const c_void;
type CFMachPortRef = *mut c_void;
type CFRunLoopSourceRef = *mut c_void;
type CFRunLoopRef = *mut c_void;
type CGEventRef = *mut c_void;
type CGEventSourceRef = *mut c_void;
type CGEventTapProxy = *mut c_void;
type CGDirectDisplayID = u32;
type TISInputSourceRef = *mut c_void;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct CGPoint {
    x: f64,
    y: f64,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct CGSize {
    width: f64,
    height: f64,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct CGRect {
    origin: CGPoint,
    size: CGSize,
}

type CGEventTapCallBack =
    unsafe extern "C" fn(proxy: CGEventTapProxy, etype: u32, event: CGEventRef, user_info: *mut c_void) -> CGEventRef;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventTapCreate(
        tap: u32,
        place: u32,
        options: u32,
        events_of_interest: u64,
        callback: CGEventTapCallBack,
        user_info: *mut c_void,
    ) -> CFMachPortRef;
    fn CGEventTapEnable(tap: CFMachPortRef, enable: bool);
    fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
    fn CGEventGetIntegerValueField(event: CGEventRef, field: u32) -> i64;
    fn CGEventGetDoubleValueField(event: CGEventRef, field: u32) -> f64;
    fn CGEventSetIntegerValueField(event: CGEventRef, field: u32, value: i64);
    fn CGEventSetDoubleValueField(event: CGEventRef, field: u32, value: f64);
    fn CGEventGetFlags(event: CGEventRef) -> u64;
    fn CGEventSetFlags(event: CGEventRef, flags: u64);
    fn CGEventCreate(source: CGEventSourceRef) -> CGEventRef;
    fn CGEventCreateMouseEvent(source: CGEventSourceRef, mouse_type: u32, pos: CGPoint, button: u32) -> CGEventRef;
    fn CGEventCreateKeyboardEvent(source: CGEventSourceRef, keycode: u16, down: bool) -> CGEventRef;
    fn CGEventCreateScrollWheelEvent2(
        source: CGEventSourceRef,
        units: u32,
        wheel_count: u32,
        wheel1: i32,
        wheel2: i32,
        wheel3: i32,
    ) -> CGEventRef;
    fn CGEventPost(tap: u32, event: CGEventRef);
    fn CGEventSourceCreate(state: i32) -> CGEventSourceRef;
    fn CGEventSourceSetLocalEventsSuppressionInterval(source: CGEventSourceRef, seconds: f64);
    fn CGWarpMouseCursorPosition(p: CGPoint) -> i32;
    fn CGAssociateMouseAndMouseCursorPosition(connected: u32) -> i32;
    fn CGGetActiveDisplayList(max: u32, displays: *mut CGDirectDisplayID, count: *mut u32) -> i32;
    fn CGDisplayBounds(display: CGDirectDisplayID) -> CGRect;
    fn CGMainDisplayID() -> CGDirectDisplayID;
    fn CGDisplayHideCursor(display: CGDirectDisplayID) -> i32;
    fn CGDisplayShowCursor(display: CGDirectDisplayID) -> i32;
    fn CGRequestListenEventAccess() -> bool;
    fn CGRequestPostEventAccess() -> bool;
}

#[repr(C)]
struct CFDictionaryCallBacks {
    _opaque: [u8; 0],
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopCommonModes: CFStringRef;
    static kCFBooleanTrue: CFTypeRef;
    static kCFTypeDictionaryKeyCallBacks: CFDictionaryCallBacks;
    static kCFTypeDictionaryValueCallBacks: CFDictionaryCallBacks;
    fn CFMachPortCreateRunLoopSource(alloc: CFTypeRef, port: CFMachPortRef, order: isize) -> CFRunLoopSourceRef;
    fn CFRunLoopGetCurrent() -> CFRunLoopRef;
    fn CFRunLoopAddSource(rl: CFRunLoopRef, source: CFRunLoopSourceRef, mode: CFStringRef);
    fn CFRunLoopRun();
    fn CFRelease(cf: CFTypeRef);
    fn CFStringCreateWithCString(alloc: CFTypeRef, s: *const c_char, encoding: u32) -> CFStringRef;
    fn CFDictionaryCreate(
        alloc: CFTypeRef,
        keys: *const CFTypeRef,
        values: *const CFTypeRef,
        count: isize,
        key_cb: *const CFDictionaryCallBacks,
        value_cb: *const CFDictionaryCallBacks,
    ) -> CFDictionaryRef;
    fn CFArrayGetCount(array: CFArrayRef) -> isize;
    fn CFArrayGetValueAtIndex(array: CFArrayRef, idx: isize) -> CFTypeRef;
    fn CFEqual(a: CFTypeRef, b: CFTypeRef) -> u8;
    fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
}

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    static kAXTrustedCheckOptionPrompt: CFStringRef;
    fn AXIsProcessTrustedWithOptions(options: CFDictionaryRef) -> u8;
}

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    static kTISPropertyInputSourceCategory: CFStringRef;
    static kTISCategoryKeyboardInputSource: CFStringRef;
    static kTISPropertyInputSourceIsSelectCapable: CFStringRef;
    static kTISPropertyInputSourceID: CFStringRef;
    fn TISCopyCurrentKeyboardInputSource() -> TISInputSourceRef;
    fn TISCreateInputSourceList(properties: CFDictionaryRef, include_all_installed: u8) -> CFArrayRef;
    fn TISSelectInputSource(source: TISInputSourceRef) -> i32;
    fn TISGetInputSourceProperty(source: TISInputSourceRef, key: CFStringRef) -> CFTypeRef;
}

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOPMAssertionDeclareUserActivity(name: CFStringRef, user_type: u32, id: *mut u32) -> i32;
}

#[repr(C)]
struct DispatchQueue {
    _opaque: [u8; 0],
}

extern "C" {
    static _dispatch_main_q: DispatchQueue;
    fn dispatch_async_f(queue: *const DispatchQueue, context: *mut c_void, work: extern "C" fn(*mut c_void));
}

/// Looks up an optional (private or deprecated) symbol at run time.
fn dlsym(name: &CStr) -> *mut c_void {
    // SAFETY: dlsym with RTLD_DEFAULT and a NUL-terminated name.
    unsafe { libc::dlsym(libc::RTLD_DEFAULT, name.as_ptr()) }
}

fn cfstr(s: &CStr) -> CFStringRef {
    // SAFETY: valid C string; kCFStringEncodingUTF8.
    unsafe { CFStringCreateWithCString(std::ptr::null(), s.as_ptr(), 0x0800_0100) }
}

// Event types.
const LEFT_DOWN: u32 = 1;
const LEFT_UP: u32 = 2;
const RIGHT_DOWN: u32 = 3;
const RIGHT_UP: u32 = 4;
const MOUSE_MOVED: u32 = 5;
const LEFT_DRAGGED: u32 = 6;
const RIGHT_DRAGGED: u32 = 7;
const KEY_DOWN: u32 = 10;
const KEY_UP: u32 = 11;
const FLAGS_CHANGED: u32 = 12;
const SCROLL_WHEEL: u32 = 22;
const OTHER_DOWN: u32 = 25;
const OTHER_UP: u32 = 26;
const OTHER_DRAGGED: u32 = 27;
const TAP_DISABLED_BY_TIMEOUT: u32 = 0xFFFF_FFFE;
const TAP_DISABLED_BY_USER_INPUT: u32 = 0xFFFF_FFFF;
// Trackpad gesture event types (swallowed while grabbed).
const GESTURE_TYPES: [u32; 8] = [18, 19, 20, 29, 30, 31, 32, 34];

// Event fields.
const F_CLICK_STATE: u32 = 1;
const F_BUTTON_NUMBER: u32 = 3;
const F_DELTA_X: u32 = 4;
const F_DELTA_Y: u32 = 5;
const F_AUTOREPEAT: u32 = 8;
const F_KEYCODE: u32 = 9;
const F_SCROLL_DELTA1: u32 = 11;
const F_SCROLL_DELTA2: u32 = 12;
const F_SOURCE_USER_DATA: u32 = 42;
const F_SCROLL_IS_CONTINUOUS: u32 = 88;
const F_SCROLL_FIXED1: u32 = 93;
const F_SCROLL_FIXED2: u32 = 94;
const F_SCROLL_POINT1: u32 = 96;
const F_SCROLL_POINT2: u32 = 97;

// Flags.
const FLAG_SHIFT: u64 = 0x0002_0000;
const FLAG_CONTROL: u64 = 0x0004_0000;
const FLAG_ALTERNATE: u64 = 0x0008_0000;
const FLAG_COMMAND: u64 = 0x0010_0000;
const FLAG_NUMERIC_PAD: u64 = 0x0020_0000;
const FLAG_SECONDARY_FN: u64 = 0x0080_0000;
const FLAG_NON_COALESCED: u64 = 0x0000_0100;
const DEV_LCTL: u64 = 0x0000_0001;
const DEV_LSHIFT: u64 = 0x0000_0002;
const DEV_RSHIFT: u64 = 0x0000_0004;
const DEV_LCMD: u64 = 0x0000_0008;
const DEV_RCMD: u64 = 0x0000_0010;
const DEV_LALT: u64 = 0x0000_0020;
const DEV_RALT: u64 = 0x0000_0040;
const DEV_RCTL: u64 = 0x0000_2000;

const HID_TAP: u32 = 0;
const HEAD_INSERT: u32 = 0;
const TAP_DEFAULT: u32 = 0;
const SOURCE_HID_STATE: i32 = 1;
const SCROLL_UNIT_PIXEL: u32 = 0;

/// Marks events we post so our own tap ignores them.
const MAGIC: i64 = 0x444C_4E4B;

// ---------------------------------------------------------------------------
// Capture

struct TapState {
    tx: UnboundedSender<CapEvent>,
    grab: AtomicBool,
    park: Mutex<CGPoint>,
    filter: Mutex<KeyFilter>,
    tap: AtomicPtr<c_void>,
    iso: bool,
    status: Mutex<InputStatus>,
}

static STATE: OnceLock<TapState> = OnceLock::new();

fn event_mask() -> u64 {
    let mut m = 0u64;
    for t in [
        LEFT_DOWN,
        LEFT_UP,
        RIGHT_DOWN,
        RIGHT_UP,
        MOUSE_MOVED,
        LEFT_DRAGGED,
        RIGHT_DRAGGED,
        KEY_DOWN,
        KEY_UP,
        FLAGS_CHANGED,
        SCROLL_WHEEL,
        OTHER_DOWN,
        OTHER_UP,
        OTHER_DRAGGED,
    ]
    .into_iter()
    .chain(GESTURE_TYPES)
    {
        m |= 1u64 << t;
    }
    m
}

/// For a modifier key code: the device-specific flag bit and generic flag.
fn modifier_bits(vk: u16) -> Option<(u64, u64)> {
    Some(match vk {
        0x38 => (DEV_LSHIFT, FLAG_SHIFT),
        0x3C => (DEV_RSHIFT, FLAG_SHIFT),
        0x3B => (DEV_LCTL, FLAG_CONTROL),
        0x3E => (DEV_RCTL, FLAG_CONTROL),
        0x3A => (DEV_LALT, FLAG_ALTERNATE),
        0x3D => (DEV_RALT, FLAG_ALTERNATE),
        0x37 => (DEV_LCMD, FLAG_COMMAND),
        0x36 => (DEV_RCMD, FLAG_COMMAND),
        _ => return None,
    })
}

fn send(st: &TapState, ev: CapEvent) {
    let _ = st.tx.send(ev);
}

unsafe extern "C" fn tap_callback(
    _proxy: CGEventTapProxy,
    etype: u32,
    event: CGEventRef,
    _info: *mut c_void,
) -> CGEventRef {
    let Some(st) = STATE.get() else { return event };
    if etype == TAP_DISABLED_BY_TIMEOUT || etype == TAP_DISABLED_BY_USER_INPUT {
        let tap = st.tap.load(Ordering::Relaxed);
        if !tap.is_null() {
            CGEventTapEnable(tap, true);
        }
        return event;
    }
    if event.is_null() || CGEventGetIntegerValueField(event, F_SOURCE_USER_DATA) == MAGIC {
        return event;
    }
    let grab = st.grab.load(Ordering::Relaxed);
    let swallow = std::ptr::null_mut();
    match etype {
        MOUSE_MOVED | LEFT_DRAGGED | RIGHT_DRAGGED | OTHER_DRAGGED => {
            let loc = CGEventGetLocation(event);
            let dx = CGEventGetDoubleValueField(event, F_DELTA_X);
            let dy = CGEventGetDoubleValueField(event, F_DELTA_Y);
            send(st, CapEvent::Move { x: loc.x, y: loc.y, dx, dy });
            if grab {
                // Keep the (hidden) cursor parked so it can't reach the Dock
                // or hot corners.
                let p = *st.park.lock().unwrap();
                CGWarpMouseCursorPosition(p);
                CGAssociateMouseAndMouseCursorPosition(1);
                return swallow;
            }
            event
        }
        LEFT_DOWN | LEFT_UP | RIGHT_DOWN | RIGHT_UP | OTHER_DOWN | OTHER_UP => {
            let button = match etype {
                LEFT_DOWN | LEFT_UP => Some(Button::Left),
                RIGHT_DOWN | RIGHT_UP => Some(Button::Right),
                _ => match CGEventGetIntegerValueField(event, F_BUTTON_NUMBER) {
                    2 => Some(Button::Middle),
                    3 => Some(Button::Back),
                    4 => Some(Button::Forward),
                    _ => None,
                },
            };
            if let Some(button) = button {
                let down = matches!(etype, LEFT_DOWN | RIGHT_DOWN | OTHER_DOWN);
                send(st, CapEvent::Button { button, down });
            }
            if grab {
                swallow
            } else {
                event
            }
        }
        SCROLL_WHEEL => {
            if grab {
                let continuous = CGEventGetIntegerValueField(event, F_SCROLL_IS_CONTINUOUS) != 0;
                // 120 units per wheel notch; trackpad pixels × 3.
                let (vy, vx) = if continuous {
                    (
                        CGEventGetIntegerValueField(event, F_SCROLL_POINT1) as f64 * 3.0,
                        CGEventGetIntegerValueField(event, F_SCROLL_POINT2) as f64 * 3.0,
                    )
                } else {
                    let f1 = CGEventGetDoubleValueField(event, F_SCROLL_FIXED1);
                    let f2 = CGEventGetDoubleValueField(event, F_SCROLL_FIXED2);
                    let (f1, f2) = if f1 == 0.0 && f2 == 0.0 {
                        (
                            CGEventGetIntegerValueField(event, F_SCROLL_DELTA1) as f64,
                            CGEventGetIntegerValueField(event, F_SCROLL_DELTA2) as f64,
                        )
                    } else {
                        (f1, f2)
                    };
                    (f1 * 120.0, f2 * 120.0)
                };
                let (dy, dx) = (vy.round() as i32, -(vx.round() as i32));
                if dx != 0 || dy != 0 {
                    send(st, CapEvent::Wheel { dx, dy });
                }
                return swallow;
            }
            event
        }
        KEY_DOWN | KEY_UP => {
            let vk = CGEventGetIntegerValueField(event, F_KEYCODE) as u16;
            match keys::from_mac(vk, st.iso) {
                Some(code) => {
                    let down = etype == KEY_DOWN;
                    let d = st.filter.lock().unwrap().on_key(code, down, grab);
                    if d.forward {
                        send(st, CapEvent::Key { code, down });
                    }
                    if d.suppress {
                        return swallow;
                    }
                    event
                }
                None => {
                    if grab {
                        swallow
                    } else {
                        event
                    }
                }
            }
        }
        FLAGS_CHANGED => {
            let vk = CGEventGetIntegerValueField(event, F_KEYCODE) as u16;
            let flags = CGEventGetFlags(event);
            if vk == 0x39 {
                // Caps Lock only reports toggles: send a press and release.
                if grab {
                    send(st, CapEvent::Key { code: keys::CAPS_LOCK, down: true });
                    send(st, CapEvent::Key { code: keys::CAPS_LOCK, down: false });
                    return swallow;
                }
                return event;
            }
            let Some((dev, generic)) = modifier_bits(vk) else {
                // Fn and friends.
                return if grab { swallow } else { event };
            };
            // Prefer the left/right-specific bits; some keyboards only set
            // the generic ones.
            let device_bits = flags & 0xFFFF & !FLAG_NON_COALESCED;
            let down = if device_bits != 0 {
                flags & dev != 0
            } else {
                flags & generic != 0
            };
            let Some(code) = keys::from_mac(vk, st.iso) else { return event };
            let d = st.filter.lock().unwrap().on_key(code, down, grab);
            if d.forward {
                send(st, CapEvent::Key { code, down });
            }
            if d.suppress {
                swallow
            } else {
                event
            }
        }
        t if GESTURE_TYPES.contains(&t) => {
            if grab {
                swallow
            } else {
                event
            }
        }
        _ => event,
    }
}

fn tap_thread() {
    let Some(st) = STATE.get() else { return };
    loop {
        // SAFETY: creating an event tap with a valid callback.
        let tap = unsafe {
            CGEventTapCreate(
                HID_TAP,
                HEAD_INSERT,
                TAP_DEFAULT,
                event_mask(),
                tap_callback,
                std::ptr::null_mut(),
            )
        };
        if tap.is_null() {
            *st.status.lock().unwrap() = InputStatus::NeedsPermission;
            std::thread::sleep(Duration::from_secs(2));
            continue;
        }
        st.tap.store(tap, Ordering::SeqCst);
        *st.status.lock().unwrap() = InputStatus::Ready;
        // SAFETY: attach the tap to this thread's run loop and run it.
        unsafe {
            let src = CFMachPortCreateRunLoopSource(std::ptr::null(), tap, 0);
            CFRunLoopAddSource(CFRunLoopGetCurrent(), src, kCFRunLoopCommonModes);
            CGEventTapEnable(tap, true);
            CFRunLoopRun();
        }
        // CFRunLoopRun only returns if the source goes away.
        st.tap.store(std::ptr::null_mut(), Ordering::SeqCst);
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn keyboard_is_iso() -> bool {
    let lm = dlsym(c"LMGetKbdType");
    let kb = dlsym(c"KBGetLayoutType");
    if lm.is_null() || kb.is_null() {
        return false;
    }
    // SAFETY: the symbols have these C signatures (Carbon HIToolbox).
    unsafe {
        let lm: extern "C" fn() -> u8 = std::mem::transmute(lm);
        let kb: extern "C" fn(i16) -> u32 = std::mem::transmute(kb);
        kb(lm() as i16) == u32::from_be_bytes(*b"ISO ")
    }
}

/// Lets a background app hide the cursor (private API used by Synergy-like
/// tools). Harmless if missing.
fn allow_background_cursor_hiding() {
    let conn_fn = dlsym(c"_CGSDefaultConnection");
    let set_fn = dlsym(c"CGSSetConnectionProperty");
    if conn_fn.is_null() || set_fn.is_null() {
        return;
    }
    // SAFETY: the private functions have these signatures.
    unsafe {
        let conn_fn: extern "C" fn() -> i32 = std::mem::transmute(conn_fn);
        let set_fn: extern "C" fn(i32, i32, CFStringRef, CFTypeRef) -> i32 = std::mem::transmute(set_fn);
        let key = cfstr(c"SetsCursorInBackground");
        let cid = conn_fn();
        set_fn(cid, cid, key, kCFBooleanTrue);
        CFRelease(key);
    }
}

fn disable_local_event_suppression() {
    let f = dlsym(c"CGSetLocalEventsSuppressionInterval");
    if f.is_null() {
        return;
    }
    // SAFETY: deprecated public function with this signature.
    unsafe {
        let f: extern "C" fn(f64) -> i32 = std::mem::transmute(f);
        f(0.0);
    }
}

/// Keeps macOS from App-Napping us (which would make the pointer lag).
fn disable_app_nap() {
    use objc2_foundation::{NSActivityOptions, NSProcessInfo, NSString};
    let info = NSProcessInfo::processInfo();
    let reason = NSString::from_str("Sharing mouse and keyboard");
    let token = info.beginActivityWithOptions_reason(
        NSActivityOptions::UserInitiatedAllowingIdleSystemSleep | NSActivityOptions::LatencyCritical,
        &reason,
    );
    std::mem::forget(token);
}

// ---------------------------------------------------------------------------
// 한/영: switch to the previously used input source, like ⌃Space.

static PREV_SOURCE: Mutex<usize> = Mutex::new(0); // CFStringRef (retained) as usize

extern "C" fn toggle_input_source(_ctx: *mut c_void) {
    // SAFETY: Text Input Sources calls on the main thread with owned/borrowed
    // references handled per the Create/Copy/Get rules.
    unsafe {
        let keys = [kTISPropertyInputSourceCategory, kTISPropertyInputSourceIsSelectCapable];
        let vals = [kTISCategoryKeyboardInputSource, kCFBooleanTrue];
        let dict = CFDictionaryCreate(
            std::ptr::null(),
            keys.as_ptr(),
            vals.as_ptr(),
            2,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        );
        if dict.is_null() {
            return;
        }
        let list = TISCreateInputSourceList(dict, 0);
        CFRelease(dict);
        if list.is_null() {
            return;
        }
        let n = CFArrayGetCount(list);
        let cur = TISCopyCurrentKeyboardInputSource();
        let cur_id = if cur.is_null() {
            std::ptr::null()
        } else {
            TISGetInputSourceProperty(cur, kTISPropertyInputSourceID)
        };
        let id_at = |i: isize| TISGetInputSourceProperty(CFArrayGetValueAtIndex(list, i) as TISInputSourceRef, kTISPropertyInputSourceID);
        let cur_idx = (0..n).find(|&i| !cur_id.is_null() && CFEqual(id_at(i), cur_id) != 0);
        let mut prev = PREV_SOURCE.lock().unwrap();
        let prev_idx = if *prev != 0 {
            (0..n).find(|&i| CFEqual(id_at(i), *prev as CFTypeRef) != 0 && Some(i) != cur_idx)
        } else {
            None
        };
        let target = prev_idx.or_else(|| {
            if n > 1 {
                Some(cur_idx.map(|c| (c + 1) % n).unwrap_or(0))
            } else {
                None
            }
        });
        if let Some(t) = target {
            TISSelectInputSource(CFArrayGetValueAtIndex(list, t) as TISInputSourceRef);
            if !cur_id.is_null() {
                if *prev != 0 {
                    CFRelease(*prev as CFTypeRef);
                }
                *prev = CFRetain(cur_id) as usize;
            }
        }
        if !cur.is_null() {
            CFRelease(cur);
        }
        CFRelease(list);
    }
}

// ---------------------------------------------------------------------------
// Injection

struct Injector {
    source: CGEventSourceRef,
    keys_down: HashSet<u16>,
    mods: HashSet<u16>,
    buttons: HashSet<Button>,
    pos: CGPoint,
    last_click: Option<(Button, Instant, CGPoint, i64)>,
    wheel_rest: (f64, f64),
    last_wake: Option<Instant>,
}

// SAFETY: the event source is a CF object that may be used from any thread;
// access is serialized by the Mutex in MacPlatform.
unsafe impl Send for Injector {}

impl Injector {
    fn flags(&self, vk: Option<u16>) -> u64 {
        let mut f = FLAG_NON_COALESCED;
        for &m in &self.mods {
            f |= match m {
                keys::LEFT_SHIFT => FLAG_SHIFT | DEV_LSHIFT,
                keys::RIGHT_SHIFT => FLAG_SHIFT | DEV_RSHIFT,
                keys::LEFT_CTRL => FLAG_CONTROL | DEV_LCTL,
                keys::RIGHT_CTRL => FLAG_CONTROL | DEV_RCTL,
                keys::LEFT_ALT => FLAG_ALTERNATE | DEV_LALT,
                keys::RIGHT_ALT => FLAG_ALTERNATE | DEV_RALT,
                keys::LEFT_META => FLAG_COMMAND | DEV_LCMD,
                keys::RIGHT_META => FLAG_COMMAND | DEV_RCMD,
                _ => 0,
            };
        }
        if let Some(vk) = vk {
            match vk {
                // Arrows.
                0x7B..=0x7E => f |= FLAG_NUMERIC_PAD | FLAG_SECONDARY_FN,
                // Keypad.
                0x41 | 0x43 | 0x45 | 0x47 | 0x4B | 0x4C | 0x4E | 0x51..=0x59 | 0x5B | 0x5C => f |= FLAG_NUMERIC_PAD,
                // F-keys, Home/End/Page, forward delete, help.
                0x60..=0x6F | 0x71..=0x7A | 0x40 | 0x4F | 0x50 | 0x5A => f |= FLAG_SECONDARY_FN,
                _ => {}
            }
        }
        f
    }

    fn post(&self, ev: CGEventRef, vk: Option<u16>) {
        if ev.is_null() {
            return;
        }
        // SAFETY: ev is a valid event we own.
        unsafe {
            CGEventSetFlags(ev, self.flags(vk));
            CGEventSetIntegerValueField(ev, F_SOURCE_USER_DATA, MAGIC);
            CGEventPost(HID_TAP, ev);
            CFRelease(ev);
        }
    }

    fn mouse_type(&self) -> (u32, u32) {
        if self.buttons.contains(&Button::Left) {
            (LEFT_DRAGGED, 0)
        } else if self.buttons.contains(&Button::Right) {
            (RIGHT_DRAGGED, 1)
        } else if let Some(b) = [Button::Middle, Button::Back, Button::Forward]
            .into_iter()
            .find(|b| self.buttons.contains(b))
        {
            (OTHER_DRAGGED, other_number(b))
        } else {
            (MOUSE_MOVED, 0)
        }
    }

    fn inject(&mut self, ev: Inject) {
        match ev {
            Inject::MoveTo { x, y, dx, dy } => {
                self.pos = CGPoint { x, y };
                let (t, b) = self.mouse_type();
                // SAFETY: creating and filling an event we own.
                unsafe {
                    let e = CGEventCreateMouseEvent(self.source, t, self.pos, b);
                    if !e.is_null() {
                        CGEventSetIntegerValueField(e, F_DELTA_X, dx.round() as i64);
                        CGEventSetIntegerValueField(e, F_DELTA_Y, dy.round() as i64);
                        CGEventSetDoubleValueField(e, F_DELTA_X, dx);
                        CGEventSetDoubleValueField(e, F_DELTA_Y, dy);
                    }
                    self.post(e, None);
                }
            }
            Inject::Button { button, down, x, y } => {
                self.pos = CGPoint { x, y };
                let (t, num) = match (button, down) {
                    (Button::Left, true) => (LEFT_DOWN, 0),
                    (Button::Left, false) => (LEFT_UP, 0),
                    (Button::Right, true) => (RIGHT_DOWN, 1),
                    (Button::Right, false) => (RIGHT_UP, 1),
                    (b, true) => (OTHER_DOWN, other_number(b)),
                    (b, false) => (OTHER_UP, other_number(b)),
                };
                // Double clicks: macOS needs the click count on synthetic events.
                let count = if down {
                    let now = Instant::now();
                    let c = match self.last_click {
                        Some((lb, t0, p, c))
                            if lb == button
                                && now.duration_since(t0) < Duration::from_millis(500)
                                && (p.x - x).abs() < 5.0
                                && (p.y - y).abs() < 5.0 =>
                        {
                            c + 1
                        }
                        _ => 1,
                    };
                    self.last_click = Some((button, now, self.pos, c));
                    self.buttons.insert(button);
                    c
                } else {
                    self.buttons.remove(&button);
                    self.last_click.map(|l| l.3).unwrap_or(1)
                };
                // SAFETY: creating and filling an event we own.
                unsafe {
                    let e = CGEventCreateMouseEvent(self.source, t, self.pos, num);
                    if !e.is_null() {
                        CGEventSetIntegerValueField(e, F_CLICK_STATE, count);
                        CGEventSetIntegerValueField(e, F_BUTTON_NUMBER, num as i64);
                    }
                    self.post(e, None);
                }
            }
            Inject::Wheel { dx, dy } => {
                // 120 units (one notch) ≈ 40 pixels.
                let fy = dy as f64 / 3.0 + self.wheel_rest.1;
                let fx = -(dx as f64) / 3.0 + self.wheel_rest.0;
                let (py, px) = (fy.trunc(), fx.trunc());
                self.wheel_rest = (fx - px, fy - py);
                if px == 0.0 && py == 0.0 {
                    return;
                }
                // SAFETY: creating an event we own.
                unsafe {
                    let e = CGEventCreateScrollWheelEvent2(self.source, SCROLL_UNIT_PIXEL, 2, py as i32, px as i32, 0);
                    self.post(e, None);
                }
            }
            Inject::Key { code, down } => {
                if code == keys::LANG1 {
                    if down {
                        // SAFETY: schedule on the main queue; the function
                        // ignores its context.
                        unsafe {
                            dispatch_async_f(&_dispatch_main_q, std::ptr::null_mut(), toggle_input_source);
                        }
                    }
                    return;
                }
                let Some(vk) = keys::to_mac(code, keyboard_iso()) else { return };
                let repeat = down && self.keys_down.contains(&code);
                if down {
                    self.keys_down.insert(code);
                    if keys::is_modifier(code) {
                        self.mods.insert(code);
                    }
                } else {
                    self.keys_down.remove(&code);
                    self.mods.remove(&code);
                }
                // SAFETY: creating and filling an event we own.
                unsafe {
                    let e = CGEventCreateKeyboardEvent(self.source, vk, down);
                    if !e.is_null() && repeat {
                        CGEventSetIntegerValueField(e, F_AUTOREPEAT, 1);
                    }
                    self.post(e, Some(vk));
                }
            }
        }
    }
}

fn other_number(b: Button) -> u32 {
    match b {
        Button::Left => 0,
        Button::Right => 1,
        Button::Middle => 2,
        Button::Back => 3,
        Button::Forward => 4,
    }
}

fn keyboard_iso() -> bool {
    STATE.get().map(|s| s.iso).unwrap_or(false)
}

// ---------------------------------------------------------------------------

pub struct MacPlatform {
    injector: Mutex<Injector>,
    cursor_hidden: AtomicBool,
}

impl MacPlatform {
    pub fn start(tx: UnboundedSender<CapEvent>) -> Arc<dyn Platform> {
        disable_app_nap();
        allow_background_cursor_hiding();
        let _ = STATE.set(TapState {
            tx,
            grab: AtomicBool::new(false),
            park: Mutex::new(CGPoint { x: 0.0, y: 0.0 }),
            filter: Mutex::new(KeyFilter::default()),
            tap: AtomicPtr::new(std::ptr::null_mut()),
            iso: keyboard_is_iso(),
            status: Mutex::new(InputStatus::NeedsPermission),
        });
        // SAFETY: creating an event source.
        let source = unsafe {
            let s = CGEventSourceCreate(SOURCE_HID_STATE);
            if !s.is_null() {
                CGEventSourceSetLocalEventsSuppressionInterval(s, 0.0);
            }
            s
        };
        std::thread::Builder::new()
            .name("event-tap".into())
            .spawn(tap_thread)
            .expect("spawning event tap thread");
        let pos = current_location();
        Arc::new(MacPlatform {
            injector: Mutex::new(Injector {
                source,
                keys_down: HashSet::new(),
                mods: HashSet::new(),
                buttons: HashSet::new(),
                pos,
                last_click: None,
                wheel_rest: (0.0, 0.0),
                last_wake: None,
            }),
            cursor_hidden: AtomicBool::new(false),
        })
    }
}

fn current_location() -> CGPoint {
    // SAFETY: CGEventCreate(NULL) returns an event carrying the cursor position.
    unsafe {
        let e = CGEventCreate(std::ptr::null_mut());
        if e.is_null() {
            return CGPoint { x: 0.0, y: 0.0 };
        }
        let p = CGEventGetLocation(e);
        CFRelease(e);
        p
    }
}

fn warp_to(p: CGPoint) {
    // SAFETY: plain Quartz calls.
    unsafe {
        CGWarpMouseCursorPosition(p);
        // Re-associating right after a warp removes the short freeze macOS
        // otherwise applies to the mouse.
        CGAssociateMouseAndMouseCursorPosition(1);
    }
}

impl Platform for MacPlatform {
    fn os(&self) -> Os {
        Os::Mac
    }

    fn screens(&self) -> Vec<Rect> {
        let mut ids = [0u32; 16];
        let mut count = 0u32;
        // SAFETY: ids has room for 16 displays.
        unsafe {
            if CGGetActiveDisplayList(ids.len() as u32, ids.as_mut_ptr(), &mut count) != 0 {
                return Vec::new();
            }
        }
        ids[..count as usize]
            .iter()
            .map(|&id| {
                // SAFETY: id comes from the active display list.
                let b = unsafe { CGDisplayBounds(id) };
                Rect::new(b.origin.x, b.origin.y, b.size.width, b.size.height)
            })
            .collect()
    }

    fn cursor_pos(&self) -> (f64, f64) {
        let p = current_location();
        (p.x, p.y)
    }

    fn set_grab(&self, grab: bool, park: (f64, f64)) {
        let Some(st) = STATE.get() else { return };
        if grab {
            *st.park.lock().unwrap() = CGPoint { x: park.0, y: park.1 };
            disable_local_event_suppression();
            st.grab.store(true, Ordering::SeqCst);
            if !self.cursor_hidden.swap(true, Ordering::SeqCst) {
                // SAFETY: plain Quartz call.
                unsafe {
                    CGDisplayHideCursor(CGMainDisplayID());
                }
            }
        } else {
            st.grab.store(false, Ordering::SeqCst);
            if self.cursor_hidden.swap(false, Ordering::SeqCst) {
                // SAFETY: plain Quartz call.
                unsafe {
                    CGDisplayShowCursor(CGMainDisplayID());
                }
            }
        }
    }

    fn warp(&self, x: f64, y: f64) {
        warp_to(CGPoint { x, y });
    }

    fn inject(&self, ev: Inject) {
        self.injector.lock().unwrap().inject(ev);
    }

    fn input_status(&self) -> InputStatus {
        let s = STATE
            .get()
            .map(|s| s.status.lock().unwrap().clone())
            .unwrap_or(InputStatus::NeedsPermission);
        if s == InputStatus::Ready {
            return s;
        }
        // Once granted, the tap thread picks it up within a couple of
        // seconds (sometimes macOS wants the app restarted first).
        InputStatus::NeedsPermission
    }

    fn request_permissions(&self) {
        // SAFETY: building a one-entry options dictionary and querying.
        unsafe {
            let keys = [kAXTrustedCheckOptionPrompt];
            let vals = [kCFBooleanTrue];
            let dict = CFDictionaryCreate(
                std::ptr::null(),
                keys.as_ptr(),
                vals.as_ptr(),
                1,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            );
            AXIsProcessTrustedWithOptions(dict);
            if !dict.is_null() {
                CFRelease(dict);
            }
            CGRequestListenEventAccess();
            CGRequestPostEventAccess();
        }
    }

    fn wake_display(&self) {
        let mut inj = self.injector.lock().unwrap();
        if inj.last_wake.is_some_and(|t| t.elapsed() < Duration::from_secs(5)) {
            return;
        }
        inj.last_wake = Some(Instant::now());
        let name = cfstr(c"DeskLink remote input");
        let mut id = 0u32;
        // SAFETY: valid CFString and out pointer.
        unsafe {
            IOPMAssertionDeclareUserActivity(name, 0, &mut id);
            CFRelease(name);
        }
    }

    fn shutdown(&self) {
        self.set_grab(false, (0.0, 0.0));
    }
}
