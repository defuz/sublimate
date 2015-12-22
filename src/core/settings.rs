use std::io::Read;

pub use rustc_serialize::json::Json as Settings;
pub use rustc_serialize::json::Array as SettingsArray;
pub use rustc_serialize::json::Object as SettingsObject;

use plist::xml::StreamingParser;

pub trait FromSettings : Sized {
    fn from_settings(settings: Settings) -> Self;
}

pub trait ParseSettings : Sized {
    type Error;
    fn parse_settings(settings: Settings) -> Result<Self, Self::Error>;
}

pub trait FromPlist<R: Read> : Sized {
    fn from_plist(parser: &mut StreamingParser<R>) -> Self;
}

pub trait ParsePlist<R: Read> : Sized {
    type Error;
    fn parse_plist(parser: &mut StreamingParser<R>) -> Result<Self, Self::Error>;
}
