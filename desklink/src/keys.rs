//! Platform-neutral key codes.
//!
//! Keys travel over the wire as USB HID usage IDs (keyboard page 0x07) so that
//! a key is identified by its *physical position*. The receiving computer then
//! applies its own keyboard layout / input method, exactly as if the key had
//! been pressed on a keyboard plugged into it.
//!
//! A few keys that do not live on the keyboard page (media keys) use private
//! codes above 0x100.

#![allow(dead_code)]

pub const A: u16 = 0x04;
pub const B: u16 = 0x05;
pub const C: u16 = 0x06;
pub const D: u16 = 0x07;
pub const E: u16 = 0x08;
pub const F: u16 = 0x09;
pub const G: u16 = 0x0A;
pub const H: u16 = 0x0B;
pub const I: u16 = 0x0C;
pub const J: u16 = 0x0D;
pub const K: u16 = 0x0E;
pub const L: u16 = 0x0F;
pub const M: u16 = 0x10;
pub const N: u16 = 0x11;
pub const O: u16 = 0x12;
pub const P: u16 = 0x13;
pub const Q: u16 = 0x14;
pub const R: u16 = 0x15;
pub const S: u16 = 0x16;
pub const T: u16 = 0x17;
pub const U: u16 = 0x18;
pub const V: u16 = 0x19;
pub const W: u16 = 0x1A;
pub const X: u16 = 0x1B;
pub const Y: u16 = 0x1C;
pub const Z: u16 = 0x1D;
pub const N1: u16 = 0x1E;
pub const N2: u16 = 0x1F;
pub const N3: u16 = 0x20;
pub const N4: u16 = 0x21;
pub const N5: u16 = 0x22;
pub const N6: u16 = 0x23;
pub const N7: u16 = 0x24;
pub const N8: u16 = 0x25;
pub const N9: u16 = 0x26;
pub const N0: u16 = 0x27;
pub const ENTER: u16 = 0x28;
pub const ESCAPE: u16 = 0x29;
pub const BACKSPACE: u16 = 0x2A;
pub const TAB: u16 = 0x2B;
pub const SPACE: u16 = 0x2C;
pub const MINUS: u16 = 0x2D;
pub const EQUAL: u16 = 0x2E;
pub const LEFT_BRACKET: u16 = 0x2F;
pub const RIGHT_BRACKET: u16 = 0x30;
pub const BACKSLASH: u16 = 0x31;
pub const NON_US_HASH: u16 = 0x32;
pub const SEMICOLON: u16 = 0x33;
pub const QUOTE: u16 = 0x34;
pub const GRAVE: u16 = 0x35;
pub const COMMA: u16 = 0x36;
pub const PERIOD: u16 = 0x37;
pub const SLASH: u16 = 0x38;
pub const CAPS_LOCK: u16 = 0x39;
pub const F1: u16 = 0x3A;
pub const F2: u16 = 0x3B;
pub const F3: u16 = 0x3C;
pub const F4: u16 = 0x3D;
pub const F5: u16 = 0x3E;
pub const F6: u16 = 0x3F;
pub const F7: u16 = 0x40;
pub const F8: u16 = 0x41;
pub const F9: u16 = 0x42;
pub const F10: u16 = 0x43;
pub const F11: u16 = 0x44;
pub const F12: u16 = 0x45;
pub const PRINT_SCREEN: u16 = 0x46;
pub const SCROLL_LOCK: u16 = 0x47;
pub const PAUSE: u16 = 0x48;
pub const INSERT: u16 = 0x49;
pub const HOME: u16 = 0x4A;
pub const PAGE_UP: u16 = 0x4B;
pub const DELETE: u16 = 0x4C;
pub const END: u16 = 0x4D;
pub const PAGE_DOWN: u16 = 0x4E;
pub const RIGHT: u16 = 0x4F;
pub const LEFT: u16 = 0x50;
pub const DOWN: u16 = 0x51;
pub const UP: u16 = 0x52;
pub const NUM_LOCK: u16 = 0x53;
pub const KP_DIVIDE: u16 = 0x54;
pub const KP_MULTIPLY: u16 = 0x55;
pub const KP_MINUS: u16 = 0x56;
pub const KP_PLUS: u16 = 0x57;
pub const KP_ENTER: u16 = 0x58;
pub const KP1: u16 = 0x59;
pub const KP2: u16 = 0x5A;
pub const KP3: u16 = 0x5B;
pub const KP4: u16 = 0x5C;
pub const KP5: u16 = 0x5D;
pub const KP6: u16 = 0x5E;
pub const KP7: u16 = 0x5F;
pub const KP8: u16 = 0x60;
pub const KP9: u16 = 0x61;
pub const KP0: u16 = 0x62;
pub const KP_DECIMAL: u16 = 0x63;
pub const NON_US_BACKSLASH: u16 = 0x64;
pub const APPLICATION: u16 = 0x65;
pub const POWER: u16 = 0x66;
pub const KP_EQUAL: u16 = 0x67;
pub const F13: u16 = 0x68;
pub const F14: u16 = 0x69;
pub const F15: u16 = 0x6A;
pub const F16: u16 = 0x6B;
pub const F17: u16 = 0x6C;
pub const F18: u16 = 0x6D;
pub const F19: u16 = 0x6E;
pub const F20: u16 = 0x6F;
pub const F21: u16 = 0x70;
pub const F22: u16 = 0x71;
pub const F23: u16 = 0x72;
pub const F24: u16 = 0x73;
pub const HELP: u16 = 0x75;
pub const MUTE: u16 = 0x7F;
pub const VOLUME_UP: u16 = 0x80;
pub const VOLUME_DOWN: u16 = 0x81;
pub const KP_COMMA: u16 = 0x85;
pub const INTL1_RO: u16 = 0x87;
pub const INTL2_KANA: u16 = 0x88;
pub const INTL3_YEN: u16 = 0x89;
pub const INTL4_HENKAN: u16 = 0x8A;
pub const INTL5_MUHENKAN: u16 = 0x8B;
/// 한/영 (Hangul/English toggle). Also the JIS "かな" key on Apple keyboards.
pub const LANG1: u16 = 0x90;
/// 한자 (Hanja). Also the JIS "英数" key on Apple keyboards.
pub const LANG2: u16 = 0x91;
pub const LEFT_CTRL: u16 = 0xE0;
pub const LEFT_SHIFT: u16 = 0xE1;
pub const LEFT_ALT: u16 = 0xE2;
pub const LEFT_META: u16 = 0xE3;
pub const RIGHT_CTRL: u16 = 0xE4;
pub const RIGHT_SHIFT: u16 = 0xE5;
pub const RIGHT_ALT: u16 = 0xE6;
pub const RIGHT_META: u16 = 0xE7;

