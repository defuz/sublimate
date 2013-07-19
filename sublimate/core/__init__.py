# -*- coding: utf-8 -*-

class Sublimate(object):

	project = MonitoredProperty()

	def __init__(self):
		self.repository = PackageRepository('/home/defuz/Projects/sublimate/packages')
		self.project = Project('/home/defuz/Projects/sublimate/sublimate.sublime-project')
