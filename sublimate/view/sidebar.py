# -*- coding: utf-8 -*-
from sublimate.widgets import Widget, HorzFlowContainer
from sublimate.utils import ListWalker


class FileWidget(Widget):

	def __init__(self, parent, file):
		self.file = file
		self.parent = parent
		self.padding = parent.padding + 2

	@property
	def style(self):
		if self.focused:
			return 'sidebar-sub-focused'
		return 'sidebar-sub'

	@property
	def width(self):
		return self.padding + len(self.file.name)

	@property
	def height(self):
		return 1

	def _render(self, canvas):
		canvas.set_style(self.style).padding(left=self.padding).draw_text(self.file.name)


class FolderHeader(Widget):

	def __init__(self, parent):
		self.parent = parent

	@property
	def style(self):
		if self.focused:
			return 'sidebar-selected'

	@property
	def icon_style(self):
		if self.focused:
			return 'sidebar-low-selected'
		return 'sidebar-low'

	@property
	def icon(self):
		if self.parent.opened:
			return 'v'
		return '>'

	@property
	def width(self):
		return self.padding + 2 + len(self.parent.folder.name)

	@property
	def height(self):
		return 1

	def _render(self, canvas):
		canvas.set_style(self.style)
		icon_canvas, name_canvas = canvas.padding(left=self.parent.padding).horz[2, ...]
		icon_canvas.set_style(self.icon_style).draw_text(self.icon)
		name_canvas.draw_text(self.parent.folder.name)


class FolderWidget(HorzFlowContainer):

	def __init__(self, parent, folder):
		if parent:
			self.parent = parent
			self.padding = parent.padding + 2
		else:
			self.padding = 0
		self.folder = folder
		self.header = FolderHeader(parent)
		self.widgets = ListWalker(folder.content, self.create_file_widget)

	def create_file_widget(self, file):
		if hasattr(file, 'content'):
			return FolderHeader(self, file)
		return FileWidget(self, file)