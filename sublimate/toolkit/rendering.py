# -*- coding: utf-8 -*-
from .widget import Widget

def sum_width(*widgets):
    return sum(widget.width 
               for widget in widgets
               if getattr(widget, 'visible', True))


def max_width(*widgets):
    return max(widget.width
               for widget in widgets
               if getattr(widget, 'visible', True))


def sum_height(*widgets):
    return sum(widget.height
               for widget in widgets
               if getattr(widget, 'visible', True))


def max_height(*widgets):
    return max(widget.height
               for widget in widgets
               if getattr(widget, 'visible', True))


def horz_rendering(canvas, *widgets):
    for widget in widgets:
        if not getattr(widget, 'visible', True):
            continue
        if widget.width > canvas.width:
            break
        widget_canvas, canvas = canvas.horz[widget.width, ...]
        widget.render(widget_canvas)
    if canvas.width and canvas.height:
        return canvas


def vert_rendering(canvas, *widgets):
    for widget in widgets:
        if not getattr(widget, 'visible', True):
            continue
        if widget.height > canvas.height:
            break
        widget_canvas, canvas = canvas.vert[widget.height, ...]
        widget.render(widget_canvas)
    if canvas.width and canvas.height:
        return canvas


def horz_rendering_offset(canvas, offset, *widgets):
    raise NotImplementedError() # todo: implement me


def vert_rendering_offset(canvas, offset, *widgets):
    for widget in widgets:
        if not getattr(widget, 'visible', True):
            continue        
        if canvas.height == 0:
            break
        if widget.height <= offset:
            offset -= widget.height
            continue
        widget_height = min(canvas.height, widget.height - offset)
        widget_canvas, canvas = canvas.vert[widget_height, ...]        
        if offset == 0 and widget_height == widget.height:
            widget.render(widget_canvas)
        else:
            widget.render_offset(widget_canvas, 0, offset)
        offset = 0
    if canvas.width and canvas.height:
        return canvas


class HorzRenderingMixin(object):

    @property
    def width(self):
        return sum_width(*self.children)

    @property
    def height(self):
        return max_height(*self.children)

    def render(self, canvas, offset=0):
        canvas.set_style(self.style)
        rest = horz_rendering(canvas, *self.children)
        if rest:
            rest.draw_fill()

    def render_offset(self, canvas, offset_x, offset_y):
        canvas.set_style(self.style)
        if offset_y or self.height > canvas.height:
            canvas = canvas.super(0, offset_y, self.width, self.height)
        rest = horz_rendering_offset(canvas, offset_x, *self.children)
        if rest:
            rest.draw_fill()


class VertRenderingMixin(object):

    @property
    def width(self):
        return max_width(*self.children)

    @property
    def height(self):
        return sum_height(*self.children)

    def render(self, canvas):        
        canvas.set_style(self.style)
        rest = vert_rendering(canvas, *self.children)
        if rest:
            rest.draw_fill()

    def render_offset(self, canvas, offset_x, offset_y):        
        canvas.set_style(self.style)
        if offset_x or self.width > canvas.width:
            canvas = canvas.super(offset_x, 0, self.width, self.height)
        rest = vert_rendering_offset(canvas, offset_y, *self.children)
        if rest:
            rest.draw_fill()


