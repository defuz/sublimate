mod settings;
pub mod menu;
mod project;
mod packages;
mod build;
pub mod command;
pub mod keymap;
pub mod context;

use core::menu::Menu;
use core::project::Project;
use core::packages::PackageRepository;

#[derive(Debug)]
pub struct Core {
    pub package_repository: PackageRepository,
    pub project: Project
}

impl Core {

    pub fn load() -> Core {
        let repository = PackageRepository::open("/Users/defuz/Projects/sublimate/packages/");
        Core {
            project: Project::open("/Users/defuz/Projects/sublimate/sublimate.sublime-project"),
            package_repository: repository
        }
    }


}
