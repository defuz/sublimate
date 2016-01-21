use std::collections::BTreeMap;
use std::path::PathBuf;
use std::io::{BufReader, Error as IoError};
use std::fs::{File};

use glob::{Pattern, PatternError};

use core::settings::{Settings, ParseSettings, SettingsError, read_json};

use self::ParseProjectError::*;

#[derive(Debug)]
pub struct Project {
    path: Option<PathBuf>,
    folders: Vec<ProjectFolder>,
    settings: Option<Settings>,
    // todo: add buils system here
}

#[derive(Debug)]
struct ProjectFolder {
    name: Option<String>,
    path: String,
    folder: Option<Folder>,
    settings: ProjectFolderSettings
}

#[derive(Debug, Default)]
struct ExcludePatterns {
    patterns: Vec<Pattern>
}

#[derive(Debug)]
struct ProjectFolderSettings {
    folder_exclude_patterns: ExcludePatterns,
    file_exclude_patterns: ExcludePatterns,
    follow_symlinks: bool
}

#[derive(Debug)]
struct Folder {
    folders: BTreeMap<String, Folder>,
    files: Vec<String>
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
                Settings::String(s) => match Pattern::new(&s) {
                    Ok(pattern) => patterns.push(pattern),
                    Err(err) => return Err(IncorrectPattern(err))
                },
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

        let path = match obj.remove("name") {
            Some(Settings::String(s)) => s,
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

        Ok(ProjectFolder {
            name: name,
            path: path,
            folder: None,
            settings: ProjectFolderSettings {
                follow_symlinks: follow_symlinks,
                file_exclude_patterns: file_exclude_patterns,
                folder_exclude_patterns: folder_exclude_patterns
            }
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

        let mut folders = Vec::new();

        for folder_settings in folders_arr {
            folders.push(try!(ProjectFolder::parse_settings(folder_settings)))
        }

        let settings = obj.remove("settings");

        Ok(Project {
            path: None,
            folders: folders,
            settings: settings
        })
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
        Ok(project)
    }
}
