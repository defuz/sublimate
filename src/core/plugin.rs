impl Sublime {
    /// Runs the callback in the main thread after the given delay (in milliseconds). Callbacks with an equal delay will be run in the order they were added.
    fn set_timeout(&self, callback, delay);
    /// Runs the callback on an alternate thread after the given delay (in milliseconds).
    fn set_async_timeout(&self, callback, delay);
    /// Sets the message that appears in the status bar.
    fn status_message(&self, string);
    /// Displays an error dialog to the user.
    fn error_message(&self, string);
    /// Displays a message dialog to the user.
    fn message_dialog(&self, string);
    /// Displays an ok / cancel question dialog to the user. If ok_title is provided, this may be used as the text on the ok button. Returns True if the user presses the ok button.
    fn ok_cancel_dialog(&self, string, <ok_title>) -> bool;
    /// Displays a yes / no / cancel question dialog to the user. If yes_title and/or no_title are provided, they will be used as the text on the corresponding buttons on some platforms. Returns sublime.DIALOG_YES, sublime.DIALOG_NO or sublime.DIALOG_CANCEL.
    fn yes_no_cancel_dialog(&self, string, <yes_title>, <no_title>) -> Int;

    /// Loads the given resource. The name should be in the format `Packages/Default/Main.sublime-menu`.
    fn load_resource(&self, name) -> String;
    /// Loads the given resource. The name should be in the format `Packages/Default/Main.sublime-menu`.
    fn load_binary_resource(&self, name) -> bytes;
    /// Finds resources whose file name matches the given pattern.
    fn find_resources(&self, pattern) -> [String];

    /// Encode a JSON compatible value into a string representation. If pretty is set to True, the string will include newlines and indentation.
    fn encode_value(&self, value, <pretty>) -> String;
    /// Decodes a JSON string into an object. If the string is invalid, a `ValueError` will be thrown.
    fn decode_value(&self, string) -> value;

    /// Expands any variables in the string `value` using the variables defined in the dictionary `variables`. `value` may also be an array or dict, in which case the structure will be recursively expanded. Strings should use snippet syntax, for example: expand_variables("Hello, ${name}", {"name": "Foo"})
    fn expand_variables(&self, value, variables) -> value;


    /// Loads the named settings. The name should include a file name and extension, but not a path. The packages will be searched for files matching the base name, and the results will be collated into the settings object. Subsequent calls to load_settings with the name base_name will return the same object, and not load the settings from disk again.
    fn load_settings(&self, base_name) -> Settings;
    /// Flushes any in-memory changes to the named settings object to disk.
    fn save_settings(&self, base_name);
    // /// Displays a message box to the user.
    // fn message_box(&self, string);
    // /// Displays a yes / no message box to the user, return True iff they selected yes.
    // fn question_box(&self, string) -> bool;
    // /// Returns a reference to the application options.
    // fn options(&self) -> Options;
    /// Returns a list of all the open windows.
    fn windows(&self) -> [Window];
    /// Returns the most recently used window.
    fn active_window(&self) -> Window;
    // /// Runs the named ApplicationCommand with the (optional) given arguments.
    // fn run_command(&self, string, <args>);
    // /// Returns True iff the command is enabled with the (optional) given arguments
    // fn canRunCommand(&self, string, <args>);
    // /// Builds a command string from a command name and arguments. This string is suitable to use as an argument to showCompletions().
    // fn makeCommand(&self, string, <args>) -> String;
    /// Returns the base path to the packages.
    fn packages_path(&self) -> String;
    /// Returns the path where all the user's *.sublime-package files are.
    fn installed_packages_path(&self) -> String;
    /// Returns the path where Sublime Text stores cache files.
    fn cache_path(&self) -> String;
    /// Returns the contents of the clipboard. size_limit is there to protect against unnecessarily large data, defaults to 16,777,216 characters
    fn get_clipboard(&self, <size_limit>) -> String;
    /// Sets the contents of the clipboard.
    fn set_clipboard(&self, string);
    // /// Returns the current macro. The macro is represented as a list of commands to run.
    // fn getMacro(&self) -> [String];
    // /// Sets the current macro.
    // fn setMacro(&self, [string]);
    /// Matches the selector against the given scope, returning a score. A score of 0 means no match, above 0 means a match. Different selectors may be compared against the same scope: a higher score means the selector is a better match for the scope.
    fn score_selector(&self, scope, selector) -> Int;
    /// Runs the named ApplicationCommand with the (optional) given arguments.
    fn run_command(&self, string, <args>);
    /// Controls command logging. If enabled, all commands run from key bindings and the menu will be logged to the console.
    fn log_commands(&self, flag);
    /// Controls input logging. If enabled, all key presses will be logged to the console.
    fn log_input(&self, flag);
    /// Controls result regex logging. This is useful for debugging regular expressions used in build systems.
    fn log_result_regex(&self, flag);
    /// Returns the version number
    fn version(&self) -> String;
    /// Returns the platform, which may be "osx", "linux" or "windows"
    fn platform(&self) -> String;
    /// Returns the CPU architecture, which may be "x32" or "x64"
    fn arch(&self) -> String;
}


