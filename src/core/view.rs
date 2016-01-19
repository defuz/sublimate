use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead, Error as IoError};

use core::syntax::{Parser, ParserState, Highlighter, HighlightIterator};

#[derive(Debug)]
pub struct View {
    path: Option<PathBuf>,
    lines: Vec<Line>
}

#[derive(Debug)]
pub struct Line {
    text: String,
    parser_state: ParserState,
}

impl Line {
    fn new(text: String) -> Line {
        Line {
            text: text,
            parser_state: ParserState::new()
        }
    }

    pub fn parse(&mut self, parser: &mut Parser, state: &mut ParserState) {
        self.parser_state = state.clone();
        parser.parse(&self.text, state);
        self.parser_state.swap_changes(state);
    }

    pub fn highlight<'a>(&'a self, highlighter: &'a Highlighter) -> HighlightIterator<'a> {
        HighlightIterator::new(
            self.parser_state.scope_path.clone(),
            &self.parser_state.changes,
            &self.text,
            highlighter
        )
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

    pub fn parse(&mut self, parser: &mut Parser) {
        let mut state = ParserState::new();
        for line in self.lines.iter_mut() {
            line.parse(parser, &mut state);
        }
    }
}
