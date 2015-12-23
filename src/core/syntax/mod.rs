mod definition;
mod scope;

pub use self::definition::{SyntaxDefinition, ParseSyntaxDefinitonError};
pub use self::scope::{SyntaxScope, SyntaxScopeSelector, ParseSyntaxScopeError};