/// Represents a view into a text buffer. Note that multiple views may refer to the same buffer, but they have their own unique selection and geometry.
struct View;

impl View {
    /// Returns a number that uniquely identifies this view.
    fn id(&self) -> int;
    /// Returns a number that uniquely identifies the buffer underlying this view.
    fn buffer_id(&self) -> int;
    /// The full name file the file associated with the buffer, or None if it doesn't exist on disk.
    fn file_name(&self) -> String;
    /// The name assigned to the buffer, if any
    fn name(&self) -> String;
    /// Assigns a name to the buffer
    fn set_name(&self, name);
    /// Returns true if the buffer is still loading from disk, and not ready for use.
    fn is_loading(&self) -> bool;
    /// Returns true if there are any unsaved modifications to the buffer.
    fn is_dirty(&self) -> bool;
    /// Returns true if the buffer may not be modified.
    fn is_read_only(&self) -> bool;
    /// Sets the read only property on the buffer.
    fn set_read_only(&self, value);
    /// Returns true if the buffer is a scratch buffer. Scratch buffers never report as being dirty.
    fn is_scratch(&self) -> bool;
    /// Sets the scratch property on the buffer.
    fn set_scratch(&self, value);
    /// Returns a reference to the views settings object. Any changes to this settings object will be private to this view.
    fn settings(&self) -> Settings;
    /// Returns a reference to the window containing the view.
    fn window(&self) -> Window;
    /// Runs the named TextCommand with the (optional) given arguments.
    fn run_command(&self, string, <args>);
    // /// Returns True iff the command is enabled with the (optional) given arguments
    // fn canRunCommand(&self, string, <args>);
    /// Returns the number of character in the file.
    fn size(&self) -> int;
    /// Returns the contents of the region as a string.
    fn substr(&self, region) -> String;
    /// Returns the character to the right of the point.
    fn substr(&self, point) -> String;
    /// Inserts the given string in the buffer at the specified point. Returns the number of characters inserted: this may be different if tabs are being translated into spaces in the current buffer.
    fn insert(&self, edit, point, string) -> int;
    /// Erases the contents of the region from the buffer.
    fn erase(&self, edit, region);
    /// Replaces the contents of the region with the given string.
    fn replace(&self, edit, region, string);
    /// Returns a reference to the selection.
    fn sel(&self) -> Selection;
    /// Returns the line that contains the point.
    fn line(&self, point) -> Region;
    /// Returns a modified copy of region such that it starts at the beginning of a line, and ends at the end of a line. Note that it may span several lines.
    fn line(&self, region) -> Region;
    /// As line(), but the region includes the trailing newline character, if any.
    fn full_line(&self, point) -> Region;
    /// As line(), but the region includes the trailing newline character, if any.
    fn full_line(&self, region) -> Region;
    /// Returns a list of lines (in sorted order) intersecting the region.
    fn lines(&self, region) -> [Region];
    /// Splits the region up such that each region returned exists on exactly one line.
    fn split_by_newlines(&self, region) -> [Region];
    /// Returns the word that contains the point.
    fn word(&self, point) -> Region;
    /// Returns a modified copy of region such that it starts at the beginning of a word, and ends at the end of a word. Note that it may span several words.
    fn word(&self, region) -> Region;

