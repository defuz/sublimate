# -*- coding: utf-8 -*-
from sublimate.core.command import ApplicationCommand


class SelectByIndexCommand(ApplicationCommand):

    def run(self, index):
        pass


class OpenRecentProjectCommand(ApplicationCommand):

    def run(self, index):
        pass


class FindPrevCommand(ApplicationCommand):

    def run(self):
        pass


class MoveCommand(ApplicationCommand):

    def run(self, forward, by):
        pass


class PrevResultCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleCommentCommand(ApplicationCommand):

    def run(self, block):
        pass


class UpperCaseCommand(ApplicationCommand):

    def run(self):
        pass


class InsertBestCompletionCommand(ApplicationCommand):

    def run(self, default, exact):
        pass


class SortLinesCommand(ApplicationCommand):

    def run(self, case_sensitive):
        pass


class RemoveFolderCommand(ApplicationCommand):

    def run(self, dirs):
        pass


class SwapCaseCommand(ApplicationCommand):

    def run(self):
        pass


class RemoveLicenseCommand(ApplicationCommand):

    def run(self):
        pass


class CutCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleDistractionFreeCommand(ApplicationCommand):

    def run(self):
        pass


class AddWhereSnippetCommand(ApplicationCommand):

    def run(self, snippet):
        pass


class PrevBookmarkCommand(ApplicationCommand):

    def run(self):
        pass


class NewSnippetCommand(ApplicationCommand):

    def run(self):
        pass


class SwapWithMarkCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleSettingCommand(ApplicationCommand):

    def run(self, setting):
        pass


class ResetFontSizeCommand(ApplicationCommand):

    def run(self):
        pass


class OpenRecentFileCommand(ApplicationCommand):

    def run(self, index):
        pass


class FindUnderPrevCommand(ApplicationCommand):

    def run(self):
        pass


class SetBuildSystemCommand(ApplicationCommand):

    def run(self):
        pass


class CloseTagCommand(ApplicationCommand):

    def run(self):
        pass


class SaveCommand(ApplicationCommand):

    def run(self):
        pass


class ClearRecentProjectsCommand(ApplicationCommand):

    def run(self):
        pass


class WrapLinesCommand(ApplicationCommand):

    def run(self):
        pass


class ReopenLastFileCommand(ApplicationCommand):

    def run(self):
        pass


class HideOverlayCommand(ApplicationCommand):

    def run(self):
        pass


class OpenFileSettingsCommand(ApplicationCommand):

    def run(self):
        pass


class NextResultCommand(ApplicationCommand):

    def run(self):
        pass


class MoveToCommand(ApplicationCommand):

    def run(self, to):
        pass


class OpenRecentFolderCommand(ApplicationCommand):

    def run(self, index):
        pass


class NextBookmarkCommand(ApplicationCommand):

    def run(self):
        pass


class SwapLineUpCommand(ApplicationCommand):

    def run(self):
        pass


class SelectAllBookmarksCommand(ApplicationCommand):

    def run(self):
        pass


class InsertCommand(ApplicationCommand):

    def run(self, characters):
        pass


class ClearRecentFilesCommand(ApplicationCommand):

    def run(self):
        pass


class RevertCommand(ApplicationCommand):

    def run(self):
        pass


class ReplaceNextCommand(ApplicationCommand):

    def run(self):
        pass


class CloseAllCommand(ApplicationCommand):

    def run(self):
        pass


class ReplaceAllCommand(ApplicationCommand):

    def run(self, close_panel):
        pass


class SelectLinesCommand(ApplicationCommand):

    def run(self, forward):
        pass


class OpenContainingFolderCommand(ApplicationCommand):

    def run(self, files):
        pass


class NextViewInStackCommand(ApplicationCommand):

    def run(self):
        pass


class RightDeleteCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleMenuCommand(ApplicationCommand):

    def run(self):
        pass


class OpenUrlCommand(ApplicationCommand):

    def run(self, url):
        pass


