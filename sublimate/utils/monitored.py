# -*- coding: utf-8 -*-
from collections import OrderedDict

class Monitored(object):

	def __init__(self):
		self.listeners = []

	def _changed(self):
		for listener in self.listeners:
			listener._changed(self)

	def _add_listeners(self, *listeners):
		self.listeners.extend(listeners)


class MonitoredList(list, Monitored):

	def __init__(self, *args, **kwargs):
		list.__init__(self, *args, **kwargs)
		Monitored.__init__(self)


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


class MonitoredProperty(object):

	def __init__(self, name):
		self.name = "__%s" % name

	def __get__(self, obj, cls):
		if obj is None:
			return self
		return getattr(obj, self.name)

	def __set__(self, obj, new_value):
		old_value = getattr(obj, self.name, None)
		if old_value is not None:
			for name, old_attr in dir(old_value).items():
				if isinstance(old_attr, Monitored):
					new_attr = getattr(new_value, name)
					new_attr._add_listeners(*attr._listeners)
					new_attr._changed(new_attr)
		setattr(obj, self.name, new_value)
