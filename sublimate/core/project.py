# -*- coding: utf-8 -*-
import os
from fnmatch import fnmatch

from .settings import load_settings, SettingsFile, SettingsObject
from .utils import files_compare_key

class Project(object):

    def __init__(self, path=None):
        self.path = path
        if self.path:
            self.reload()

    @property
    def settings(self):
        return dict(folders=[folder.settings for folder in self.folders])

    @property
    def saved(self):
        return bool(self.path)

    def reload(self):
        settings = load_settings(self.path)
        if settings.folders:
            self.folders = (ProjectFolder.from_settings(self, folder_settings) 
                            for folder_settings in settings.folders or [])

    def save(self, path=None):
        path = path or self.path
        if path:
            save_settings(path, self.settings)
            self.path = path

    def add_folder(self, path):
        self.folders.append(ProjectFolder(path))
        self.save()

    def remove_folder(self, path):
        for folder in self.folders:
            if folder.path == path:
                self.folders.remove(folder)

    def remove_all_folders(self, path):
        self.folders.clear()
        self.save()

    def refresh_folders(self):
        for folder in folders:
            folder.reload()


class File(object):

    def __init__(self, path, name=None):
        self.path = path
        self.name = name or os.path.basename(path)


class Folder(File):

    def __init__(self, path, name=None):
        File.__init__(self, path, name)
        self.content = []

    def reload(self, folder_filter, file_filter, follow_symlinks):
        content = []
        for name in os.listdir(self.path):
            path = os.path.join(self.path, name)
            if os.path.isdir(path):
                if not folder_filter(name):
                    continue
                if not follow_symlinks and os.path.islink(path):
                    continue
                folder = Folder(path)
                folder.reload(folder_filter, file_filter, follow_symlinks)
                content.append(folder)
            else:
                if not file_filter(name):
                    continue
                content.append(File(path))
        self.content[:] = sorted(content, key=files_compare_key)


class ProjectFolder(Folder):

    def __init__(self, path, name=None, 
                 folder_exclude_patterns=None,
                 file_exclude_patterns=None, 
                 follow_symlinks=False):
        Folder.__init__(self, path, name)
        self.folder_exclude_patterns = folder_exclude_patterns
        self.file_exclude_patterns = file_exclude_patterns
        self.follow_symlinks = follow_symlinks
        self.reload()

    @classmethod
    def from_settings(cls, project, settings):
        if os.path.isabs(settings.path):
            path = settings.path
        else:
            path = os.path.join(os.path.dirname(project.path), settings.path)
        return cls(path, os.path.basename(path),
                   settings.folder_exclude_patterns,
                   settings.file_exclude_patterns,
                   settings.follow_symlinks or False)

    @property
    def settings(self):
        return dict(path=path, name=name,
                    folder_exclude_patterns=folder_exclude_patterns,
                    file_exclude_patterns=file_exclude_patterns,
                    follow_symlinks=follow_symlinks)

    def is_folder_included(self, name):
        if not self.folder_exclude_patterns:
            return True
        return not any(fnmatch(name, pattern) for pattern in self.folder_exclude_patterns)

    def is_file_included(self, name):
        if not self.file_exclude_patterns:
            return True
        return not any(fnmatch(name, pattern) for pattern in self.file_exclude_patterns)

    def reload(self):
        Folder.reload(self, self.is_folder_included, self.is_file_included, self.follow_symlinks)