class DeleteWordCommand(ApplicationCommand):

    def run(self, forward):
        pass


class PrevMisspellingCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleCaseSensitiveCommand(ApplicationCommand):

    def run(self):
        pass


class PasteSelectionClipboardCommand(ApplicationCommand):

    def run(self):
        pass


class FoldCommand(ApplicationCommand):

    def run(self):
        pass


class FindUnderExpandCommand(ApplicationCommand):

    def run(self):
        pass


class CopyPathCommand(ApplicationCommand):

    def run(self):
        pass


class CloseCommand(ApplicationCommand):

    def run(self):
        pass


class NewFileAtCommand(ApplicationCommand):

    def run(self, dirs):
        pass


class CloseByIndexCommand(ApplicationCommand):

    def run(self, index, group):
        pass


class ExpandTabsCommand(ApplicationCommand):

    def run(self, set_translate_tabs):
        pass


class OpenFileCommand(ApplicationCommand):

    def run(self, file):
        pass


class RedoCommand(ApplicationCommand):

    def run(self):
        pass


class FocusSideBarCommand(ApplicationCommand):

    def run(self):
        pass


class SetLayoutCommand(ApplicationCommand):

    def run(self, cells, rows, cols):
        pass


class RunMacroFileCommand(ApplicationCommand):

    def run(self, file):
        pass


class IncreaseFontSizeCommand(ApplicationCommand):

    def run(self):
        pass


class ExitCommand(ApplicationCommand):

    def run(self):
        pass


class PromptOpenCommand(ApplicationCommand):

    def run(self):
        pass


class AutoCompleteCommand(ApplicationCommand):

    def run(self):
        pass


class ExpandSelectionToParagraphCommand(ApplicationCommand):

    def run(self):
        pass


class ReplaceCompletionWithNextCompletionCommand(ApplicationCommand):

    def run(self):
        pass


class SlurpFindStringCommand(ApplicationCommand):

    def run(self):
        pass


class FindUnderCommand(ApplicationCommand):

    def run(self):
        pass


class DeleteFileCommand(ApplicationCommand):

    def run(self, files):
        pass


class NewFileCommand(ApplicationCommand):

    def run(self):
        pass


class TransposeCommand(ApplicationCommand):

    def run(self):
        pass


class UndoCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleRecordMacroCommand(ApplicationCommand):

    def run(self):
        pass


class InsertSnippetCommand(ApplicationCommand):

    def run(self):
        pass


class UnfoldCommand(ApplicationCommand):

    def run(self):
        pass


class PasteCommand(ApplicationCommand):

    def run(self):
        pass


class RunMacroCommand(ApplicationCommand):

    def run(self):
        pass


class UnindentCommand(ApplicationCommand):

    def run(self):
        pass


class IndentCommand(ApplicationCommand):

    def run(self):
        pass


class UnexpandTabsCommand(ApplicationCommand):

    def run(self, set_translate_tabs):
        pass


class PermuteLinesCommand(ApplicationCommand):

    def run(self, operation):
        pass


class ToggleSaveAllOnBuildCommand(ApplicationCommand):

    def run(self):
        pass


class FoldByLevelCommand(ApplicationCommand):

    def run(self, level):
        pass


class JoinLinesCommand(ApplicationCommand):

    def run(self):
        pass


class SaveProjectAsCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleTabsCommand(ApplicationCommand):

    def run(self):
        pass


class ShowPanelCommand(ApplicationCommand):

    def run(self, panel):
        pass


class FindAllCommand(ApplicationCommand):

    def run(self, close_panel):
        pass


class SetMarkCommand(ApplicationCommand):

    def run(self):
        pass


class NextFieldCommand(ApplicationCommand):

    def run(self):
        pass


class HideAutoCompleteCommand(ApplicationCommand):

    def run(self):
        pass


class CloseFolderListCommand(ApplicationCommand):

    def run(self):
        pass


