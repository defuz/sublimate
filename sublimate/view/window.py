# -*- coding: utf-8 -*-
from sublimate.toolkit import Widget, OverlayMixin

from .menubar import Menubar
from .sidebar import Sidebar
from .tabs import Tabs
from .editor import Editor
from .statusbar import Statusbar


class Window(Widget, OverlayMixin):

    def __init__(self, app):
        self.menubar = self.create_widget(Menubar, app.get_menu('Main'))
        self.sidebar = self.create_widget(Sidebar, app.project)
        self.tabs = self.create_widget(Tabs, app.views)
        self.editor = self.create_widget(Editor)
        self.statusbar = self.create_widget(Statusbar)

    # def on_mouse(self, event): # todo: not just on_mouse
        # self.editor.capture_focus()

    def render(self, canvas):
        menubar_canvas, frame_canvas, statusbar_canvas = \
            canvas.vert[self.menubar.height, ..., self.statusbar.height]
        sidebar_canvas, content_canvas = \
            frame_canvas.horz[20, ...]

        tabs_canvas, editor_canvas = \
            content_canvas.vert[self.tabs.height, ...]

        self.menubar.render(menubar_canvas)
        self.sidebar.render(sidebar_canvas)
        self.tabs.render(tabs_canvas)
        self.editor.render(editor_canvas)
        self.statusbar.render(statusbar_canvas)
        self.render_modals(canvas)
