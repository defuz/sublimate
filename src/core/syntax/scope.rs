use std::str::FromStr;
use std::collections::BTreeMap;

pub type Rank = u64;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope {
    name: String
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScopeSelector {
    path: Vec<Scope>,
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
    pub fn path(&self) -> &[Scope] {
        &self.path
    }

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

pub struct ScopeTree<T: Clone> {
    subscopes: BTreeMap<String, ScopeTree<T>>,
    parents: BTreeMap<String, ScopeTree<T>>,
    value: Option<T>
}

impl<T: Clone> ScopeTree<T> {
    pub fn new() -> ScopeTree<T> {
        ScopeTree {
            subscopes: BTreeMap::new(),
            parents: BTreeMap::new(),
            value: None
        }
    }

    fn get(&self, key: &str, is_parent: bool) -> Option<&ScopeTree<T>> {
        if is_parent {
            self.parents.get(key)
        } else {
            self.subscopes.get(key)
        }
    }

    fn get_or_create(&mut self, key: &str, is_parent: bool) -> &mut ScopeTree<T> {
        if is_parent {
            self.parents.entry(key.to_owned()).or_insert_with(ScopeTree::new)
        } else {
            self.subscopes.entry(key.to_owned()).or_insert_with(ScopeTree::new)
        }
    }

    fn add_subpath(&mut self, path: &[Scope], depth: usize, shift: usize, value: T) {
        let part = &path[depth].name[shift..];
        if part.is_empty() {
            if depth == 0 {
                // TODO: warning: reassigned style
                self.value = Some(value)
            } else {
                self.add_subpath(path, depth - 1, 0, value);
            }
        } else {
            let next_shift = part.find('.').unwrap_or(part.len());
            let key = &part[shift..next_shift];
            let node = self.get_or_create(key, shift == 0);
            node.add_subpath(path, depth, next_shift, value);
        }
    }

    pub fn add(&mut self, path: &[Scope], value: T) {
        self.add_subpath(path, path.len(), 0, value)
    }

    fn find_subpath(&self, path: &[Scope], depth: usize, shift: usize) -> Option<T> {
        let part = &path[depth].name[shift..];
        if part.is_empty() {
            if depth == 0 {
                self.value.clone()
            } else {
                self.find_subpath(path, depth - 1, 0)
            }
        } else {
            let next_shift = part.find('.').unwrap_or(part.len());
            let key = &part[shift..next_shift];
            if let Some(node) = self.get(key, shift == 0) {
                let r = node.find_subpath(path, depth, next_shift);
                if r.is_some() {
                    return r
                }
            }
            for d in (0..depth).rev() {
                let r = self.find_subpath(path, d, 0);
                if r.is_some() {
                    return r
                }
            }
            self.value.clone()
        }
    }

    pub fn find(&self, path: &[Scope]) -> Option<T> {
        self.find_subpath(path, path.len(), 0)
    }
}
