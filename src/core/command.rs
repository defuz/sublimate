use std::convert::From;
use std::hash::{Hash, Hasher};
use core::settings::Settings;

#[derive(Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub args: Option<Settings>
}

impl Eq for Command {}

impl Hash for Command {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// impl From<Json>
