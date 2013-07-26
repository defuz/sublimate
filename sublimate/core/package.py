# -*- coding: utf-8 -*-
import os
from fnmatch import fnmatch

from .settings import load_settings
from .utils import packages_compare_key


class Package(object):

    def __init__(self, path, name):
        self.path = path
        self.name = name
        self.reload()

    def reload(self):
        self.settings = {}
        for name in os.listdir(self.path):
            path = os.path.join(self.path, name)
            if fnmatch(name, '*.sublime-menu'):
                self.settings[name] = load_settings(path)


class PackageStorage(object):

    def __init__(self, path):
        self.path = path
        self.packages = []
        self.reload()

    def reload(self):
        packages = []
        for name in os.listdir(self.path):
            path = os.path.join(self.path, name)
            if os.path.isdir(path):
                packages.append(Package(path, name))
        self.packages[:] = sorted(packages, key=packages_compare_key)        

    def get_settings(self, name):
        merged = None
        for package in self.packages:
            settings = package.settings.get(name)
            if merged:
                merged.extend(settings)
            else:
                merged = settings
        return merged

    def get_menu(self, name, get_action):
        return 
