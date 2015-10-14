use toolkit::canvas::Canvas;
use toolkit::draw::Drawing;
use toolkit::core::*;
use toolkit::style::Style;

pub trait View<C> {
    fn width(&self, context: C) -> usize;
    fn height(&self, context: C) -> usize;
    fn render(&self, context: C, canvas: Canvas);
}

//////////////////////////////////// Text //////////////////////////////////////////////////////////

pub struct Text;

impl<'a> View<&'a str> for Text {
    fn width(&self, context: &str) -> usize {
        context.len()
    }

    fn height(&self, context: &str) -> usize {
        1
    }

    fn render(&self, context: &str, canvas: Canvas) {
        canvas.text(context.as_ref(), 0, 0);
    }
}

//////////////////////////////////// Decorator /////////////////////////////////////////////////////

pub struct Decorator<I>(pub Style, pub I);

impl<C, I> View<C> for Decorator<I> where I: View<C> {
    fn width(&self, context: C) -> usize {
        let Decorator(_, ref item) = *self;
        item.width(context)
    }

    fn height(&self, context: C) -> usize {
        let Decorator(_, ref item) = *self;
        item.height(context)
    }

    fn render(&self, context: C, canvas: Canvas) {
        let Decorator(ref style, ref item) = *self;
        let prev = Style::current();
        style.set();
        item.render(context, canvas);
        prev.set();
    }
}

//////////////////////////////////// Selected //////////////////////////////////////////////////////

pub struct Selected<I>(pub Style, pub I);

impl<C, I> View<(bool, C)> for Selected<I> where I: View<C> {
    fn width(&self, (selected, context): (bool, C)) -> usize {
        let Selected(_, ref item) = *self;
        item.width(context)
    }

    fn height(&self, (selected, context): (bool, C)) -> usize {
        let Selected(_, ref item) = *self;
        item.height(context)
    }

    fn render(&self, (selected, context): (bool, C), canvas: Canvas) {
        let Selected(ref style, ref item) = *self;
        if selected {
            let prev = Style::current();
            style.set();
            item.render(context, canvas);
            prev.set();
        } else {
            item.render(context, canvas);
        }
    }
}

//////////////////////////////////// VerticalList //////////////////////////////////////////////////

pub struct VerticalList<I>(pub I);

impl<I, R> View<R> for VerticalList<I> where R: Iterator, I: View<R::Item>, R::Item: Copy {
    fn width(&self, context: R) -> usize {
        let VerticalList(ref item) = *self;
        context.map(|c| item.width(c)).max().unwrap_or(0)
    }

    fn height(&self, context: R) -> usize {
        let VerticalList(ref item) = *self;
        context.fold(0, |acc, c| acc + item.height(c))
    }

    fn render(&self, context: R, mut canvas: Canvas) {
        let VerticalList(ref item) = *self;
        for c in context {
            let h = item.height(c);
            if h < canvas.height() {
                break
            }
            item.render(c, canvas.cut_top(h))
        }
    }
}

//////////////////////////////////// HorizontalList ////////////////////////////////////////////////

pub struct HorizontalList<I>(pub I);

impl<I, R> View<R> for HorizontalList<I> where R: Iterator, I: View<R::Item>, R::Item: Copy {
    fn width(&self, context: R) -> usize {
        let HorizontalList(ref item) = *self;
        context.fold(0, |acc, c| acc + item.width(c))
    }

    fn height(&self, context: R) -> usize {
        let HorizontalList(ref item) = *self;
        context.map(|c| item.height(c)).max().unwrap_or(0)
    }

    fn render(&self, context: R, mut canvas: Canvas) {
        let HorizontalList(ref item) = *self;
        for c in context {
            let w = item.width(c);
            if w < canvas.width() {
                break
            }
            item.render(c, canvas.cut_left(w))
        }
    }
}
