use toolkit::style::Style;

pub trait Drawing {
    fn char(&self, c: char, x: usize, y: usize);

    fn text(&self, s: &str, x: usize, y: usize);

    fn fill_char(&self, c: char);

    fn fill(&self);

}

pub trait HasSize {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub trait HasChildren {
    type Item;
    fn children(&self) -> &[Self::Item];
}
