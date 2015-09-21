use toolkit::core::*;
use toolkit::draw::Drawing;
use toolkit::canvas::Canvas;
use toolkit::style::Style;

pub trait Rendering {
    fn render(&self, canvas: Canvas);
}

fn sum_width<T: HasWidth>(widgets: &[T]) -> u16 {
    widgets.iter().fold(0, |acc, w| acc + w.width())
}

fn max_width<T: HasWidth>(widgets: &[T]) -> u16 {
    widgets.iter().map(|w| w.width()).max().unwrap_or(0)
}

fn sum_height<T: HasHeight>(widgets: &[T]) -> u16 {
    widgets.iter().fold(0, |acc, w| acc + w.height())
}

fn max_height<T: HasHeight>(widgets: &[T]) -> u16 {
    widgets.iter().map(|w| w.height()).max().unwrap_or(0)
}

#[derive(Debug)]
pub struct VerticalWidget<T: HasHeight+Rendering> {
    pub children: Vec<T>
}

impl<T: HasHeight+Rendering> Rendering for VerticalWidget<T> {
    fn render(&self, mut canvas: Canvas) {
        for widget in self.children.iter() {
            if widget.height() > canvas.height() {
                break
            }
            let (widget_canvas, canvas) = canvas.split_vert(widget.height());
            widget.render(widget_canvas);
        }
        // canvas.fill(self.style())
    }
}

impl<T: HasHeight+Rendering> HasHeight for VerticalWidget<T> {
    fn height(&self) -> u16 {
        sum_height(&self.children)
    }
}

impl<T: HasHeight+Rendering+HasWidth> HasWidth for VerticalWidget<T> {
    fn width(&self) -> u16 {
        max_width(&self.children)
    }
}

#[derive(Debug)]
pub struct HorizontalWidget<T: HasWidth+Rendering> {
    pub children: Vec<T>
}

impl<T> Rendering for HorizontalWidget<T> where T: HasWidth+Rendering {
    fn render(&self, mut canvas: Canvas) {
        for widget in self.children.iter() {
            if widget.width() > canvas.width() {
                break
            }
            let (widget_canvas, tail) = canvas.split_horz(widget.width());
            widget.render(widget_canvas);
            canvas = tail;
        }
        // canvas.fill(self.style())
    }
}

impl<T> HasHeight for HorizontalWidget<T> where T: HasWidth+Rendering+HasHeight {
    fn height(&self) -> u16 {
        max_height(&self.children)
    }
}

impl<T> HasWidth for HorizontalWidget<T> where T: HasWidth+Rendering {
    fn width(&self) -> u16 {
        sum_width(&self.children)
    }
}


pub struct Decorator<T> {
    pub style: Style,
    pub item: T
}

impl<T> HasWidth for Decorator<T> where T: HasWidth {
    fn width(&self) -> u16 {
        self.item.width()
    }
}

impl<T> HasHeight for Decorator<T> where T: HasHeight {
    fn height(&self) -> u16 {
        self.item.height()
    }
}

impl<T> Rendering for Decorator<T> where T: Rendering {
    fn render(&self, canvas: Canvas) {
        let _style_context = self.style.context();
        self.item.render(canvas);
    }
}