    /// Classifies pt, returning a bitwise OR of zero or more of these flags:
    /// # CLASS_WORD_START
    /// # CLASS_WORD_END
    /// # CLASS_PUNCTUATION_START
    /// # CLASS_PUNCTUATION_END
    /// # CLASS_SUB_WORD_START
    /// # CLASS_SUB_WORD_END
    /// # CLASS_LINE_START
    /// # CLASS_LINE_END
    /// # CLASS_EMPTY_LINE
    fn classify(&self, point) -> int;
    /// Finds the next location after point that matches the given classes. If forward is False, searches backwards instead of forwards. classes is a bitwise OR of the sublime.CLASS_XXX flags. separators may be passed in, to define what characters should be considered to separate words.
    fn find_by_class(&self, point, forward, classes, <separators>) -> Region;
    /// Expands point to the left and right, until each side lands on a location that matches classes. classes is a bitwise OR of the sublime.CLASS_XXX flags. separators may be passed in, to define what characters should be considered to separate words.
    fn expand_by_class(&self, point, classes, <separators>) -> Region;
    /// Expands region to the left and right, until each side lands on a location that matches classes. classes is a bitwise OR of the sublime.CLASS_XXX flags. separators may be passed in, to define what characters should be considered to separate words.
    fn expand_by_class(&self, region, classes, <separators>) -> Region;
    /// Returns the first Region matching the regex pattern, starting from the given point, or None if it can't be found. The optional flags parameter may be sublime.LITERAL, sublime.IGNORECASE, or the two ORed together.
    fn find(&self, pattern, fromPosition, <flags>) -> Region;
    /// Returns all (non-overlapping) regions matching the regex pattern. The optional flags parameter may be sublime.LITERAL, sublime.IGNORECASE, or the two ORed together. If a format string is given, then all matches will be formatted with the formatted string and placed into the extractions list.
    fn find_all(&self, pattern, <flags>, <format>, <extractions>) -> [Region];
    /// Calculates the 0 based line and column numbers of the point.
    fn rowcol(&self, point) -> (int, int);
    /// Calculates the character offset of the given, 0 based, row and column. Note that 'col' is interpreted as the number of characters to advance past the beginning of the row.
    fn text_point(&self, row, col) -> int;

    /// Changes the syntax used by the view. `syntax_file` should be a name along the lines of `Packages/Python/Python.tmLanguage`. To retrieve the current syntax, use `view.settings().get('syntax').`
    fn set_syntax_file(&self, syntax_file);

    /// Returns the extent of the syntax name assigned to the character at the given point.
    fn extract_scope(&self, point) -> Region;
    /// Returns the syntax name assigned to the character at the given point.
    fn scope_name(&self, point) -> String;
    /// Matches the selector against the scope at the given location, returning a score. A score of 0 means no match, above 0 means a match. Different selectors may be compared against the same scope: a higher score means the selector is a better match for the scope.
    fn score_selector(&self, point, selector) -> Int;
    /// Finds all regions in the file matching the given selector, returning them as a list.
    fn find_by_selector(&self, selector) -> [Regions];

    /// Scroll the view to show the given point.
    fn show(&self, point, <show_surrounds>);
    /// Scroll the view to show the given region.
    fn show_region(&self, region, <show_surrounds>);
    /// Scroll the view to show the given region set.
    fn show_regions(&self, region_set, <show_surrounds>);
    /// Scroll the view to center on the point.
    fn show_at_center(&self, point);
    /// Scroll the view to center on the region.
    fn show_at_center(&self, region);
    /// Returns the currently visible area of the view.
    fn visible_region(&self) -> Region;
    /// Returns the offset of the viewport in layout coordinates.
    fn viewport_position(&self) -> Vector;
    /// Scrolls the viewport to the given layout position.
    fn set_viewport_position(&self, vector, <animate<);
    /// Returns the width and height of the viewport.
    fn viewport_extent(&self) -> vector;
    /// Returns the width and height of the layout.
    fn layout_extent(&self) -> vector;
    /// Converts a text position to a layout position
    fn text_to_layout(&self, point) -> vector;
    /// Converts a layout position to a text position
    fn layout_to_text(&self, vector) -> point;
    /// Converts a window position to a layout position
    fn window_to_layout(&self, vector) -> vector;
    /// Converts a window position to a text position
    fn window_to_text(&self, vector) -> point;
    /// Returns the light height used in the layout
    fn line_height(&self) -> real;
    /// Returns the typical character width used in the layout
    fn em_width(&self) -> real;