class ContextMenuCommand(ApplicationCommand):

    def run(self):
        pass


class ShowAboutWindowCommand(ApplicationCommand):

    def run(self):
        pass


class RefreshFolderListCommand(ApplicationCommand):

    def run(self):
        pass


class ShowOverlayCommand(ApplicationCommand):

    def run(self, overlay):
        pass


class PasteAndIndentCommand(ApplicationCommand):

    def run(self):
        pass


class CloseOthersByIndexCommand(ApplicationCommand):

    def run(self, index, group):
        pass


class PurchaseLicenseCommand(ApplicationCommand):

    def run(self):
        pass


class ReopenCommand(ApplicationCommand):

    def run(self, encoding):
        pass


class CloseToRightByIndexCommand(ApplicationCommand):

    def run(self, index, group):
        pass


class FindAllUnderCommand(ApplicationCommand):

    def run(self):
        pass


class HidePanelCommand(ApplicationCommand):

    def run(self):
        pass


class DuplicateLineCommand(ApplicationCommand):

    def run(self):
        pass


class TogglePreserveCaseCommand(ApplicationCommand):

    def run(self):
        pass


class SoftUndoCommand(ApplicationCommand):

    def run(self):
        pass


class ShowLicenseWindowCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleFullScreenCommand(ApplicationCommand):

    def run(self):
        pass


class PromptSaveAsCommand(ApplicationCommand):

    def run(self):
        pass


class FoldTagAttributesCommand(ApplicationCommand):

    def run(self):
        pass


class OpenInBrowserCommand(ApplicationCommand):

    def run(self):
        pass


class PromptOpenFileCommand(ApplicationCommand):

    def run(self):
        pass


class NewFolderCommand(ApplicationCommand):

    def run(self, dirs):
        pass


class CommitCompletionCommand(ApplicationCommand):

    def run(self):
        pass


class NewPluginCommand(ApplicationCommand):

    def run(self):
        pass


class MoveToGroupCommand(ApplicationCommand):

    def run(self, group):
        pass


class ToggleShowOpenFilesCommand(ApplicationCommand):

    def run(self):
        pass


class SetSettingCommand(ApplicationCommand):

    def run(self, setting, value):
        pass


class SelectBookmarkCommand(ApplicationCommand):

    def run(self, index):
        pass


class ReplaceCompletionWithAutoCompleteCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleSideBarCommand(ApplicationCommand):

    def run(self):
        pass


class DeleteToMarkCommand(ApplicationCommand):

    def run(self):
        pass


class PromptAddFolderCommand(ApplicationCommand):

    def run(self):
        pass


class FindUnderExpandSkipCommand(ApplicationCommand):

    def run(self):
        pass


class PrevFieldCommand(ApplicationCommand):

    def run(self):
        pass


class FocusGroupCommand(ApplicationCommand):

    def run(self, group):
        pass


class RevealInSideBarCommand(ApplicationCommand):

    def run(self):
        pass


class ShowAtCenterCommand(ApplicationCommand):

    def run(self):
        pass


class PromptOpenFolderCommand(ApplicationCommand):

    def run(self):
        pass


class NextViewCommand(ApplicationCommand):

    def run(self):
        pass


class OpenDirCommand(ApplicationCommand):

    def run(self, dir):
        pass


class CopyCommand(ApplicationCommand):

    def run(self):
        pass


class LeftDeleteCommand(ApplicationCommand):

    def run(self):
        pass


class SplitSelectionIntoLinesCommand(ApplicationCommand):

    def run(self):
        pass


class SlurpReplaceStringCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleStatusBarCommand(ApplicationCommand):

    def run(self):
        pass


class RedoOrRepeatCommand(ApplicationCommand):

    def run(self):
        pass


class CloseFileCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleOverwriteCommand(ApplicationCommand):

    def run(self):
        pass


class SwitchFileCommand(ApplicationCommand):

    def run(self, extensions):
        pass


