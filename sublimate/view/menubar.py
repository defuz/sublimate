# -*- coding: utf-8 -*-
from sublimate.toolkit import (Widget, ContainerWidget,
                                 HorzRenderingMixin, VertRenderingMixin,
                                 SelectedMixin, ControlListMixin,
                                 ModalMixin)

from sublimate.core import menu


class Menubar(ContainerWidget,
              HorzRenderingMixin,
              ControlListMixin):

    def __init__(self, items):
        self.children = [self.create_widget(Group, group.caption, group.items)
                         for group in items]

    @property
    def style(self):
        return "menubar"

    def on_left(self):
        return self.get_prev().submenu.capture_focus()

    def on_right(self):
        return self.get_next().submenu.capture_focus()


class Group(Widget, SelectedMixin):

    def __init__(self, caption, items):
        self.caption = caption
        self.submenu = self.create_widget(MenuBox, items)

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

    def on_select(self):
        self.submenu.capture_focus()
        
    def render(self, canvas):
        canvas.set_mouse_target(self)
        canvas.set_style(self.style)
        canvas.draw_text(" %s " % self.caption)
        self.submenu.set_position(canvas, 'left', 'bottom')


class MenuBox(ContainerWidget, VertRenderingMixin, ModalMixin, ControlListMixin):

    def __init__(self, items):
        self.children = []
        for item in items:            
            if isinstance(item, menu.Button):
                self.add_widget(Button(item.caption, item.action, item.is_checkbox))
            elif isinstance(item, menu.Group):
                self.add_widget(InnerGroup(item.caption, item.items))
            else:
                self.add_widget(Divider())

    @property
    def style(self):
        return 'modal'

    def on_down(self):
        return self.focus_next()

    def on_up(self):
        return self.focus_prev()


class Divider(Widget):

    enabled = False

    @property
    def width(self):
        return 0

    @property
    def height(self):
        return 1

    @property
    def style(self):
        return 'modal-disabled-low'

    def render(self, canvas):
        canvas.set_style(self.style).draw_fill(u"─")


class Button(Widget, SelectedMixin):

    def __init__(self, caption, action, is_checkbox):
        if action == None:
            raise Exception(caption)
        self._caption = caption
        self.action = action
        self.is_checkbox = is_checkbox

    @property
    def width(self):
        return len(self.checkbox) + len(self.caption) + len(self.hotkey) + 4

    @property
    def height(self):
        if self.action.visible:
            return 1
        return 0

    @property
    def caption(self):
        if self._caption:
            return self._caption
        return self.action.description

    @property
    def hotkey(self):
        return self.action.hotkey

    @property
    def enabled(self):
        return self.action.enabled

    @property
    def checked(self):
        return self.action.checked

    @property
    def style(self):
        if not self.enabled:
            return 'modal-low'
        if self.focused:
            return 'modal-selected'

    @property
    def hotkey_style(self):
        if self.focused:
            return 'modal-low-selected'
        return 'modal-low'

    def on_select(self):
        self.action.run()

    @property
    def checkbox(self):
        if self.is_checkbox and self.checked:
            return u' ✓ '
        return u' '

    def render(self, canvas):
        canvas.set_mouse_target(self).set_style(self.style).draw_fill()
        caption_canvas, hotkey_canvas = canvas.alignment(len(self.caption)+1, len(self.hotkey)+1)
        caption_canvas.draw_text("%s%s" % (self.checkbox, self.caption))
        hotkey_canvas.set_style(self.hotkey_style).draw_text(u"%s " % self.hotkey)


class InnerGroup(ContainerWidget, SelectedMixin):

    def __init__(self, caption, items):
        self.caption = caption
        self.submenu = self.create_widget(InnerMenuBox, items)

    @property
    def width(self):
        return len(self.caption) + 3

    @property
    def height(self):
        return 1

    @property
    def style(self):
        if self.focused:
            return 'modal-selected'

    @property
    def arrow_style(self):
        if self.focused:
            return 'modal-low-selected'
        return 'modal-low'

    def on_right(self):
        if not self.submenu.focused:
            return self.submenu.focus_first()
        return False

    def render(self, canvas):
        canvas.set_mouse_target(self).set_style(self.style).draw_fill()
        caption_canvas, arrow_canvas = canvas.alignment(len(self.caption)+1, 2)
        caption_canvas.draw_text(" %s" % self.caption)
        arrow_canvas.set_style(self.arrow_style).draw_text(u'▸ ')
        self.submenu.set_position(canvas, 'right', 'top')


class InnerMenuBox(MenuBox):

    def on_left(self):
        self.parent.take_focus()
        return True