use std::cmp::min;

use toolkit::canvas::Canvas;
use toolkit::base::Point;
use toolkit::base::Rect;
use toolkit::base::AsRect;
use toolkit::canvas::RootCanvas;
use toolkit::canvas::SubCanvas;
use toolkit::canvas::SuperCanvas;

pub struct MouseHandler;

pub trait MouseTarget {
    fn set_mouse_handler(&self, target: MouseHandler, rect: Rect);
}

impl MouseTarget for RootCanvas {
    fn set_mouse_handler(&self, target: MouseHandler, rect: Rect) {
        // width, height = width or self.width, height or self.height
        // assert x + width <= self.width, y + height <= self.height
        // for i in range(height):
        //     self.mouse_target[y+i][x:x+width] = AttrFlow.fill(width, target)
    }
}

impl<'base> MouseTarget for SubCanvas<'base> {
    fn set_mouse_handler(&self, target: MouseHandler, rect: Rect) {
        // todo:
        // width, height = width or self.width, height or self.height
        // assert x + width <= self.width, y + height <= self.height
        // self.base_canvas.set_mouse_target(target, self.x+x, self.y+y, width, height)
    }
}

impl<'base> MouseTarget for SuperCanvas<'base> {
    fn set_mouse_handler(&self, target: MouseHandler, mut rect: Rect) {
        rect.x = rect.x - self.rect.x;
        rect.y = rect.y - self.rect.y;
        if rect.x < 0 {
            rect.w += rect.x;
            rect.x = 0;
        }
        if rect.y < 0 {
            rect.h += rect.y;
            rect.y = 0;
        }
        rect.w = min(rect.w, self.base.width() - rect.x);
        rect.h = min(rect.h, self.base.height() - rect.y);
        if rect.w > 0 && rect.y > 0 {
            self.base.set_mouse_handler(target, rect);
        }
    }
}
