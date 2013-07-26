# -*- coding: utf-8 -*-

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


def horz_rendering(canvas, offset, *widgets):
    for widget in widgets:
        if not getattr(widget, 'visible', True):
            continue
        if widget.width > canvas.width:
            break
        widget_canvas, canvas = canvas.horz[widget.width, ...]
        widget.render(widget_canvas)
    canvas.draw_fill()


def vert_rendering(canvas, offset, *widgets):
    for widget in widgets:
        if not getattr(widget, 'visible', True):
            continue        
        if canvas.height == 0:
            break
        if widget.height <= offset:
            offset -= widget.height
            continue
        widget_height = min(canvas.height, widget.height - offset)
        # if widget_height == 0:
            # continue
        widget_canvas, canvas = canvas.vert[widget_height, ...]        
        if offset == 0:
            widget.render(widget_canvas)
        else:            
            widget.render(widget_canvas, offset)
        offset = 0
    canvas.draw_fill()


class HorzRenderingMixin(object):

    @property
    def width(self):
        return sum_width(*self.children)

    @property
    def height(self):
        return max_height(*self.children)

    def render(self, canvas, offset=0):
        canvas.set_style(self.style)
        horz_rendering(canvas, offset, *self.children)


class VertRenderingMixin(object):

    @property
    def width(self):
        return max_width(*self.children)

    @property
    def height(self):
        return sum_height(*self.children)

    def render(self, canvas, offset=0):
        canvas.set_style(self.style)
        vert_rendering(canvas, offset, *self.children)