class SetLineEndingCommand(ApplicationCommand):

    def run(self, type):
        pass


class CloseProjectCommand(ApplicationCommand):

    def run(self):
        pass


class CloneFileCommand(ApplicationCommand):

    def run(self):
        pass


class FindNextCommand(ApplicationCommand):

    def run(self):
        pass


class ReindentCommand(ApplicationCommand):

    def run(self):
        pass


class CloseWindowCommand(ApplicationCommand):

    def run(self):
        pass


class PermuteSelectionCommand(ApplicationCommand):

    def run(self, operation):
        pass


class DeleteFolderCommand(ApplicationCommand):

    def run(self, dirs):
        pass


class SaveAllCommand(ApplicationCommand):

    def run(self):
        pass


class EncodeHtmlEntitiesCommand(ApplicationCommand):

    def run(self):
        pass


class NoopCommand(ApplicationCommand):

    def run(self):
        pass


class DragSelectCommand(ApplicationCommand):

    def run(self):
        pass


class NewBuildSystemCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleBookmarkCommand(ApplicationCommand):

    def run(self):
        pass


class LowerCaseCommand(ApplicationCommand):

    def run(self):
        pass


class SortSelectionCommand(ApplicationCommand):

    def run(self, case_sensitive):
        pass


class SelectAllCommand(ApplicationCommand):

    def run(self):
        pass


class PromptOpenProjectCommand(ApplicationCommand):

    def run(self):
        pass


class SoftRedoCommand(ApplicationCommand):

    def run(self):
        pass


class RenamePathCommand(ApplicationCommand):

    def run(self, paths):
        pass


class SelectToMarkCommand(ApplicationCommand):

    def run(self):
        pass


class Rot13Command(ApplicationCommand):

    def run(self):
        pass


class BuildCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleMinimapCommand(ApplicationCommand):

    def run(self):
        pass


class ExpandSelectionCommand(ApplicationCommand):

    def run(self, to):
        pass


class SaveMacroCommand(ApplicationCommand):

    def run(self):
        pass


class DecreaseFontSizeCommand(ApplicationCommand):

    def run(self):
        pass


class SwapLineDownCommand(ApplicationCommand):

    def run(self):
        pass


class ToggleWholeWordCommand(ApplicationCommand):

    def run(self):
        pass


class ExecCommand(ApplicationCommand):

    def run(self, kill):
        pass


class ToggleRegexCommand(ApplicationCommand):

    def run(self):
        pass


class SingleSelectionCommand(ApplicationCommand):

    def run(self):
        pass


class FindInFolderCommand(ApplicationCommand):

    def run(self, dirs):
        pass


class AddDirectoryCommand(ApplicationCommand):

    def run(self):
        pass


class ClearBookmarksCommand(ApplicationCommand):

    def run(self):
        pass


class NewWindowCommand(ApplicationCommand):

    def run(self):
        pass


class PromptSelectProjectCommand(ApplicationCommand):

    def run(self):
        pass


class UnfoldAllCommand(ApplicationCommand):

    def run(self):
        pass


class DetectIndentationCommand(ApplicationCommand):

    def run(self):
        pass


class ClearLocationCommand(ApplicationCommand):

    def run(self):
        pass


class TitleCaseCommand(ApplicationCommand):

    def run(self):
        pass


class NextMisspellingCommand(ApplicationCommand):

    def run(self):
        pass


class PrevViewInStackCommand(ApplicationCommand):

    def run(self):
        pass


class PrevViewCommand(ApplicationCommand):

    def run(self):
        pass


class ScrollLinesCommand(ApplicationCommand):

    def run(self, amount):
        pass


class YankCommand(ApplicationCommand):

    def run(self):
        pass


class ClearFieldsCommand(ApplicationCommand):

    def run(self):
        pass


class ShowScopeNameCommand(ApplicationCommand):

    def run(self):
        pass
