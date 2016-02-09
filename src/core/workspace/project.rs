use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::io::{BufReader, Error as IoError};
use std::fs::{File, Metadata, read_dir, metadata, symlink_metadata};

use glob::{Pattern, PatternError};

use core::settings::{Settings, ParseSettings, SettingsError, read_json};

use self::ParseProjectError::*;

#[derive(Debug)]
pub struct Project {
    pub path: Option<PathBuf>,
    pub folders: Vec<ProjectFolder>,
    pub settings: Option<Settings>,
    // todo: add buils system here
}

#[derive(Debug)]
pub struct ProjectFolder {
    name: Option<String>,
    pub path: PathBuf,
    pub folder: Folder,
    settings: ProjectFolderSettings
}

#[derive(Debug, Default)]
pub struct ExcludePatterns {
    patterns: Vec<Pattern>
}

#[derive(Debug)]
struct ProjectFolderSettings {
    folder_exclude_patterns: ExcludePatterns,
    file_exclude_patterns: ExcludePatterns,
    follow_symlinks: bool
}

#[derive(Debug, Default)]
pub struct Folder {
    pub folders: BTreeMap<String, Folder>,
    pub files: Vec<String>
}

#[derive(Debug)]
pub enum ProjectError {
    Settings(SettingsError),
    Parse(ParseProjectError),
    Io(IoError)
}

#[derive(Debug)]
pub enum ParseProjectError {
    ProjectIsNotObject,
    ProjectFoldersIsNotArray,
    ProjectFolderIsNotObject,
    FolderNameIsNotString,
    FolderPathIsNotString,
    FollowSymlinksIsNotBoolean,
    ExcludePatternsIsNotArray,
    PatternIsNotString,
    IncorrectPattern(PatternError)
}

impl From<PatternError> for ParseProjectError {
    fn from(error: PatternError) -> ParseProjectError {
        IncorrectPattern(error)
    }
}

impl From<ParseProjectError> for ProjectError {
    fn from(error: ParseProjectError) -> ProjectError {
        ProjectError::Parse(error)
    }
}

impl From<SettingsError> for ProjectError {
    fn from(error: SettingsError) -> ProjectError {
        ProjectError::Settings(error)
    }
}

impl From<IoError> for ProjectError {
    fn from(error: IoError) -> ProjectError {
        ProjectError::Io(error)
    }
}

impl ParseSettings for ExcludePatterns {
    type Error = ParseProjectError;

    fn parse_settings(settings: Settings) -> Result<ExcludePatterns, ParseProjectError> {
        let arr = match settings {
            Settings::Array(arr) => arr,
            _ => return Err(ExcludePatternsIsNotArray)
        };

        let mut patterns = Vec::new();

        for pattern in arr {
            match pattern {
                Settings::String(s) => patterns.push(try!(Pattern::new(&s))),
                _ => return Err(PatternIsNotString)
            }
        }

        Ok(ExcludePatterns {
            patterns: patterns
        })
    }
}

impl ParseSettings for ProjectFolder {
    type Error = ParseProjectError;

    fn parse_settings(settings: Settings) -> Result<ProjectFolder, ParseProjectError> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(ProjectFolderIsNotObject)
        };

        let name = match obj.remove("name") {
            Some(Settings::String(s)) => Some(s),
            None => None,
            Some(_) => return Err(FolderNameIsNotString)
        };

        let path = match obj.remove("path") {
            Some(Settings::String(s)) => PathBuf::from(s),
            _ => return Err(FolderPathIsNotString)
        };

        let folder_exclude_patterns = match obj.remove("folder_exclude_patterns") {
            Some(obj) => try!(ExcludePatterns::parse_settings(obj)),
            None => ExcludePatterns::default()
        };

        let file_exclude_patterns = match obj.remove("file_exclude_patterns") {
            Some(obj) => try!(ExcludePatterns::parse_settings(obj)),
            None => ExcludePatterns::default()
        };

        let follow_symlinks = match obj.remove("follow_symlinks") {
            Some(Settings::Boolean(v)) => v,
            None => false,
            Some(..) => return Err(FollowSymlinksIsNotBoolean),
        };

        let settings = ProjectFolderSettings {
            follow_symlinks: follow_symlinks,
            file_exclude_patterns: file_exclude_patterns,
            folder_exclude_patterns: folder_exclude_patterns
        };

        Ok(ProjectFolder {
            name: name,
            path: path,
            folder: Folder::default(),
            settings: settings
        })
    }
}

impl ParseSettings for Project {
    type Error = ParseProjectError;

    fn parse_settings(settings: Settings) -> Result<Project, ParseProjectError> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(ProjectIsNotObject)
        };

        let folders_arr = match obj.remove("folders") {
            Some(Settings::Array(arr)) => arr,
            _ => return Err(ProjectFoldersIsNotArray),
        };

        let folders = try!(folders_arr.into_iter().map(ProjectFolder::parse_settings).collect());
        let settings = obj.remove("settings");

        Ok(Project {
            path: None,
            folders: folders,
            settings: settings
        })
    }
}

impl ProjectFolderSettings {
    fn metadata(&self, path: &Path) -> Result<Metadata, IoError> {
        if self.follow_symlinks {
            metadata(path)
        } else {
            symlink_metadata(path)
        }
    }

    fn file_matched(&self, path: &Path) -> bool {
        !self.file_exclude_patterns.patterns.iter().any(|p| p.matches_path(path))
    }

    fn folder_matched(&self, path: &Path) -> bool {
        !self.folder_exclude_patterns.patterns.iter().any(|p| p.matches_path(path))
    }
}

impl Folder {
    fn walk(path: &Path, settings: &ProjectFolderSettings) -> Result<Folder, IoError> {
        let mut files = Vec::new();
        let mut folders = BTreeMap::new();
        for entry in try!(read_dir(path)) {
            let entry = try!(entry);
            let path = entry.path();
            let metadata = try!(settings.metadata(&path));
            let name = entry.file_name().to_string_lossy().into_owned();
            if metadata.is_file() {
                if settings.file_matched(&path) {
                    files.push(name);
                }
            } else if metadata.is_dir() && settings.folder_matched(&path) {
                let folder = try!(Folder::walk(&path, settings));
                folders.insert(name, folder);
            }
        }
        Ok(Folder {
            files: files,
            folders: folders
        })
    }
}

impl ProjectFolder {
    pub fn name(&self) -> Cow<str> {
        match self.name {
            Some(ref s) => Cow::Borrowed(s),
            None => match self.path.file_name() {
                Some(s) => s.to_string_lossy(),
                None => self.path.to_string_lossy()
            }
        }
    }
}

impl Project {
    pub fn new() -> Project {
        Project {
            path: None,
            folders: Vec::new(),
            settings: None,
        }
    }

    pub fn open(path: PathBuf) -> Result<Project, ProjectError> {
        let file = try!(File::open(&path));
        let reader = BufReader::new(file);
        let settings = try!(read_json(reader));
        let mut project = try!(Project::parse_settings(settings));
        project.path = Some(path);
        try!(project.walk());
        Ok(project)
    }

    pub fn walk(&mut self) -> Result<(), ProjectError> {
        for pf in &mut self.folders {
            pf.folder = try!(Folder::walk(&pf.path, &pf.settings));
        }
        Ok(())
    }
}
