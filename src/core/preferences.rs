pub enum WordWrap {
    Auto,
    Wrap,
    NoWrap
}

pub enum CaretStyle {
    Smooth,
    Phase,
    Blink,
    Solid
}

pub enum DrawWhiteSpace {
    None,
    Selection,
    All
}

pub enum DefaultLineEnding {
    System,
    Windows,
    Unix
}

pub enum GPUWindowBuffer {
    Auto,
    Enabled,
    Disabled
}

pub enum OverlayScrollBars {
    System,
    Enabled,
    Disabled
}

pub enum EnableTelemetry {
	Auto,
    Enabled,
    Disabled
}

pub struct Trigger {
   pub selector: ScopeSelector,
   pub characters: String
}

pub struct Preferences {
	/// Sets the colors used within the text area
	pub color_scheme: std::path::PathBuf,

	/// Note that the font_face and font_size are overridden in the platform
    /// specific settings file, for example, "Preferences (Linux).sublime-settings".
    /// Because of this, setting them here will have no effect: you must set them
    /// in your User File Preferences.
	pub font_face: String,
	pub font_size: i32,

	/// Valid  are "no_bold", "no_italic", "no_antialias", "gray_antialias",
    /// "subpixel_antialias", "no_round" (OS X only), "gdi" (Windows only) and
    /// "directwrite" (Windows only)
	pub font_options: Vec<String>,

	/// Characters that are considered to separate words
	pub word_separators: core::regex::Regex,

	/// Set to false to prevent line numbers being drawn in the gutter
	pub line_numbers: bool,

	/// Set to false to hide the gutter altogether
	pub gutter: bool,

	/// Spacing between the gutter and the text
	pub margin: i32,

	/// Fold buttons are the triangles shown in the gutter to fold regions of text
	pub fold_buttons: bool,

	/// Hides the fold buttons unless the mouse is over the gutter
	pub fade_fold_buttons: bool,

	/// Columns in which to display vertical rulers
	pub rulers: Vec<String>,

	/// Set to true to turn spell checking on by default
	pub spell_check: bool,

	/// The number of spaces a tab is considered equal to
	pub tab_size: i32,

	/// Set to true to insert spaces when tab is pressed
	pub translate_tabs_to_spaces: bool,

	/// If translate_tabs_to_spaces is true, use_tab_stops will make tab and
    /// backspace insert/delete up to the next tabstop
    pub use_tab_stops: bool,

    /// Set to false to disable detection of tabs vs. spaces on load
    pub detect_indentation: bool,

    /// Calculates indentation automatically when pressing enter
    pub auto_indent: bool,

    /// Makes auto indent a little smarter, e.g., by indenting the next line
    /// after an if statement in C. Requires auto_indent to be enabled.
    pub smart_indent: bool,

    /// Adds whitespace up to the first open bracket when indenting. Requires
    /// auto_indent to be enabled.
    pub indent_to_bracket: bool,

    /// Trims white space added by auto_indent when moving the caret off the
    /// line.
    pub trim_automatic_white_space: bool,

    /// Disables horizontal scrolling if enabled.
    /// May be set to true, false, or "auto", where it will be disabled for
    /// source code, and otherwise enabled.
    pub word_wrap: WordWrap,

    /// Set to a value other than 0 to force wrapping at that column rather than the
    /// window width
    pub wrap_width: i32,

    /// Set to false to prevent word wrapped lines from being indented to the same
    /// level
    pub indent_subsequent_lines: bool,

    /// Draws text centered in the window rather than left aligned
    pub draw_centered: bool,

    /// Controls auto pairing of quotes, brackets etc
    pub auto_match_enabled: bool,

    /// Word list to use for spell checking
    pub dictionary: std::path::PathBuf,

    /// Sets which scopes are checked for spelling errors
    pub spelling_selector: Vec<Trigger>,

    /// Set to true to draw a border around the visible rectangle on the minimap.
    /// The color of the border will be determined by the "minimapBorder" key in
    /// the color scheme
    pub draw_minimap_border: String,

    /// Always visualise the viewport on the minimap, as opposed to only
    /// showing it on mouse over
    pub always_show_minimap_viewport: bool,

    /// If enabled, will highlight any line with a caret
    pub highlight_line: bool,

    /// Valid values are "smooth", "phase", "blink" and "solid".
    pub caret_style: CaretStyle,

    /// These settings control the size of the caret
    pub caret_extra_top: 	i32,
    pub caret_extra_bottom: i32,
    pub caret_extra_width: 	i32,

    /// Set to false to disable underlining the brackets surrounding the caret
    pub match_brackets: bool,

    /// Set to false if you'd rather only highlight the brackets when the caret is
    /// next to one
    pub match_brackets_content: bool,

    /// Set to false to not highlight square brackets. This only takes effect if
    /// match_brackets is true
    pub match_brackets_square: bool,

    // Set to false to not highlight curly brackets. This only takes effect if
    // match_brackets is true
    pub match_brackets_braces: bool,

    // Set to false to not highlight angle brackets. This only takes effect if
    // match_brackets is true
    pub match_brackets_angle: bool,

