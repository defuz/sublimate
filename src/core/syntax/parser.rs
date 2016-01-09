use core::regex::{Regex, Region, OPTION_NONE};

use super::scope::Scope;
use super::definition::Captures;

pub type ContextId = usize;

pub struct Parser {
    pub contexts: Vec<ParserContext>
}

pub struct ParserContext {
    pub matches: Vec<ParserMatch>,
    pub regex: Regex
}

pub struct ParserMatch {
    pub before: ScopeCommand,
    pub after: ScopeCommand,
    pub context: ContextCommand,
    pub captures_len: usize,
    pub captures_map: Captures
}

#[derive(Clone)]
pub enum ScopeCommand {
    Push(Scope),
    Pop,
    Noop
}

#[derive(Clone, Copy)]
pub enum ContextCommand {
    Push(ContextId),
    Pop,
    Noop
}

pub struct ParserState<'a> {
    pos: usize,
    text: &'a str,
    region: &'a mut Region,
    scope_path: Vec<Scope>,
    context_path: Vec<ContextId>,
    changes: Vec<(usize, ParserStateChange)>
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum ParserStateChange {
    Push(Scope),
    Pop
}

impl<'a> ParserState<'a> {
    fn scope_change(&mut self, pos: usize, command: ScopeCommand) {
        match command {
            ScopeCommand::Push(scope) => {
                self.scope_path.push(scope.clone());
                self.changes.push((self.pos + pos, ParserStateChange::Push(scope)));
            },
            ScopeCommand::Pop => {
                self.scope_path.pop();
                self.changes.push((self.pos + pos, ParserStateChange::Pop));
            }
            ScopeCommand::Noop => ()
        }
    }

    fn context_change(&mut self, command: ContextCommand) {
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

    fn set_text(&mut self, text: &'a str) {
        self.pos = 0;
        self.text = text;
    }

    fn text_move(&mut self, pos: usize) {
        self.pos += pos;
        self.text = &self.text[pos..];
    }

    fn context_id(&self) -> usize {
        match self.context_path.last() {
            Some(id) => *id,
            None => 0
        }
    }

    fn apply_match(&mut self, parser_match: &ParserMatch, capture_index: usize) -> bool {
        match self.region.pos(capture_index) {
            Some((beg, end)) => {
                self.scope_change(beg, parser_match.before.clone());
                self.scope_change(end, parser_match.after.clone());
                self.text_move(end);
            }
            None => return false
        };
        for (capture_id, scope) in &parser_match.captures_map {
            match self.region.pos(capture_index + *capture_id) {
                Some((beg, end)) => {
                    self.scope_change(beg, ScopeCommand::Push(scope.clone()));
                    self.scope_change(end, ScopeCommand::Pop);
                },
                None => ()
            }
        }
        self.context_change(parser_match.context);
        return true;
    }

    fn apply_context(&mut self, context: &ParserContext) -> bool {
        self.region.clear();
        // todo: error handling
        let r = context.regex.match_with_region(self.text, self.region, OPTION_NONE);
        if r.unwrap().is_none() {
            false;
        }
        let mut capture_index = 1;
        for parser_match in &context.matches {
            if self.apply_match(&parser_match, capture_index) {
                break
            }
            capture_index += parser_match.captures_len;
        }
        return true;
    }
}

impl Parser {
    pub fn parse(&self, state: &mut ParserState) {
        while !state.text.is_empty() {
            let context = &self.contexts[state.context_id()];
            if !state.apply_context(context) {
                break
            }
        }
        state.changes.sort();
    }
}
