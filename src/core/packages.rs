use std::io::{Error as IoError, BufReader};
use std::fs::File;
use std::path::{Path, PathBuf};

use core::settings::{Settings, SettingsError, read_json, read_plist, ParseSettings};
use core::menu::{Menu, ParseMenuError};
use core::bindings::{Keymap, ParseKeymapError};
use core::syntax::{Syntax, ParseSyntaxError, Theme, ParseThemeError};

#[derive(Debug)]
pub struct PackageRepository {
    path: PathBuf
}

#[derive(Debug)]
pub enum PackageError {
    ReadSettings(SettingsError),
    ParseTheme(ParseThemeError),
    ParseSyntax(ParseSyntaxError),
    ParseKeymap(ParseKeymapError),
    ParseMenu(ParseMenuError),
    Io(IoError)
}

impl From<SettingsError> for PackageError {
    fn from(error: SettingsError) -> PackageError {
        PackageError::ReadSettings(error)
    }
}

impl From<ParseThemeError> for PackageError {
    fn from(error: ParseThemeError) -> PackageError {
        PackageError::ParseTheme(error)
    }
}

impl From<ParseSyntaxError> for PackageError {
    fn from(error: ParseSyntaxError) -> PackageError {
        PackageError::ParseSyntax(error)
    }
}

impl From<ParseKeymapError> for PackageError {
    fn from(error: ParseKeymapError) -> PackageError {
        PackageError::ParseKeymap(error)
    }
}

impl From<ParseMenuError> for PackageError {
    fn from(error: ParseMenuError) -> PackageError {
        PackageError::ParseMenu(error)
    }
}

impl From<IoError> for PackageError {
    fn from(error: IoError) -> PackageError {
        PackageError::Io(error)
    }
}

impl PackageRepository {
    pub fn open(path: PathBuf) -> PackageRepository {
        PackageRepository { path: path }
    }

    pub fn read_file(&self, path: &Path) -> Result<BufReader<File>, PackageError> {
        let reader = try!(File::open(self.path.join(path)));
        Ok(BufReader::new(reader))
    }

    pub fn read_json(&self, path: &Path) -> Result<Settings, PackageError> {
        Ok(try!(read_json(try!(self.read_file(path)))))
    }

    pub fn read_plist(&self, path: &Path) -> Result<Settings, PackageError> {
        Ok(try!(read_plist(try!(self.read_file(path)))))
    }

    pub fn get_menu<P: AsRef<Path>>(&self, path: P) -> Result<Menu, PackageError> {
        Ok(try!(Menu::parse_settings(try!(self.read_json(path.as_ref())))))
    }

    pub fn get_keymap<P: AsRef<Path>>(&self, path: P) -> Result<Keymap, PackageError> {
        Ok(try!(Keymap::parse_settings(try!(self.read_json(path.as_ref())))))
    }

    pub fn get_theme<P: AsRef<Path>>(&self, path: P) -> Result<Theme, PackageError> {
        Ok(try!(Theme::parse_settings(try!(self.read_plist(path.as_ref())))))
    }

    pub fn get_syntax<P: AsRef<Path>>(&self, path: P) -> Result<Syntax, PackageError> {
        Ok(try!(Syntax::parse_settings(try!(self.read_plist(path.as_ref())))))
    }
}
