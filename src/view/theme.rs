use toolkit::{Color, ColorPair, Style, NORMAL};

const BRIDGEST: i8 = 1;
const CONTRAST: i8 = 17;
const ACCENT: i8 = 4;

// editor
pub const EDITOR_STYLE: Style = Style {
    colors: ColorPair(0),
    attrs: NORMAL
};
pub const EDITOR_SELECTED_STYLE: Style = Style {
    colors: ColorPair(1),
    attrs: NORMAL
};

// sidebar
pub const SIDEBAR_STYLE: Style = Style {
    colors: ColorPair(2),
    attrs: NORMAL
};
pub const SIDEBAR_SELECTED_STYLE: Style = Style {
    colors: ColorPair(3),
    attrs: NORMAL
};
pub const SIDEBAR_LOW_STYLE: Style = Style {
    colors: ColorPair(4),
    attrs: NORMAL
};
pub const SIDEBAR_LOW_SELECTED_STYLE: Style = Style {
    colors: ColorPair(5),
    attrs: NORMAL
};

// tabs
pub const TABS_STYLE: Style = Style {
    colors: ColorPair(6),
    attrs: NORMAL
};
pub const TABS_SELECTED_STYLE: Style = Style {
    colors: ColorPair(7),
    attrs: NORMAL
};
pub const TABS_LOW_STYLE: Style = Style {
    colors: ColorPair(8),
    attrs: NORMAL
};
pub const TABS_LOW_SELECTED_STYLE: Style = Style {
    colors: ColorPair(9),
    attrs: NORMAL
};

// menubar
pub const MENUBAR_STYLE: Style = Style {
    colors: ColorPair(10),
    attrs: NORMAL
};
pub const MENUBAR_SELECTED_STYLE: Style = Style {
    colors: ColorPair(11),
    attrs: NORMAL
};

// statusbar
pub const STATUSBAR_STYLE: Style = Style {
    colors: ColorPair(12),
    attrs: NORMAL
};

// modals
pub const MODAL_STYLE: Style = Style {
    colors: ColorPair(13),
    attrs: NORMAL
};
pub const MODAL_SELECTED_STYLE: Style = Style {
    colors: ColorPair(14),
    attrs: NORMAL
};
pub const MODAL_LOW_STYLE: Style = Style {
    colors: ColorPair(15),
    attrs: NORMAL
};
pub const MODAL_SELECTED_LOW_STYLE: Style = Style {
    colors: ColorPair(16),
    attrs: NORMAL
};
pub const MODAL_DISABLED_STYLE: Style = Style {
    colors: ColorPair(17),
    attrs: NORMAL
};
pub const MODAL_DISABLED_LOW_STYLE: Style = Style {
    colors: ColorPair(18),
    attrs: NORMAL
};

macro_rules! create_color_pair {
    ($base:expr, $accent:expr) => ((
        Color(232 + (BRIDGEST * $base + CONTRAST + $accent * ACCENT) as u8),
        Color(232 + (BRIDGEST * $base) as u8)
    ))
}

pub const PALETTE: [(Color, Color); 19] = [// editor
                                           create_color_pair!(0, 0), // EDITOR
                                           create_color_pair!(1, 0), // EDITOR_SELECTED

                                           // sidebar
                                           create_color_pair!(1, 0), // SIDEBAR
                                           create_color_pair!(2, 0), // SIDEBAR_SELECTED
                                           create_color_pair!(1, -1), // SIDEBAR_LOW
                                           create_color_pair!(2, -1), // SIDEBAR_LOW_SELECTED

                                           // tabs
                                           create_color_pair!(2, 0), // TABS
                                           create_color_pair!(0, 0), // TABS_SELECTED
                                           create_color_pair!(2, -1), // TABS_LOW
                                           create_color_pair!(0, -1), // TABS_LOW_SELECTED

                                           // menubar
                                           create_color_pair!(3, -1), // MENUBAR
                                           create_color_pair!(4, 0), // MENUBAR_SELECTED

                                           // statusbar
                                           create_color_pair!(3, -1), // STATUSBAR

                                           // modals
                                           create_color_pair!(4, 0), // MODAL
                                           create_color_pair!(5, 0), // MODAL_SELECTED
                                           create_color_pair!(4, -1), // MODAL_LOW
                                           create_color_pair!(5, -1), // MODAL_LOW_SELECTED
                                           create_color_pair!(4, -1), // MODAL_DISABLED
                                           create_color_pair!(4, -2) /* MODAL_DISABLED_LOW */];
