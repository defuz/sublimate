# -*- coding: utf-8 -*-
import logging
from json import load, dump


class SettingsObject(dict):
    __getattr__ = dict.get
    __setattr__ = dict.__setitem__
    __delattr__ = dict.__delitem__


class SettingsFile(SettingsObject):

    def __init__(self, path):
        self.path = path
        logging.info("Open settings file %s", self.path)
        # settings = load(open(path), object_hook=SettingsObject)
        # SettingsObject.__init__(self, settings)

    def save(self, path=None):
        path = path or self.path
        logging.info("Save settings file %s", path)
        dump(open(path, 'w'), self, indent=4)
        self.path = path


def load_settings(path):
    return load(open(path), object_hook=SettingsObject)


def save_settings(path, settings):
    json.dump(open(path, 'w'), settings)