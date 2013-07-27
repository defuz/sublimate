# -*- coding: utf-8 -*-
from blinker import Signal
from .base import ObservableAttributeBase

class ObservableList(ObservableAttributeBase):

	def bound(self, obj):
		return BoundObservableList()

	def __set__(self, obj, value):
		obj._observables[self][:] = value


class BoundObservableMapper(object):

	def __init__(self):
		self.lst = []
		self.changed = Signal()
		self.reseted = Signal()
		self.inserted = Signal()
		self.deleted = Signal()

	def insert(self, value, index):
		self.lst.insert(value, index)
		self.added.send(value, index)
		self.changed.send()

	def append(self, value):
		self.lst.insert(value)
		self.added.send(value, len(self.lst))
		self.changed.send()

	def delete(self, index):
		del self.lst[index]
		self.deleted.send(index)
		self.changed.send()


class ObservableListMapper(object):

	def __init__(self, source, mapping):
		self.source = source
		self.mapping = mapping
		self.reset()
		source.inserted.connect(self.insert)
		source.deleted.connect(self.delete)
		source.reseted.connect(self.reset)

	def insert(self, value, index):
		self.lst.insert(self.mapping(value), index)

	def delete(self, index):
		del self.lst[index]

	def reset(self):
		self.lst = map(self.mapping, self.source)