# -*- coding: utf-8 -*-
import urwid

from sublimate.toolkit import UrwidWidgetAdapter
from .window import Window
from .palette import palette


class ConsoleView(object):

    def __init__(self, app):
        self.window = UrwidWidgetAdapter(Window(app))
        self.loop = urwid.MainLoop(self.window, palette,
                                   unhandled_input=self.unhandled_input)
        self.loop.screen.set_terminal_properties(colors=256)

    def unhandled_input(self, k):
        if k in ('q', 'Q'):
            raise urwid.ExitMainLoop()
