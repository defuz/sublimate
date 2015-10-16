use std::cmp::{min, max};

use toolkit::core::*;
use toolkit::style::Style;
use toolkit::draw::Drawing;

use ncurses::{mvaddstr, mvaddch};

#[derive(Debug)]
pub struct Canvas {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

impl HasSize for Canvas {
    fn width(&self) -> usize {
        self.x2 - self.x1 + 1
    }

    fn height(&self) -> usize {
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

    fn padding(&self, left: usize, right: usize, top: usize, bottom: usize) -> Canvas {
        assert!(left + right <= self.width());
        assert!(top + bottom <= self.height());
        Canvas {
            x1: self.x1 + left,
            y1: self.y1 + top,
            x2: self.x2 - right,
            y2: self.y2 - bottom,
        }
    }

    fn left(&self, width: usize) -> Canvas {
        Canvas {
            x1: self.x1,
            y1: self.y1,
            x2: self.x1 + width,
            y2: self.y2,
        }
    }

    fn right(&self, width: usize) -> Canvas {
        Canvas {
            x1: self.x2 - width,
            y1: self.y1,
            x2: self.x2,
            y2: self.y2,
        }
    }

    fn top(&self, height: usize) -> Canvas {
        Canvas {
            x1: self.x1,
            y1: self.y1,
            x2: self.x2,
            y2: self.y1 + height,
        }
    }

    fn bottom(&self, height: usize) -> Canvas {
        Canvas {
            x1: self.x1,
            y1: self.y2 - height,
            x2: self.x2,
            y2: self.y2,
        }
    }

    pub fn cut_left(&mut self, mut width: usize) -> Canvas {
        width = min(width, self.width());
        let left = self.left(width);
        self.x1 += width;
        left
    }

    pub fn cut_top(&mut self, mut height: usize) -> Canvas {
        height = min(height, self.height());
        let top = self.top(height);
        self.y1 += height;
        top
    }
}

impl Drawing for Canvas {
    fn fill(&self) {
    }

    fn char(&self, c: char, y: usize, x: usize) {
        mvaddch((self.y1 + y) as i32, (self.x1 + x) as i32, c as u64);
    }

    fn text(&self, s: &str, y: usize, x: usize) {
        mvaddstr((self.y1 + y) as i32, (self.x1 + x) as i32, s);
    }
}
