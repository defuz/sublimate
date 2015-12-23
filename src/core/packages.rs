use std::io::{Read, Error as IoError, BufReader};
use std::fs::File;
use std::path::{Path, PathBuf};

use core::settings::{Settings, SettingsError, read_json, read_plist, FromSettings, ParseSettings};
use core::menu::Menu;
use core::keymap::Keymap;
use core::color_scheme::{ColorScheme, ParseColorSchemeError};
use core::syntax::SyntaxDefinition;

#[derive(Debug)]
pub struct PackageRepository {
    path: PathBuf
}

#[derive(Debug)]
enum PackageError {
    ReadSettings(SettingsError),
    ParseColorScheme(ParseColorSchemeError),
    Io(IoError)
}

impl From<SettingsError> for PackageError {
    fn from(error: SettingsError) -> PackageError {
        PackageError::ReadSettings(error)
    }
}

impl From<ParseColorSchemeError> for PackageError {
    fn from(error: ParseColorSchemeError) -> PackageError {
        PackageError::ParseColorScheme(error)
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

    pub fn read_json(&self, path: &Path) -> Result<Settings, PackageError> {
        Ok(try!(read_json(try!(self.read_file(path)))))
    }

    pub fn read_plist(&self, path: &Path) -> Result<Settings, PackageError> {
        Ok(try!(read_plist(try!(self.read_file(path)))))
    }

    pub fn get_menu<P: AsRef<Path>>(&self, path: P) -> Menu {
        match self.read_json(path.as_ref()) {
            Ok(settings) => Menu::from_settings(settings),
            Err(..) => Menu::default()
        }
    }

    pub fn get_keymap<P: AsRef<Path>>(&self, path: P) -> Keymap {
        match self.read_json(path.as_ref()) {
            Ok(settings) => Keymap::from_settings(settings),
            Err(..) => Keymap::default()
        }
    }

    pub fn get_color_scheme<P: AsRef<Path>>(&self, path: P) -> Result<ColorScheme, PackageError> {
        Ok(try!(ColorScheme::parse_settings(try!(self.read_plist(path.as_ref())))))
    }

    pub fn get_syntax_definition<P: AsRef<Path>>(&self, path: P) -> SyntaxDefinition {
        match self.read_plist(path.as_ref()) {
            Ok(settings) => match SyntaxDefinition::parse_settings(settings) {
                Ok(syntax_definition) => syntax_definition,
                Err(..) => SyntaxDefinition::default()
            },
            Err(..) => SyntaxDefinition::default()
        }
    }
}
