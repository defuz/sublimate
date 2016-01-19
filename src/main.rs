#![allow(dead_code)]
#![feature(slice_patterns)]

mod core;
mod toolkit;
mod view;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate oniguruma;
extern crate unicode_width;

extern crate ncurses;

extern crate rustc_serialize;
extern crate weakjson;
extern crate plist;


use ncurses::*;

use core::Core;
use core::bindings::Key;
use view::window::Window;
use toolkit::*;
use view::theme::PALETTE;

fn main() {

    setlocale(LcCategory::all, "en_US.utf-8");

    initscr();

    noecho();
    keypad(stdscr, true);
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    raw();

    start_color();
    use_default_colors();

    for (i, &(ref fg, ref bg)) in PALETTE.iter().enumerate() {
        init_pair(i as i16, fg.to_term(), bg.to_term());
    }


    let mut window = Window::new(Core::load());
    window.render(Canvas::screen());
    loop {
        if let Some(key) = Key::from_keycode(getch()) {
            if key == Key::Enter {
                break;
            }
            window.on_keypress(Canvas::screen(), key);
        }
    }


    // println!("{:?}", window);

    // core.package_repository.get_keymap("default/Default (OSX).sublime-keymap");
    // core.package_repository.get_keymap("default/Default
    // (Windows).sublime-keymap");
    // core.package_repository.get_keymap("default/Default (Linux).sublime-keymap");


    // info!("hello world!");

    // menu.render(Canvas { x1: 0, y1: 0, x2: 80, y2: 20});


    // Start ncurses.

    // mousemask(ALL_MOUSE_EVENTS as u64, None);

    // attron(COLOR_PAIR(150));

    // mvaddstr(10, 10, "hello world");
    // clear();
    // /* Print to the back buffer. */
    // // attron(COLOR_PAIR(0));
    // // printw("Базовый 0..15\n\n");
    // // for i in 0..15+1 {
    // //   attron(COLOR_PAIR(i));
    // //   printw("██");
    // // }

    // // printw("\n\nColor cube 16..231\n\n");
    // // for i in 16..231+1 {
    // //   attron(COLOR_PAIR(i));
    // //   printw("██");
    // // }

    // // printw("\n\nGrayscale 232..255\n\n");
    // // for i in 232..255+1 {
    // //   attron(COLOR_PAIR(i));
    // //   printw("██");
    // // }


    // Wait for a key press.
    // loop {
    //     let c = getch();
    //     println!("{:?}\r", c);
    //     if c == 10 {
    //         break;
    //     }
    // }

    // Terminate ncurses.
    endwin();
}
