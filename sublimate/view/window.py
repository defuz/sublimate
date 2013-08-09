# -*- coding: utf-8 -*-
from sublimate.toolkit import Widget, OverlayMixin

from .menubar import Menubar
from .sidebar import Sidebar
from .tabgroup import TabGroup
from .statusbar import Statusbar


class Window(Widget, OverlayMixin):

    def __init__(self, app):
        self.menubar = self.create_widget(Menubar, app.get_menu('Main'))
        self.sidebar = self.create_widget(Sidebar, app.project)
        self.tabgroup = self.create_widget(TabGroup, app.group)
        self.statusbar = self.create_widget(Statusbar)

    # def on_mouse(self, event): # todo: not just on_mouse
        # self.editor.capture_focus()

    def render(self, canvas):
        menubar_canvas, frame_canvas, statusbar_canvas = \
            canvas.vert[self.menubar.height, ..., self.statusbar.height]
        sidebar_canvas, tabgroup_canvas = \
            frame_canvas.horz[25, ...]

        self.menubar.render(menubar_canvas)
        self.sidebar.render(sidebar_canvas)
        self.tabgroup.render(tabgroup_canvas)
        self.statusbar.render(statusbar_canvas)
        self.render_modals(canvas)