// Private codes for consumer-page keys.
pub const MEDIA_PLAY_PAUSE: u16 = 0x101;
pub const MEDIA_NEXT: u16 = 0x102;
pub const MEDIA_PREV: u16 = 0x103;
pub const MEDIA_STOP: u16 = 0x104;

pub fn is_modifier(code: u16) -> bool {
    (LEFT_CTRL..=RIGHT_META).contains(&code)
}

/// Swaps Ctrl and Cmd/Win. Used when a Mac drives a PC (or the reverse) so
/// that Cmd+C on the Mac keyboard becomes Ctrl+C on Windows and Ctrl+C on the
/// PC keyboard becomes Cmd+C on the Mac.
pub fn swap_ctrl_meta(code: u16) -> u16 {
    match code {
        LEFT_CTRL => LEFT_META,
        LEFT_META => LEFT_CTRL,
        RIGHT_CTRL => RIGHT_META,
        RIGHT_META => RIGHT_CTRL,
        other => other,
    }
}

/// macOS virtual key code (kVK_*) → HID usage.
///
/// `iso` selects the ISO keyboard quirk: on ISO Apple keyboards macOS reports
/// the key left of `1` as kVK_ISO_Section (0x0A) and the key right of the left
/// Shift as kVK_ANSI_Grave (0x32), i.e. swapped relative to their positions.
pub fn from_mac(vk: u16, iso: bool) -> Option<u16> {
    Some(match vk {
        0x00 => A,
        0x01 => S,
        0x02 => D,
        0x03 => F,
        0x04 => H,
        0x05 => G,
        0x06 => Z,
        0x07 => X,
        0x08 => C,
        0x09 => V,
        0x0A => {
            if iso {
                GRAVE
            } else {
                NON_US_BACKSLASH
            }
        }
        0x0B => B,
        0x0C => Q,
        0x0D => W,
        0x0E => E,
        0x0F => R,
        0x10 => Y,
        0x11 => T,
        0x12 => N1,
        0x13 => N2,
        0x14 => N3,
        0x15 => N4,
        0x16 => N6,
        0x17 => N5,
        0x18 => EQUAL,
        0x19 => N9,
        0x1A => N7,
        0x1B => MINUS,
        0x1C => N8,
        0x1D => N0,
        0x1E => RIGHT_BRACKET,
        0x1F => O,
        0x20 => U,
        0x21 => LEFT_BRACKET,
        0x22 => I,
        0x23 => P,
        0x24 => ENTER,
        0x25 => L,
        0x26 => J,
        0x27 => QUOTE,
        0x28 => K,
        0x29 => SEMICOLON,
        0x2A => BACKSLASH,
        0x2B => COMMA,
        0x2C => SLASH,
        0x2D => N,
        0x2E => M,
        0x2F => PERIOD,
        0x30 => TAB,
        0x31 => SPACE,
        0x32 => {
            if iso {
                NON_US_BACKSLASH
            } else {
                GRAVE
            }
        }
        0x33 => BACKSPACE,
        0x35 => ESCAPE,
        0x36 => RIGHT_META,
        0x37 => LEFT_META,
        0x38 => LEFT_SHIFT,
        0x39 => CAPS_LOCK,
        0x3A => LEFT_ALT,
        0x3B => LEFT_CTRL,
        0x3C => RIGHT_SHIFT,
        0x3D => RIGHT_ALT,
        0x3E => RIGHT_CTRL,
        0x40 => F17,
        0x41 => KP_DECIMAL,
        0x43 => KP_MULTIPLY,
        0x45 => KP_PLUS,
        0x47 => NUM_LOCK, // keypad Clear
        0x48 => VOLUME_UP,
        0x49 => VOLUME_DOWN,
        0x4A => MUTE,
        0x4B => KP_DIVIDE,
        0x4C => KP_ENTER,
        0x4E => KP_MINUS,
        0x4F => F18,
        0x50 => F19,
        0x51 => KP_EQUAL,
        0x52 => KP0,
        0x53 => KP1,
        0x54 => KP2,
        0x55 => KP3,
        0x56 => KP4,
        0x57 => KP5,
        0x58 => KP6,
        0x59 => KP7,
        0x5A => F20,
        0x5B => KP8,
        0x5C => KP9,
        0x5D => INTL3_YEN,
        0x5E => INTL1_RO,
        0x5F => KP_COMMA,
        0x60 => F5,
        0x61 => F6,
        0x62 => F7,
        0x63 => F3,
        0x64 => F8,
        0x65 => F9,
        0x66 => LANG2,
        0x67 => F11,
        0x68 => LANG1,
        0x69 => F13,
        0x6A => F16,
        0x6B => F14,
        0x6D => F10,
        0x6E => APPLICATION,
        0x6F => F12,
        0x71 => F15,
        0x72 => INSERT, // "Help" sits where Insert is on PC keyboards
        0x73 => HOME,
        0x74 => PAGE_UP,
        0x75 => DELETE,
        0x76 => F4,
        0x77 => END,
        0x78 => F2,
        0x79 => PAGE_DOWN,
        0x7A => F1,
        0x7B => LEFT,
        0x7C => RIGHT,
        0x7D => DOWN,
        0x7E => UP,
        _ => return None,
    })
}

