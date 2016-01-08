use std::str::FromStr;
use std::collections::BTreeMap;

use core::settings::{Settings, ParseSettings};
use core::regex::{Regex, RegexError};

use super::scope::{Scope, ParseScopeError};
use self::ParseSyntaxError::*;

#[derive(Debug, Default)]
pub struct Syntax {
    /// Descriptive name for the syntax definition. Shows up in the syntax definition dropdown menu
    /// located in the bottom right of the Sublime Text interface. It’s usually the name of the
    /// programming language or equivalent.
    pub name: String,
    /// Name of the topmost scope for this syntax definition. Either `source.<lang>` or
    /// `text.<lang>.` Use source for programming languages and text for markup and everything else.
    pub scope_name: Scope,
    /// This is a list of file extensions (without the leading dot). When opening files of these
    /// types, Sublime Text will automatically activate this syntax definition for them. Optional.
    pub file_types: Vec<String>,
    /// Array of patterns to match against the buffer’s text.
    pub patterns: Patterns,
    /// Array of patterns abstracted out from the patterns element. Useful to keep the syntax
    /// definition tidy as well as for specialized uses like recursive patterns or re-using
    /// the same pattern. Optional.
    pub repository: Repository,
}

pub type Patterns = Vec<Pattern>;
pub type Repository = BTreeMap<String, Patterns>;

#[derive(Debug)]
pub enum Pattern {
    Match(MatchPattern),
    ScopeMatch(ScopeMatchPattern),
    ContextId(usize),
    Include(Include)
}

#[derive(Debug)]
pub struct MatchPattern {
    pub name: Option<Scope>,
    pub content: RegexPattern,
}

#[derive(Debug)]
pub struct ScopeMatchPattern {
    pub name: Option<Scope>,
    pub content_name: Option<Scope>,
    pub begin: RegexPattern,
    pub end: RegexPattern,
    pub patterns: Patterns
}

#[derive(Debug)]
pub struct RegexPattern {
    pub regex: String,
    pub captures_len: usize,
    pub captures_map: Captures
}

pub type Captures = BTreeMap<usize, Scope>;

#[derive(Debug)]
pub enum Include {
    FromSelf,
    FromRepository(String),
    FromSyntax(String)
}

#[derive(Debug)]
pub enum ParseSyntaxError {
    IncorrectInclude,
    CapturesIsNotObject,
    IncorrectCaptureIndex,
    IncorrectCaptureValue,
    IncorrectName,
    IncorrectMatch,
    IncorrectRegex,
    IncorrectPatterns,
    PatternIsNotObject,
    IncorrectRepository,
    SyntaxIsNotObject,
    IncorrectSyntaxName,
    IncorrectScope,
    IncorrectFileTypes,
    RegexParse(RegexError),
    ScopeParse(ParseScopeError)
}

impl From<RegexError> for ParseSyntaxError {
    fn from(error: RegexError) -> ParseSyntaxError {
        RegexParse(error)
    }
}

impl From<ParseScopeError> for ParseSyntaxError {
    fn from(error: ParseScopeError) -> ParseSyntaxError {
        ScopeParse(error)
    }
}

impl RegexPattern {
    fn new(regex: String, captures: Captures) -> Result<RegexPattern, ParseSyntaxError> {
        let captures_len = try!(Regex::new(&regex)).captures_len();
        Ok(RegexPattern {
            regex: regex,
            captures_len: captures_len,
            captures_map: captures
        })
    }
}

impl From<String> for Include {
    fn from(s: String) -> Include {
        if s == "$self" {
            Include::FromSelf
        } else if s.starts_with('#') {
            Include::FromRepository(s.trim_left_matches('#').to_owned())
        } else {
            Include::FromSyntax(s)
        }
    }
}

impl ParseSettings for Captures {
    type Error = ParseSyntaxError;

    fn parse_settings(settings: Settings) -> Result<Captures, ParseSyntaxError> {
        let obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(CapturesIsNotObject)
        };
        let mut captures = BTreeMap::new();
        for (key, value) in obj {
            let index = match key.parse() {
                Ok(index) => index,
                Err(..) => return Err(IncorrectCaptureIndex)
            };
            let mut obj = match value {
                Settings::Object(obj) => obj,
                _ => return Err(IncorrectCaptureValue)
            };
            let scope = match obj.remove("name") {
                Some(Settings::String(s)) => try!(Scope::from_str(&s)),
                _ => return Err(IncorrectCaptureValue)
            };
            captures.insert(index, scope);
        };
        Ok(captures)
    }
}

impl ParseSettings for Pattern {
    type Error = ParseSyntaxError;

