pub use core::settings::Settings;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Option<Settings>
}
