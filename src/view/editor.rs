use unicode_width::UnicodeWidthStr;

use toolkit::*;

use core::Core;
use core::view::{View as CoreView};
use core::bindings::Key;
use core::syntax::Highlighter;

#[derive(Debug)]
pub struct Editor {
    highlighter: Highlighter,
    palette: ColorPalette
}

pub struct EditorView<'a> {
    view: &'a CoreView,
    highlighter: &'a Highlighter,
    palette: &'a ColorPalette
}

impl Editor {
    pub fn new(highlighter: Highlighter, palette: ColorPalette) -> Editor {
        Editor {
            highlighter: highlighter,
            palette: palette
        }
    }
}

impl<'a> Widget<'a> for Editor {
    type Context = &'a Core;
    type View = EditorView<'a>;

    fn view(&'a self, core: &'a Core) -> EditorView<'a> {
        EditorView {
            view: &core.view,
            highlighter: &self.highlighter,
            palette: &self.palette
        }
    }

    #[allow(unused_variables)]
    fn on_keypress(&mut self, core: &Core, canvas: Canvas, key: Key) -> bool {
        false
    }

}

impl<'a> View for EditorView<'a> {
    fn width(&self) -> usize {
        self.view.lines.iter().map(|line| line.text.width()).max().unwrap_or(0)
    }

    fn height(&self) -> usize {
        self.view.lines.len()
    }

    fn render(&self, canvas: Canvas) {
        for (y, line) in self.view.lines.iter().enumerate() {
            let mut x = 0;
            for (style, text) in line.highlight(self.highlighter) {
                let foreground = Color::from_rgb256(
                    style.foreground.r,
                    style.foreground.g,
                    style.foreground.b
                );
                let background = Color::from_rgb256(
                    style.background.r,
                    style.background.g,
                    style.background.b
                );
                canvas.style(Style {
                    colors: self.palette.color_pair(foreground, background),
                    attrs: Attr::empty() // impl convert
                });
                canvas.text(text, y, x);
                x += text.width();
            }
            for x in x..canvas.width() {
                canvas.char(' ', y, x);
            }
        }
    }
}
