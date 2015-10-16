use core::build::BuildSystem;
use core::settings::Settings;

#[derive(Debug)]
struct Folder {
    name: String,
    folders: Vec<Folder>,
    files: Vec<String>,
}

#[derive(Debug)]
struct ProjectFolder {
    path: String,
    folder_exclude_patterns: Vec<String>,
    file_exclude_patterns: Vec<String>,
    follow_symlinks: bool,
}

#[derive(Debug)]
pub struct Project {
    path: Option<String>,
    folders: Vec<ProjectFolder>,
    settings: Option<Settings>,
    build_systems: Vec<BuildSystem>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            path: None,
            folders: Vec::new(),
            settings: None,
            build_systems: Vec::new(),
        }
    }

    pub fn open(path: &str) -> Project {
        Project {
            path: None,
            folders: Vec::new(),
            settings: None,
            build_systems: Vec::new(),
        }
    }
}
