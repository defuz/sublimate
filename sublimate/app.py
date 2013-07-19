# -*- coding: utf-8 -*-
import urwid

from sublimate.palette import palette
from sublimate.view import get_view

class Sublimate(object):

    def __init__(self):
        self.palette = palette
        self.view = get_view(self)

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