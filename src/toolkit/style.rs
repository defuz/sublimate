use ncurses::{attr_get, attr_set, attr_t, A_NORMAL};

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

impl Color {
    pub fn rgb(rgb: u64) -> Color {
        Color(0)
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
        // const ITALIC    = 0b00000100,
        // const STRIKE    = 0b00001000,
        const BLINK     = 1 << (8 + 11),
        // const INVERT    = 0b00100000,
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
    pub attrs: Attr,
}

struct StyleContext {
    prev_style: Style,
}

impl Style {
    fn normal(colors: ColorPair) -> Style {
        Style {
            colors: colors,
            attrs: NORMAL,
        }
    }

    pub fn current() -> Style {
        let mut attrs: attr_t = 0;
        let mut colors: i16 = 0;
        attr_get(&mut attrs, &mut colors);
        Style {
            colors: ColorPair::from_term(colors),
            attrs: Attr::from_bits(attrs).unwrap_or(NORMAL),
        }
    }

    pub fn set(&self) {
        attr_set(self.attrs.bits, self.colors.to_term());
    }

    pub fn context(&self) -> StyleContext {
        self.set();
        StyleContext { prev_style: Style::current() }
    }

}

impl Drop for StyleContext {
    fn drop(&mut self) {
        self.prev_style.set();
    }
}
