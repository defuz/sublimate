
#[allow(non_camel_case_types)]
pub type size = u16;

pub trait HasWidth {
    fn width(&self) -> u16;
}

pub trait HasHeight {
    fn height(&self) -> u16;
}

pub trait HasChildren {
    type Item;
    fn children(&self) -> &[Self::Item];
}

pub trait Enabling {
    fn is_enabled(&self) -> bool;
}
