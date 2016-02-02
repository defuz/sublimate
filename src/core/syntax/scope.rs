use std::str::FromStr;
use std::collections::BTreeMap;

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

#[derive(Debug, Clone)]
pub enum ScopeCommand {
    Push(Scope),
    Pop,
    Noop
}

#[derive(Debug)]
pub struct ScopeTree<T: Clone> {
    subscopes: BTreeMap<String, ScopeTree<T>>,
    parents: BTreeMap<String, ScopeTree<T>>,
    value: Option<T>
}

#[derive(Debug)]
pub struct ParseScopeError(char);

impl ScopeCommand {
    pub fn push_or_noop(scope: &Option<Scope>) -> ScopeCommand {
        match *scope {
            Some(ref s) => ScopeCommand::Push(s.to_owned()),
            None => ScopeCommand::Noop
        }
    }

    pub fn pop_or_noop(scope: &Option<Scope>) -> ScopeCommand {
        match *scope {
            Some(..) => ScopeCommand::Pop,
            None => ScopeCommand::Noop
        }
    }
}

impl FromStr for Scope {
    type Err = ParseScopeError;

    fn from_str(s: &str) -> Result<Scope, ParseScopeError> {
        let name = s.trim().to_lowercase();
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

    fn from_str(s: &str) -> Result<ScopeSelectors, ParseScopeError> {
        let mut selectors = Vec::new();
        for selector in s.split(',') {
            selectors.push(try!(ScopeSelector::from_str(selector)))
        };
        Ok(ScopeSelectors {
            selectors: selectors
        })
    }
}

impl ScopeSelector {
    pub fn path(&self) -> &[Scope] {
        &self.path
    }
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
        let key = key.trim_right_matches('.');
        if is_parent {
            self.parents.get(key)
        } else {
            self.subscopes.get(key)
        }
    }

    fn get_or_create(&mut self, key: &str, is_parent: bool) -> &mut ScopeTree<T> {
        let key = key.trim_right_matches('.');
        if is_parent {
            self.parents.entry(key.to_owned()).or_insert_with(ScopeTree::new)
        } else {
            self.subscopes.entry(key.to_owned()).or_insert_with(ScopeTree::new)
        }
    }

    fn add_subpath(&mut self, path: &[Scope], depth: usize, shift: usize, value: T) {
        // println!("Add subpath, depth={}, shift={}, path={:?}", depth, shift, path);
        let part = &path[depth].name[shift..];
        // println!("   part={:?}", part);
        if part.is_empty() {
            if depth == 0 {
                // TODO: warning: reassigned style
                self.value = Some(value)
            } else {
                self.add_subpath(path, depth - 1, 0, value);
            }
        } else {
            let next_shift = part.find('.').map_or(part.len(), |i| i + 1);
            let key = &part[..next_shift];
            // println!("   key={:?}", key);
            let node = self.get_or_create(key, shift == 0);
            node.add_subpath(path, depth, shift + next_shift, value);
        }
    }

    pub fn add(&mut self, path: &[Scope], value: T) {
        self.add_subpath(path, path.len() - 1, 0, value)
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
            let next_shift = part.find('.').map_or(part.len(), |i| i + 1);
            let key = &part[..next_shift];
            if let Some(node) = self.get(key, shift == 0) {
                let r = node.find_subpath(path, depth, shift + next_shift);
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
        self.find_subpath(path, path.len() - 1, 0)
    }
}
