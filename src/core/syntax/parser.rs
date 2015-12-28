use oniguruma::Regex;

use super::scope::SyntaxScope;
use super::definition::Captures;

type ContextId = usize;

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

pub enum ScopeCommand {
    Noop,
    Pop,
    Push(SyntaxScope)
}

pub enum ContextCommand {
    Noop,
    Pop,
    Push(ContextId)
}
