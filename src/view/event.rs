use toolkit::Canvas;
use core::keymap::Key;

pub trait OnKeypress<C> {
    fn on_keypress(&mut self, core: C, canvas: Canvas, key: Key) -> bool;
}

// pub trait OnKeypressComponent<C> {
//     fn focused(&mut self, core: C, canvas: Canvas) -> Option<(&mut OnKeypress<C>, Canvas)>;

//     fn on_keypress(&mut self, core: C, canvas: Canvas, key: Key) -> bool {
//         false
//     }
// }

// impl<T, C> OnKeypress<C> for T where T: OnKeypressComponent<C>, C: Copy {
//     fn on_keypress(&mut self, core: C, canvas: Canvas, key: Key) -> bool {
//         if let Some((child, canvas)) = self.focused(core, canvas) {
//             if child.on_keypress(core, canvas, key) {
//                 return true;
//             }
//         }
//         OnKeypressComponent::on_keypress(self, core, canvas, key)
//     }
// }

// pub enum Event {
//     KeyPress(Key)
// }

// pub trait ComponentView<C> : View<C> {
//     type T;

//     fn focused(&self) -> Option<&Self::T> {
//         None
//     }

//     fn mouse_target(&self, x: usize, y: usize) -> Option<&Self::T> {
//         None
//     }
// }

// pub trait DispatchEvent<C> : ComponentView<C> {

//     fn on_key_press(&mut self, core: C, key: Key) -> bool {
//         false
//     }

//     fn dispatch_event(&mut self, core: C, event: Event) -> bool {
//         match event {
//             Event::KeyPress(key) => {
//                 self.on_key_press(core, key)
//             }
//         }
//     }
// }
