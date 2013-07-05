# -*- coding: utf-8 -*-
from .widget import Widget


class DecoratorWidget(Widget):

    def __init__(self, inner):
        self.inner = inner
        self.inner.parent = self

    @property
    def width(self):
        return self.inner.width

    @property
    def height(self):
        return self.inner.height

    def get_height(self, width):
        return self.inner.get_height(width)

    def render(self, canvas):
        return self.inner.render(canvas)

    @classmethod
    def apply(cls, *args, **kwargs):
        def decorator(inner):
            return cls(inner=inner, *args, **kwargs)
        return decorator


class PaddingDecorator(DecoratorWidget):

    def __init__(self, inner, left=0, right=0, top=0, bottom=0):
        DecoratorWidget.__init__(self, inner)
        self.left = left
        self.right = right
        self.top = top
        self.bottom = bottom

    @property
    def width(self):
        return self.left + self.inner.width + self.right

    @property
    def height(self):
        return self.top + self.inner.height + self.bottom

    def get_height(self, width):
        return self.top + self.inner.get_height(width - self.left - self.right) + self.bottom

    def render(self, canvas):
        if self.left or self.right:
            _, canvas, _ = canvas.horz[self.left, ..., self.right]
        if self.top or self.bottom:
            _, canvas, _ = canvas.vert[self.top, ..., self.bottom]            
        return self.inner.render(canvas)


class OverlayDecorator(DecoratorWidget):

    def __init__(self, inner):
        DecoratorWidget.__init__(self, inner)
        self.inner = inner
        self.modals = []

    @property
    def opened(self):
        return any(modal.opened for modal in self.modals)

    def on_mouse(self, ovent):
        self.inner.set_focus()
        return True

    def render(self, canvas):
        self.inner.render(canvas)
        if self.opened:
            canvas.set_mouse_target(self)
            for modal in self.modals:
                if modal.opened:
                    modal.render(canvas.base_canvas)


class ModalDecorator(DecoratorWidget):

    def __init__(self, parent, overlay, inner):
        DecoratorWidget.__init__(self, inner)
        self.parent = parent
        overlay.modals.append(self)

    @property
    def opened(self):
        return False

    @property
    def x(self):
        raise NotImplementedError("%s.x" % type(self))

    @property
    def y(self):
        raise NotImplementedError("%s.x" % type(self))

    def render(self, canvas):
        inner_canvas = canvas.overlay(self.x, self.y, self.width, self.height)
        self.inner.render(inner_canvas)