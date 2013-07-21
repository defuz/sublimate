# -*- coding: utf-8 -*-
from .command import CommandPerformer, ApplicationCommand
from .package import PackageStorage
from .project import Project
from .menu import create_menu

class Sublimate(CommandPerformer):

    def __init__(self):
        self.bind_commands(ApplicationCommand)
        self.packages = PackageStorage('/Users/defuz/Projects/sublimate/packages')
        self.project = Project('/Users/defuz/Projects/sublimate/sublimate.sublime-project')

    def get_menu(self, name):        
        return create_menu(self.packages.get_settings('%s.sublime-menu' % name), self.get_action)
