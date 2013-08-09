# -*- coding: utf-8 -*-
from sublimate.toolkit import Widget


class Editor(Widget):

	def __init__(self, view):
		self.view = view

	@property
	def style(self):
		return 'editor'

	def render(self, canvas):
		canvas.set_style(self.style)
		for number, line in enumerate(self.view.lines):
			if number == canvas.height:
				return
			canvas.draw_text(line, 0, number)