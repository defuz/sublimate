use std::str::FromStr;

use core::command::{Command, ParseCommandError};
use core::context::{Context, ParseContextError};
use core::settings::{Settings, FromSettings};

use self::ParseHotkeyError::*;
use self::ParseHotkeyBindingError::*;

#[derive(Debug, Default)]
pub struct Keymap {
    pub bindings: Box<[HotkeyBinding]>
}

#[derive(Debug)]
pub struct HotkeyBinding {
    pub hotkeys: HotkeySequence,
    pub command: Command,
    pub context: Context
}

pub type HotkeySequence = Box<[Hotkey]>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Hotkey {
    key: Key,
    modifiers: Modifiers
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Key {
    ContextMenu,
    Tab,
    Enter,
    Escape,
    Backspace,
    Right,
    Left,
    Up,
    Down,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Pause,
    Clear,
    Sysreq,
    Break,
    /// "Browser" keys
    BrowserBack,
    BrowserForward,
    BrowserRefresh,
    BrowserStop,
    BrowserSearch,
    BrowserFavorites,
    BrowserHome,
    /// Keypad special keys
    KeypadPeriod,
    KeypadDivide,
    KeypadMultiply,
    KeypadMinus,
    KeypadPlus,
    KeypadEnter,
    /// Keypad digit keys
    Keypad(u8),
    /// F1, F2, ..., F20 keys
    F(u8),
    /// Single character keys
    Char(char),
}

bitflags! {
    flags Modifiers: u8 {
        const MODIFIER_NONE  = 0,
        const MODIFIER_SUPER = 1,
        const MODIFIER_CTRL  = 2,
        const MODIFIER_ALT   = 4,
        const MODIFIER_SHIFT = 8
    }
}

pub enum ParseHotkeyBindingError {
    BindingIsNotObject,
    HotkeySequenceIsNotArray,
    HotKeyIsNotString,
    HotKeyError(ParseHotkeyError),
    CommandError(ParseCommandError),
    ContextError(ParseContextError)
}

#[derive(Debug)]
pub enum ParseHotkeyError {
    IncorrectKey(String),
    IncorrectModifier(String),
}

impl FromStr for Key {
    type Err = ParseHotkeyError;
    fn from_str(s: &str) -> Result<Key, Self::Err> {
        if s.len() == 1 {
            // Single character keys
            return Ok(Key::Char(s.chars().next().unwrap()));
        }
        let key = match s {
            // Named keys
            "context_menu" => Key::ContextMenu,
            "tab" => Key::Tab,
            "enter" => Key::Enter,
            "escape" => Key::Escape,
            "backspace" => Key::Backspace,

            "right" => Key::Right,
            "left" => Key::Left,
            "up" => Key::Up,
            "down" => Key::Down,

            "delete" => Key::Delete,
            "insert" => Key::Insert,
            "home" => Key::Home,
            "end" => Key::End,
            "pageup" => Key::PageUp,
            "pagedown" => Key::PageDown,

            "pause" => Key::Pause,
            "clear" => Key::Clear,
            "sysreq" => Key::Sysreq,
            "break" => Key::Break,

            // Single character synonyms
            "space" => Key::Char(' '),
            "plus" => Key::Char('+'),
            "minus" => Key::Char('-'),
            "equals" => Key::Char('='),
            "backquote" => Key::Char('`'),
            "forward_slash" => Key::Char('\\'),

            _ => {
                if s.starts_with("f") {
                    // F1, F2, ..., F20 keys
                    match u8::from_str(&s[1..]) {
                        Ok(i) if 1 <= i && i <= 20 => Key::F(i),
                        _ => return Err(IncorrectKey(s.to_string())),
                    }
                } else if s.starts_with("keypad_") {
                    // Keypad special keys
                    match &s[7..] {
                        "period" => Key::KeypadPeriod,
                        "divide" => Key::KeypadDivide,
                        "multiply" => Key::KeypadMultiply,
                        "minus" => Key::KeypadMinus,
                        "plus" => Key::KeypadPlus,
                        "enter" => Key::KeypadEnter,
                        _ => return Err(IncorrectKey(s.to_string())),
                    }
                } else if s.starts_with("keypad") {
                    // Keypad digits
                    match u8::from_str(&s[6..]) {
                        Ok(i) if 0 <= i && i <= 9 => Key::Keypad(i),
                        _ => return Err(IncorrectKey(s.to_string())),
                    }
                } else if s.starts_with("browser_") {
                    // "Browser" keys
                    match &s[8..] {
                        "back" => Key::BrowserBack,
                        "forward" => Key::BrowserForward,
                        "refresh" => Key::BrowserRefresh,
                        "stop" => Key::BrowserStop,
                        "search" => Key::BrowserSearch,
                        "favorites" => Key::BrowserFavorites,
                        "home" => Key::BrowserHome,
                        _ => return Err(IncorrectKey(s.to_string())),
                    }
                } else {
                    return Err(IncorrectKey(s.to_string()));
                }
            }
        };
        Ok(key)
    }
}

impl FromStr for Modifiers {
    type Err = ParseHotkeyError;

    fn from_str(s: &str) -> Result<Modifiers, Self::Err> {
        Ok(match s {
            "super" => MODIFIER_SUPER,
            "ctrl" => MODIFIER_CTRL,
            "alt" => MODIFIER_ALT,
            "shift" => MODIFIER_SHIFT,
            _ => return Err(IncorrectModifier(s.to_string())),
        })
    }
}

impl FromStr for Hotkey {
    type Err = ParseHotkeyError;
    fn from_str(s: &str) -> Result<Hotkey, Self::Err> {
        let mut parts = s.rsplit('+');
        let mut modifiers = MODIFIER_NONE;
        let key = if s.ends_with("++") {
            // Fix for hotkeys like "ctrl++", where second plus isn't separator
            parts.next();
            parts.next();
            Key::Char('+')
        } else {
            try!(Key::from_str(parts.next().unwrap()))
        };
        for part in parts {
            modifiers = modifiers | try!(Modifiers::from_str(part))
        }
        Ok(Hotkey {key: key, modifiers: modifiers})
    }
}

impl FromSettings for HotkeyBinding {
    type Error = ParseHotkeyBindingError;
    fn from_settings(settings: Settings) -> Result<HotkeyBinding, Self::Error> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(BindingIsNotObject),
        };

        let mut arr = match obj.remove("keys") {
            Some(Settings::Array(arr)) => arr,
            _ => return Err(HotkeySequenceIsNotArray)
        };

        let mut hotkeys = Vec::new();

        for settings in arr {
            match settings {
                Settings::String(s) => {
                    match Hotkey::from_str(&s) {
                        Ok(hotkey) => hotkeys.push(hotkey),
                        Err(err) => return Err(HotKeyError(err))
                    }
                },
                _ => return Err(HotKeyIsNotString)
            }
        }

        let context = match obj.remove("context") {
            Some(settings) => {
                match Context::from_settings(settings) {
                    Ok(context) => context,
                    Err(err) => return Err(ContextError(err))
                }
            },
            _ => Context::default()
        };

        let command = match Command::from_settings(Settings::Object(obj)) {
            Ok(command) => command,
            Err(err) => return Err(CommandError(err))
        };

        Ok(HotkeyBinding {
            hotkeys: hotkeys.into_boxed_slice(),
            command: command,
            context: context
        })
    }
}

impl From<Settings> for Keymap {
    fn from(settings: Settings) -> Keymap {
        let arr = match settings {
            Settings::Array(arr) => arr,
            _ => {
                // TODO: warning
                return Keymap::default();
            }
        };
        let mut bindings = Vec::new();
        for settings in arr {
            match HotkeyBinding::from_settings(settings) {
                Ok(binding) => bindings.push(binding),
                Err(err) => {
                    // TODO: warning
                }
            }
        }
        Keymap { bindings: bindings.into_boxed_slice() }
    }
}
