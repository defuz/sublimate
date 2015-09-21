use toolkit::event::{OnKeyboard, KeyboardEvent};

trait Tree {
    fn is_opened(&self) -> Option<&mut bool>;
}

impl OnKeyboard for Tree {
    fn on_keyboard(&self, event: KeyboardEvent) -> bool {
        match event {
            KeyboardEvent::Up => {
                false
            },
            KeyboardEvent::Down => {
                false
            },
            KeyboardEvent::Left => {
                false
            },
            KeyboardEvent::Right => {
                false
            },
        }
    }
}
