# -*- coding: utf-8 -*-
from sublimate.rendering import Widget


def get_statusbar(app):
	return Statusbar()


class Statusbar(Widget):

	@property
	def height(self):
		return 1

	@property
	def style(self):
		return 'statusbar'

	def render(self, canvas):
		left_text, right_text = " Line 11, Column 49", "Tab Size: 4  Python "
		canvas.set_style(self.style).draw_fill()
		left_canvas, right_canvas = canvas.alignment(len(left_text), len(right_text))
		left_canvas.draw_text(left_text)
		right_canvas.draw_text(right_text)