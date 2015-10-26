use vec_map::VecMap;

use core::keymap::Key;

use toolkit::*;

use view::event::OnKeypress;


pub enum ModalPosition {
    AboveLeft(Canvas),
    AboveRight(Canvas),
    UnderLeft(Canvas),
    UnderRight(Canvas)
}

trait Widget<C>: View<C>+OnKeypress<C> {
}

struct ModalManager<C> {
    base: Box<Widget<C>>,
    modals: Vec<Box<Widget<C>>>,
    opened: VecMap<ModalPosition>
}

struct Modal;

// impl Modal {
//     fn show() {

//     }

//     fn hide() {

//     }
// }

impl ModalPosition {
    fn get_window(&self, canvas: Canvas) -> Canvas {
        canvas
    }
}

impl<C> ModalManager<C> {
    pub fn show_modal_window(&mut self, id: usize, position: ModalPosition) {
        self.opened.insert(id, position);
    }

    pub fn hide_modal_window(&mut self, id: &usize) {
        self.opened.remove(id);
    }

    pub fn replace_modal_window(&mut self, id: usize, position: ModalPosition) {
        self.opened.clear();
        self.opened.insert(id, position);
    }
}

impl<C> View<C> for ModalManager<C> where C: Copy {
    fn width(&self, context: C) -> usize {
        self.base.width(context)
    }

    fn height(&self, context: C) -> usize {
        self.base.height(context)
    }

    fn render(&self, context: C, canvas: Canvas) {
        self.base.render(context, canvas);
        for (id, position) in self.opened.iter() {
            self.modals[id].render(context, position.get_window(canvas));
        }
    }
}

impl<C> OnKeypress<C> for ModalManager<C> {
    fn on_keypress(&mut self, core: C, canvas: Canvas, key: Key) -> bool {
        self.base.on_keypress(core, canvas, key)
    }
}
