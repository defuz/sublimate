use std::iter::Iterator;

use super::scope::{Scope, ScopePath, ScopeCommand, ScopeTree};
use super::theme::{Theme, ThemeSettings};
use super::style::{Style, StyleModifier, Color, FontStyle, BLACK, WHITE};

#[derive(Debug)]
pub struct Highlighter {
    settings: ThemeSettings,
    foreground_tree: ScopeTree<Color>,
    background_tree: ScopeTree<Color>,
    font_style_tree: ScopeTree<FontStyle>,
}

pub struct HighlightIterator<'a> {
    index: usize,
    pos: usize,
    path: ScopePath,
    changes: &'a [(usize, ScopeCommand)],
    text: &'a str,
    highlighter: &'a Highlighter,
    styles: Vec<Style>
}

impl<'a> HighlightIterator<'a> {
    pub fn new(path: Vec<Scope>,
           changes: &'a [(usize, ScopeCommand)],
           text: &'a str,
           highlighter: &'a Highlighter) -> HighlightIterator<'a> {

        let style = highlighter.get_default();
        for i in 1..path.len() {
            style.apply(highlighter.get_style(&path[0..i]));
        }

        HighlightIterator {
            index: 0,
            pos: 0,
            path: path,
            changes: changes,
            text: text,
            highlighter: highlighter,
            styles: vec![style]
        }
    }
}

impl<'a> Iterator for HighlightIterator<'a> {
    type Item = (Style, &'a str);

    fn next(&mut self) -> Option<(Style, &'a str)> {
        if self.pos == self.text.len() {
            return None
        }
        let (end, command) = if self.index < self.changes.len() {
            self.changes[self.index].clone()
        } else {
            (self.text.len(), ScopeCommand::Noop)
        };
        let style = self.styles.last().unwrap().clone();
        let text = &self.text[self.pos..end];
        match command {
            ScopeCommand::Push(scope) => {
                self.path.push(scope);
                self.styles.push(style.apply(self.highlighter.get_style(&self.path)));
            },
            ScopeCommand::Pop => {
                self.path.pop();
                self.styles.pop();
            }
            ScopeCommand::Noop => ()
        };
        self.pos = end;
        self.index += 1;
        if text.is_empty() {
            self.next()
        } else {
            Some((style, text))
        }
    }
}

impl Highlighter {
    pub fn new(theme: Theme) -> Highlighter {
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

    pub fn get_default(&self) -> Style {
        Style {
            foreground: self.settings.foreground.unwrap_or(WHITE),
            background: self.settings.background.unwrap_or(BLACK),
            font_style: FontStyle::empty()
        }
    }

    pub fn get_style(&self, path: &[Scope]) -> StyleModifier {
        StyleModifier {
            foreground: self.foreground_tree.find(path),
            background: self.background_tree.find(path),
            font_style: self.font_style_tree.find(path),
        }
    }
}
