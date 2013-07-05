# -*- coding: utf-8 -*-

class OverlayMixin(object):

	@property
	def modals(self):
		curr, modals = self.focus, []
		while curr and curr != self:
			if isinstance(curr, ModalMixin):
				modals.append(curr)
			curr = curr.parent
		return reversed(curr)

	@property
	def opened_modals(self):
		return filter(lambda modal: modal.opened, self.modals)


class ModalMixin(object):
	
	@property
	def opened(self):
		return self.focused
