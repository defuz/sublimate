# -*- coding: utf-8 -*-
from sublimate.rendering import Widget, UrwidWidgetAdapter, OverlayMixin

from .menubar import get_menubar
from .sidebar import get_sidebar
from .editor import get_editor
from .statusbar import get_statusbar
from .palette import palette

def get_view(app):
    menubar = get_menubar(app)
    sidebar = get_sidebar(app)
    editor = get_editor(app)
    statusbar = get_statusbar(app)
    return UrwidWidgetAdapter(View(menubar, sidebar, editor, statusbar))


class View(Widget, OverlayMixin):

    def __init__(self, menubar, sidebar, editor, statusbar):
        self.menubar = menubar
        self.sidebar = sidebar
        self.editor = editor
        self.statusbar = statusbar

    def on_mouse(self):
        self.frame.editor.capture_focus()

    def render(self, canvas):
        menubar_canvas, frame_canvas, statusbar_canvas = \
            canvas.vert[self.menubar.height, ..., self.statusbar.height]
        sidebar_canvas, editor_canvas = \
            frame_canvas.horz[self.sidebar.width, ...]

        menubar.render(menubar_canvas)
        sidebar.render(sidebar_canvas)
        editor.render(editor_canvas)
        statusbar.render(statusbar_canvas)

        opened_modals = self.get_opened_modals()
        if opened_modals:
            canvas.set_mouse_target(self)
            for modal in opened_modals:
                modal.render(canvas)
