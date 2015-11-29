pub use rustc_serialize::json::Json as Settings;
pub use rustc_serialize::json::Array as SettingsArray;
pub use rustc_serialize::json::Object as SettingsObject;

pub trait FromSettings : Sized {
    fn from_settings(settings: Settings) -> Self;
}

pub trait ParseSettings : Sized {
    type Error;
    fn parse_settings(settings: Settings) -> Result<Self, Self::Error>;
}
