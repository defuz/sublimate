# -*- coding: utf-8 -*-

class HorzRenderingMixin(object):

    @property
    def width(self):
        return sum(widget.width 
                   for widget in self.children
                   if getattr(widget, 'visible', True))

    @property
    def height(self):
        return max(widget.height
                   for widget in self.children
                   if getattr(widget, 'visible', True))

    def render(self, canvas):
        canvas.set_style(self.style)
        for widget in self.children:
            if not getattr(widget, 'visible', True):
                continue
            if widget.width > canvas.width:
                break
            widget_canvas, canvas = canvas.horz[widget.width, ...]
            widget.render(widget_canvas)
        canvas.draw_fill()


class VertRenderingMixin(object):

    @property
    def width(self):
        return max(widget.width
                   for widget in self.children
                   if getattr(widget, 'visible', True))

    @property
    def height(self):
        return sum(widget.height
                   for widget in self.children
                   if getattr(widget, 'visible', True))

    def render(self, canvas):
        canvas.set_style(self.style)
        for widget in self.children:
            if not getattr(widget, 'visible', True):
                continue
            if widget.get_height(canvas.width) > canvas.height:
                break
            widget_canvas, canvas = canvas.vert[widget.get_height(canvas.width), ...]
            widget.render(widget_canvas)
        canvas.draw_fill()



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

    on_left_press = \
    on_right_press = on_enter


class OverlayMixin(object):

    @property
    def modals(self):
        curr, modals = self.focus, []
        while curr and curr != self:
            if isinstance(curr, ModalMixin):
                modals.append(curr)
            curr = curr.parent
        return reversed(modals)

    @property
    def opened_modals(self):
        return filter(lambda modal: modal.opened, self.modals)

    def render_modals(self, canvas):
        modals = self.opened_modals
        for modal in modals:
            modal.render_modal(canvas)
        


class ModalMixin(object):

    @property
    def opened(self):
        return self.focused
    
    def set_position(self, canvas, horz, vert):
        self.x = canvas.x
        self.y = canvas.y + canvas.height

    def render_modal(self, canvas):
        self.render(canvas.overlay(self.x, self.y, self.width, self.height))