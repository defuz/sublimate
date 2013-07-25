# -*- coding: utf-8 -*-

class OverlayMixin(object):

    @property
    def modals(self):
        curr, modals = self.focus, []
        while curr and curr != self:
            if isinstance(curr, ModalMixin):
                modals.append(curr)
            curr = curr.parent
        return reversed(modals)

    @property
    def opened_modals(self):
        return filter(lambda modal: modal.opened, self.modals)

    def render_modals(self, canvas):
        modals = self.opened_modals
        for modal in modals:
            modal.render_modal(canvas)
        

class ModalMixin(object):

    @property
    def opened(self):
        return self.focused
    
    def set_position(self, canvas, horz, vert):
        if horz == 'left':
            self.x = canvas.x
        elif horz == 'center':
            self.x = canvas.x + (canvas.width / 2)
        elif horz == 'right':
            self.x = canvas.x + canvas.width
        else:
            raise ValueError('Horizontal alignment', horz)
        if vert == 'top':
            self.y = canvas.y
        elif vert == 'center':
            self.y = canvas.y + (canvas.height / 2)
        elif vert == 'bottom':
            self.y = canvas.y + canvas.height
        else:
            raise ValueError('Vertical alignment', vert)

    def render_modal(self, canvas):
        self.render(canvas.overlay(self.x, self.y, self.width, self.height))