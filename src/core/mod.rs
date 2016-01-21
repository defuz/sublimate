pub mod settings;
pub mod menu;
mod workspace;
mod packages;
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
use core::syntax::{Parser, Highlighter};

#[derive(Debug)]
pub struct Core {
    pub package_repository: PackageRepository,
    pub project: Project,
    pub hotkeys: HotkeyPerformer,
    pub view: View
}

impl Core {

    pub fn load() -> Core {
        let packages_path = PathBuf::from("/Users/defuz/Projects/sublimate/packages/");
        let view_path = PathBuf::from("/Users/defuz/Projects/sublimate/src/core/syntax/builder.rs");
        let project_path = PathBuf::from("/Users/defuz/Projects/sublimate/sublimate.sublime-project");
        let repository = PackageRepository::open(packages_path);
        let mut view = View::open(view_path).unwrap();
        let syntax = repository.get_syntax("Rust/Rust.tmLanguage").unwrap();
        let mut parser = Parser::from_syntax(syntax);
        view.parse(&mut parser);
        let mut hotkeys = HotkeyPerformer::new();
        hotkeys.add_keymap(repository.get_keymap("default/Default (OSX).sublime-keymap"));
        Core {
            project: Project::open(project_path).unwrap(),
            package_repository: repository,
            hotkeys: hotkeys,
            view: view
        }
    }

    pub fn create_highlighter(&self) -> Result<Highlighter, PackageError> {
        let theme = try!(self.package_repository.get_theme("themes/Monokai.tmTheme"));
        let highlighter = Highlighter::new(theme);
        Ok(highlighter)
    }

}
