use core::regex::{Regex, Region, OPTION_NONE};

use super::scope::{ScopePath, ScopeCommand};
use super::builder::ParserBuilder;
use super::definition::{Syntax, Captures};

pub type ContextId = usize;

#[derive(Debug)]
pub struct Parser {
    pub contexts: Vec<ParserContext>,
    pub region: Region
}

#[derive(Debug)]
pub struct ParserContext {
    pub matches: Vec<ParserMatch>,
    pub regex: Regex
}

#[derive(Debug)]
pub struct ParserMatch {
    pub before: ScopeCommand,
    pub after: ScopeCommand,
    pub context: ContextCommand,
    pub captures_len: usize,
    pub captures_map: Captures
}

#[derive(Debug, Clone, Copy)]
pub enum ContextCommand {
    Push(ContextId),
    Pop,
    Noop
}

#[derive(Debug, Clone)]
pub struct ParserState {
    context_path: Vec<ContextId>,
    pub scope_path: ScopePath,
    pub changes: Vec<(usize, ScopeCommand)>
}

impl ParserState {
    pub fn new() -> ParserState {
        ParserState {
            scope_path: Vec::new(),
            context_path: Vec::new(),
            changes: Vec::new()
        }
    }

    pub fn swap_changes(&mut self, other: &mut ParserState) {
        ::std::mem::swap(&mut self.changes, &mut other.changes);
    }

    fn change_scope(&mut self, pos: usize, command: ScopeCommand) {
        match command {
            ScopeCommand::Push(ref scope) => {
                self.scope_path.push(scope.clone())
            },
            ScopeCommand::Pop => {
                self.scope_path.pop();
            },
            ScopeCommand::Noop => return
        };
        let mut index = self.changes.len();
        while index > 0 && self.changes[index - 1].0 > pos {
            index -= 1;
        }
        self.changes.insert(index, (pos, command))
    }

    fn change_context(&mut self, command: ContextCommand) {
        match command {
            ContextCommand::Push(id) => {
                self.context_path.push(id);
            },
            ContextCommand::Pop => {
                self.context_path.pop();
            },
            ContextCommand::Noop => ()
        }
    }
}

impl Parser {
    pub fn new(contexts: Vec<ParserContext>) -> Parser {
        Parser {
            contexts: contexts,
            region: Region::new()
        }
    }

    pub fn from_syntax(mut syntax: Syntax) -> Parser {
        ParserBuilder::new().build(&mut syntax)
    }

    pub fn parse(&mut self, text: &str, state: &mut ParserState) {
        let mut pos = 0;
        while pos < text.len() {
            let context = match state.context_path.last() {
                Some(id) => &self.contexts[*id],
                None => &self.contexts[0]
            };
            self.region.clear();
            let r = context.regex.search_with_region(&text[pos..], &mut self.region, OPTION_NONE);
            if r.unwrap().is_some() {
                let (_, end) = self.region.pos(0).unwrap();
                if end == 0 {
                    // TODO: Warning
                    break
                }
                let mut capture_index = 1;
                for parser_match in &context.matches {
                    let (beg, end) = match self.region.pos(capture_index) {
                        Some(range) => range,
                        None => {
                            capture_index += parser_match.captures_len + 1;
                            continue
                        }
                    };
                    state.change_scope(pos + beg, parser_match.before.clone());
                    for (capture_id, scope) in &parser_match.captures_map {
                        let (beg, end) = match self.region.pos(capture_index + *capture_id) {
                            Some(range) => range,
                            None => continue
                        };
                        state.change_scope(pos + beg, ScopeCommand::Push(scope.clone()));
                        state.change_scope(pos + end, ScopeCommand::Pop);
                    }
                    state.change_scope(pos + end, parser_match.after.clone());
                    state.change_context(parser_match.context);
                    break
                }
                pos += end;
            } else {
                break
            }
        }
    }
}
