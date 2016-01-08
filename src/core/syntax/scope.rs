use std::str::FromStr;

pub type Rank = u64;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope {
    name: String
}

pub type ScopePath = Vec<Scope>;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScopeSelector {
    path: ScopePath,
    exclude: Option<Scope>
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScopeSelectors {
    pub selectors: Vec<ScopeSelector>
}

#[derive(Debug)]
pub struct ParseScopeError(char);

impl FromStr for Scope {
    type Err = ParseScopeError;

    fn from_str(s: &str) -> Result<Scope, ParseScopeError> {
        let mut name = s.trim().to_lowercase();
        for c in name.chars() {
            match c {
                'a' ... 'z' | '0' ... '9' | '-' | '.' => (),
                c => return Err(ParseScopeError(c))
            }
        };
        Ok(Scope { name: name })
    }
}

impl FromStr for ScopeSelector {
    type Err = ParseScopeError;

    fn from_str(mut s: &str) -> Result<ScopeSelector, ParseScopeError> {
        let exclude = match s.find(" - ") {
            Some(index) => {
                let (path_str, exclude_str) = s.split_at(index);
                s = path_str;
                Some(try!(Scope::from_str(exclude_str)))
            },
            None => None
        };
        let mut path = Vec::new();
        for name in s.split_whitespace() {
            path.push(try!(Scope::from_str(name)))
        };
        Ok(ScopeSelector {
            path: path,
            exclude: exclude
        })
    }
}

impl FromStr for ScopeSelectors {
    type Err = ParseScopeError;

    fn from_str(mut s: &str) -> Result<ScopeSelectors, ParseScopeError> {
        let mut selectors = Vec::new();
        for selector in s.split(',') {
            selectors.push(try!(ScopeSelector::from_str(selector)))
        };
        Ok(ScopeSelectors {
            selectors: selectors
        })
    }
}

impl Scope {
    pub fn rank(&self) -> Rank {
        self.name.split('.').count() as u64 + 1
    }

    pub fn matched(&self, scope: &str) -> bool {
        scope.starts_with(&self.name) &&
        (self.name.len() == scope.len() || scope[self.name.len()..].starts_with('.'))
    }
}

impl ScopeSelector {
    pub fn rank(&self) -> Rank { // todo: implement Ord and PartialOrd instead of rank()
        let mut rank = 0;
        for scope in self.path.iter().rev() {
            rank <<= 4;
            rank += scope.rank();
        }
        rank << 4 * (16 - self.path.len())
    }

    pub fn matched(&self, path: &[Scope]) -> bool {
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