    // Enable visualization of the matching tag in HTML and XML
    pub match_tags: bool,

    // Highlights other occurrences of the currently selected text
    pub match_selection: bool,

    // Additional spacing at the top of each line, in pixels
    pub line_padding_top: i32,

    // Additional spacing at the bottom of each line, in pixels
    pub line_padding_bottom: i32,

    // Set to false to disable scrolling past the end of the buffer.
    // On OS X, this value is overridden in the platform specific settings, so
    // you'll need to place this line in your user settings to override it.
    pub scroll_past_end: bool,

    // This controls what happens when pressing up or down when on the first
    // or last line.
    // On OS X, this value is overridden in the platform specific settings, so
    // you'll need to place this line in your user settings to override it.
    pub move_to_limit_on_up_down: bool,

    // Set to "none" to turn off drawing white space, "selection" to draw only the
    // white space within the selection, and "all" to draw all white space
    pub draw_white_space: DrawWhiteSpace,

    // Set to false to turn off the indentation guides.
    // The color and width of the indent guides may be customized by editing
    // the corresponding .tmTheme file, and specifying the colors "guide",
    // "activeGuide" and "stackGuide"
    pub draw_indent_guides: bool, 

    // Controls how the indent guides are drawn, valid options are
    // "draw_normal" and "draw_active". draw_active will draw the indent
    // guides containing the caret in a different color.
    pub indent_guide_options: Vec<String>,

    // Set to true to removing trailing white space on save
    pub trim_trailing_white_space_on_save: bool,

    // Set to true to ensure the last line of the file ends in a newline
    // character when saving
    pub ensure_newline_at_eof_on_save: bool,

    // Set to true to automatically save files when switching to a different file
    // or application
    pub save_on_focus_lost: bool,

    // Save via writing to an alternate file, and then renaming it over the
    // original file.
    pub atomic_save: bool,

    // The encoding to use when the encoding can't be determined automatically.
    // ASCII, UTF-8 and UTF-16 encodings will be automatically detected.
    pub fallback_encoding: String,

    // Encoding used when saving new files, and files opened with an undefined
    // encoding (e.g., plain ascii files). If a file is opened with a specific
    // encoding (either detected or given explicitly), this setting will be
    // ignored, and the file will be saved with the encoding it was opened
    // with.
    pub default_encoding: String,

    // Files containing null bytes are opened as hexadecimal by default
    pub enable_hexadecimal_encoding: bool,

    // Determines what character(s) are used to terminate each line in new files.
    // Valid values are 'system' (whatever the OS uses), 'windows' (CRLF) and
    // 'unix' (LF only).
    pub default_line_ending: DefaultLineEnding,

    // When enabled, pressing tab will insert the best matching completion.
    // When disabled, tab will only trigger snippets or insert a tab.
    // Shift+tab can be used to insert an explicit tab when tab_completion is
    // enabled.
    pub tab_completion: bool,

    // Enable auto complete to be triggered automatically when typing.
    pub auto_complete: bool,

    // The maximum file size where auto complete will be automatically triggered.
    pub auto_complete_size_limit: i32,

    // The delay, in ms, before the auto complete window is shown after typing
    pub auto_complete_delay: i32,

    // Controls what scopes auto complete will be triggered in
    pub auto_complete_selector: ScopeSelector,

    // Additional situations to trigger auto complete
    pub auto_complete_triggers: Vec<Trigger>,

    // By default, auto complete will commit the current completion on enter.
    // This setting can be used to make it complete on tab instead.
    // Completing on tab is generally a superior option, as it removes
    // ambiguity between committing the completion and inserting a newline.
    pub auto_complete_commit_on_tab: bool,

    // Controls if auto complete is shown when snippet fields are active.
    // Only relevant if auto_complete_commit_on_tab is true.
    pub auto_complete_with_fields: bool,

    // Controls what happens when pressing the up key while the first item in
    // the auto complete window is selected: if false, the window is hidden,
    // otherwise the last item in the window is selected. Likewise for the
    // down key when the last item is selected.
    pub auto_complete_cycle: bool,

    // Automatically close HTML and XML tags when </ is entered.
    pub auto_close_tags: bool,

   	// By default, shift+tab will only unindent if the selection spans
    // multiple lines. When pressing shift+tab at other times, it'll insert a
    // tab character - this allows tabs to be inserted when tab_completion is
    // enabled. Set this to true to make shift+tab always unindent, instead of
    // inserting tabs.
    pub shift_tab_unindent: bool,

    // If true, the copy and cut commands will operate on the current line
    // when the selection is empty, rather than doing nothing.
    pub copy_with_empty_selection: bool,

    // If true, the selected text will be copied into the find panel when it's
    // shown.
    // On OS X, this value is overridden in the platform specific settings, so
    // you'll need to place this line in your user settings to override it.
    pub find_selected_text: bool, 

    // When auto_find_in_selection is enabled, the "Find in Selection" flag
    // will be enabled automatically when multiple lines of text are selected
    pub auto_find_in_selection: bool,

