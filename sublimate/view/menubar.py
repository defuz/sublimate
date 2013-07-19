# -*- coding: utf-8 -*-
from sublimate.rendering import (Widget,
                                 HorzRenderingMixin, VertRenderingMixin,
                                 SelectedMixin, ControlListMixin,
                                 ModalMixin)


def get_menubar(app):
    pass


def get_menu_item(app, settings):
    if settings.caption == "-":
        return MenuDivider()
    # if settings.children:
        # return MenuModal(...)
    return MenuButton()


class Menubar(Widget,
              HorzRenderingMixin,
              ControlListMixin):

    @classmethod
    def from_settings(cls, settings):
        return cls([MenuGroup.from_settings(self, group_settings) 
                    for group_settings in settings])        

    @property
    def style(self):
        return "menubar"

    def on_left(self):
        return self.focus_next()

    def on_right(self):
        return self.focus_prev()


class GroupButton(Widget,
                  SelectedMixin):

    def __init__(self, parent, caption, submenu):
        self.parent = parent
        self.caption = caption
        self.submenu = submenu

    @classmethod
    def from_settings(cls, parent, settings):
        return cls(parent, settings.caption,
                   MenuGroupBox.from_settings(self, settings.children))

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

    def on_up(self):
        return self.submenu.select_first()

    def on_down(self):
        return self.submenu.select_last()

    def render(self, canvas):
        canvas.set_mouse_target(self)
        canvas.set_style(self.style)
        canvas.draw_text(" %s " % self.caption)
        self.submenu.set_position(canvas, 'left', 'bottom')


class GroupBox(Widget,
               VertRenderingMixin,
               ModalMixin,
               ControlListMixin):

    @classmethod
    def from_settings(cls, parent, children):
        return cls(parent, [get_menu_item(self, item_settings)
                            for item_settings in children])

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
        return 'modal-low'

    def render(self, canvas):
        canvas.set_style(self.style).draw_fill("─")


class Button(Widget, SelectedMixin):

    def __init__(self, parent, command, caption):
        self.parent = parent
        self.command = command
        self._caption = caption

    @property
    def width(self):
        return len(self.caption) + len(self.hotkey) + 5

    @property
    def height(self):
        return 1

    @property
    def caption(self):
        if self._caption:
            return self._caption
        return self.command.description

    @property
    def hotkey(self):
        return self.command.hotkey

    @property
    def enabled(self):
        return self.command.enabled

    @property
    def visible(self):
        return self.command.visible

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

    def on_select(self):
        self.command.run()

    def render(self, canvas):
        canvas.set_mouse_target(self).set_style(self.style)
        caption_canvas, hotkey_canvas = canvas.alignment(len(self.caption)+1, len(self.hotkey)+1)
        caption_canvas.draw_text(" %s" % self.caption)
        hotkey_canvas.set_style(self.hotkey_style).draw_text("%s " % self.hotkey)


class Checkbox(Button):

    @property
    def checked(self):
        return self.command.checked

    @property
    def checkbox(self):
        if self.checked:
            return u'✔'
        return ' '

    def render(self, canvas):
        canvas.set_mouse_target(self).set_style(self.style)
        caption_canvas, hotkey_canvas = canvas.alignment(len(self.caption)+1, len(self.hotkey)+1)
        caption_canvas.draw_text("%s%s" % (self.checkbox, self.caption))
        hotkey_canvas.set_style(self.hotkey_style).draw_text("%s " % self.hotkey)


class Submenu(Widget, SelectedMixin):

    def __init__(self, parent, capture, submenu):
        self.parent = parent
        self.capture = capture
        self.submenu = submenu

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
        self.submenu.select_first()

    def render(self, canvas):
        canvas.set_mouse_target(self).set_style(self.style)
        caption_canvas, arrow_canvas = canvas.alignment(len(self.caption)+1, 2)
        caption_canvas.draw_text(" %s" % self.caption)
        arrow_canvas.set_style(self.arrow_style).draw_text(u'▸ ')
        self.submenu.set_position(canvas, 'right', 'top')


class SubmenuBox(Widget,
                 VertRenderingMixin,
                 ModalMixin,
                 ControlListMixin):

    @classmethod
    def from_settings(cls, parent, children):
        return cls(parent, [get_menu_item(self, item_settings)
                            for item_settings in children])

    @property
    def style(self):
        return 'modal'

    def on_down(self):
        return self.focus_next()

    def on_up(self):
        return self.focus_prev()