# -*- coding: utf-8 -*-
from blinker import Signal
from .base import ObservableObject, ObservableAttributeBase

class ComputedProperty(ObservableAttributeBase):

	def __init__(self, getter, modified=False):
		self.getter = getter
		self.modified = modified

	def bound(self, obj):
		return BoundComputedProperty(obj, self.getter)

	def __set__(self, obj, value):
		if not self.modified:
			raise ValueError("Observable property is not modified", obj, self.getter.__name__)
		return obj._observables[self].set(value)


class ComputedMethod(ObservableAttributeBase):

	def __init__(self, method):
		self.method = method

	def bound(self, obj):
		return BoundComputedMethod(obj, self.method)


class BoundComputedBase(object):

	def __init__(self, instance, method):
		self.instance = instance
		self.proxy = DependenceLoggerProxy(instance, self.connect_dependence)
		self.method = method		
		self.dependences = []
		self.initialized = False
		self.changed = Signal()

	def initialize(self):
		if self.initialized:
			return		
		self.recalculate()
		self.initialized = True

	def connect_dependence(self, dependence):
		if dependence == self.recalculate:
			raise ValueError("Recursive dependence", self.instance, self.method)
		self.dependences.append(dependence)
		dependence.connect(self.recalculate)

	def disconnect_all_dependences(self):
		reciever = self.recalculate # just optimization		
		for dependece in self.dependences:
			dependece.disconnect(reciever)
		self.dependences = []


class BoundComputedProperty(BoundComputedBase):

	def get(self):
		self.initialize()
		return self.value

	def recalculate(self, sender=None):
		self.disconnect_all_dependences()
		value = self.method(self.proxy)
		if not self.initialized or value != self.value:
			self.value = value
			self.changed.send()


class BoundComputedMethod(BoundComputedBase):

	def get(self):
		return self.initialize

	def recalculate(self, sender=None):
		self.disconnect_all_dependences()
		self.method(self.proxy)
		self.changed.send()


class DependenceLoggerProxy(object):

	def __init__(self, instance, connect_dependence_callback):
		self.__connect_dependence = connect_dependence_callback
		self.__instance = instance
		self.__type = type(instance)

	def __getattr__(self, name):
		observable_prop = self.__type._observables_map.get(name)
		if observable_prop:
			bound_prop = self.__instance._observables[observable_prop]
			self.__connect_dependence(bound_prop.changed)
			value = bound_prop.get()
			if isinstance(value, ObservableObject):
				return DependenceLoggerProxy(value, self.__connect_dependence)
			return value
		return getattr(self.__instance, name)

	def __setattr__(self, name, value):
		if name.startswith('_DependenceLoggerProxy__'):
			object.__setattr__(self, name, value)
			return
		setattr(self.__instance, name, value)
