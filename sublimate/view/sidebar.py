# -*- coding: utf-8 -*-
from sublimate.rendering import Widget, HorzRenderingMixin, ControlListMixin

def get_sidebar(app):
	return Sidebar(app.project.folders)

class Sidebar(Widget, HorzRenderingMixin):

	def __init__(self, folders):
		self.padding = 0
		self.children = map(self.create_child, folders)		

	def create_child(self, folder):
		return FolderWidget(self, folder)


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

	def render(self, canvas):
		canvas.set_style(self.style).padding(left=self.padding).draw_text(self.file.name)


class FolderWidget(Widget, ControlListMixin):

	def __init__(self, parent, folder):
		self.parent = parent
		self.padding = parent.padding + 2
		self.folder = folder
		self.opened = False
		self.widgets = map(self.create_child, folder.content)

	def create_child(self, file):
		if hasattr(file, 'content'):
			return FolderWidget(self, file)
		return FileWidget(self, file)

	@property
	def width(self):
		max_children_width = max(widget.width for widget in self.children) \
							 if self.children else 0
		return max(max_children_width, self.folder.name) + 2

	@property
	def height(self):
		if not self.opened:
			return 1
		return 1 + sum(widget.height for widget in self.children)

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
		if self.opened:
			return 'v'
		return '>'

	def render_header(self, canvas):
		canvas.set_style(self.style)
		icon_canvas, name_canvas = canvas.padding(left=self.parent.padding).horz[2, ...]
		icon_canvas.set_style(self.icon_style).draw_text(self.icon)
		name_canvas.draw_text(self.parent.folder.name)

	def render(self, canvas):
		if not self.opened:
			return self.render_header(canvas)
		header_canvas, content_canvas = canvas.vert[1, ...]
		self.render_header(header_canvas)
		children_heights = (widget.height for widget in self.children)
		children_canvases = content_canvas.vert[children_heights]
		for widget, widget_canvas in zip(self.children, children_canvases):
			widget.render(widget_canvas)
