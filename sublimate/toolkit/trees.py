# -*- coding: utf-8 -*-
from .controls import ControlListMixin

class TreeMixin(ControlListMixin):

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


class TreeNodeMixin(object):

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