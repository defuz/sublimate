# -*- coding: utf-8 -*-
from sublimate.core import Sublimate
from sublimate.view import ConsoleView

app = Sublimate()
view = ConsoleView(app)

if __name__ == '__main__':
    view.loop.run()
