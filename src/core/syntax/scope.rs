use std::str::FromStr;

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
