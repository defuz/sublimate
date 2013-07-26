# -*- coding: utf-8 -*-

class ControlListMixin(object):

    def get_focused_index(self):
        for i, widget in enumerate(self.children):
            if widget.focus == self.focus:
                return i

    def get_focused_widget(self):
        for widget in self.children:
            if widget.focus == self.focus:
                return widget

    def get_first(self):
        for widget in self.children:
            if getattr(widget, 'enabled', True):
                return widget

    def is_first_focused(self):
    	widget = self.get_first()
    	if widget:
    		return widget.focused
		return False

    def focus_first(self):
        widget = self.get_first()
        if widget:
            widget.capture_focus()
            return True
        return False

    def get_last(self):
        for widget in reversed(self.children):
            if getattr(widget, 'enabled', True):
                return widget

    def is_last_focused(self):
    	widget = self.get_last()
    	if widget:
    		return widget.focused
		return False

    def focus_last(self):
        widget = self.get_last()
        if widget:
            widget.capture_focus()
            return True
        return False

    def get_next(self):
        prev_index = index = self.get_focused_index()
        if index is None:
            return self.get_first()
        while True:
            index = (index + 1) % len(self.children)
            if getattr(self.children[index], 'enabled', True):
                break
            if index == prev_index:
                return None        
        return self.children[index]

    def focus_next(self):
        widget = self.get_next()
        if widget:
            widget.capture_focus()
            return True
        return False

    def get_prev(self):
        prev_index = index = self.get_focused_index()
        if index is None:
            return self.get_last()
        while True:
            index = (index - 1) % len(self.children)
            if getattr(self.children[index], 'enabled', True):
                break
            if index == prev_index:
                return None
        return self.children[index]

    def focus_prev(self):
        widget = self.get_prev()
        if widget:
            widget.capture_focus()
            return True
        return False


class SelectedMixin(object):

    def on_select(self):
        self.capture_focus()
        return True
    
    def on_enter(self):
        return self.on_select()

    def on_left_press(self):
        return self.on_select()
