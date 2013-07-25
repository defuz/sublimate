# -*- coding: utf-8 -*-
from sublimate.toolkit import Widget, ContainerWidget, VertRenderingMixin, TreeMixin, TreeListMixin, TreeNodeMixin


class Sidebar(ContainerWidget, VertRenderingMixin, TreeListMixin):

	def __init__(self, project):
		self.padding = 0
		self.children = [self.create_widget(FolderWidget, folder)
		                 for folder in project.folders]

 	@property
 	def style(self):
 		return 'sidebar'


class FileWidget(Widget, TreeNodeMixin):

	def __init__(self, file):
		self.file = file

	@property
	def style(self):
		if self.has_focus:
			return 'sidebar-low-selected'
		return 'sidebar-low'	

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
		canvas.set_style(self.style).draw_fill().padding(left=self.padding).draw_text(self.file.name)


class FolderWidget(Widget, TreeMixin):

	def __init__(self, folder):
		self.folder = folder
		self.opened = False
		self.children = map(self.create_child, folder.content)

	def create_child(self, file):
		if hasattr(file, 'content'):
			return self.create_widget(FolderWidget, file)
		return self.create_widget(FileWidget, file)

	def on_mouse(self, event):
		self.take_focus()
		return True

	@property
	def padding(self):
		return self.parent.padding + 2

	@property
	def width(self):
		max_children_width = max(widget.width for widget in self.children) \
							 if self.children else 0
		return max(max_children_width, len(self.folder.name)) + 3

	@property
	def height(self):
		if not self.opened:
			return 1
		return 1 + sum(widget.height for widget in self.children)

	@property
	def style(self):
		if self.has_focus:
			return 'sidebar-selected'

	@property
	def icon_style(self):
		if self.has_focus:
			return 'sidebar-low-selected'
		return 'sidebar-low'

	@property
	def icon(self):
		if self.opened:
			return u'▾'
		return u'▸'

	def render_header(self, canvas):
		canvas.set_style(self.style).draw_fill()
		icon_canvas, name_canvas = canvas.padding(left=self.parent.padding).horz[2, ...]
		icon_canvas.set_style(self.icon_style).draw_text(self.icon)
		name_canvas.draw_text(self.folder.name)

	def render(self, canvas):
		canvas.set_mouse_target(self)
		if not self.opened:
			return self.render_header(canvas)
		header_canvas, content_canvas = canvas.vert[1, ...]
		self.render_header(header_canvas)
		children_heights = [widget.height for widget in self.children]
		children_canvases = content_canvas.vert[children_heights]
		for widget, widget_canvas in zip(self.children, children_canvases):
			widget.render(widget_canvas)
