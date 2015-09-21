# -*- coding: utf-8 -*-
from .controls import ControlListMixin, SelectedMixin
from .rendering import max_width, sum_height, vert_rendering, vert_rendering_offset

class TreeMixin(ControlListMixin, SelectedMixin):

    @property
    def width(self):
        if not self.opened:
            return self.header.width
        return max_width(self.header, *self.children)

    @property
    def height(self):
        if not self.opened:
            return self.header.height
        return sum_height(self.header, *self.children)

    def on_select(self):
        self.opened = not self.opened
        return True

    def get_last_leaf(self):
        curr = self
        while isinstance(curr, TreeMixin) and curr.opened:
            last = curr.get_last()
            if not last:
                return curr
            curr = curr.get_last()
        return curr

    def on_left(self):
        if self.opened:
            self.opened = False
            return True
        if isinstance(self.parent, TreeMixin):
            self.parent.take_focus()
            return True
        return False

    def on_right(self):
        self.opened = True
        return True

    def on_up(self):
        if self.has_focus:
            return False
        if self.is_first_focused():
            self.take_focus()
            return True
        self.get_prev().get_last_leaf().take_focus()
        return True

    def on_down(self):
        if self.is_last_focused():
            return False
        if not self.opened:
            return False
        self.get_next().take_focus()
        return True

    def render(self, canvas):
        vert_rendering(canvas, self.header, *self.children)

    def render_offset(self, canvas, offset_x, offset_y):
        if offset_x or self.width > canvas.width:
            canvas = canvas.super(offset_x, 0, self.width, self.height)
        if self.opened:
            vert_rendering_offset(canvas, offset_y,
                                  self.header, *self.children)
        else:
            self.header.render(canvas)


class LeafMixin(SelectedMixin):

    def on_left(self):
        self.parent.take_focus()
        return True

    def get_last_leaf(self):
        return self


class TreeListMixin(ControlListMixin):

    def on_up(self):
        return self.get_prev().get_last_leaf().take_focus()

    def on_down(self):
        return self.get_next().take_focus()
