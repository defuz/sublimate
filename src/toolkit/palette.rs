use std::collections::HashMap;

use toolkit::style::{Color, ColorPair};

struct ColorPalette {
    index: u8,
    end: u8,
    map: HashMap<(Color, Color), ColorPair>
}

impl ColorPalette {
    fn new(from: u8, to: u8) -> ColorPalette {
        ColorPalette {
            index: from,
            end: to,
            map: HashMap::new()
        }
    }

    fn get_color_pair(&mut self, foreground: Color, background: Color) -> ColorPair {
        let (map, index, end) = (&mut self.map, &mut self.index, &self.end);
        map.entry((foreground, background)).or_insert_with(|| {
            if *index == *end {
                // TODO: warning
                return ColorPair(*index)
            }
            *index += 1;
            ColorPair(*index - 1)
        }).clone()
    }
}