/// HID usage → macOS virtual key code.
pub fn to_mac(code: u16, iso: bool) -> Option<u16> {
    // Exceptions where the reverse mapping is not unique.
    match code {
        GRAVE => return Some(if iso { 0x0A } else { 0x32 }),
        NON_US_BACKSLASH => return Some(if iso { 0x32 } else { 0x0A }),
        NUM_LOCK => return Some(0x47),
        INSERT | HELP => return Some(0x72),
        // No equivalent on a Mac keyboard; map to the nearest useful keys.
        PRINT_SCREEN => return Some(0x69), // F13
        SCROLL_LOCK => return Some(0x6B),  // F14
        PAUSE => return Some(0x71),        // F15
        _ => {}
    }
    (0u16..0x80).find(|&vk| from_mac(vk, iso) == Some(code))
}

/// A Windows key as seen by a low-level keyboard hook or sent with SendInput.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WinKey {
    /// Set-1 scan code (without the E0 prefix).
    pub scan: u16,
    /// The key has the E0 prefix ("extended").
    pub extended: bool,
    /// If non-zero the key must be sent by virtual-key code instead of scan
    /// code (keys whose scan codes are unreliable, e.g. 한/영, Pause, media).
    pub vk: u16,
}

const VK_HANGUL: u16 = 0x15;
const VK_HANJA: u16 = 0x19;
const VK_PAUSE: u16 = 0x13;
const VK_SNAPSHOT: u16 = 0x2C;
const VK_NUMLOCK: u16 = 0x90;
const VK_LWIN: u16 = 0x5B;
const VK_RWIN: u16 = 0x5C;
const VK_APPS: u16 = 0x5D;
const VK_VOLUME_MUTE: u16 = 0xAD;
const VK_VOLUME_DOWN: u16 = 0xAE;
const VK_VOLUME_UP: u16 = 0xAF;
const VK_MEDIA_NEXT_TRACK: u16 = 0xB0;
const VK_MEDIA_PREV_TRACK: u16 = 0xB1;
const VK_MEDIA_STOP: u16 = 0xB2;
const VK_MEDIA_PLAY_PAUSE: u16 = 0xB3;

