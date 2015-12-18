use std::fmt::Debug;

use toolkit::canvas::Canvas;
use toolkit::draw::*;
use toolkit::style::Style;

pub trait View<C>: Debug where C: Debug {
    fn width(&self, context: &C) -> usize;
    fn height(&self, context: &C) -> usize;
    fn render(&self, context: &C, canvas: Canvas);
}

pub fn sum_width<'c, C, V, I>(context: &C, views: I) -> usize
    where V: View<C> + 'c,
          I: Iterator<Item = &'c V>,
          C: Debug
{
    let mut r = 0;
    for v in views {
        r += v.width(context);
    }
    return r;
}

pub fn max_width<'c, C, V, I>(context: &C, views: I) -> usize
    where V: View<C> + 'c,
          I: Iterator<Item = &'c V>,
          C: Debug
{
    let mut r = 0;
    for v in views {
        r = ::std::cmp::max(v.width(context), r);
    }
    return r;
}
