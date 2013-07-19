# -*- coding: utf-8 -*-
from sublimate.utils import packages_compare_key


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
                self.settings[name] = SettingsFile(path)


class PackageRepository(object):

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
        settings = SettingsObject()
        for package in self.packages:
            settings.extend(package.settings.get(name))
        return settings