# -*- coding: utf-8 -*-
import os

class View(object):

    def __init__(self, path, name=None):
        self.path = path
        self.name = name or os.path.basename(path)