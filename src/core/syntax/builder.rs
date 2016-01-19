use core::regex::Regex;

use super::scope::ScopeCommand;

use super::parser::{
    Parser, ParserMatch, ContextCommand, ParserContext, ContextId
};
use super::definition::{
    Syntax, Pattern, Patterns, Include, MatchPattern,
    ScopeMatchPattern, RegexPattern
};

struct ParserBuilder {
    scopes: Vec<ScopeMatchPattern>,
    contexts: Vec<ParserContext>,
}

struct ParserContextBuilder<'a> {
    matches: Vec<ParserMatch>,
    regex: String,
    syntax: &'a Syntax,
    scopes: &'a [ScopeMatchPattern]
}

impl<'a> ParserContextBuilder<'a> {
    fn new(syntax: &'a Syntax,
           scopes: &'a [ScopeMatchPattern]) -> ParserContextBuilder<'a> {
        ParserContextBuilder {
            matches: Vec::new(),
            regex: String::new(),
            syntax: syntax,
            scopes: scopes
        }
    }

    fn push(&mut self,
            before: ScopeCommand, after: ScopeCommand,
            command: ContextCommand, pattern: &RegexPattern) {
        self.matches.push(ParserMatch {
            before: before,
            after: after,
            context: command,
            captures_len: pattern.captures_len,
            captures_map: pattern.captures_map.clone(),
        });
        if !self.matches.is_empty() {
            self.regex.push('|');
        }
        self.regex.push('(');
        self.regex.push_str(&pattern.regex);
        self.regex.push(')');
    }

    fn push_match(&mut self, pattern: &MatchPattern) {
        let before = ScopeCommand::push_or_noop(&pattern.name);
        let after = ScopeCommand::pop_or_noop(&pattern.name);
        self.push(before, after, ContextCommand::Noop, &pattern.content);
    }

    fn push_scope_begin(&mut self, id: ContextId, pattern: &ScopeMatchPattern) {
        let before = ScopeCommand::push_or_noop(&pattern.name);
        let after = ScopeCommand::push_or_noop(&pattern.content_name);
        self.push(before, after, ContextCommand::Push(id), &pattern.begin);
    }

    fn push_scope_end(&mut self, pattern: &ScopeMatchPattern) {
        let before = ScopeCommand::pop_or_noop(&pattern.name);
        let after = ScopeCommand::pop_or_noop(&pattern.content_name);
        self.push(before, after, ContextCommand::Pop, &pattern.end);
    }

    fn push_include(&mut self, include: &Include) {
        // todo: add infinite recursive checks
        let patterns = match *include {
            Include::FromSelf => &self.syntax.patterns,
            Include::FromRepository(ref name) => &self.syntax.repository[name], // todo: check index
            Include::FromSyntax(_) => unimplemented!() // todo: implement
        };
        self.push_patterns(patterns);
    }

    fn push_patterns(&mut self, patterns: &Patterns) {
        for pattern in patterns {
            match *pattern {
                Pattern::Match(ref pattern) => self.push_match(pattern),
                Pattern::ContextId(id) => self.push_scope_begin(id, &self.scopes[id]),
                Pattern::Include(ref include) => self.push_include(include),
                Pattern::ScopeMatch(..) => unreachable!("Scope match shoud be identified before")
            }
        }
    }

    fn build(self) -> ParserContext {
        ParserContext {
            matches: self.matches,
            regex: Regex::new(&self.regex).unwrap() // TODO: fix unwrap
        }
    }
}

impl ParserBuilder {
    fn new() -> ParserBuilder {
        ParserBuilder {
            scopes: Vec::new(),
            contexts: Vec::new()
        }
    }

    fn build(mut self, syntax: &mut Syntax) -> Parser {
        // identificate context scopes
        self.identificate_patterns(&mut syntax.patterns);
        for (_, patterns) in syntax.repository.iter_mut() {
            self.identificate_patterns(patterns);
        }
        // build context parsers
        let context = self.build_root(&syntax.patterns, &syntax);
        self.contexts.push(context);
        for pattern in &self.scopes {
            let context = self.build_scope(pattern, &syntax);
            self.contexts.push(context);
        }

        Parser::new(self.contexts)
    }

    fn build_scope(&self, pattern: &ScopeMatchPattern, syntax: &Syntax) -> ParserContext {
        let mut builder = ParserContextBuilder::new(syntax, &self.scopes);
        builder.push_patterns(&pattern.patterns);
        builder.push_scope_end(pattern);
        builder.build()
    }

    fn build_root(&self, patterns: &Patterns, syntax: &Syntax) -> ParserContext {
        let mut builder = ParserContextBuilder::new(syntax, &self.scopes);
        builder.push_patterns(patterns);
        builder.build()
    }

    fn identificate_patterns(&mut self, patterns: &mut Patterns) {
        for pattern in patterns.iter_mut() {
            if let Pattern::ScopeMatch(..) = *pattern {
                let mut new = Pattern::ContextId(self.scopes.len());
                ::std::mem::swap(&mut new, pattern);
                match new {
                    Pattern::ScopeMatch(scope) => self.scopes.push(scope),
                    _ => unreachable!()
                };
            }
        }
    }
}
