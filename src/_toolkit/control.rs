use toolkit::core::*;

trait HasFocus<T: Enabling>: HasChildren<T> {
    fn focus(&mut self, index: Option<usize>);
    fn focus_index(&self) -> Option<usize>;

    fn focused_widget(&self) -> Option<&T> {
        self.focus_index().map(|index| &self.children()[index])
    }

    fn first_index(&self) -> Option<usize> {
        // self.children().iter().enumerate().filter(|&(i, w)| w.is_enabled()).next()
        for (i, w) in self.children().iter().enumerate() {
            if w.is_enabled() {
                return Some(i);
            }
        }
        return None;
    }

    fn is_first_focused(&self) -> bool {
        self.first_index() == self.focus_index()
    }

    fn focus_first(&mut self) {
        let i = self.first_index();
        self.focus(i)
    }

    fn next_index(&self) -> Option<usize> {
        match self.focus_index() {
            Some(index) => {
                for (shift, w) in self.children()[index..].iter().enumerate() {
                    if w.is_enabled() {
                        return Some(index + shift);
                    }
                }
                None
            }
            None => None
        }
    }

    fn focus_next(&mut self) {
        let i = self.next_index();
        self.focus(i)
    }
}
