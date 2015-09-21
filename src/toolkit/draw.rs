use toolkit::core::size;
use toolkit::style::Style;

pub trait Drawing {
    fn char(&self, c: char, x: size, y: size);

    fn text(&self, s: &str, x: size, y: size);

    fn fill(&self);

}
