use core::bindings::Key;

// Ctrl AZ: 1 - 26
// az 97 122
// Shift AZ: 65 90

// backspace 127
// 09       48 57

// f1-f12   265 276

// left     260
// right    261
// up       259
// down     258

// shift left       393
// shift right      402
// shift up         337
// shift down       336

// alt left         27 27 91 68
// alt right        27 27 91 67
// alt up           27 27 91 65
// alt down         27 27 91 66

// ctrl shift left       542
// ctrl shift right      557
// ctrl shift up         563
// ctrl shift down       522

// ctrl left       541
// ctrl right      556
// ctrl up         562
// ctrl down       521

// tab 9
// shift tab 353

impl Key {
    pub fn from_keycode(keycode: i32) -> Option<Key> {
        match keycode {
            10  => Some(Key::Enter),

            260 => Some(Key::Left),
            261 => Some(Key::Right),
            259 => Some(Key::Up),
            258 => Some(Key::Down),
            _ => {

                // TODO: warning
                None
            }
        }
    }
}
