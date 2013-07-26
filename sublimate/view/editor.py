# -*- coding: utf-8 -*-
from sublimate.toolkit import Widget


class Editor(Widget):

	text = 'hello'
	
	@property
	def style(self):
		if self.focused:
			return 'editor-selected'
		return 'editor'

	def on_mouse(self, event):
		self.text = "mouse, %s" % event
		self.take_focus()
		return True

	def on_keyboard(self, event):
		self.text = "keyboard, %s" % event
		return True

	def render(self, canvas):
		canvas.set_mouse_target(self)
		canvas.set_style(self.style)
		canvas.draw_fill()
		canvas.draw_text(self.text)