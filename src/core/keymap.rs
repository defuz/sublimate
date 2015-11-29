use std::str::FromStr;
use std::fmt::{Display, Formatter, Write, Error as FormatterError};

use core::command::{Command, ParseCommandError};
use core::context::{Context, ParseContextError};
use core::settings::{Settings, FromSettings, ParseSettings};

use self::ParseHotkeyError::*;
use self::ParseHotkeyBindingError::*;

pub type Keymap = Vec<HotkeyBinding>;

#[derive(Debug)]
pub struct HotkeyBinding {
    pub hotkeys: HotkeySequence,
    pub command: Command,
    pub context: Context
}

pub type HotkeySequence = Vec<Hotkey>;

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
    ContextError(ParseContextError),
}

#[derive(Debug)]
pub enum ParseHotkeyError {
    IncorrectKey(String),
    IncorrectModifier(String),
}

impl Display for Modifiers {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatterError> {
        if self.contains(MODIFIER_CTRL) {
            try!(fmt.write_str("Ctrl+"));
        }
        if self.contains(MODIFIER_ALT) {
            try!(fmt.write_str("Alt+"));
        }
        if self.contains(MODIFIER_SHIFT) {
            try!(fmt.write_str("Shift+"));
        }
        if self.contains(MODIFIER_SUPER) {
            try!(fmt.write_str("Super+"));
        }
        Ok(())
    }
}

impl Display for Key {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatterError> {
        match *self {
            Key::ContextMenu => fmt.write_str("Context Menu"),
            Key::Tab => fmt.write_str("Tab"),
            Key::Enter => fmt.write_str("Enter"),
            Key::Escape => fmt.write_str("Escape"),
            Key::Backspace => fmt.write_str("Backspace"),
            Key::Right => fmt.write_str("Right"),
            Key::Left => fmt.write_str("Left"),
            Key::Up => fmt.write_str("Up"),
            Key::Down => fmt.write_str("Down"),
            Key::Delete => fmt.write_str("Delete"),
            Key::Insert => fmt.write_str("Insert"),
            Key::Home => fmt.write_str("Home"),
            Key::End => fmt.write_str("End"),
            Key::PageUp => fmt.write_str("Page Up"),
            Key::PageDown => fmt.write_str("Page Down"),
            Key::Pause => fmt.write_str("Pause"),
            Key::Clear => fmt.write_str("Clear"),
            Key::Sysreq => fmt.write_str("Sysreq"),
            Key::Break => fmt.write_str("Break"),
            /// "Browser" keys
            Key::BrowserBack => fmt.write_str("Browser Back"),
            Key::BrowserForward => fmt.write_str("Browser Forward"),
            Key::BrowserRefresh => fmt.write_str("Browser Refresh"),
            Key::BrowserStop => fmt.write_str("Browser Stop"),
            Key::BrowserSearch => fmt.write_str("Browser Search"),
            Key::BrowserFavorites => fmt.write_str("Browser Favorites"),
            Key::BrowserHome => fmt.write_str("Browser Home"),
            /// Keypad special keys
            Key::KeypadPeriod => fmt.write_str("Keypad %"),
            Key::KeypadDivide => fmt.write_str("Keypad /"),
            Key::KeypadMultiply => fmt.write_str("Keypad *"),
            Key::KeypadMinus => fmt.write_str("Keypad -"),
            Key::KeypadPlus => fmt.write_str("Keypad +"),
            Key::KeypadEnter => fmt.write_str("Keypad Enter"),
            /// Keypad digit keys
            Key::Keypad(num) => write!(fmt, "Keypad {}", num),
            /// Functional keys
            Key::F(num) => write!(fmt, "F{}", num),
            /// Single character keys
            Key::Char(mut c) => {
                if c == ' ' {
                    fmt.write_str("Space")
                } else {
                    fmt.write_str(&c.to_uppercase().collect::<String>())
                }
            },
        }
    }
}

impl Display for Hotkey {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FormatterError> {
        write!(fmt, "{}{}", self.modifiers, self.key)
    }
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
        let mut modifiers = Modifiers::empty();
        let key = if s.ends_with("++") {
            // Fix for hotkeys like "ctrl++", where second plus isn't separator
            parts.next();
            parts.next();
            Key::Char('+')
        } else {
            try!(Key::from_str(parts.next().unwrap()))
        };
        for part in parts {
            modifiers.insert(try!(Modifiers::from_str(part)))
        }
        Ok(Hotkey {
            key: key,
            modifiers: modifiers
        })
    }
}

impl ParseSettings for HotkeyBinding {
    type Error = ParseHotkeyBindingError;
    fn parse_settings(settings: Settings) -> Result<HotkeyBinding, Self::Error> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(BindingIsNotObject),
        };

        let mut arr = match obj.remove("keys") {
            Some(Settings::Array(arr)) => arr,
            _ => return Err(HotkeySequenceIsNotArray),
        };

        let mut hotkeys = Vec::new();

        for settings in arr {
            match settings {
                Settings::String(s) => {
                    match Hotkey::from_str(&s) {
                        Ok(hotkey) => hotkeys.push(hotkey),
                        Err(err) => return Err(HotKeyError(err)),
                    }
                }
                _ => return Err(HotKeyIsNotString),
            }
        }

        let context = match obj.remove("context") {
            Some(settings) => {
                match Context::parse_settings(settings) {
                    Ok(context) => context,
                    Err(err) => return Err(ContextError(err)),
                }
            }
            _ => Context::default(),
        };

        let command = match Command::parse_settings(Settings::Object(obj)) {
            Ok(command) => command,
            Err(err) => return Err(CommandError(err)),
        };

        // TODO: check that obj is empty

        Ok(HotkeyBinding {
            hotkeys: hotkeys,
            command: command,
            context: context
        })
    }
}

impl FromSettings for Keymap {
    fn from_settings(settings: Settings) -> Keymap {
        let arr = match settings {
            Settings::Array(arr) => arr,
            _ => {
                // TODO: warning
                return Keymap::default();
            }
        };
        let mut keymap = Keymap::new();
        for settings in arr {
            match HotkeyBinding::parse_settings(settings) {
                Ok(binding) => keymap.push(binding),
                Err(err) => {
                    // TODO: warning
                }
            }
        }
        keymap
    }
}
