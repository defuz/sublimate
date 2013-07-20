# -*- coding: utf-8 -*-
from .package import PackageRepository
from .project import Project

class Sublimate(object):

	def __init__(self):
		self.repository = PackageRepository('/Users/defuz/Projects/sublimate/packages')
		self.project = Project('/Users/defuz/Projects/sublimate/sublimate.sublime-project')
		self.view = self.create_view()

	def create_view(self):
		raise NotImplementedError()

	def run(self):
		raise NotImplementedError()