use std::hash::{Hash, Hasher};
use core::settings::{Settings, SettingsObject, ParseSettings};

use self::ParseCommandError::*;

#[derive(Debug, PartialEq, Clone)]
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
    CommandArgsIsNotObject,
}

impl ParseSettings for Command {
    type Error = ParseCommandError;
    fn parse_settings(settings: Settings) -> Result<Command, Self::Error> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(CommandIsNotObject),
        };

        let name = match obj.remove("command") {
            Some(Settings::String(name)) => name,
            _ => return Err(CommandNameIsNotString),
        };

        let args = match obj.remove("args") {
            Some(Settings::Object(args)) => args,
            None => SettingsObject::default(),
            _ => return Err(CommandArgsIsNotObject),
        };

        // TODO: check obj is empty

        Ok(Command {
            name: name,
            args: args
        })
    }
}
