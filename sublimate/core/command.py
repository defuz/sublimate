# -*- coding: utf-8 -*-

class Command(object):

    def __call__(self, *args):
        return self.run(*args)

    def run(self, *args):
        raise NotImplementedError("command %s" % type(self).__name__)

    def description(self, *args):
        name = type(self).__name__
        arguments = args + kwargs.values()
        if arguments:
            return "%s: %s" % (name, ', '.join(map(str, arguments)))
        return name

    def is_enabled(self, *args):
        return True

    def is_visible(self, *args):
        return True

    def is_checked(self, *args):
        return True


class ApplicationCommand(Command):

    def __init__(self, app):
        self.app = app


class TextCommand(Command):

    def __init__(self, view):
        self.view = view


class BoundCommand(object):
    
    def __init__(self, command, args):
        self.command = command
        self.args = args

    def __call__(self):
        return self.command.run(*self.args)

    @property
    def description(self):
        return self.command.description(*self.args)

    @property
    def enabled(self):
        return self.command.is_enabled(*self.args)

    @property
    def visible(self):
        return self.command.is_visible(*self.args)


def find_command_classes(base_class):
    subclasses = base_class.__subclasses__()
    if not subclasses:
        return ()
    commands = set(subclasses)
    for subclass in subclasses:
        commands.update(find_command_classes(subclass))
    return commands


def bind_commands(instance, base_command_class):
    for cls in find_command_classes(base_command_class):
        setattr(instance, cls.__name__, cls(instance))


class CommandPerformer(object):

    def bind_commands(self, base_command_class):
        self.commands = {camelcase2underscore(cls.__name__): cls 
                         for cls in find_command_classes(base_command_class)}

    def get_command(self, name, args=None):
        command = self.commands[name]
        if args:
            return BoundCommand(command, args)
        return command

    def run_command(self, name, *args):
        return self.commands[name].run(*args)