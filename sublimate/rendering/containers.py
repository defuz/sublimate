# -*- coding: utf-8 -*-
from widget import Widget, NullWidget

class ContainerWidget(Widget):

    def __init__(self, widgets):
        self.widgets = filter(None, widgets)
        for widget in self.widgets:
            widget.parent = self

    def get_focused_index(self):
        for i, widget in enumerate(self.widgets):
            if widget.focus == self.focus:
                return i

    def get_focused_widget(self):
        for widget in self.widgets:
            if widget.focus == self.focus:
                return widget


class HorzFrameContainer(ContainerWidget):

    def __init__(self, body, left=None, right=None):
        ContainerWidget.__init__(self, [left, body, right])
        self.body = body
        self.left = left or NullWidget()
        self.right = right or NullWidget()

    def _render(self, canvas):
        left_canvas, body_canvas, right_canvas = \
            canvas.horz[20, ..., self.right.width]
        self.left.render(left_canvas)
        self.body.render(body_canvas)
        self.right.render(right_canvas)


class VertFrameContainer(ContainerWidget):

    def __init__(self, body, top=None, bottom=None):
        ContainerWidget.__init__(self, [top, body, bottom])
        self.body = body
        self.top = top or NullWidget()
        self.bottom = bottom or NullWidget()

    def _render(self, canvas):
        top_canvas, body_canvas, bottom_canvas = \
            canvas.vert[self.top.get_height(canvas.width),
                        ...,
                        self.bottom.get_height(canvas.width)]

        self.top.render(top_canvas)
        self.body.render(body_canvas)
        self.bottom.render(bottom_canvas)


class HorzFlowContainer(ContainerWidget):

    @property
    def width(self):
        return sum(widget.width for widget in self.widgets)

    @property
    def height(self):
        return max(widget.height for widget in self.widgets)

    def _render(self, canvas):
        for widget in self.widgets:
            if widget.width > canvas.width:
                break
            widget_canvas, canvas = canvas.horz[widget.width, ...]
            widget.render(widget_canvas)
        canvas.draw_fill()


class VertFlowContainer(ContainerWidget):

    @property
    def width(self):
        return max(widget.width for widget in self.widgets)

    @property
    def height(self):
        return sum(widget.height for widget in self.widgets)

    def _render(self, canvas):
        for widget in self.widgets:
            if widget.get_height(canvas.width) > canvas.height:
                break
            widget_canvas, canvas = canvas.vert[widget.get_height(canvas.width), ...]
            widget.render(widget_canvas)
        canvas.draw_fill()


class HorzAligmentContainer(ContainerWidget):

    def __init__(self, left, right, indent=3):
        ContainerWidget.__init__(self, [left, right])
        self.left = left or NullWidget()
        self.right = right or NullWidget()
        self.indent = indent

    @property
    def width(self):
        return self.left.width + self.indent + self.right.width

    @property
    def height(self):
        return max(self.left.height, self.right.height)

    def _render(self, canvas):
        left_canvas, _, right_canvas = \
            canvas.horz[self.left.width, ..., self.right.width]
        self.left.render(left_canvas)
        self.right.render(right_canvas)