    // /// Returns the completions for the given prefix, based on the contents of the buffer. Completions will be ordered by frequency, and distance from the given point, if supplied.
    // fn extract_completions(&self, prefix, <point>) -> [String];

    // /// Shows the autocomplete menu, at the given point, with the given completions. If an entry is selected, the given prefix will be replaced with the selected completion. Each completion may be either a string, or a tuple consisting of a description and a command to run.
    // fn show_completions(&self, point, prefix, [completions]);

    /// Add a set of regions to the view. If a set of regions already exists with the given key, they will be overwritten. The scope is used to source a color to draw the regions in, it should be the name of a scope, such as "comment" or "string". If the scope is empty, the regions won't be drawn.
    /// The optional icon name, if given, will draw the named icons in the gutter next to each region. The icon will be tinted using the color associated with the scope. Valid icon names are `dot`, `circle`, `bookmark` and `cross`. The icon name may also be a full package relative path, such as `Packages/Theme - Default/dot.png`.
    /// The optional flags parameter is a bitwise combination of:
    /// # `sublime.DRAW_EMPTY`. Draw empty regions with a vertical bar. By default, they aren't drawn at all.
    /// # `sublime.HIDE_ON_MINIMAP`. Don't show the regions on the minimap.
    /// # `sublime.DRAW_EMPTY_AS_OVERWRITE`. Draw empty regions with a horizontal bar instead of a vertical one.
    /// # `sublime.DRAW_NO_FILL`. Disable filling the regions, leaving only the outline.
    /// # `sublime.DRAW_NO_OUTLINE`. Disable drawing the outline of the regions.
    /// # `sublime.DRAW_SOLID_UNDERLINE`. Draw a solid underline below the regions.
    /// # `sublime.DRAW_STIPPLED_UNDERLINE`. Draw a stippled underline below the regions.
    /// # `sublime.DRAW_SQUIGGLY_UNDERLINE`. Draw a squiggly underline below the regions.
    /// # `sublime.PERSISTENT`. Save the regions in the session.
    /// # `sublime.HIDDEN`. Don't draw the regions.
    /// The underline styles are exclusive, either zero or one of them should be given. If using an underline, DRAW_NO_FILL and DRAW_NO_OUTLINE should generally be passed in.
    fn add_regions(&self, key, [regions], <scope>, <icon>, <flags>);
    /// Return the regions associated with the given key, if any
    fn get_regions(&self, key) -> [regions];
    /// Removed the named regions
    fn erase_regions(&self, key);

    /// Adds the status key to the view. The value will be displayed in the status bar, in a comma separated list of all status values, ordered by key. Setting the value to the empty string will clear the status.
    fn set_status(&self, key, value);
    /// Returns the previously assigned value associated with the key, if any.
    fn get_status(&self, key) -> String;
    /// Clears the named status.
    fn erase_status(&self, key);

    /// Returns the command name, command arguments, and repeat count for the given history entry, as stored in the undo / redo stack.
    /// Index 0 corresponds to the most recent command, -1 the command before that, and so on. Positive values for index indicate to look in the redo stack for commands. If the undo / redo history doesn't extend far enough, then (None, None, 0) will be returned.
    /// Setting modifying_only to True (the default is False) will only return entries that modified the buffer.
    fn command_history(&self, index, <modifying_only>) -> (String,Dict,int);
    /// Returns the current change count. Each time the buffer is modified, the change count is incremented. The change count can be used to determine if the buffer has changed since the last it was inspected.
    fn change_count(&self) -> int;
    /// Folds the given region, returning False if it was already folded
    fn fold(&self, region) -> bool;
    /// Folds the given regions, returning False if they were already folded
    fn fold_regions(&self, [regions]) -> bool;
    /// Unfolds all text in the region, returning the unfolded regions
    fn unfold(&self, region) -> [regions];
    /// Unfolds all text in the regions, returning the unfolded regions
    fn unfold_regions(&self, [regions]) -> [regions];
    /// Returns the encoding currently associated with the file
    fn encoding(&self) -> String;
    /// Applies a new encoding to the file. This encoding will be used the next time the file is saved.
    fn set_encoding(&self, encoding);
    /// Returns the line endings used by the current file.
    fn line_endings(&self) -> String;
    /// Sets the line endings that will be applied when next saving.
    fn set_line_endings(&self, line_endings);
    /// Returns the overwrite status, which the user normally toggles via the insert key.
    fn overwrite_status(&self) -> Bool;
    /// Sets the overwrite status.
    fn set_overwrite_status(&self, enabled);
    /// Extract all the symbols defined in the buffer.
    fn symbols(&self, line_endings) -> [(Region, String)];
    /// Shows a pop up menu at the caret, to select an item in a list. on_done will be called once, with the index of the selected item. If the pop up menu was cancelled, on_done will be called with an argument of -1.
    /// Items is an array of strings. Flags currently only has no option.
    fn show_popup_menu(&self, items, on_done, <flags>);
}

