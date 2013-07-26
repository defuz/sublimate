# -*- coding: utf-8 -*-
from .rendering import VertRenderingMixin, vert_rendering

class VertScrolledMixin(VertRenderingMixin):

	scroll_offset, scroll_height = 0, 0

	def on_scrollup_press(self):
		if self.scroll_offset > 0:
			self.scroll_offset -= 1
			return True
		return False
		
	def on_scrolldown_press(self):
		if self.scroll_offset + self.scroll_height < self.height:
			self.scroll_offset += 1
			return True
		return False

	def scroll_to(self, offset, height=1):
		if self.scroll_offset > offset:
			self.scroll_offset = offset
		elif self.scroll_offset + self.height > offset + height:
			self.scroll_offset = offset + height - self.height

	def render(self, canvas):		
		self.scroll_height = canvas.height
		canvas.set_style(self.style).set_mouse_target(self)
		vert_rendering(canvas, self.scroll_offset, *self.children)