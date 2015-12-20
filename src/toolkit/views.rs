use std::fmt::Debug;

use toolkit::canvas::Canvas;
use toolkit::draw::*;

pub trait Widget<'a> {
    type Context;
    type View: View + 'a;

    fn view(&'a self, context: &Self::Context) -> Self::View;
}

pub trait View {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn render(&self, canvas: Canvas);
}
