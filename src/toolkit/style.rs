use ncurses::{attr_get, attr_t};

pub struct Color(pub u8);

impl ColorPair {
    pub fn from_term(colors: i16) -> ColorPair {
        ColorPair(colors as u8)
    }

    pub fn to_term(&self) -> i16 {
        match self {
            &ColorPair(colors) => colors as i16,
        }
    }
}

pub struct ColorPair(pub u8);

const BLACK : Color = Color(16);
const WHITE : Color = Color(231);

fn color_chanel_256_to_6(x: u8) -> (u8, u8) {
    match x {
        0x00...0x2F => (0, x - 0x00),
        0x2F...0x5F => (1, 0x5F - x),
        0x5F...0x73 => (1, x - 0x5F),
        0x73...0x87 => (2, 0x87 - x),
        0x87...0x9B => (2, x - 0x87),
        0x9B...0xAF => (3, 0xAF - x),
        0xAF...0xC3 => (3, x - 0xAF),
        0xC3...0xD7 => (4, 0xD7 - x),
        0xD7...0xEB => (4, x - 0xD7),
        0xEB...0xFF => (5, 0xFF - x),
        _ => unreachable!()
    }
}

impl Color {
    pub fn from_rgb256(r: u8, g: u8, b: u8) -> Color {
        // find color in color cube
        let (r6, r6diff) = color_chanel_256_to_6(r);
        let (g6, g6diff) = color_chanel_256_to_6(g);
        let (b6, b6diff) = color_chanel_256_to_6(b);
        let cube_diff = r6diff + g6diff + b6diff;
        // find color in grayscale palette
        let value = ((r as u16 + g as u16 + b as u16) / 3) as u8;
        let (gray_color, origin_value) = match value {
            0x00...0x04 => (BLACK, 0x00),
            0x04...0xF4 => {
                let index = (value - 0x04) / 10;
                (Color::grayscale(index), 0x08 + index * 0xA)
            }
            0xF4...0xFF => (WHITE, 0xff),
            _ => unreachable!(),
        };
        let gray_diff = (origin_value - r) + (origin_value - g) + (origin_value - b);
        // compare best match and return
        if gray_diff < cube_diff {
            gray_color
        } else {
            Color::color_cube(r6, g6, b6)
        }
    }

    fn color_cube(r: u8, g: u8, b: u8) -> Color {
        Color(16 + r * 36 + g * 6 + b)
    }

    pub fn grayscale(index: u8) -> Color {
        assert!(index <= 23);
        Color(232 + index)
    }

    pub fn to_term(&self) -> i16 {
        match self {
            &Color(color) => color as i16,
        }
    }
}

bitflags! {
    flags Attr: u64 {
        const NORMAL    = 0,
        const BOLD      = 1 << (8 + 13),
        const UNDERLINE = 1 << (8 +  9),
        // todo: add italic, invert and blink
    }
}

// impl Attr {
//     fn from_term(attrs: attr_t) -> Attr {
//         attrs as Attr
//     }

//     fn to_term(&self) -> attr_t {
//         *self as attr_t
//     }
// }

pub struct Style {
    pub colors: ColorPair,
    pub attrs: Attr
}

struct StyleContext {
    prev_style: Style
}

impl Attr {
    pub fn to_term(&self) -> u64 {
        self.bits
    }
}

impl Style {
    fn normal(colors: ColorPair) -> Style {
        Style {
            colors: colors,
            attrs: NORMAL
        }
    }

    pub fn current() -> Style {
        let mut attrs: attr_t = 0;
        let mut colors: i16 = 0;
        attr_get(&mut attrs, &mut colors);
        Style {
            colors: ColorPair::from_term(colors),
            attrs: Attr::from_bits(attrs).unwrap_or(NORMAL)
        }
    }

    // pub fn context(&self) -> StyleContext {
    //     self.set();
    //     StyleContext { prev_style: Style::current() }
    // }

}

// impl Drop for StyleContext {
//     fn drop(&mut self) {
//         self.prev_style.set();
//     }
// }
