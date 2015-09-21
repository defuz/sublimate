use weakjson::from_str;
use std::io::Read;
use std::fs::File;
use std::path::{Path, PathBuf};

use core::settings::Settings;
use core::menu::Menu;

#[derive(Debug)]
pub struct PackageRepository {
    path: PathBuf
}

impl PackageRepository {

    pub fn open(path: &str) -> PackageRepository {
        PackageRepository { path: PathBuf::from(path) }
    }

    pub fn load_settings(&self, path: &Path) -> Option<Settings> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return None
        };
        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Ok(_) => {},
            Err(_) => return None
        }
        match from_str(&data) {
            Ok(settings) => Some(settings),
            Err(_) => None
        }
    }

    pub fn get_menu<M: Menu>(&self, filename: &str) -> M {
        M::from_json(self.load_settings(self.path.join(filename).as_path()))
    }
}
