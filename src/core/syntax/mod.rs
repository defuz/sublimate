mod definition;
mod scope;
mod parser;
mod builder;
mod theme;
mod highlighter;
mod style;

pub use self::definition::{Syntax, ParseSyntaxError};
pub use self::scope::{Scope, ScopePath, ScopeSelector, ScopeSelectors, ScopeCommand, ParseScopeError};
pub use self::theme::{Theme, ParseThemeError};
pub use self::parser::{Parser, ParserState};
pub use self::highlighter::{Highlighter, HighlightIterator};
pub use self::style::{
    Style, StyleModifier, FontStyle, Color,
    BLACK, WHITE, FONT_STYLE_BOLD, FONT_STYLE_UNDERLINE, FONT_STYLE_ITALIC
};
