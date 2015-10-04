use toolkit::style::Style;

pub trait Drawing {
    fn char(&self, c: char, x: usize, y: usize);

    fn text(&self, s: &str, x: usize, y: usize);

    fn fill(&self);

}
