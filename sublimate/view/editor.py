# -*- coding: utf-8 -*-
from sublimate.toolkit import Widget


class Editor(Widget):
	
	@property
	def style(self):
		if self.focused:
			return 'editor-selected'
		return 'editor'

	def on_mouse(self, event):
		self.capture_focus()
		raise Exception("blah!!!")
		return True

	def render(self, canvas):
		canvas.set_mouse_target(self)
		canvas.set_style(self.style)
		canvas.draw_fill()