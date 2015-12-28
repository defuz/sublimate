mod definition;
mod scope;
mod parser;
mod builder;

pub use self::definition::{SyntaxDefinition, ParseSyntaxDefinitonError};
pub use self::scope::{SyntaxScope, SyntaxScopeSelector, SyntaxScopeSelectors, ParseSyntaxScopeError};
