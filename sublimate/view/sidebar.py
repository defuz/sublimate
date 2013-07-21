# -*- coding: utf-8 -*-
from sublimate.rendering import Widget, ContainerWidget, VertRenderingMixin, ControlListMixin


class Sidebar(ContainerWidget, VertRenderingMixin):

	def __init__(self, project):
		self.padding = 0
		self.children = [self.create_widget(FolderWidget, folder)
		                 for folder in project.folders]

 	@property
 	def style(self):
 		return 'sidebar'


class FileWidget(Widget):

	def __init__(self, file):
		self.file = file

	@property
	def style(self):
		if self.focused:
			return 'sidebar-sub-focused'
		return 'sidebar-sub'

	@property
	def padding(self):
		return self.parent.padding + 2

	@property
	def width(self):
		return self.padding + len(self.file.name)

	@property
	def height(self):
		return 1

	def render(self, canvas):
		canvas.set_style(self.style).padding(left=self.padding).draw_text(self.file.name)


class FolderWidget(Widget, ControlListMixin):

	def __init__(self, folder):
		self.folder = folder
		self.opened = False
		self.children = map(self.create_child, folder.content)

	def create_child(self, file):
		if hasattr(file, 'content'):
			return self.create_widget(FolderWidget, file)
		return self.create_widget(FileWidget, file)

	@property
	def padding(self):
		return self.parent.padding + 2

	@property
	def width(self):
		max_children_width = max(widget.width for widget in self.children) \
							 if self.children else 0
		return max(max_children_width, len(self.folder.name)) + 2

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
		name_canvas.draw_text(self.folder.name)

	def render(self, canvas):
		if not self.opened:
			return self.render_header(canvas)
		header_canvas, content_canvas = canvas.vert[1, ...]
		self.render_header(header_canvas)
		children_heights = (widget.height for widget in self.children)
		children_canvases = content_canvas.vert[children_heights]
		for widget, widget_canvas in zip(self.children, children_canvases):
			widget.render(widget_canvas)
