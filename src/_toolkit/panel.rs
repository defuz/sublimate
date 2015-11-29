struct Position {
    x: i32,
    y: i32
}


struct Panel {
    id: PANEL
}

impl Panel {
    pub static fn new(window: Window) -> Panel {

    }

    pub static fn update() {

    }

    pub fn visible(&self) -> bool {

    }

    pub fn window(&self) -> Window {

    }

    // pub fn top() -> Option<&mut Panel> {

    // }

    // pub fn bottom() -> Option<&mut Panel> {

    // }

    pub fn to_top(&mut self) {

    }

    pub fn to_bottom(&mut self) {

    }

    pub fn move(&mut self, position: Position) {

    }

    pub fn set_window(&mut self, window: Window) {

    }

    // pub fn next(&self) -> Option<&Panel> {

    // }

    // pub fn prev(&self) -> Option<&Panel> {

    // }

}

impl Drop for Panel {
    fn drop(&mut self) {

    }
}
