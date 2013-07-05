# -*- coding: utf-8 -*-

class Context(object):
	x = 0
	y = 0
	width = 0
	height = 0

	screen = ...
	overlay = ...
	canvas = ...
	style = ...

	def create_overlay(self):
		overlay = Overlay()
		self.screen.overlays.append(overlay)
		return self.push(overlay=overlay, canvas=None)

	def create_canvas(self, x=0, y=0, width=None, height=None):


class Overlay(object):
	canvases = []
	mouse_target = 


class Canvas(object):
	pass


class Screen(object):
	overlays = []