/// Maintains a set of Regions, ensuring that none overlap. The regions are kept in sorted order.
struct Selection;

impl Selection {
    /// Removes all regions.
    fn clear(&self);
    /// Adds the given region. It will be merged with any intersecting regions already contained within the set.
    fn add(&self, region);
    /// Adds all regions in the given set.
    fn add_all(&self, region_set);
    /// Subtracts the region from all regions in the set.
    fn subtract(&self, region);
    /// Returns true iff the given region is a subset.
    fn contains(&self, region) -> bool;
}

/// Represents an area of the buffer. Empty regions, where a == b are valid.
struct Region {
    /// The first end of the region.
    a: usize,
    /// The second end of the region. May be less that a, in which case the region is a reversed one.
    b: usize,
    /// The target horizontal position of the region, or -1 if undefined. Effects behavior when pressing the up or down keys.
    xpos: Option<usize>
}

impl Region {
    /// Creates a Region with initial values a and b.
    fn new(a, b) -> Region;
    /// Returns the minimum of a and b.
    fn begin(&self) -> int;
    /// Returns the maximum of a and b.
    fn end(&self) -> int;
    /// Returns the number of characters spanned by the region. Always >= 0.
    fn size(&self) -> int;
    /// Returns true iff begin() == end().
    fn empty(&self) -> bool;
    /// Returns a Region spanning both this and the given regions.
    fn cover(&self, region) -> Region;
    /// Returns the set intersection of the two regions.
    fn intersection(&self, region) -> Region;
    /// Returns True iff this == region or both include one or more positions in common.
    fn intersects(&self, region) -> bool;
    /// Returns True iff begin() <= point <= end().
    fn contains(&self, point) -> bool;
    /// Returns True iff the given region is a subset.
    fn contains_region(&self, region) -> bool;
}


/// Edit objects have no functions, they exist to group buffer modifications.
/// Edit objects are passed to TextCommands, and are not user createable. Using an invalid Edit object, or an Edit object from a different view, will cause the functions that require them to fail.
struct Edit;


struct Window;

impl Window {
    /// Returns a number that uniquely identifies this window.
    fn id(&self) -> int;
    /// Creates a new file. The returned view will be empty, and its is_loaded method will return True.
    fn new_file(&self) -> View;

    /// Opens the named file, and returns the corresponding view. If the file is already opened, it will be brought to the front. Note that as file loading is asynchronous, operations on the returned view won't be possible until its is_loading() method returns False.
    /// The optional flags parameter is a bitwise combination of:
    /// # `sublime.ENCODED_POSITION`. Indicates the file_name should be searched for a `:row` or `:row:col` suffix
    /// # `sublime.TRANSIENT`. Open the file as a preview only: it won't have a tab assigned it until modified
    fn open_file(&self, file_name, <flags>) -> View;
    /// Finds the named file in the list of open files, and returns the corresponding View, or None if no such file is open.
    fn find_open_file(&self, file_name) -> View;
    /// Returns the currently edited view.
    fn active_view(&self) -> View;
    /// Returns the currently edited view in the given group.
    fn active_view_in_group(&self, group) -> View;
    /// Returns all open views in the window.
    fn views(&self) -> [View];
    /// Returns all open views in the given group.
    fn views_in_group(&self, group) -> [View];
    /// Returns the number of view groups in the window.
    fn num_groups(&self) -> int;
    /// Returns the index of the currently selected group.
    fn active_group(&self) -> int;
    /// Makes the given group active.
    fn focus_group(&self, group);
    /// Switches to the given view.
    fn focus_view(&self, view);
    /// Returns the group, and index within the group of the view. Returns -1 if not found.
    fn get_view_index(&self, view) -> (group, index);
    /// Moves the view to the given group and index.
    fn set_view_index(&self, view, group, index);
    /// Returns a list of the currently open folders.
    fn folders(&self) -> [String];
    /// Returns name of the currently opened project file, if any.
    fn project_file_name(&self) -> String;
    /// Returns the project data associated with the current window. The data is in the same format as the contents of a .sublime-project
    fn project_data(&self) -> Dictionary;
    /// Updates the project data associated with the current window. If the window is associated with a .sublime-project file, the project file will be updated on disk, otherwise the window will store the data intern
    fn set_project_data(&self, data);

