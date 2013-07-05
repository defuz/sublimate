# -*- coding: utf-8 -*-
import urwid

from sublimate.palette import palette

from sublimate.rendering import VertFrameContainer, HorzFrameContainer, OverlayDecorator
from sublimate.view import get_menubar, get_sidebar, get_editor, get_statusbar

class Sublimate(object):

    def __init__(self):
        self.palette = palette
        self.editor = get_editor(self)
        self.sidebar = get_sidebar(self)
        self.body = HorzFrameContainer(self.editor, self.sidebar)
        self.overlay = OverlayDecorator(self.body)
        self.menubar = get_menubar(self)
        self.statusbar = get_statusbar(self)
        self.view = VertFrameContainer(self.overlay, self.menubar, self.statusbar)

    def run(self):
        self.loop = urwid.MainLoop(self.view.as_urwid, self.palette,
                                   unhandled_input=self.unhandled_input)
        self.loop.screen.set_terminal_properties(colors=256)
        self.loop.run()

    def unhandled_input(self, k):
        if k in ('q', 'Q'):
            raise urwid.ExitMainLoop()

if __name__ == '__main__':

    sublimate = Sublimate()
    sublimate.run()