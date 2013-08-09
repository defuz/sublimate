# -*- coding: utf-8 -*-

class OverlayMixin(object):

    @property
    def opened_modals(self):
        curr, modals = self.focus, []
        while curr and curr != self:
            if isinstance(curr, ModalMixin) and curr.opened:
                modals.append(curr)
            curr = curr.parent
        modals.reverse()
        return modals

    def render_modals(self, canvas, overlay=None):
        modals = self.opened_modals
        if not modals:
            return
        overlay = overlay or canvas
        overlay.set_mouse_target(self) # fixme: self -> self.on_overlay_mouse
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