    fn parse_settings(settings: Settings) -> Result<Pattern, ParseSyntaxError> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(PatternIsNotObject)
        };

        match obj.remove("include") {
            // parse include pattern
            Some(Settings::String(s)) => return Ok(Pattern::Include(Include::from(s))),
            None => (),
            _ => return Err(IncorrectInclude)
        }

        match obj.remove("match") {
            // parse match pattern
            Some(Settings::String(r)) => {
                let regex = r;
                let captures = match obj.remove("captures") {
                    Some(settings) => try!(Captures::parse_settings(settings)),
                    None => Captures::default()
                };
                let name = match obj.remove("name") {
                    Some(Settings::String(s)) => Some(try!(Scope::from_str(&s))),
                    None => None,
                    _ => return Err(IncorrectName)
                };

                return Ok(Pattern::Match(MatchPattern {
                    name: name,
                    content: try!(RegexPattern::new(regex, captures))
                }))
            },
            None => (),
            _ => return Err(IncorrectMatch)
        }
        // parse scope match pattern
        let name = match obj.remove("name") {
            Some(Settings::String(s)) => Some(try!(Scope::from_str(&s))),
            None => None,
            _ => return Err(IncorrectName)
        };
        let content_name = match obj.remove("contentName") {
            Some(Settings::String(s)) => Some(try!(Scope::from_str(&s))),
            None => None,
            _ => return Err(IncorrectName)
        };
        let begin_regex = match obj.remove("begin") {
            Some(Settings::String(s)) => s,
            _ => return Err(IncorrectRegex)
        };
        let end_regex = match obj.remove("end") {
            Some(Settings::String(s)) => s,
            _ => return Err(IncorrectRegex)
        };
        let begin_captures = match obj.remove("beginCaptures") {
            Some(settings) => try!(Captures::parse_settings(settings)),
            None => Captures::default()
        };
        let end_captures = match obj.remove("endCaptures") {
            Some(settings) => try!(Captures::parse_settings(settings)),
            None => Captures::default()
        };
        let patterns = match obj.remove("patterns") {
            Some(settings) => try!(Patterns::parse_settings(settings)),
            None => Patterns::default()
        };

        Ok(Pattern::ScopeMatch(ScopeMatchPattern {
            name: name,
            content_name: content_name,
            begin: try!(RegexPattern::new(begin_regex, begin_captures)),
            end: try!(RegexPattern::new(end_regex, end_captures)),
            patterns: patterns
        }))

    }
}

impl ParseSettings for Patterns {
    type Error = ParseSyntaxError;

    fn parse_settings(settings: Settings) -> Result<Patterns, ParseSyntaxError> {
        let mut arr = match settings {
            Settings::Array(arr) => arr,
            _ => return Err(IncorrectPatterns),
        };
        let mut patterns = Patterns::new();
        for settings in arr {
            patterns.push(try!(Pattern::parse_settings(settings)));
        }
        Ok(patterns)
    }
}

impl ParseSettings for Repository {
    type Error = ParseSyntaxError;

    fn parse_settings(settings: Settings) -> Result<Repository, ParseSyntaxError> {
        let obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(IncorrectRepository)
        };
        let mut repository = Repository::new();
        for (key, settings) in obj {
            let mut obj = match settings {
                Settings::Object(obj) => obj,
                _ => return Err(IncorrectRepository)
            };
            let patterns = if obj.len() == 1 && obj.contains_key("patterns") {
                try!(Patterns::parse_settings(obj.remove("patterns").unwrap()))
            } else {
                vec![try!(Pattern::parse_settings(Settings::Object(obj)))]
            };
            repository.insert(key, patterns);
        }
        Ok(repository)
    }
}

impl ParseSettings for Syntax {
    type Error = ParseSyntaxError;

    fn parse_settings(settings: Settings) -> Result<Syntax, ParseSyntaxError> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(SyntaxIsNotObject)
        };
        let name = match obj.remove("name") {
            Some(Settings::String(name)) => name,
            _ => return Err(IncorrectSyntaxName)
        };
        let scope_name = match obj.remove("scopeName") {
            Some(Settings::String(name)) => try!(Scope::from_str(&name)),
            _ => return Err(IncorrectScope)
        };
        let file_types = match obj.remove("fileTypes") {
            Some(Settings::Array(arr)) => {
                let mut file_types = Vec::new();
                for settings in arr {
                    match settings {
                        Settings::String(s) => file_types.push(s),
                        _ => return Err(IncorrectFileTypes)
                    }
                }
                file_types
            },
            None => Vec::new(),
            _ => return Err(IncorrectFileTypes)
        };
        let patterns = match obj.remove("patterns") {
            Some(settings) => try!(Patterns::parse_settings(settings)),
            None => return Err(IncorrectPatterns)
        };
        let repository = match obj.remove("repository") {
            Some(settings) => try!(Repository::parse_settings(settings)),
            None => Repository::default()
        };
        Ok(Syntax {
            name: name,
            scope_name: scope_name,
            file_types: file_types,
            patterns: patterns,
            repository: repository
        })
    }
}
