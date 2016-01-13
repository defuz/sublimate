use super::scope::{Scope, ScopeTree};
use super::theme::{Theme, ThemeSettings};
use super::style::{Style, StyleModifier, Color, FontStyle, BLACK, WHITE};

struct Highlighter {
    settings: ThemeSettings,
    foreground_tree: ScopeTree<Color>,
    background_tree: ScopeTree<Color>,
    font_style_tree: ScopeTree<FontStyle>,
}

impl Highlighter {
    fn new(theme: Theme) -> Highlighter {
        let mut foreground_tree = ScopeTree::new();
        let mut background_tree = ScopeTree::new();
        let mut font_style_tree = ScopeTree::new();
        for scope in theme.scopes {
            for selector in scope.scope.selectors {
                if let Some(foreground) = scope.style.foreground {
                    foreground_tree.add(selector.path(), foreground);
                }
                if let Some(background) = scope.style.background {
                    background_tree.add(selector.path(), background);
                }
                if let Some(font_style) = scope.style.font_style {
                    font_style_tree.add(selector.path(), font_style);
                }
            }
        }
        Highlighter {
            settings: theme.settings,
            foreground_tree: foreground_tree,
            background_tree: background_tree,
            font_style_tree: font_style_tree
        }
    }

    fn get_default(&self) -> Style {
        Style {
            foreground: self.settings.foreground.unwrap_or(WHITE),
            background: self.settings.foreground.unwrap_or(BLACK),
            font_style: FontStyle::empty()
        }
    }

    fn get_style(&self, path: &[Scope]) -> StyleModifier {
        StyleModifier {
            foreground: self.foreground_tree.find(path),
            background: self.background_tree.find(path),
            font_style: self.font_style_tree.find(path),
        }
    }
}
