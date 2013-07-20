# -*- coding: utf-8 -*-
import urwid

from sublimate.core import Sublimate
from sublimate.view import get_view, palette


class ConsoleSublimate(Sublimate):

    def create_view(self):
        return get_view(self)

    def run(self):
        self.loop = urwid.MainLoop(self.view, palette,
                                   unhandled_input=self.unhandled_input)
        self.loop.screen.set_terminal_properties(colors=256)
        self.loop.run()

    def unhandled_input(self, k):
        if k in ('q', 'Q'):
            raise urwid.ExitMainLoop()


if __name__ == '__main__':
    ConsoleSublimate().run()
