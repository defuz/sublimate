use toolkit::core::*;

bitflags!(
    flags Control: u8 {
        const CTRL  = 1u8 << 0,
        const SHIFT = 1u8 << 1,
        const ALT   = 1u8 << 2,
        const SUPER = 1u8 << 3,
    }
);

#[derive(Debug)]
pub enum KeyboardEvent {
    Up,
    Down,
    Left,
    Right
}

pub enum MouseButton {
    None,
    Left,
    Right,
    Middle,
    ScrollUp,
    ScrollDown
}

pub enum MouseEvent {
    Press(MouseButton),
    Release(MouseButton),
    Drag(size, size)
}

pub trait OnKeyboard {
    fn on_keyboard(&self, event: KeyboardEvent) -> bool;
}

pub trait OnMouse {
    fn on_mouse(&self, event: MouseEvent) -> bool;
}
