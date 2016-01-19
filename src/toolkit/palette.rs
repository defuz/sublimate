use std::cell::{Cell, RefCell};
use std::collections::HashMap;

use toolkit::style::{Color, ColorPair};

#[derive(Debug)]
pub struct ColorPalette {
    index: Cell<u8>,
    end: u8,
    map: RefCell<HashMap<(Color, Color), ColorPair>>
}

impl ColorPalette {
    pub fn new(from: u8, to: u8) -> ColorPalette {
        ColorPalette {
            index: Cell::new(from),
            end: to,
            map: RefCell::new(HashMap::new())
        }
    }

    pub fn color_pair(&self, foreground: Color, background: Color) -> ColorPair {
        let index = self.index.get();
        self.map.borrow_mut().entry((foreground, background)).or_insert_with(|| {
            if index < self.end {
                self.index.set(index + 1);
            } else {
                // TODO: warning
            }
            ColorPair(index)
        }).clone()
    }
}
