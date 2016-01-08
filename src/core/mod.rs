pub mod settings;
pub mod menu;
mod project;
mod packages;
mod build;
pub mod command;
pub mod syntax;
pub mod regex;
pub mod bindings;
pub mod view;

use std::path::PathBuf;

use core::project::Project;
use core::packages::PackageRepository;
use core::bindings::HotkeyPerformer;
use core::view::View;

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
        let view_path = PathBuf::from("/Users/defuz/Projects/sublimate/src/core/color_scheme.rs");
        let repository = PackageRepository::open(packages_path);
        let mut hotkeys = HotkeyPerformer::new();
        hotkeys.add_keymap(repository.get_keymap("default/Default (OSX).sublime-keymap"));
        Core {
            project: Project::open("/Users/defuz/Projects/sublimate/sublimate.sublime-project"),
            package_repository: repository,
            hotkeys: hotkeys,
            view: View::open(view_path).unwrap()
        }
    }


}
