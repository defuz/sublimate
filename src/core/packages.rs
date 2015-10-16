use weakjson::from_str;
use std::io::Read;
use std::fs::File;
use std::path::{Path, PathBuf};

use core::settings::Settings;
use core::menu::Menu;
use core::keymap::Keymap;

#[derive(Debug)]
pub struct PackageRepository {
    path: PathBuf
}

impl PackageRepository {
    pub fn open(path: &str) -> PackageRepository {
        PackageRepository { path: PathBuf::from(path) }
    }

    pub fn load_settings(&self, filename: &str) -> Option<Settings> {
        let mut file = match File::open(self.path.join(filename)) {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Ok(_) => {}
            Err(_) => return None,
        }
        match from_str(&data) {
            Ok(settings) => Some(settings),
            Err(_) => None,
        }
    }

    pub fn get_menu(&self, filename: &str) -> Menu {
        self.load_settings(filename).map_or_else(Menu::default, Menu::from)
    }

    pub fn get_keymap(&self, filename: &str) -> Keymap {
        self.load_settings(filename).map_or_else(Keymap::default, Keymap::from)
    }
}
