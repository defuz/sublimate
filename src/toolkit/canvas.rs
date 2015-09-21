use std::cmp::{min, max};

use toolkit::core::*;
use toolkit::style::Style;
use toolkit::draw::Drawing;

use ncurses::{mvaddstr, mvaddch};

#[derive(Debug)]
pub struct Canvas {
    pub x1: size,
    pub y1: size,
    pub x2: size,
    pub y2: size
}

impl HasWidth for Canvas {
    fn width(&self) -> size {
        self.x2 - self.x1 + 1
    }
}

impl HasHeight for Canvas {
    fn height(&self) -> size {
        self.y2 - self.y1 + 1
    }
}

impl Canvas {

    fn inner(&self, inner: Canvas) -> Canvas {
        Canvas {
            x1: max(self.x1, self.x1 + inner.x1),
            y1: max(self.y1, self.y1 + inner.y1),
            x2: min(self.x2, self.x1 + inner.x2),
            y2: min(self.y2, self.y1 + inner.y2),
        }
    }

    fn padding(&self, left: size, right: size, top: size, bottom: size) -> Canvas {
        assert!(left + right <= self.width());
        assert!(top + bottom <= self.height());
        Canvas {
            x1: self.x1 + left, y1: self.y1 + top,
            x2: self.x2 - right, y2: self.y2 - bottom
        }
    }

    fn left(&self, width: size) -> Canvas {
        Canvas {x1: self.x1, y1: self.y1, x2: self.x1 + width, y2: self.y2}
    }

    fn right(&self, width: size) -> Canvas {
        Canvas {x1: self.x2 - width, y1: self.y1, x2: self.x2, y2: self.y2}
    }

    fn top(&self, height: size) -> Canvas {
        Canvas {x1: self.x1, y1: self.y1, x2: self.x2, y2: self.y1 + height}
    }

    fn bottom(&self, height: size) -> Canvas {
        Canvas {x1: self.x1, y1: self.y2 - height, x2: self.x2, y2: self.y2}
    }

    pub fn split_horz(&self, mut width: size) -> (Canvas, Canvas) {
        width = min(width, self.width());
        (self.left(width), self.right(self.width() - width - 1))
    }

    pub fn split_vert(&self, mut height: size) -> (Canvas, Canvas) {
        height = min(height, self.height());
        (self.top(height), self.bottom(self.height() - height - 1))
    }
}

impl Drawing for Canvas {
    fn fill(&self) {}

    fn char(&self, c: char, x: size, y: size) {
        mvaddch((self.y1 + y) as i32, (self.x1 + x) as i32, c as u64);
    }

    fn text(&self, s: &str, x: size, y: size)  {
        mvaddstr((self.y1 + y) as i32, (self.x1 + x) as i32, s);
    }
}
