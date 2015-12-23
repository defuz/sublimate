use std::io::{Read, Seek};
use weakjson::from_reader as json_from_reader;
use plist::Plist;
use rustc_serialize::json::ParserError as JsonError;

pub use rustc_serialize::json::Json as Settings;
pub use rustc_serialize::json::Array as SettingsArray;
pub use rustc_serialize::json::Object as SettingsObject;

type PlistError = (); /// FIXME: change this bullshit

pub trait FromSettings : Sized {
    fn from_settings(settings: Settings) -> Self;
}

pub trait ParseSettings : Sized {
    type Error;
    fn parse_settings(settings: Settings) -> Result<Self, Self::Error>;
}

#[derive(Debug)]
pub enum SettingsError {
    Plist(PlistError),
    Json(JsonError)
}

impl From<PlistError> for SettingsError {
    fn from(error: PlistError) -> SettingsError {
        SettingsError::Plist(error)
    }
}

impl From<JsonError> for SettingsError {
    fn from(error: JsonError) -> SettingsError {
        SettingsError::Json(error)
    }
}

pub fn read_json<R: Read>(mut reader: R) -> Result<Settings, SettingsError> {
    Ok(try!(json_from_reader(&mut reader as &mut Read)))
}

pub fn read_plist<R: Read+Seek>(reader: R) -> Result<Settings, SettingsError> {
    Ok(try!(Plist::read(reader)).into_rustc_serialize_json())
}