    // /// Returns the active view in the given group.
    // fn activeViewInGroup(&self, groupIdx) -> View;
    // /// Returns a list of all the views in the current window.
    // fn views(&self) -> [View];
    // /// Returns a list of all the views in given group.
    // fn viewsInGroup(&self, groupIdx) -> [View];
    // /// Focuses the given view.
    // fn focusView(&self, view);
    // /// Returns the group, and the index within the group, of the given view.
    // fn getViewPosition(&self, view) -> groupIdx, viewIdx;
    // /// Moves the view to to the given group and index within the group.
    // fn setViewPosition(&self, view, groupIdx, viewIdx);
    /// Runs the named Command with the (optional) given arguments. Window.run_command is able to run both any sort of command, dispatching the command via input focus.
    fn run_command(&self, string, <args>);

    // /// Returns True iff the command is enabled with the (optional) given arguments
    // fn canRunCommand(&self, string, <args>);
    // /// Returns true of the Window is currently in full screen mode.
    // fn isFullScreen(&self) -> bool;
    // /// Shows the quick panel, populated with displayArgs. When an entry is selected, the command is run, with the arg corresponding to the selected display arg as a parameter. key should be used if updating the list asynchronously, or left blank otherwise.
    // /// Argument displayArgs is optional, and will default to the list given for args.
    // /// The optional flags parameter is a bitwise combination of:
    // /// # `sublime.QUICK_PANEL_FILES`. Indicates that the args correspond to file names, which changes how they're drawn.
    // /// # `sublime.QUICK_PANEL_MULTI_SELECT`. Enables Ctrl+Enter to select all displayed items in the quick panel, up to a maximum of 16.
    // /// # `sublime.QUICK_PANEL_NO_MULTI_SELECT_SAFETY_CHECK`. Used in conjunction with the above, removes the 16 item limit.
    // /// # `sublime.QUICK_PANEL_UPDATE_ONLY`. When asynchronously updating the panel, ignore the update call if the panel has been closed.
    // /// # `sublime.QUICK_PANEL_MONOSPACE_FONT`. Use a monospace font to draw the quick panel.
    // fn showQuickPanel(&self, key, command, args, <displayArgs>, <flags>);
    // /// Shows the quick panel, populated with displayArgs.
    // /// Callback onSelect will be run for each item selected, with the index of the item passed in as a parameter.
    // /// Callback onCancel will be run if the panel is closed without any items being selected.
    // /// The flags parameter should be 0, or a bitwise combination of:
    // /// #`sublime.SELECT_PANEL_FILES`. Indicates that the args correspond to file names, which changes how they're drawn.
    // /// #`sublime.SELECT_PANEL_MULTI_SELECT`. Enables Ctrl+Enter to select all displayed items in the quick panel, up to a maximum of 16.
    // /// #`sublime.SELECT_PANEL_NO_MULTI_SELECT_SAFETY_CHECK`. Used in conjunction with the above, removes the 16 item limit.
    // /// #`sublime.SELECT_PANEL_UPDATE_ONLY`. When asynchronously updating the panel, ignore the update call if the panel has been closed.
    // /// #`sublime.SELECT_PANEL_MONOSPACE_FONT`. Use a monospace font to draw the quick panel.
    // /// Argument key should be used if updating the list asynchronously, or left blank otherwise.
    // /// Argument selectedIndex should be the index of the item to be initially selected, or omitted otherwise.
    // fn showSelectPanel(&self, displayArgs, onSelect, onCancel, flags, <key>, <selectedIndex>);

