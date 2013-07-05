# -*- coding: utf-8 -*-
from sublimate.rendering import (FixedButtonWidget, 
                                 HorzFlowContainer, VertFlowContainer, HorzAligmentContainer, 
                                 ModalDecorator, 
                                 SolidWidget, TextWidget)


def get_menubar(app):
    pass

def get_menu_item(app, settings):
    if settings.caption == "-":
        return GroupDivider()
    if settings.children:
        return GroupMenu(**settings)
    return GroupButton(**settings)
 

class MenuBar(HorzFlowContainer):

    @property
    def style(self):
        return "menubar"

    def on_left(self):
        index = self.get_focused_index()
        if index is None:
            return False
        index = (index - 1) % len(self.widgets)
        self.widgets[index].set_focus() 
        return True

    def on_right(self):
        index = self.get_focused_index()
        if index is None:
            return False
        index = (index + 1) % len(self.widgets)
        self.widgets[index].set_focus() 
        return True


class MenuButton(FixedButtonWidget):

    def __init__(self, caption, submenu): # fixme: remove "=None"
        self.caption = caption
        self.submenu = submenu

    @property
    def style(self):
        if self.focused:
            return "menubar-selected"

    @property
    def width(self):
        return len(self.caption) + 2

    @property
    def height(self):
        return 1

    def on_mouse(self, event):
        self.set_focus()

    def on_up(self):
        for widget in reversed(self.submenu.inner.widgets):
            if not widget.disabled:
                widget.set_focus()
                return True
        return False

    def on_down(self):
        for widget in self.submenu.inner.widgets:
            if not widget.disabled:
                widget.set_focus()
                return True
        return False

    def render(self, canvas):
        canvas.set_mouse_target(self).set_style(self.style).draw_text(" %s " % self.caption)


class MenuModal(ModalDecorator):

    def __init__(self, parent, overlay, items):
        ModalDecorator.__init__(self, parent, overlay, MenuBox(items))

    @property
    def x(self):
        return self.parent.x

    @property
    def y(self):
        return self.parent.y + 1

    @property
    def opened(self):
        return self.parent.focused


class MenuBox(VertFlowContainer):

    @property
    def style(self):
        return 'modal'

    def on_down(self):
        prev_index = index = self.get_focused_index()
        if index is None:
            return False
        while True:
            index = (index + 1) % len(self.widgets)
            if not self.widgets[index].disabled:
                break
            if index == prev_index:
                return False
        self.widgets[index].set_focus() 
        return True

    def on_up(self):
        prev_index = index = self.get_focused_index()
        if index is None:
            return False
        while True:
            index = (index - 1) % len(self.widgets)
            if not self.widgets[index].disabled:
                break
            if index == prev_index:
                return False
        self.widgets[index].set_focus() 
        return True


class GroupButton(HorzAligmentContainer):

    def __init__(self, label, hotkey=None, disabled=False):
        HorzAligmentContainer.__init__(self, FixedButtonWidget(label), GroupButtonHotkey(hotkey) if hotkey else None, 0)
        self.disabled = disabled

    @property
    def width(self):
        return len(self.caption) + len(self.hotkey) + 5

    @property
    def height(self):
        return 1

    @property
    def style(self):
        if self.disabled:
            return 'modal-low'
        if self.focused:
            return 'modal-selected'

    @property
    def hotkey_style(self):
        if self.focused:
            return 'modal-low-selected'
        return 'modal-low'

    def render(self, canvas):
        canvas.set_mouse_target(self).set_style(self.style)
        caption_canvas, hotkey_canvas = canvas.alignment(len(self.caption)+1, len(self.hotkey)+1)
        caption_canvas.draw_text(" %s" % self.caption)
        hotkey_canvas.set_style(self.hotkey_style).draw_text("%s " % self.hotkey)


class GroupDivider(Widget):

    disabled = True

    @property
    def width(self):
        return 0

    @property
    def height(self):
        return 1

    @property
    def style(self):
        return 'modal-low'

    def render(self, canvas):
        canvas.set_style(self.style).draw_fill("─")


class GroupMenu(HorzAligmentContainer):

    def __init__(self, label, overlay, items, disabled=False):
        HorzAligmentContainer.__init__(self, FixedButtonWidget(label), GroupButtonHotkey(u'▸ '))
        self.submenu = GroupModal(self, overlay, items)
        self.disabled = disabled

    @property
    def style(self):
        if self.disabled:
            return 'modal-low'
        if self.focused:
            return 'modal-selected'

    def on_right(self):
        for widget in self.submenu.inner.widgets:
            if not widget.disabled:
                widget.set_focus()
                return True
        return False        


class GroupModal(ModalDecorator):

    def __init__(self, parent, overlay, items):
        ModalDecorator.__init__(self, parent, overlay, GroupBox(items))

    @property
    def x(self):
        return self.parent.x + self.parent.parent.width

    @property
    def y(self):
        return self.parent.y

    @property
    def opened(self):
        return self.parent.focused

        
class GroupBox(VertFlowContainer):

    @property
    def style(self):
        return 'modal'

    def on_left(self):
        self.set_focus(self.parent)
        return True

    def on_down(self):
        prev_index = index = self.get_focused_index()
        if index is None:
            return False
        while True:
            index = (index + 1) % len(self.widgets)
            if not self.widgets[index].disabled:
                break
            if index == prev_index:
                return False
        self.widgets[index].set_focus() 
        return True

    def on_up(self):
        prev_index = index = self.get_focused_index()
        if index is None:
            return False
        while True:
            index = (index - 1) % len(self.widgets)
            if not self.widgets[index].disabled:
                break
            if index == prev_index:
                return False
        self.widgets[index].set_focus() 
        return True
