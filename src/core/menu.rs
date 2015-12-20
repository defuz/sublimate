use super::settings::{Settings, FromSettings, ParseSettings};
use core::command::{Command, ParseCommandError};

use self::ParseMenuError::*;

pub type Menu = Vec<MenuItem>;

#[derive(Debug)]
pub enum MenuItem {
    Button(Option<String>, Command, bool),
    Group(String, Menu),
    Divider,
}

pub enum ParseMenuError {
    ItemIsNotObject,
    CaptionIsNotString,
    CaptionIsNotDefinedForGroup,
    CheckboxIsNotBoolean,
    CommandError(ParseCommandError)
}

impl ParseSettings for MenuItem {
    type Error = ParseMenuError;
    fn parse_settings(settings: Settings) -> Result<MenuItem, Self::Error> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(ItemIsNotObject),
        };
        let caption = match obj.remove("caption") {
            Some(Settings::String(caption)) => {
                if caption == "-" {
                    // TODO: check obj is empty
                    return Ok(MenuItem::Divider);
                }
                Some(caption)
            },
            None => None,
            _ => return Err(CaptionIsNotString)
        };
        // parse group
        if let Some(settings) = obj.remove("children") {
            let caption = match caption {
                Some(caption) => caption,
                None => return Err(CaptionIsNotDefinedForGroup)
            };
            // TODO: check obj is empty
            return Ok(MenuItem::Group(caption, Menu::from_settings(settings)));
        };
        // parse button
        let is_checkbox = match obj.remove("checkbox") {
            Some(Settings::Boolean(v)) => v,
            None => false,
            _ => return Err(CheckboxIsNotBoolean)
        };
        let command = match Command::parse_settings(Settings::Object(obj)) {
            Ok(command) => command,
            Err(err) => return Err(CommandError(err))
        };
        // TODO: check obj is empty
        Ok(MenuItem::Button(caption, command, is_checkbox))
    }
}

impl FromSettings for Menu {
    fn from_settings(settings: Settings) -> Menu {
        let arr = match settings {
            Settings::Array(arr) => arr,
            _ => {
                // TODO: warning
                return Menu::default();
            }
        };
        let mut menu = Menu::new();
        for settings in arr {
            match MenuItem::parse_settings(settings) {
                Ok(item) => menu.push(item),
                Err(_) => {
                    // TODO: warning
                }
            }
        }
        menu
    }
}
