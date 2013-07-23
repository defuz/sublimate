# -*- coding: utf-8 -*-

BRIDGEST, CONTRAST, ACCENT = 5, 40, 15

def grayscale(i):
	return 'g%d' % i

def style(name, base, accent=0):
	background = BRIDGEST * base
	foreground = background + CONTRAST + accent * ACCENT
	return (name, 'default', 'default', None, grayscale(foreground), grayscale(background))