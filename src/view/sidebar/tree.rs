use unicode_width::UnicodeWidthStr;

use std::slice::{Iter as SliceIter};

use core::workspace::{Project, Folder};

use toolkit::*;
use view::theme::*;

#[derive(Debug)]
pub struct ProjectEntries {
    entries: Vec<ProjectEntry>
}

pub struct ProjectEntriesIterator<'a> {
    path: Vec<SliceIter<'a, ProjectEntry>>
}

#[derive(Debug)]
pub enum ProjectEntry {
    File(ProjectFile),
    Folder(ProjectFolder)
}

#[derive(Debug)]
pub struct ProjectFolder {
    name: String,
    opened: bool,
    entries: ProjectEntries
}

#[derive(Debug)]
pub struct ProjectFile {
    name: String
}

#[derive(Debug)]
pub struct ProjectEntriesView<'a> {
    entries: Vec<ProjectEntryView<'a>>
}

#[derive(Debug)]
pub struct ProjectEntryView<'a> {
    level: u8,
    name: &'a str,
    entry_type: ProjectEntryType
}

#[derive(Debug)]
enum ProjectEntryType {
    OpenedFolder,
    ClosedFolder,
    File
}

impl ProjectEntry {
    pub fn file(name: String) -> ProjectEntry {
        let file = ProjectFile {
            name: name
        };
        ProjectEntry::File(file)
    }

    pub fn folder(name: String, folder: &Folder) -> ProjectEntry {
        let folder = ProjectFolder {
            name: name,
            opened: true,
            entries: ProjectEntries::from_folder(folder)
        };
        ProjectEntry::Folder(folder)
    }
}

impl ProjectEntries {
    pub fn from_project(project: &Project) -> ProjectEntries {
        let mut entries = Vec::new();
        for pf in &project.folders {
            entries.push(ProjectEntry::folder(pf.name().into_owned(), &pf.folder));
        }
        ProjectEntries {
            entries: entries
        }
    }

    pub fn from_folder(folder: &Folder) -> ProjectEntries {
        let mut entries = Vec::new();
        for (name, folder) in &folder.folders {
            entries.push(ProjectEntry::folder(name.to_owned(), &folder))
        }
        for name in &folder.files {
            entries.push(ProjectEntry::file(name.to_owned()))
        }
        ProjectEntries {
            entries: entries
        }
    }

    pub fn iter<'a>(&'a self) -> SliceIter<'a, ProjectEntry> {
        self.entries.iter()
    }

    pub fn views<'a>(&'a self) -> ProjectEntriesIterator<'a> {
        ProjectEntriesIterator {
            path: vec![self.iter()]
        }
    }
}

impl<'a> Iterator for ProjectEntriesIterator<'a> {
    type Item = ProjectEntryView<'a>;

    fn next(&mut self) -> Option<ProjectEntryView<'a>> {
        let next = match self.path.last_mut() {
            Some(iter) => iter.next(),
            None => return None
        };
        match next {
            Some(entry) => {
                let level = self.path.len() as u8 - 1;
                let r = match *entry {
                    ProjectEntry::Folder(ref folder) => ProjectEntryView {
                        level: level,
                        name: &folder.name,
                        entry_type: if folder.opened {
                            self.path.push(folder.entries.iter());
                            ProjectEntryType::OpenedFolder
                        } else {
                            ProjectEntryType::ClosedFolder
                        }
                    },
                    ProjectEntry::File(ref file) => ProjectEntryView {
                        level: level,
                        name: &file.name,
                        entry_type: ProjectEntryType::File
                    },
                };
                Some(r)
            }
            None => {
                self.path.pop();
                self.next()
            }
        }
    }
}

impl<'a> Widget<'a> for ProjectEntries {
    type Context = ();
    type View = ProjectEntriesView<'a>;

    fn view(&'a self, _: ()) -> ProjectEntriesView<'a> {
        ProjectEntriesView {
            entries: self.views().collect()
        }
    }
}

impl<'a> View for ProjectEntriesView<'a> {
    fn width(&self) -> usize {
        self.entries.iter().map(|v| v.width()).max().unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.entries.len()
    }

    fn render(&self, mut canvas: Canvas) {
        for view in self.entries.iter() {
            let h = view.height();
            if h > canvas.height() {
                break;
            }
            view.render(canvas.cut_top(h));
        }
    }
}

impl<'a> ProjectEntryView<'a> {
    fn style(&self) -> Style {
        match self.entry_type {
            ProjectEntryType::File => SIDEBAR_LOW_STYLE,
            ProjectEntryType::OpenedFolder |
            ProjectEntryType::ClosedFolder => SIDEBAR_STYLE,
        }
    }

    fn marker(&self) -> &'static str {
        match self.entry_type {
            ProjectEntryType::File => "  ",
            ProjectEntryType::OpenedFolder => "▾ ",
            ProjectEntryType::ClosedFolder => "▸ ",
        }
    }
}

impl<'a> View for ProjectEntryView<'a> {
    fn width(&self) -> usize {
        self.name.width() + self.level as usize * 2
    }

    fn height(&self) -> usize {
        1
    }

    fn render(&self, mut canvas: Canvas) {
        canvas.style(SIDEBAR_LOW_STYLE);
        canvas.cut_left(2 * self.level as usize + 1).fill();
        canvas.cut_left(2).text(self.marker(), 0, 0);
        canvas.style(SIDEBAR_STYLE);
        canvas.cut_left(self.name.width()).text(self.name, 0, 0);
        canvas.fill();
    }
}

