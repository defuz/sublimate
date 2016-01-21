use core::Core;

#[derive(Debug)]
pub struct Sidebar {
    root: ProjectTreeFolder
}

#[derive(Debug)]
struct ProjectTreeFolder {
    opened: bool,
    name: String,
    folders: Vec<ProjectTreeFolder>,
    files: Vec<String>
}

#[derive(Debug)]
struct ProjectTreeEntryView<'a> {
    level: u8,
    name: &'a str,
    view_type: ProjectTreeEntryViewType
}

#[derive(Debug)]
enum ProjectTreeEntryViewType {
    OpenedFolder,
    ClosedFolder,
    File
}

impl Sidebar {
    pub fn new(core: &Core) -> Sidebar {
        unimplemented!();
        // folders = Vec::new();
        // for folder in core.project.folders {

        // }
    }
}
