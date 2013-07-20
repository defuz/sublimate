# -*- coding: utf-8 -*-

class ObservableType(type):

	def __init__(cls):
		cls._observables = [attr for attr in dir(self) if isinstance(attr, ObservableAttributeBase)]


class ObservableObject(object):

	__metaclass__ = ObservableType

	def __new__(cls, *args, **kwargs):
		obj = cls.__new__(cls, *args, **kwargs)		
		obj._observables = {prop: prop.bound(obj) for prop in cls._observables}


class ObservableAttributeBase(object):
	
	def bound(self, obj):
		raise NotImplementedError()

	def __get__(self, obj, cls):
		if obj is None:
			return self
		return obj._observables[self].value


class ObservableAttribute(ObservableAttributeBase):

	def __init__(self, default=None):
		self.default = default

	def bound(self, obj):
		return BoundObservableAttribute(self.default)

	def __set__(self, obj, cls, value):
		return obj._observables[self].set(value)


class BoundObservableAttribute(object):

	def __init__(self, default=None):
		self.value = default
		self.changed = Signal()

	def set(self, value):
		self.value = value
		self.changed.send(self.value)