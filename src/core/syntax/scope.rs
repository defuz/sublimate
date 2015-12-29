use std::str::FromStr;

pub type Rank = u64;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SyntaxScope {
    name: String
}

pub type SyntaxScopePath = Vec<SyntaxScope>;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SyntaxScopeSelector {
    path: SyntaxScopePath,
    exclude: Option<SyntaxScope>
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SyntaxScopeSelectors {
    selectors: Vec<SyntaxScopeSelector>
}

#[derive(Debug)]
pub struct ParseSyntaxScopeError(char);

impl FromStr for SyntaxScope {
    type Err = ParseSyntaxScopeError;

    fn from_str(s: &str) -> Result<SyntaxScope, ParseSyntaxScopeError> {
        let mut name = s.trim().to_lowercase();
        for c in name.chars() {
            match c {
                'a' ... 'z' | '0' ... '9' | '-' | '.' => (),
                c => return Err(ParseSyntaxScopeError(c))
            }
        };
        Ok(SyntaxScope { name: name })
    }
}

impl FromStr for SyntaxScopeSelector {
    type Err = ParseSyntaxScopeError;

    fn from_str(mut s: &str) -> Result<SyntaxScopeSelector, ParseSyntaxScopeError> {
        let exclude = match s.find(" - ") {
            Some(index) => {
                let (path_str, exclude_str) = s.split_at(index);
                s = path_str;
                Some(try!(SyntaxScope::from_str(exclude_str)))
            },
            None => None
        };
        let mut path = Vec::new();
        for name in s.split_whitespace() {
            path.push(try!(SyntaxScope::from_str(name)))
        };
        Ok(SyntaxScopeSelector {
            path: path,
            exclude: exclude
        })
    }
}

impl FromStr for SyntaxScopeSelectors {
    type Err = ParseSyntaxScopeError;

    fn from_str(mut s: &str) -> Result<SyntaxScopeSelectors, ParseSyntaxScopeError> {
        let mut selectors = Vec::new();
        for selector in s.split(',') {
            selectors.push(try!(SyntaxScopeSelector::from_str(selector)))
        };
        Ok(SyntaxScopeSelectors {
            selectors: selectors
        })
    }
}

impl SyntaxScope {
    pub fn rank(&self) -> Rank {
        self.name.split('.').count() as u64 + 1
    }

    pub fn matched(&self, scope: &str) -> bool {
        scope.starts_with(&self.name) &&
        (self.name.len() == scope.len() || scope[self.name.len()..].starts_with('.'))
    }
}

impl SyntaxScopeSelector {
    pub fn rank(&self) -> Rank {
        let mut rank = 0;
        for scope in self.path.iter().rev() { // todo: do we need rev?
            rank <<= 4;
            rank += scope.rank();
        }
        rank
    }

    pub fn matched(&self, path: &[SyntaxScope]) -> bool {
        let mut iter = path.iter();
        for scope in &self.path {
            while let Some(s) = iter.next() {
                if scope.matched(&s.name) {
                    continue
                }
            }
            return false
        }
        true
    }
}
