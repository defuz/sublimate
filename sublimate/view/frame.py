# -*- coding: utf-8 -*-
from sublimate.rendering import (FixedButtonWidget, 
                                 HorzFlowContainer, VertFlowContainer, HorzAligmentContainer, 
                                 ModalDecorator, 
                                 SolidWidget, TextWidget)


class FrameWidget(Widget):

	def __init__(self, sidebar, editor):
		self.sidebar = sidebar
		self.editor = editor
		self.modals = []

	def render(self, canvas):
