
pub trait HasSize {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub trait HasChildren {
    type Item;
    fn children(&self) -> &[Self::Item];
}
