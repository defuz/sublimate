use std::convert::From;
use std::hash::{Hash, Hasher};
use core::settings::{Settings, SettingsObject, FromSettings};

use self::ParseCommandError::*;

#[derive(Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub args: SettingsObject
}

impl Eq for Command {}

impl Hash for Command {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub enum ParseCommandError {
    CommandIsNotObject,
    CommandNameIsNotString,
    CommandArgsIsNotObject
}

impl FromSettings for Command {
    type Error = ParseCommandError;
    fn from_settings(settings: Settings) -> Result<Command, Self::Error> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(CommandIsNotObject),
        };

        let name = match obj.remove("name") {
            Some(Settings::String(name)) => name,
            _ => return Err(CommandNameIsNotString)
        };

        let args = match obj.remove("args") {
            Some(Settings::Object(args)) => args,
            None => SettingsObject::default(),
            _ => return Err(CommandArgsIsNotObject)
        };

        Ok(Command { name: name, args: args})
    }
}
