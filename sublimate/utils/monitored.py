# -*- coding: utf-8 -*-
from collections import OrderedDict

class MonitoredList(object):

	def __init__(self, lst, walkers=None):
		self.lst = lst
		self.walkers = walkers or []

	def __len__(self):
		return len(self.lst)

	def __iter__(self):
		return iter(self.lst)

	def __getitem__(self, index):
		return self.lst[index]


	def append(self, item):
		self.lst.append(item)
		for walker in self.walkers:
			walker._append(item)

	def remove(self, item):
		self.lst.remove(item):
		for walker in self.walkers:
			walker._remove(item)



class ListWalker(object):

	def __init__(self, lst, mapper):
		self.lst.walkers.append(self) # todo: use weak references
		self.mapper = mapper
		self.data = OrderedDict(map(self.mapper, self.lst))

	def __len__(self):
		return len(self.data)

	def __iter__(self):
		return iter(self.data.values())

	def __getitem__(self, index):
		return self.data.values()[index]

	def _append(self, item):
		self.data[item] = self.mapper(item)

	def _remove(self, item):
		del self.data[item]