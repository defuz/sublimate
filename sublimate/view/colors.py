# -*- coding: utf-8 -*-

BRIDGEST, CONTRAST, ACCENT = 5, 50, 15

def grayscale(i):
	return 'g%d' % i

def style(name, base, accent=0, attr=None):
	background = BRIDGEST * base
	foreground = background + CONTRAST + accent * ACCENT
	return (name, 'default', 'default', attr, grayscale(foreground), grayscale(background))