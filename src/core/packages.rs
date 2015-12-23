use std::io::{Read, Error as IoError, BufReader};
use std::fs::File;
use std::path::{Path, PathBuf};

use weakjson;
use rustc_serialize::json::ParserError;
use plist::xml::StreamingParser;

use core::settings::{Settings, FromSettings, FromPlist, ParsePlist, Plist};
use core::menu::Menu;
use core::keymap::Keymap;
use core::color_scheme::ColorScheme;

#[derive(Debug)]
pub struct PackageRepository {
    path: PathBuf
}

enum PackageError {
    ParseSettings(ParserError),
    Io(IoError)
}

impl From<ParserError> for PackageError {
    fn from(error: ParserError) -> PackageError {
        PackageError::ParseSettings(error)
    }
}

impl From<IoError> for PackageError {
    fn from(error: IoError) -> PackageError {
        PackageError::Io(error)
    }
}

impl PackageRepository {
    pub fn open(path: &str) -> PackageRepository {
        PackageRepository { path: PathBuf::from(path) }
    }

    pub fn read_file(&self, path: &Path) -> Result<BufReader<File>, PackageError> {
        let mut reader = try!(File::open(self.path.join(path)));
        Ok(BufReader::new(reader))
    }

    pub fn read_settings(&self, path: &Path) -> Result<Settings, PackageError> {
        let mut reader = try!(self.read_file(path));
        Ok(try!(weakjson::from_reader(&mut reader as &mut Read)))
    }

    pub fn read_plist(&self, path: &Path) -> Result<Plist, PackageError> {
        let reader = try!(self.read_file(path));
        Ok(Plist::new(reader))
    }

    pub fn get_menu<P: AsRef<Path>>(&self, path: P) -> Menu {
        match self.read_settings(path.as_ref()) {
            Ok(settings) => Menu::from_settings(settings),
            Err(..) => Menu::default()
        }
    }

    pub fn get_keymap<P: AsRef<Path>>(&self, path: P) -> Keymap {
        match self.read_settings(path.as_ref()) {
            Ok(settings) => Keymap::from_settings(settings),
            Err(..) => Keymap::default()
        }
    }

    pub fn get_color_scheme<P: AsRef<Path>>(&self, path: P) -> ColorScheme {
        match self.read_plist(path.as_ref()) {
            Ok(ref mut plist) => match ColorScheme::parse_plist(plist) {
                Ok(color_scheme) => color_scheme,
                Err(..) => ColorScheme::default()
            },
            Err(..) => ColorScheme::default()
        }
    }
}
