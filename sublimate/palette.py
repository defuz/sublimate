# -*- coding: utf-8 -*-
lightness, brightness, contrast = 0, 10, 40

backgrounds = range(lightness, 101-contrast, brightness)
foregrounds = [i+contrast for i in backgrounds]

def create_style(name, subname, index, offset=0):
	if not (0 <= index <= len(backgrounds) and 0 <= index+offset <= len(foregrounds)):
		return None
	if subname:
		name = '%s-%s' % (name, subname)
	background = 'g%s' % backgrounds[index]
	foreground = 'g%s' % foregrounds[index+offset]
	return (name, 'light gray', 'black', '', foreground, background)

def create_set(name, index):
	return filter(None, [
		create_style(name, None, index),
		create_style(name, "low", index, -1),
		create_style(name, "high", index, 1),
		create_style(name, "selected", index+1),
		create_style(name, "low-selected", index+1, -1),
		create_style(name, "high-selected", index+1, 1),
	])

palette = (
	create_set('editor', 0) + 
	create_set('sidebar', 1) +
	create_set('tabs', 2) +
	create_set('menubar', 3) +
	create_set('modal', 4) +
	create_set('inputs', 3)
)
