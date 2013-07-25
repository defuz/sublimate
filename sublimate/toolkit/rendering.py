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


