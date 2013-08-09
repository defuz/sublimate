# -*- coding: utf-8 -*-

class VertScrolledMixin(object):

	offset_y, outer_height = 0, 0

	def on_scrollup_press(self):
		if self.offset_y > 0:
			self.offset_y -= 1
			return True
		return False
		
	def on_scrolldown_press(self):
		if self.offset_y + self.outer_height < self.height:
			self.offset_y += 1
			return True
		return False

	def scroll_to(self, offset, height=1):
		if self.offset_y > offset:
			self.offset_y = offset
		elif self.scroll_offset + self.height > offset + height:
			self.offset_y = offset + height - self.height

	def render(self, canvas):		
		self.outer_height = canvas.height
		canvas.set_mouse_target(self)
		self.render_offset(canvas, 0, self.offset_y)