    /// Shows a quick panel, to select an item in a list. on_done will be called once, with the index of the selected item. If the quick panel was cancelled, on_done will be called with an argument of -1.
    /// Items may be an array of strings, or an array of string arrays. In the latter case, each entry in the quick panel will show multiple rows.
    /// Flags currently only has one option, `sublime.MONOSPACE_FONT`
    /// Callback on_highlighted, if given, will be called every time the highlighted item in the quick panel is changed.
    fn show_quick_panel(&self, items, on_done, <flags>, <selected_index>, <on_highlighted>);
    /// Shows the input panel, to collect a line of input from the user. on_done and on_change, if not None, should both be functions that expect a single string argument. on_cancel should be a function that expects no arguments. The view used for the input widget is returned.
    fn show_input_panel(&self, caption, initial_text, on_done, on_change, on_cancel) -> View;
    /// Returns the view associated with the named output panel, created it if required. The output panel can be shown by running the `show_panel` window command, with the `panel` argument set to the name with an  "output." prefix
    fn create_output_panel(&self, name) -> View;
    /// Returns all locations where the symbol is defined across files in the current project.
    fn lookup_symbol_in_index(&self, symbol) -> [Location];
    /// Returns all locations where the symbol is defined across open files.
    fn lookup_symbol_in_open_files(&self, symbol) -> [Location];
    /// Returns a dictionary of strings populated with contextual keys: `packages, platform, file, file_path, file_name, file_base_name, file_extension, folder, project, project_path, project_name, project_base_name, project_extension`. This dictionary is suitable for passing to `sublime.expand_variables()`.
    fn extract_variables(&self) -> Dictionary;
}

struct Settings;

impl Settings {
    /// Returns the named setting.
    fn get(&self, name) -> value;
    /// Sets the named setting. Only primitive types, lists, and dictionaries are accepted.
    fn set(&self, name, value);
    /// Removes the named setting. Does not remove it from any parent Settings.
    fn erase(&self, name);
    /// Returns true iff the named option exists in this set of Settings or one of its parents.
    fn has(&self, name) -> bool;
    /// Register a callback to be run whenever a setting in this object is changed.
    fn add_on_change(&self, key, on_change);
    /// Remove all callbacks registered with the given key.
    fn clear_on_change(&self, key);
}


/// Note that many of these events are triggered by the buffer underlying the view, and thus the method is only called once, with the first view as the parameter.
struct EventListener;

impl EventListener {
    /// Called when a new buffer is created.
    fn on_new(&self, view);
    /// Called when a new buffer is created. Runs in a separate thread, and does not block the application.
    fn on_new_async(&self, view);
    /// Called when a view is cloned from an existing one.
    fn on_clone(&self, view);
    /// Called when a view is cloned from an existing one. Runs in a separate thread, and does not block the application.
    fn on_clone_async(&self, view);
    /// Called when the file is finished loading.
    fn on_load(&self, view);
    /// Called when the file is finished loading. Runs in a separate thread, and does not block the application.
    fn on_load_async(&self, view);
    /// Called when a view is about to be closed. The view will still be in the window at this point.
    fn on_pre_close(&self, view);
    /// Called when a view is closed (note, there may still be other views into the same buffer).
    fn on_close(&self, view);
    /// Called just before a view is saved.
    fn on_pre_save(&self, view);
    /// Called just before a view is saved. Runs in a separate thread, and does not block the application.
    fn on_pre_save_async(&self, view);
    /// Called after a view has been saved.
    fn on_post_save(&self, view);
    /// Called after a view has been saved. Runs in a separate thread, and does not block the application.
    fn on_post_save_async(&self, view);
    /// Called after changes have been made to a view.
    fn on_modified(&self, view);
    /// Called after changes have been made to a view. Runs in a separate thread, and does not block the application.
    fn on_modified_async(&self, view);
    /// Called after the selection has been modified in a view.
    fn on_selection_modified(&self, view);
    /// Called after the selection has been modified in a view. Runs in a separate thread, and does not block the application.
    fn on_selection_modified_async(&self, view);
    /// Called when a view gains input focus.
    fn on_activated(&self, view);
    /// Called when a view gains input focus. Runs in a separate thread, and does not block the application.
    fn on_activated_async(&self, view);
    /// Called when a view loses input focus.
    fn on_deactivated(&self, view);
    /// Called when a view loses input focus. Runs in a separate thread, and does not block the application.
    fn on_deactivated_async(&self, view);
    /// Called when a text command is issued. The listener may return a (command, arguments) tuple to rewrite the command, or None to run the command unmodified.
    fn on_text_command(&self, view, command_name, args) -> (new_command_name, new_args);
    /// Called when a window command is issued. The listener may return a (command, arguments) tuple to rewrite the command, or None to run the command unmodified.
    fn on_window_command(&self, window, command_name, args) -> (new_command_name, new_args);

