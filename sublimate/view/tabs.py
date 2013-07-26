# -*- coding: utf-8 -*-
from sublimate.toolkit import ContainerWidget, Widget, HorzRenderingMixin

class Tabs(ContainerWidget, HorzRenderingMixin):

	def __init__(self, views):
		self.children = [self.create_widget(Tab, view) for view in views]

	@property
	def style(self):
		return 'tabs'


class Tab(Widget):

	def __init__(self, view):
		self.view = view
		self.selected = False

	@property
	def width(self):
		return len(self.view.name) + 4

	@property
	def height(self):
		return 1

	@property
	def style(self):
		if self.selected:
			return 'tabs-selected'
		return 'tabs'

	@property
	def border_style(self):
		if self.selected:
			return 'tabs-low-selected'
		return 'tabs-low'

	def render(self, canvas):
		left_canvas, name_canvas, right_canvas = canvas.horz[1, ..., 3]
		left_canvas.set_style(self.border_style).draw_text(' ')
		name_canvas.set_style(self.style).draw_text(self.view.name)
		right_canvas.set_style(self.border_style).draw_text(u' × ')
