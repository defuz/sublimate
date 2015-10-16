use std::str::FromStr;
use std::convert::From;
use std::collections::HashMap;
use rustc_serialize::json::Json;
use core::command::Command;

bitflags! {
    flags Modifiers: u8 {
        const None  = 0,
        const Super = 1,
        const Ctrl  = 2,
        const Alt   = 4,
        const Shift = 8
    }
}

impl FromStr for Modifiers {
    type Err = String;

    fn from_str(s: &str) -> Result<Modifiers, Self::Err> {
        Ok(match s {
            "super" => Super,
            "ctrl" => Ctrl,
            "alt" => Alt,
            "shift" => Shift,
            _ => return Err(s.to_string()),
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Key {
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

impl FromStr for Key {
    type Err = String;
    fn from_str(s: &str) -> Result<Key, Self::Err> {
        if s.len() == 1 {
            // Single character keys
            let c = s.chars().next().unwrap();
            match c {
                '[' |
                ']' |
                '(' |
                ')' |
                '{' |
                '}' |
                '`' |
                '=' |
                ';' |
                ',' |
                '\'' |
                '\"' |
                '\\' |
                '.' |
                '/' |
                '*' |
                '-' |
                '+' |
                'a' ... 'z' |
                '0' ... '9' => return Ok(Key::Char(c)),
                _ => return Err(s.to_string()),
            }
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
                        _ => return Err(s.to_string()),
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
                        _ => return Err(s.to_string()),
                    }
                } else if s.starts_with("keypad") {
                    // Keypad digits
                    match u8::from_str(&s[6..]) {
                        Ok(i) if 0 <= i && i <= 9 => Key::Keypad(i),
                        _ => return Err(s.to_string()),
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
                        _ => return Err(s.to_string()),
                    }
                } else {
                    return Err(s.to_string());
                }
            }
        };
        Ok(key)
    }
}



#[derive(Debug, Hash, PartialEq, Eq)]
struct Hotkey {
    key: Key,
    modifiers: Modifiers,
}

#[derive(Debug)]
enum ParseHotKeyError {
    IncorrectKey(String),
    IncorrectModifier(String),
}

impl FromStr for Hotkey {
    type Err = ParseHotKeyError;
    fn from_str(s: &str) -> Result<Hotkey, Self::Err> {
        let mut parts = s.rsplit('+');
        let mut modifiers = None;
        let key = if s.ends_with("++") {
            // Fix for hotkeys like "ctrl++", where second plus isn't separator
            parts.next();
            parts.next();
            Key::Char('+')
        } else {
            match Key::from_str(parts.next().unwrap()) {
                Ok(key) => key,
                Err(key) => return Err(ParseHotKeyError::IncorrectKey(key)),
            }
        };
        for part in parts {
            modifiers = modifiers |
                        match Modifiers::from_str(part) {
                Ok(modifier) => modifier,
                Err(modifier) => return Err(ParseHotKeyError::IncorrectModifier(modifier)),
            }
        }
        Ok(Hotkey {
            key: key,
            modifiers: modifiers,
        })
    }
}

type HotkeySequence = Box<[Hotkey]>;

#[derive(Debug, Default)]
pub struct Keymap {
    commands: HashMap<HotkeySequence, Command>,
    hotkeys: HashMap<Command, HotkeySequence>,
}

impl From<Json> for Keymap {
    fn from(json: Json) -> Keymap {
        let mut keymap = Keymap {
            commands: HashMap::new(),
            hotkeys: HashMap::new(),
        };
        if let Json::Array(array) = json {
            for mut item_json in array {
                if let Some(obj) = item_json.as_object_mut() {
                    match obj.remove("keys") {
                        Some(Json::Array(keys)) => {
                            for i in keys {
                                match i {
                                    Json::String(key) => match Hotkey::from_str(&key[..]) {
                                        Ok(hotkey) => {}
                                        Err(err) => {
                                            error!("{:?}", err);
                                            error!("{:?}", key);
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
        keymap
    }
}
