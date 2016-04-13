pub mod settings;
pub mod menu;
pub mod workspace;
mod packages;
mod preferences;
pub mod command;
pub mod syntax;
pub mod regex;
pub mod bindings;
pub mod view;

use std::path::PathBuf;

use core::workspace::Project;
use core::packages::{PackageRepository, PackageError};
use core::bindings::HotkeyPerformer;
use core::view::View;
use core::menu::Menu;
use core::syntax::{Parser, Highlighter};

#[derive(Debug)]
pub struct Core {
    pub package_repository: PackageRepository,
    pub project: Project,
    pub hotkeys: HotkeyPerformer,
    pub view: View
}

impl Core {

    pub fn load(packages_path_str: &str, file_path_str: &str, project_path_str: &str) -> Core {
        let packages_path = PathBuf::from(packages_path_str);
        let view_path = PathBuf::from(file_path_str);
        let project_path = PathBuf::from(project_path_str);
        let repository = PackageRepository::open(packages_path);
        let mut view = View::open(view_path).unwrap();
        let syntax = repository.get_syntax("Rust/Rust.tmLanguage").unwrap();
        let mut parser = Parser::from_syntax(syntax);
        view.parse(&mut parser);
        let mut hotkeys = HotkeyPerformer::new();
        // TODO: fix unwrap
        hotkeys.add_keymap(repository.get_keymap("default/Default (OSX).sublime-keymap").unwrap());
        Core {
            project: Project::open(project_path).unwrap(),
            package_repository: repository,
            hotkeys: hotkeys,
            view: view
        }
    }

    pub fn create_menu(&self) -> Menu {
        // todo: fix unwrap
        self.package_repository.get_menu("default/Main.sublime-menu").unwrap()
    }

    pub fn create_highlighter(&self) -> Result<Highlighter, PackageError> {
        let theme = try!(self.package_repository.get_theme("themes/Twilight.tmTheme"));
        let highlighter = Highlighter::new(theme);
        Ok(highlighter)
    }

}
