use toolkit::{Color, ColorPair};

const BRIDGEST : i8 = 1;
const CONTRAST : i8 = 12;
const ACCENT   : i8 = 4;

// editor
pub const EDITOR               : ColorPair = ColorPair(0);
pub const EDITOR_SELECTED      : ColorPair = ColorPair(1);

// sidebar
pub const SIDEBAR              : ColorPair = ColorPair(2);
pub const SIDEBAR_SELECTED     : ColorPair = ColorPair(3);
pub const SIDEBAR_LOW          : ColorPair = ColorPair(4);
pub const SIDEBAR_LOW_SELECTED : ColorPair = ColorPair(5);

// tabs
pub const TABS                 : ColorPair = ColorPair(6);
pub const TABS_SELECTED        : ColorPair = ColorPair(7);
pub const TABS_LOW             : ColorPair = ColorPair(8);
pub const TABS_LOW_SELECTED    : ColorPair = ColorPair(9);

// menubar
pub const MENUBAR              : ColorPair = ColorPair(10);
pub const MENUBAR_SELECTED     : ColorPair = ColorPair(11);

// statusbar
pub const STATUSBAR            : ColorPair = ColorPair(12);

// modals
pub const MODAL                : ColorPair = ColorPair(13);
pub const MODAL_SELECTED       : ColorPair = ColorPair(14);
pub const MODAL_LOW            : ColorPair = ColorPair(15);
pub const MODAL_LOW_SELECTED   : ColorPair = ColorPair(16);
pub const MODAL_DISABLED       : ColorPair = ColorPair(17);
pub const MODAL_DISABLED_LOW   : ColorPair = ColorPair(18);

macro_rules! create_color_pair {
    ($base:expr, $accent:expr) => ((
        Color(232 + (BRIDGEST * $base + CONTRAST + $accent * ACCENT) as u8),
        Color(232 + (BRIDGEST * $base) as u8)
    ))
}

pub const PALETTE : [(Color, Color); 19] = [
    // editor
    create_color_pair!(0,  0),  // EDITOR
    create_color_pair!(1,  0),  // EDITOR_SELECTED

    // sidebar
    create_color_pair!(1,  0),  // SIDEBAR
    create_color_pair!(2,  0),  // SIDEBAR_SELECTED
    create_color_pair!(1, -1),  // SIDEBAR_LOW
    create_color_pair!(2, -1),  // SIDEBAR_LOW_SELECTED

    // tabs
    create_color_pair!(2,  0),  // TABS
    create_color_pair!(0,  0),  // TABS_SELECTED
    create_color_pair!(2, -1),  // TABS_LOW
    create_color_pair!(0, -1),  // TABS_LOW_SELECTED

    // menubar
    create_color_pair!(3, -1),  // MENUBAR
    create_color_pair!(4,  0),  // MENUBAR_SELECTED

    // statusbar
    create_color_pair!(3, -1),  // STATUSBAR

    // modals
    create_color_pair!(4,  0),  // MODAL
    create_color_pair!(5,  0),  // MODAL_SELECTED
    create_color_pair!(4, -1),  // MODAL_LOW
    create_color_pair!(5, -1),  // MODAL_LOW_SELECTED
    create_color_pair!(4, -1),  // MODAL_DISABLED
    create_color_pair!(4, -2),  // MODAL_DISABLED_LOW
];
