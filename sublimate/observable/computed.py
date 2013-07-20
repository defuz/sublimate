# -*- coding: utf-8 -*-
from .base import ObservableObject, ObservableAttributeBase

class ObservableProperty(ObservableAttributeBase):

	def __init__(self, getter, modified=False):
		self.getter = getter
		self.modified = modified

	def bound(self, obj):
		return BoundObservableMethod(obj, self.getter)

	def __set__(self, obj, value):
		if not self.modified:
			raise ValueError("Observable property is not modified", obj, self.getter.__name__)
		return obj._observables[self].set(value)


class ObservableMethod(ObservableAttributeBase):

	def __init__(self, method):
		self.method = method

	def bound(self, obj):
		return BoundObservableMethod(obj, self.method)


class BoundObservableMethod(object):

	def __init__(self, instance, method):
		self.instance = instance
		self.proxy = DependenceLoggerProxy(instance, self.connect_dependence)
		self.method = method		
		self.dependences = []
		self.initialized = False
		self.changed = Signal()

	def get(self):
		if not self.initialized:
			self.recalculate()
		return self.value

	def set(self, value):
		self.value = value
		self.disconnect_all_dependences()
		self.changed.send(self.value)

	def connect_dependence(self, dependence):
		if dependence == self.recalculate:
			raise ValueError("Recursive dependence", self.instance, self.method)
		self.dependences.append(dependence)
		dependence.connect(self.recalculate)

	def disconnect_all_dependences(self):
		reciever = self.recalculate # just optimization
		for dependece in self.dependences:
			dependece.disconnect(reciever)
		self.dependences.clear()

	def recalculate(self):
		self.disconnect_all_dependences()
		self.value = self.method(self.proxy)
		self.changed.send(self.value)


class DependenceLoggerProxy(object):

	def __init__(self, instance, connect_dependence_callback):
		self.__connect_dependence = connect_dependence_callback
		self.__instance = instance

	def __getattr__(self, name):		
		observable_prop = self.__instance._observables.get(name)
		if observable_prop:
			self.__connect_dependence(observable_prop.changed)
			if isinstance(observable_prop.value, ObservableObject):
				return DependenceLoggerProxy(observables_prop.value, self.__connect_dependence)
			return observables_prop.value
		return type(self.__instance).__getattr__(self, name)
