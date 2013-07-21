# -*- coding: utf-8 -*-

def create_menu(settings, action_mapper):
	def create_item(settings):
		if settings.caption == "-":
			return Divider(settings.id)
		if settings.children:
			return Group.from_settings(settings, action_mapper)
		return Button.from_settings(settings, action_mapper)
	return map(create_item, settings)


class Divider(object):
	
	def __init__(self, id):
		self.id = id


class Group(object):

	def __init__(self, caption, items):
		self.caption = caption
		self.items = items

	@classmethod
	def from_settings(cls, settings, action_mapper):
		items = create_menu(settings.children, action_mapper)
		return cls(settings.caption, items)


class Button(object):

	def __init__(self, caption, action, is_checkbox):
		self.caption = caption
		self.action = action
		self.is_checkbox = is_checkbox

	@classmethod
	def from_settings(cls, settings, action_mapper):		
		action = action_mapper(settings.command, settings.args)
		is_checkbox = bool(settings.checkbox)
		return cls(settings.caption, action, is_checkbox)
