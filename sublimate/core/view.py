# -*- coding: utf-8 -*-
import os

class Group(object):

    def __init__(self, files=None):
        if files:
            self.views = [View(self, path) for path in files]
        else:
            self.views = []
        self.selected_view = None

    def create_view(self, *args):
        view = View(self, *args)
        self.views.append(view)
        return view

    def select_view(self, view):
        assert view in self.views
        self.selected_view = view


class View(object):

    def __init__(self, group, path, name=None):
    	self.group = group
        self.path = path
        self.name = name or os.path.basename(path)
        self.lines = ['']
        self.reload()

    def reload(self):
    	self.lines = open(self.path).read().split('\n')

    @property
    def selected(self):
    	return self == group.selected_view

	def select(self):
		self.group.select(self)