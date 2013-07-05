# -*- coding: utf-8 -*-
from sublimate.rendering import HorzFlowContainer, HorzFrameContainer

from .menubar import get_menubar
from .sidebar import get_sidebar
from .editor import get_editor

def get_view(app):
    overlay = OverlayDecorator()

    menubar = get_menubar(app)
    sidebar = get_sidebar(app)
    editor = get_editor(app)
    statusbar = get_statusbar(app)

    overlay.inner = HorzFrameContainer(editor, sidebar)
    
    return VertFrameContainer(overlay, menubar, statusbar)
