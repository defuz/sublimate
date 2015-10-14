pub use rustc_serialize::json::Json as Settings;
pub use rustc_serialize::json::JsonArray as SettingsArray;
pub use rustc_serialize::json::JsonObject as SettingsObject;

trait FromSettings {
    fn from_settings(settings: Settings) -> Self;
}

trait TryFromSettings {
    type Error;
    fn try_from_settings(settings: Settings) -> Result<Self, Self::Error>
}

impl TryFromSettings for T where T: FromSettings {
    type Error = ();
    fn try_from_settings(settings: Settings) -> Result<Self, Self::Error> {
        Ok(Self::from_settings(settings))
    }
}
