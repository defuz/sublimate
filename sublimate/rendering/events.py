# -*- coding: utf-8 -*-

class KeyboardEvent(str):
	
	def __new__(cls, key):
		return key


_mouse_button_map = {0: '', 1: 'left ', 2: 'middle ', 3: 'right ', 4: 'scrollup ', 5: 'scrolldown '}


class MouseEvent(str):

	def __new__(cls, event, button):
		return event.replace('mouse ', _mouse_button_map[button])
		