    // When drag_text is enabled, clicking on selected text will begin a
    // drag-drop operation. This is not currently implemented under Linux.
    pub drag_text: bool, 

    //
    // User Interface Settings
    //

    // The theme controls the look of Sublime Text's UI (buttons, tabs, scroll bars, etc)
    pub theme: std::path::PathBuf,

    // Set to 0 to disable smooth scrolling. Set to a value between 0 and 1 to
    // scroll slower, or set to larger than 1 to scroll faster
    pub scroll_speed: f32,

    // Controls side bar animation when expanding or collapsing folders
    pub tree_animation_enabled: bool,

    // Controls animation throughout the application
    pub animation_enabled: bool,

    // Makes tabs with modified files more visible
    pub highlight_modified_tabs: bool,

    pub show_tab_close_buttons: bool,

    // Show folders in the side bar in bold
    pub bold_folder_labels: bool,

    // OS X only: Set to true to disable Lion style full screen support.
    // Sublime Text must be restarted for this to take effect.
    pub use_simple_full_screen: bool,

    // OS X only. Valid values are true, false, and "auto". Auto will enable
    // the setting when running on a screen 2560 pixels or wider (i.e., a
    // Retina display). When this setting is enabled, OpenGL is used to
    // accelerate drawing. Sublime Text must be restarted for changes to take
    // effect.
    pub gpu_window_buffer: GPUWindowBuffer,

    // Valid values are "system", "enabled" and "disabled"
    pub overlay_scroll_bars: OverlayScrollBars,

    // Allows tabs to scroll left and right, instead of simply shrinking
    pub enable_tab_scrolling: bool,

    // Display file encoding in the status bar
    pub show_encoding: bool,

    // Display line endings in the status bar
    pub show_line_endings: bool,

    //
    // Application Behavior Settings
    //

    // Exiting the application with hot_exit enabled will cause it to close
    // immediately without prompting. Unsaved modifications and open files will
    // be preserved and restored when next starting.
    //
    // Closing a window with an associated project will also close the window
    // without prompting, preserving unsaved changes in the workspace file
    // alongside the project.
    pub hot_exit: bool,

    // remember_full_screen will allow Sublime Text to start in full screen
    // mode if it was exited in full screen mode. When set to false, Sublime
    // Text will never start in full screen mode.
    pub remember_full_screen: bool,

    // Always prompt before reloading a file, even if the file hasn't been
    // modified. The default behavior is to automatically reload a file if it
    // hasn't been edited. If a file has unsaved changes, a prompt will always
    // be shown.
    pub always_prompt_for_file_reload: bool,

    // OS X only: When files are opened from finder, or by dragging onto the
    // dock icon, this controls if a new window is created or not.
    pub open_files_in_new_window: bool,

    // OS X only: This controls if an empty window is created at startup or not.
    pub create_window_at_startup: bool,

    // Set to true to close windows as soon as the last file is closed, unless
    // there's a folder open within the window.
    // On OS X, this value is overridden in the platform specific settings, so
    // you'll need to place this line in your user settings to override it.
    pub close_windows_when_empty: bool,

    // Show the full path to files in the title bar.
    // On OS X, this value is overridden in the platform specific settings, so
    // you'll need to place this line in your user settings to override it.
    pub show_full_path: bool,

    // Shows the Build Results panel when building. If set to false, the Build
    // Results can be shown via the Tools/Build Results menu.
    pub show_panel_on_build: bool,

    // Preview file contents when clicking on a file in the side bar. Double
    // clicking or editing the preview will open the file and assign it a tab.
    pub preview_on_click: bool,

    // folder_exclude_patterns and file_exclude_patterns control which files
    // are listed in folders on the side bar. These can also be set on a per-
    // project basis.
    pub folder_exclude_patterns: core::workspace::project::ExcludePatterns,
    pub file_exclude_patterns: core::workspace::project::ExcludePatterns,
    // These files will still show up in the side bar, but won't be included in
    // Goto Anything or Find in Files
    pub binary_file_patterns: core::workspace::project::ExcludePatterns,

    // File indexing parses all files in the side bar, and builds an index of
    // their symbols. This is required for Goto Definition to work.
    pub index_files: bool,

    // Set the number threads to use for indexing. A value of 0 will make
    // Sublime Text guess based on the number of cores. Use the index_files
    // setting to disable all workers.
    pub index_workers: i32,

    // index_exclude_patterns indicate which files won't be indexed.
    pub index_exclude_patterns: Vec<String>,

    // When enabled, anonymised usage data is sent back, assisting Sublime HQ
    // in making informed decisions about improving Sublime Text. File names
    // and file contents are never included, but data such as computer
    // specifications, startup time, installed packages, and edited file types
    // are. When disabled, telemetry is neither recorded or sent.
    // A setting of auto will enable telemetry in dev builds, and disable
    // telemetry in regular builds.
    pub enable_telemetry: EnableTelemetry,

    // List any packages to ignore here. When removing entries from this list,
    // a restart may be required if the package contains plugins.
    pub ignored_packages: Vec<String>
}