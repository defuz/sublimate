# -*- coding: utf-8 -*-
from sublimate.rendering import Widget


def get_editor(app):
	return Editor()


class Editor(Widget):
	
	@property
	def style(self):
		return 'editor'

	def render(self, canvas):
		canvas.set_style(self.style)
		canvas.draw_fill()