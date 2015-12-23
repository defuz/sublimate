use std::str::FromStr;

#[derive(Debug, Default)]
pub struct SyntaxScope {
    name: String
}

#[derive(Debug, Default)]
pub struct SyntaxScopeSelector {
    path: Vec<SyntaxScope>,
    exclude: Option<SyntaxScope>
}

pub struct ParseSyntaxScopeError(char);

impl FromStr for SyntaxScope {
    type Err = ParseSyntaxScopeError;

    fn from_str(s: &str) -> Result<SyntaxScope, ParseSyntaxScopeError> {
        let mut name = s.trim().to_lowercase();
        for c in name.chars() {
            match c {
                'a' ... 'z' | '.' => (),
                c => return Err(ParseSyntaxScopeError(c))
            }
        };
        Ok(SyntaxScope { name: name })
    }
}

impl FromStr for SyntaxScopeSelector {
    type Err = ParseSyntaxScopeError;

    fn from_str(mut s: &str) -> Result<SyntaxScopeSelector, ParseSyntaxScopeError> {
        let exclude = match s.find('-') {
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
