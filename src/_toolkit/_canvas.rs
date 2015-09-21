use std::cmp::min;
use std::ops::Index;
use std::slice::Iter;
use std::cell::RefCell;

use toolkit::base::Point;
use toolkit::base::Rect;
use toolkit::base::AsRect;
use toolkit::base::Size;
use toolkit::mouse::MouseHandler;
use toolkit::mouse::MouseTarget;
use toolkit::attributed::AttrFlow;
use toolkit::attributed::AttrString;


pub trait Canvas : Size + MouseTarget {
    fn as_canvas(&self) -> &Canvas;

    fn inner(&self, mut rect: Rect) -> SubCanvas {
        assert!(rect.w <= self.width());
        assert!(rect.h <= self.height());

        rect.x = if rect.x + rect.w > self.width() {
            rect.w - self.width()
        } else {
            min(0, rect.x)
        };
        rect.y = if rect.y + rect.h > self.height() {
            rect.h - self.height()
        } else {
            min(0, rect.y)
        };

        SubCanvas::new(self.as_canvas(), rect)
    }

    fn outer(&self, mut rect: Rect) -> SuperCanvas {
        SuperCanvas::new(self.as_canvas(), rect)
    }

    fn padding(&self, left: u16, right: u16, top: u16, bottom: u16) -> SubCanvas {
        assert!(left + right <= self.width());
        assert!(top + bottom <= self.height());

        SubCanvas::new(self.as_canvas(), Rect {
            x: left,
            y: top,
            w: self.width() - left - right,
            h: self.height() - top - bottom
        })
    }

    fn left_alignment(&self, width: u16) -> SubCanvas {
        SubCanvas::new(self.as_canvas(), Rect {
            x: 0,
            y: 0,
            w: width,
            h: self.height()
        })
    }

    fn right_alignment(&self, width: u16) -> SubCanvas {
        SubCanvas::new(self.as_canvas(), Rect {
            x: self.width() - width,
            y: 0,
            w: width,
            h: self.height()
        })
    }
}

pub struct RootCanvas {
    pub rect: Rect,
    mouse_target_map: RefCell<Vec<AttrFlow<MouseHandler>>>
}

impl RootCanvas {

    fn new(x: u16, y: u16, width: u16, height: u16) -> RootCanvas {
        RootCanvas {
            rect: Rect {x: x, y: y, w: width, h: height},
            mouse_target_map: RefCell::new(Vec::new())
        }
    }

    fn width(&self) -> u16 {
        self.rect.w
    }

    fn height(&self) -> u16 {
        self.rect.h
    }

    fn get_mouse_handler(&self, point: Point) -> MouseHandler {
        MouseHandler
    }

}

pub struct SubCanvas<'base> {
    pub rect: Rect,
    pub base: &'base Canvas,
}

impl<'base> SubCanvas<'base> {

    fn new(base: &Canvas, rect: Rect) -> SubCanvas {
        SubCanvas { base: base, rect: rect }
    }

    fn width(&self) -> u16 {
        self.rect.w
    }

    fn height(&self) -> u16 {
        self.rect.h
    }


}

// impl<'base> Canvas for SubCanvas<'base> {

// }

pub struct SuperCanvas<'base> {
    pub rect: Rect,
    pub base: &'base Canvas
}

impl<'base> AsRect for SuperCanvas<'base> {
    fn as_rect(&self) -> &Rect {
        &self.rect
    }
}

impl<'base> SuperCanvas<'base> {

    fn new(base: &Canvas, rect: Rect) -> SuperCanvas {
        SuperCanvas { base: base, rect: rect }
    }

    fn width(&self) -> u16 {
        self.rect.w
    }

    fn height(&self) -> u16 {
        self.rect.h
    }


}
