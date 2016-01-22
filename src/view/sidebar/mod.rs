use core::Core;

mod tree;

pub use self::tree::{ProjectEntries, ProjectEntriesView};

use toolkit::*;

#[derive(Debug)]
pub struct Sidebar {
    project_tree: ProjectEntries
}


impl Sidebar {
    pub fn new(core: &Core) -> Sidebar {
        Sidebar {
            project_tree: ProjectEntries::from_project(&core.project)
        }
    }
}

impl<'a> Widget<'a> for Sidebar {
    type Context = &'a Core;
    type View = ProjectEntriesView<'a>;

    fn view(&'a self, _: &'a Core) -> ProjectEntriesView<'a> {
        self.project_tree.view(())
    }
}