/// Windows hook data → HID usage.
pub fn from_windows(vk: u16, scan: u16, extended: bool) -> Option<u16> {
    // Keys identified by virtual-key code first.
    match vk {
        VK_HANGUL => return Some(LANG1),
        VK_HANJA => return Some(LANG2),
        VK_PAUSE => return Some(PAUSE),
        VK_SNAPSHOT => return Some(PRINT_SCREEN),
        VK_NUMLOCK => return Some(NUM_LOCK),
        VK_LWIN => return Some(LEFT_META),
        VK_RWIN => return Some(RIGHT_META),
        VK_APPS => return Some(APPLICATION),
        VK_VOLUME_MUTE => return Some(MUTE),
        VK_VOLUME_DOWN => return Some(VOLUME_DOWN),
        VK_VOLUME_UP => return Some(VOLUME_UP),
        VK_MEDIA_NEXT_TRACK => return Some(MEDIA_NEXT),
        VK_MEDIA_PREV_TRACK => return Some(MEDIA_PREV),
        VK_MEDIA_STOP => return Some(MEDIA_STOP),
        VK_MEDIA_PLAY_PAUSE => return Some(MEDIA_PLAY_PAUSE),
        _ => {}
    }
    if extended {
        Some(match scan {
            0x1C => KP_ENTER,
            0x1D => RIGHT_CTRL,
            0x35 => KP_DIVIDE,
            0x37 => PRINT_SCREEN,
            0x38 => RIGHT_ALT,
            0x45 => NUM_LOCK,
            0x47 => HOME,
            0x48 => UP,
            0x49 => PAGE_UP,
            0x4B => LEFT,
            0x4D => RIGHT,
            0x4F => END,
            0x50 => DOWN,
            0x51 => PAGE_DOWN,
            0x52 => INSERT,
            0x53 => DELETE,
            0x5B => LEFT_META,
            0x5C => RIGHT_META,
            0x5D => APPLICATION,
            0x5E => POWER,
            0x71 => LANG2,
            0x72 => LANG1,
            _ => return None,
        })
    } else {
        Some(match scan {
            0x01 => ESCAPE,
            0x02 => N1,
            0x03 => N2,
            0x04 => N3,
            0x05 => N4,
            0x06 => N5,
            0x07 => N6,
            0x08 => N7,
            0x09 => N8,
            0x0A => N9,
            0x0B => N0,
            0x0C => MINUS,
            0x0D => EQUAL,
            0x0E => BACKSPACE,
            0x0F => TAB,
            0x10 => Q,
            0x11 => W,
            0x12 => E,
            0x13 => R,
            0x14 => T,
            0x15 => Y,
            0x16 => U,
            0x17 => I,
            0x18 => O,
            0x19 => P,
            0x1A => LEFT_BRACKET,
            0x1B => RIGHT_BRACKET,
            0x1C => ENTER,
            0x1D => LEFT_CTRL,
            0x1E => A,
            0x1F => S,
            0x20 => D,
            0x21 => F,
            0x22 => G,
            0x23 => H,
            0x24 => J,
            0x25 => K,
            0x26 => L,
            0x27 => SEMICOLON,
            0x28 => QUOTE,
            0x29 => GRAVE,
            0x2A => LEFT_SHIFT,
            0x2B => BACKSLASH,
            0x2C => Z,
            0x2D => X,
            0x2E => C,
            0x2F => V,
            0x30 => B,
            0x31 => N,
            0x32 => M,
            0x33 => COMMA,
            0x34 => PERIOD,
            0x35 => SLASH,
            0x36 => RIGHT_SHIFT,
            0x37 => KP_MULTIPLY,
            0x38 => LEFT_ALT,
            0x39 => SPACE,
            0x3A => CAPS_LOCK,
            0x3B => F1,
            0x3C => F2,
            0x3D => F3,
            0x3E => F4,
            0x3F => F5,
            0x40 => F6,
            0x41 => F7,
            0x42 => F8,
            0x43 => F9,
            0x44 => F10,
            0x45 => PAUSE,
            0x46 => SCROLL_LOCK,
            0x47 => KP7,
            0x48 => KP8,
            0x49 => KP9,
            0x4A => KP_MINUS,
            0x4B => KP4,
            0x4C => KP5,
            0x4D => KP6,
            0x4E => KP_PLUS,
            0x4F => KP1,
            0x50 => KP2,
            0x51 => KP3,
            0x52 => KP0,
            0x53 => KP_DECIMAL,
            0x56 => NON_US_BACKSLASH,
            0x57 => F11,
            0x58 => F12,
            0x59 => KP_EQUAL,
            0x64 => F13,
            0x65 => F14,
            0x66 => F15,
            0x67 => F16,
            0x68 => F17,
            0x69 => F18,
            0x6A => F19,
            0x6B => F20,
            0x6C => F21,
            0x6D => F22,
            0x6E => F23,
            0x70 => INTL2_KANA,
            0x73 => INTL1_RO,
            0x76 => F24,
            0x79 => INTL4_HENKAN,
            0x7B => INTL5_MUHENKAN,
            0x7D => INTL3_YEN,
            0x7E => KP_COMMA,
            0xF1 => LANG2,
            0xF2 => LANG1,
            _ => return None,
        })
    }
}

