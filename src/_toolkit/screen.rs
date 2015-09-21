
use locale::*;
use ncurses;

fn init() -> Terminal {
    setlocale(category::all, "en_US.utf-8");
    ncurses::initscr();
    ncurses::noecho();
    // keypad();
    ncurses::raw();

    ncurses::start_color();
    ncurses::use_default_colors();
}

fn set_palette() {
    for i in 0..255+1 {
        init_pair(i, i, -1);
    }
}

fn terminate() {
  endwin();
}
