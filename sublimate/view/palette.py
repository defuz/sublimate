# -*- coding: utf-8 -*-
from .colors import style

palette = (
	# editor
	style('editor', 0),
	style('editor-selected', 1),
	# sidebar
	style('sidebar', 1),
	style('sidebar-selected', 2),
	style('sidebar-low', 1, -1),	
	style('sidebar-low-selected', 2, -1),
	# tabs
	style('tabs', 2),
	style('tabs-selected', 0),
	style('tabs-low', 2, -1),
	style('tabs-low-selected', 0, -1),
	# menubar
	style('menubar', 3, -1),
	style('menubar-selected', 4),
	# statusbar
	style('statusbar', 3, -1),
	# modal
	style('modal', 4),
	style('modal-selected', 5),
	style('modal-low', 4, -1),
	style('modal-low-selected', 5, -1),
	style('modal-disabled', 4, -1),
	style('modal-disabled-low', 4, -2),
)
