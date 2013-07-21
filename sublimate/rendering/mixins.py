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

    def focus_first(self):
        for widget in self.children:
            if getattr(widget, 'enabled', True):
                widget.capture_focus()
                return True
        return False

    def focus_last(self):
        for widget in reversed(self.children):
            if getattr(widget, 'enabled', True):
                widget.capture_focus()
                return True
        return False

    def focus_next(self):
        prev_index = index = self.get_focused_index()
        if index is None:
            return False
        while True:
            index = (index + 1) % len(self.children)
            if getattr(self.children[index], 'enabled', True):
                break
            if index == prev_index:
                return False
        self.children[index].capture_focus() 
        return True

    def focus_prev(self):
        prev_index = index = self.get_focused_index()
        if index is None:
            return False
        while True:
            index = (index - 1) % len(self.children)
            if getattr(self.children[index], 'enabled', True):
                break
            if index == prev_index:
                return False
        self.children[index].capture_focus() 
        return True


class SelectedMixin(object):

    def on_select(self):
        self.capture_focus()
        return True
    
    def on_enter(self):
        self.on_select()

    on_left_up = \
    on_left_down = \
    on_right_up = \
    on_right_down = on_enter


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

    def render_modals(self):
        modals = self.opened_modals
        


class ModalMixin(object):
    
    @property
    def opened(self):
        return self.focused

    def set_position(self, canvas, horz, vert):
        pass