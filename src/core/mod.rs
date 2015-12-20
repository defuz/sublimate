pub mod settings;
pub mod menu;
mod project;
mod packages;
mod build;
pub mod command;
pub mod keymap;
pub mod context;
pub mod performer;

use core::project::Project;
use core::packages::PackageRepository;
use core::performer::HotkeyPerformer;

#[derive(Debug)]
pub struct Core {
    pub package_repository: PackageRepository,
    pub project: Project,
    pub hotkeys: HotkeyPerformer
}

impl Core {

    pub fn load() -> Core {
        let repository = PackageRepository::open("/Users/defuz/Projects/sublimate/packages/");
        let mut hotkeys = HotkeyPerformer::new();
        hotkeys.add_keymap(repository.get_keymap("default/Default (OSX).sublime-keymap"));
        Core {
            project: Project::open("/Users/defuz/Projects/sublimate/sublimate.sublime-project"),
            package_repository: repository,
            hotkeys: hotkeys
        }
    }


}
