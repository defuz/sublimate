use std::cell::RefCell;
use std::collections::HashMap;

use super::scope::{Scope, ScopePath, ScopeSelector, Rank};
use super::theme::{Theme, ThemeSettings, Color, FontStyle};

struct Highlighter {
    settings: ThemeSettings,
    variants: Vec<HighlightVariant>,
    cache: RefCell<HashMap<ScopePath, Style>>
}

struct HighlightVariant {
    rank: Rank,
    selector: ScopeSelector,
    font_style: FontStyle,
    foreground: Option<Color>,
    background: Option<Color>
}

#[derive(Clone, Copy)]
struct Style {
    font_style: FontStyle,
    foreground: Color,
    background: Color
}

impl Highlighter {
    fn new(theme: Theme) -> Highlighter {
        let mut variants = Vec::new();
        for scope in theme.scopes {
            for selector in scope.scope.selectors {
                let rank = selector.rank();
                variants.push(HighlightVariant {
                    selector: selector,
                    rank: rank,
                    font_style: scope.font_style,
                    foreground: scope.foreground,
                    background: scope.background
                })
            }
        }
        Highlighter {
            settings: theme.settings,
            variants: variants,
            cache: RefCell::new(HashMap::new())
        }
    }

    fn eval_style(&self, path: &[Scope]) -> Style {
        let mut rank = 0;
        let mut style = Style {
            font_style: FontStyle::empty(),
            foreground: self.settings.foreground,
            background: self.settings.background
        };
        for v in &self.variants {
            if v.rank > rank && v.selector.matched(path) {
                variants.push(v)
            }
        }
        variants.sort_by(|i, j| j.rank.cmp(&i.rank));
    }

    fn get_style(&self, path: &[Scope]) -> Style {
        let mut cache = self.cache.borrow_mut();
        if let Some(style) = cache.get(path) {
            return *style
        }
        let style = self.eval_style(path);
        cache.insert(path.to_vec(), style);
        return style
    }
}