    /// Called after a text command has been executed.
    fn post_text_command(&self, view, command_name, args);
    /// Called after a window command has been executed.
    fn post_window_command(&self, window, command_name, args);

    // /// Called after a project has been loaded.
    // fn onProjectLoad(&self, window);
    // /// Called after a project has been closed.
    // fn onProjectClose(&self, window);

    /// Called when determining to trigger a key binding with the given context key. If the plugin knows how to respond to the context, it should return either True of False. If the context is unknown, it should return None.
    /// Argument `operator` is one of:
    /// # `sublime.OP_EQUAL`. Is the value of the context equal to the operand?
    /// # `sublime.OP_NOT_EQUAL`. Is the value of the context not equal to the operand?
    /// # `sublime.OP_REGEX_MATCH`. Does the value of the context match the regex given in operand?
    /// # `sublime.OP_NOT_REGEX_MATCH`. Does the value of the context not match the regex given in operand?
    /// # `sublime.OP_REGEX_CONTAINS`. Does the value of the context contain a substring matching the regex given in operand?
    /// # `sublime.OP_NOT_REGEX_CONTAINS`. Does the value of the context not contain a substring matching the regex given in operand?
    /// Argument `match_all` should be used if the context relates to the selections: does every selection have to match (match_all = True), or is at least one matching enough (match_all = False)?
    fn on_query_context(&self, view, key, operator, operand, match_all) -> bool or None;
}


struct ApplicationCommand;

impl ApplicationCommand {
    /// Called when the command is run.
    fn run(&self, <args>);
    /// Returns true if the command is able to be run at this time. The default implementation simply always returns True.
    fn is_enabled(&self, <args>) -> bool;
    /// Returns true if the command should be shown in the menu at this time. The default implementation always returns True.
    fn is_visible(&self, <args>) -> bool;
    /// Returns true if a checkbox should be shown next to the menu item. The `.sublime-menu` file must have the checkbox attribute set to true for this to be used.
    fn is_checked(&self, <args>) -> bool;
    /// Returns a description of the command with the given arguments. Used in the menu, if no caption is provided. Return None to get the default description.
    fn description(&self, <args>) -> String;
}

/// WindowCommands are instantiated once per window. The Window object may be retrieved via `self.window`
struct WindowCommand;

impl WindowCommand
    /// Called when the command is run.
    fn run(&self, <args>);
    /// Returns true if the command is able to be run at this time. The default implementation simply always returns True.
    fn is_enabled(&self, <args>) -> bool;
    /// Returns true if the command should be shown in the menu at this time. The default implementation always returns True.
    fn is_visible(&self, <args>) -> bool;
    /// Returns a description of the command with the given arguments. Used in the menu, if no caption is provided. Return None to get the default description.
    fn description(&self, <args>) -> String;
}

/// TextCommands are instantiated once per view. The View object may be retrieved via `self.view`
struct TextCommand;

impl TextCommand {
    /// Called when the command is run.
    fn run(&self, edit, <args>);
    /// Returns true if the command is able to be run at this time. The default implementation simply always returns True.
    fn is_enabled(&self, <args>) -> bool;
    /// Returns true if the command should be shown in the menu at this time. The default implementation always returns True.
    fn is_visible(&self, <args>) -> bool;
    /// Returns a description of the command with the given arguments. Used in the menus, and for Undo / Redo descriptions. Return None to get the default description.
    fn description(&self, <args>) -> String;
    /// Return True to receive an `event` argument when the command is triggered by a mouse action. The event information allows commands to determine which portion of the view was clicked on. The default implementation returns False.
    fn want_event(&self) -> bool;
}
