# -*- coding: utf-8 -*-

class Rect(object):

	def __init__(self, x, y, w, h):
		self.x, self.y, self.w, self.h = x, y, w, h

	def clip(self, other):
		# left
		if (self.x >= other.x) and (self.x < (other.x + other.w)):
		    x = self.x
		elif (other.x >= self.x) and (other.x < (self.x + self.w)):
		    x = other.x
		else:
		    return None
		# right
		if ((self.x + self.w) > other.x) and ((self.x + self.w) <= (other.x + other.w)):
		    w = (self.x + self.w) - x
		elif ((other.x + other.w) > self.x) and ((other.x + other.w) <= (self.x + self.w)):
		    w = (other.x + other.w) - x
		else:
		    return None
		# top
		if (self.y >= other.y) and (self.y < (other.y + other.h)):
		    y = self.y
		elif (other.y >= self.y) and (other.y < (self.y + self.h)):
		    y = other.y
		else:
		    return None
		# bottom
		if ((self.y + self.h) > other.y) and ((self.y + self.h) <= (other.y + other.h)):
		    h = (self.y + self.h) - y
		elif ((other.y + other.h) > self.y) and ((other.y + other.h) <= (self.y + self.h)):
		    h = (other.y + other.h) - y
		else:
		    return None
	    return Rect(x, y, w, h)