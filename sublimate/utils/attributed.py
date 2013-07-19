# -*- coding: utf-8 -*-
from collections import namedtuple

SizedAttr = namedtuple('SizedAttr', ('size', 'attr'))


class AttrFlow(object):

	def __init__(self, data=None):
		self.data = data or []

	def __len__(self):
		return sum(i.size for i in self.data)

	def __iter__(self):
		return iter(self.data)

	def __repr__(self):
		return 'AttrFlow<%s>' % ' '.join('%s:%s' % (i.size, i.attr) for i in self.data)

	def __getitem__(self, s):
		if isinstance(s, int):
			pos = 0
			for size, attr in self:
				pos += size
				if s < pos:
					return attr
		elif isinstance(s, slice):
			start = s.start if s.start != None else 0
			stop = s.stop if s.stop != None else len(self)
			r, pos = AttrFlow(), 0
			for size, attr in self.data:
				if pos + size >= start:
					if pos > stop:
						break
					x = max(0, start - pos)
					y = min(size, stop - pos)
					if x < y:
						r.append(y-x, attr)
				pos += size
			return r
		raise KeyError(s)

	def __setitem__(self, s, data):
		# todo: сделать это горе чуть более производительным
		assert isinstance(s, slice), not s.step
		self.data[:] = (self[:s.start] + data + self[s.stop:]).data

	def __add__(self, other):
		if self.data and other.data:
			if self.data[-1].attr == other.data[0].attr:
				r = AttrFlow(self.data[:-1])
				r.append(self.data[-1].size + other.data[0].size, other.data[0].attr)
				r.extend(other.data[1:])
				return r
		return AttrFlow(self.data + other.data)

	def append(self, size, attr=None):
		self.data.append(SizedAttr(size, attr))

	def extend(self, data):
		self.data.extend(data)

	@classmethod
	def fill(cls, size, attr=None):
		return cls([SizedAttr(size, attr)])


class AttrString(object):

	def __init__(self, str=None, flow=None, attr=None):
		if str:
			self.str = str
			self.flow = flow or AttrFlow.fill(len(self.str), attr)
		else:
			self.str, self.flow = "", AttrFlow()
		assert len(self.str) == len(self.flow)

	def __len__(self):
		return len(self.str)

	def __iter__(self):
		pos = 0
		for size, attr in self.flow:
			yield attr, self.str[pos:pos+size]
			pos += size

	def __repr__(self):
		return 'AttrString %s' % ''.join('<%s>%s' % i for i in self)

	def __getitem__(self, s):
		if isinstance(s, int):
			return self.flow[s], s[s]
		return AttrString(self.str[s], self.flow[s])

	def __setitem__(self, s, data):
		assert isinstance(s, slice), not s.step
		self.str[s] = data.str
		self.flow[s] = data.flow

	def __add__(self, other):
		return AttrString(self.str + other.str, self.flow + other.flow)