/// HID usage → what to send with SendInput on Windows.
pub fn to_windows(code: u16) -> Option<WinKey> {
    let by_vk = |vk: u16, extended: bool| {
        Some(WinKey {
            scan: 0,
            extended,
            vk,
        })
    };
    match code {
        LANG1 => return by_vk(VK_HANGUL, false),
        LANG2 => return by_vk(VK_HANJA, false),
        PAUSE => return by_vk(VK_PAUSE, false),
        PRINT_SCREEN => return by_vk(VK_SNAPSHOT, true),
        NUM_LOCK => return by_vk(VK_NUMLOCK, true),
        MUTE => return by_vk(VK_VOLUME_MUTE, true),
        VOLUME_DOWN => return by_vk(VK_VOLUME_DOWN, true),
        VOLUME_UP => return by_vk(VK_VOLUME_UP, true),
        MEDIA_NEXT => return by_vk(VK_MEDIA_NEXT_TRACK, true),
        MEDIA_PREV => return by_vk(VK_MEDIA_PREV_TRACK, true),
        MEDIA_STOP => return by_vk(VK_MEDIA_STOP, true),
        MEDIA_PLAY_PAUSE => return by_vk(VK_MEDIA_PLAY_PAUSE, true),
        HELP => return to_windows(INSERT),
        _ => {}
    }
    for extended in [false, true] {
        for scan in 1u16..0x100 {
            if from_windows(0, scan, extended) == Some(code) {
                return Some(WinKey {
                    scan,
                    extended,
                    vk: 0,
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_round_trip() {
        for iso in [false, true] {
            for vk in 0u16..0x80 {
                if let Some(code) = from_mac(vk, iso) {
                    let back = to_mac(code, iso).unwrap();
                    assert_eq!(
                        from_mac(back, iso),
                        Some(code),
                        "vk {vk:#x} iso {iso} code {code:#x}"
                    );
                }
            }
        }
    }

    #[test]
    fn windows_round_trip() {
        for extended in [false, true] {
            for scan in 1u16..0x100 {
                if let Some(code) = from_windows(0, scan, extended) {
                    let wk = to_windows(code).unwrap();
                    if wk.vk == 0 {
                        assert_eq!(from_windows(0, wk.scan, wk.extended), Some(code));
                    }
                }
            }
        }
    }

    #[test]
    fn every_mac_key_reaches_windows() {
        for vk in 0u16..0x80 {
            if let Some(code) = from_mac(vk, false) {
                assert!(to_windows(code).is_some(), "mac vk {vk:#x} → {code:#x}");
            }
        }
    }

    #[test]
    fn every_windows_key_reaches_mac() {
        for extended in [false, true] {
            for scan in 1u16..0x100 {
                if let Some(code) = from_windows(0, scan, extended) {
                    if code == POWER || code == INTL2_KANA || code == INTL4_HENKAN || code == INTL5_MUHENKAN || code >= F21 && code <= F24 {
                        continue; // no Mac equivalent
                    }
                    assert!(to_mac(code, false).is_some(), "win scan {scan:#x} ext {extended} → {code:#x}");
                }
            }
        }
    }

    #[test]
    fn letters_match() {
        // 'A' on both platforms.
        assert_eq!(from_mac(0x00, false), Some(A));
        assert_eq!(from_windows(0x41, 0x1E, false), Some(A));
        assert_eq!(to_windows(A).unwrap().scan, 0x1E);
        assert_eq!(to_mac(A, false), Some(0x00));
        // 한/영 from a Korean Windows keyboard (right Alt reports VK_HANGUL).
        assert_eq!(from_windows(VK_HANGUL, 0x38, true), Some(LANG1));
        assert_eq!(swap_ctrl_meta(LEFT_META), LEFT_CTRL);
    }
}
