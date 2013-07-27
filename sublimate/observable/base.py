# -*- coding: utf-8 -*-
from blinker import Signal


class ObservableAttributeBase(object):
	
	def bound(self, obj):
		raise NotImplementedError()

	def __get__(self, obj, cls):
		if obj is None:
			return self
		return obj._observables[self].get()


class ObservableAttribute(ObservableAttributeBase):

	def __init__(self, default=None):
		self.default = default

	def bound(self, obj):
		return BoundObservableAttribute(self.default)

	def __set__(self, obj, value):
		return obj._observables[self].set(value)


class BoundObservableAttribute(object):

	def __init__(self, default=None):
		self.value = default
		self.changed = Signal()

	def get(self):
		return self.value

	def set(self, value):
		if self.value != value:
			self.value = value
			self.changed.send()


class ObservableType(type):

	def __init__(self, name, bases, dct):
		observables_map = {}
		for base in bases:
			if isinstance(base, ObservableType):
				observables_map.update(base._observables_map)
		for name, attr in dct.items():
			if isinstance(attr, ObservableAttributeBase):
				observables_map[name] = attr
		self._observables_map = observables_map
		self._observables = observables_map.values()


class ObservableObject(object):

	__metaclass__ = ObservableType

	def __new__(cls, *args, **kwargs):
		obj = object.__new__(cls, *args, **kwargs)		
		obj._observables = {prop: prop.bound(obj) for prop in cls._observables}
		return obj
