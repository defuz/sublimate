# -*- coding: utf-8 -*-
from sublimate.toolkit import Widget, ContainerWidget, VertScrolledMixin, VertRenderingMixin, TreeMixin, TreeListMixin, LeafMixin


class Sidebar(VertScrolledMixin, VertRenderingMixin, TreeListMixin, ContainerWidget):

	def __init__(self, project):
		self.padding = -2
		self.children = [self.create_widget(FolderWidget, folder)
		                 for folder in project.folders]

 	@property
 	def style(self):
 		return 'sidebar'


class FileWidget(Widget, LeafMixin):

	def __init__(self, file):
		self.file = file

	@property
	def style(self):
		if self.has_focus:
			return 'sidebar-low-selected'
		return 'sidebar-low'	

	@property
	def padding(self):
		return self.parent.padding + 4

	@property
	def width(self):
		return self.padding + len(self.file.name) + 1

	@property
	def height(self):
		return 1

	def render(self, canvas):
		canvas.set_mouse_target(self)
		canvas.set_style(self.style).draw_fill().padding(left=self.padding).draw_text(self.file.name)


class FolderHeader(Widget):

	def __init__(self, folder):
		self.folder = folder

	@property
	def width(self):
		return 3 + len(self.folder.name)

	@property
	def height(self):
		return 1

	@property
	def style(self):
		if self.parent.has_focus:
			return 'sidebar-selected'

	@property
	def icon_style(self):
		if self.parent.has_focus:
			return 'sidebar-low-selected'
		return 'sidebar-low'

	@property
	def icon(self):
		if self.parent.opened:
			return u'▾'
		return u'▸'

	def render(self, canvas):
		canvas.set_mouse_target(self)
		canvas.set_style(self.style).draw_fill()
		icon_canvas, name_canvas = canvas.padding(left=self.parent.padding).horz[2, ...]
		icon_canvas.set_style(self.icon_style).draw_text(self.icon)
		name_canvas.draw_text(self.folder.name)


class FolderWidget(Widget, TreeMixin):

	def __init__(self, folder):
		self.opened = False
		self.header = self.create_widget(FolderHeader, folder)
		self.children = map(self.create_child, folder.content)

	def create_child(self, file):
		if hasattr(file, 'content'):
			return self.create_widget(FolderWidget, file)
		return self.create_widget(FileWidget, file)

	@property
	def padding(self):
		return self.parent.padding + 2
