use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead, Error as IoError};

#[derive(Debug)]
pub struct View {
    path: Option<PathBuf>,
    lines: Vec<Line>
}

#[derive(Debug)]
pub struct Line {
    text: String,
    // scope_path: Vec<Scope>,
    // context_path: Vec<ContextId>,
    // changes: Vec<(usize, ScopeCommand)>
}

impl Line {
    fn new(text: String) -> Line {
        Line {
            text: text
        }
    }
}

impl View {
    pub fn new() -> View {
        View {
            path: None,
            lines: Vec::new()
        }
    }

    pub fn open(path: PathBuf) -> Result<View, IoError> {
        let mut lines = Vec::new();
        for text in BufReader::new(try!(File::open(&path))).lines() {
            lines.push(Line::new(try!(text)));
        }
        Ok(View {
            path: Some(path),
            lines: lines
        })
    }
}
