# -*- coding: utf-8 -*-
from .canvas import Canvas
from .events import MouseEvent, KeyboardEvent


class UrwidWidgetWrapper(object):
	def __init__(self, widget):
		self.widget = widget

	def selectable(self):
		return True

	def sizing(self):
		return frozenset(['box', 'flow', 'fixed'])

	def rows(self, size, focus=False):
		return self.widget.get_height(size[0])

	def mouse_event(self, size, event, button, x, y, focus):
		if self.canvas:
			target = self.canvas.get_zone(x, y)
			if target:
				return target.on_mouse(MouseEvent(event, button))
		return False

	def keypress(self, size, key):
		if self.widget.focus:
			return self.widget.focus.on_keyboard(KeyboardEvent(key))
		return False

	def render(self, size, focus=False):
		if len(size) == 2:	
			self.canvas = Canvas(size[0], size[1])
		elif len(size) == 1:
			self.canvas = Canvas(size[0], self.widget.get_height(size[0]))
		else:
			self.canvas = Canvas(self.widget.width, self.widget.height)
		self.widget.render(self.canvas)
		return self.canvas.as_urwid


class Widget(object):
	parent, focus = None, None
	as_urwid = property(UrwidWidgetWrapper)

	@property
	def width(self):
		raise NotImplementedError("%s.width" % type(self))

	@property
	def height(self):
		raise NotImplementedError("%s.height" % type(self))

	def get_height(self, width):
		return self.height

	@property
	def style(self):
		return None

	@property
	def focused(self):
		parent, focus = self.parent, self.focus or self
		while parent:
			if parent.focus != focus:
				return False
			parent = parent.parent
		return True

	def set_focus(self, widget=None):
		parent = self
		focus = widget if widget else (self.focus or self)
		while parent:
			parent.focus = focus
			parent = parent.parent

	def on_keyboard(self, event):
		name = 'on_%s' % event.replace(' ', '_')
		method = getattr(self, name, None)
		if method:
			return method()
		if self.parent:
			return self.parent.on_keyboard(event)

	def on_mouse(self, event):
		name = 'on_%s' % event.replace(' ', '_')
		method = getattr(self, name, None)
		if method:
			return method()
		if self.parent:
			return self.parent.on_mouse(event)

	def render(self, canvas):
		raise NotImplementedError("%s._render" % type(self))


class NullWidget(Widget):

	@property
	def width(self):
		return 0

	@property
	def height(self):
		return 0

	def render(self, canvas):
		pass