pub enum WordWrap {
     Auto,
     Wrap,
     NoWrap,
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

struct Trigger {
   selector: ScopeSelector,
   characters: String
}

struct Preferences {
	/// Sets the colors used within the text area
	color_scheme: std::path::PathBuf,

	/// Note that the font_face and font_size are overridden in the platform
    /// specific settings file, for example, "Preferences (Linux).sublime-settings".
    /// Because of this, setting them here will have no effect: you must set them
    /// in your User File Preferences.
	font_face: String,
	font_size: i32,

	/// Valid  are "no_bold", "no_italic", "no_antialias", "gray_antialias",
    /// "subpixel_antialias", "no_round" (OS X only), "gdi" (Windows only) and
    /// "directwrite" (Windows only)
	font_options: Vec<String>,

	/// Characters that are considered to separate words
	word_separators: core::regex::Regex,

	/// Set to false to prevent line numbers being drawn in the gutter
	line_numbers: bool,

	/// Set to false to hide the gutter altogether
	gutter: bool,

	/// Spacing between the gutter and the text
	margin: i32,

	/// Fold buttons are the triangles shown in the gutter to fold regions of text
	fold_buttons: bool,

	/// Hides the fold buttons unless the mouse is over the gutter
	fade_fold_buttons: bool,

	/// Columns in which to display vertical rulers
	rulers: Vec<String>,

	/// Set to true to turn spell checking on by default
	spell_check: bool,

	/// The number of spaces a tab is considered equal to
	tab_size: i32,

	/// Set to true to insert spaces when tab is pressed
	translate_tabs_to_spaces: bool,

	/// If translate_tabs_to_spaces is true, use_tab_stops will make tab and
    /// backspace insert/delete up to the next tabstop
    use_tab_stops: bool,

    /// Set to false to disable detection of tabs vs. spaces on load
    detect_indentation: bool,

    /// Calculates indentation automatically when pressing enter
    auto_indent: bool,

    /// Makes auto indent a little smarter, e.g., by indenting the next line
    /// after an if statement in C. Requires auto_indent to be enabled.
    smart_indent: bool,

    /// Adds whitespace up to the first open bracket when indenting. Requires
    /// auto_indent to be enabled.
    indent_to_bracket: bool,

    /// Trims white space added by auto_indent when moving the caret off the
    /// line.
    trim_automatic_white_space: bool,

    /// Disables horizontal scrolling if enabled.
    /// May be set to true, false, or "auto", where it will be disabled for
    /// source code, and otherwise enabled.
    word_wrap: WordWrap,

    /// Set to a value other than 0 to force wrapping at that column rather than the
    /// window width
    wrap_width: i32,

    /// Set to false to prevent word wrapped lines from being indented to the same
    /// level
    indent_subsequent_lines: bool,

    /// Draws text centered in the window rather than left aligned
    draw_centered: bool,

    /// Controls auto pairing of quotes, brackets etc
    auto_match_enabled: bool,

    /// Word list to use for spell checking
    dictionary: std::path::PathBuf,

    /// Sets which scopes are checked for spelling errors
    spelling_selector: Vec<Trigger>,

    /// Set to true to draw a border around the visible rectangle on the minimap.
    /// The color of the border will be determined by the "minimapBorder" key in
    /// the color scheme
    draw_minimap_border: String,

    /// Always visualise the viewport on the minimap, as opposed to only
    /// showing it on mouse over
    always_show_minimap_viewport: bool,

    /// If enabled, will highlight any line with a caret
    highlight_line: bool,

    /// Valid values are "smooth", "phase", "blink" and "solid".
    caret_style: CaretStyle,

    /// These settings control the size of the caret
    caret_extra_top: 	i32,
    caret_extra_bottom: i32,
    caret_extra_width: 	i32,

    /// Set to false to disable underlining the brackets surrounding the caret
    match_brackets: bool,

    /// Set to false if you'd rather only highlight the brackets when the caret is
    /// next to one
    match_brackets_content: bool,

    /// Set to false to not highlight square brackets. This only takes effect if
    /// match_brackets is true
    match_brackets_square: bool,

    // Set to false to not highlight curly brackets. This only takes effect if
    // match_brackets is true
    match_brackets_braces: bool,

    // Set to false to not highlight angle brackets. This only takes effect if
    // match_brackets is true
    match_brackets_angle: bool,

    // Enable visualization of the matching tag in HTML and XML
    match_tags: bool,

    // Highlights other occurrences of the currently selected text
    match_selection: bool,

    // Additional spacing at the top of each line, in pixels
    line_padding_top: i32,

    // Additional spacing at the bottom of each line, in pixels
    line_padding_bottom: i32,

    // Set to false to disable scrolling past the end of the buffer.
    // On OS X, this value is overridden in the platform specific settings, so
    // you'll need to place this line in your user settings to override it.
    scroll_past_end: bool,

    // This controls what happens when pressing up or down when on the first
    // or last line.
    // On OS X, this value is overridden in the platform specific settings, so
    // you'll need to place this line in your user settings to override it.
    move_to_limit_on_up_down: bool,

    // Set to "none" to turn off drawing white space, "selection" to draw only the
    // white space within the selection, and "all" to draw all white space
    draw_white_space: DrawWhiteSpace,

    // Set to false to turn off the indentation guides.
    // The color and width of the indent guides may be customized by editing
    // the corresponding .tmTheme file, and specifying the colors "guide",
    // "activeGuide" and "stackGuide"
    draw_indent_guides: bool, 

    // Controls how the indent guides are drawn, valid options are
    // "draw_normal" and "draw_active". draw_active will draw the indent
    // guides containing the caret in a different color.
    indent_guide_options: Vec<String>,

    // Set to true to removing trailing white space on save
    trim_trailing_white_space_on_save: bool,

    // Set to true to ensure the last line of the file ends in a newline
    // character when saving
    ensure_newline_at_eof_on_save: bool,

    // Set to true to automatically save files when switching to a different file
    // or application
    save_on_focus_lost: bool,

    // Save via writing to an alternate file, and then renaming it over the
    // original file.
    atomic_save: bool,

    // The encoding to use when the encoding can't be determined automatically.
    // ASCII, UTF-8 and UTF-16 encodings will be automatically detected.
    fallback_encoding: String,

    // Encoding used when saving new files, and files opened with an undefined
    // encoding (e.g., plain ascii files). If a file is opened with a specific
    // encoding (either detected or given explicitly), this setting will be
    // ignored, and the file will be saved with the encoding it was opened
    // with.
    default_encoding: String,

    // Files containing null bytes are opened as hexadecimal by default
    enable_hexadecimal_encoding: bool,

    // Determines what character(s) are used to terminate each line in new files.
    // Valid values are 'system' (whatever the OS uses), 'windows' (CRLF) and
    // 'unix' (LF only).
    default_line_ending: DefaultLineEnding,

    // When enabled, pressing tab will insert the best matching completion.
    // When disabled, tab will only trigger snippets or insert a tab.
    // Shift+tab can be used to insert an explicit tab when tab_completion is
    // enabled.
    tab_completion: bool,

    // Enable auto complete to be triggered automatically when typing.
    auto_complete: bool,

    // The maximum file size where auto complete will be automatically triggered.
    auto_complete_size_limit: i32,

    // The delay, in ms, before the auto complete window is shown after typing
    auto_complete_delay: i32,

    // Controls what scopes auto complete will be triggered in
    auto_complete_selector: Vec<Trigger>,

    // Additional situations to trigger auto complete
    auto_complete_triggers: Vec<Trigger>,

    // By default, auto complete will commit the current completion on enter.
    // This setting can be used to make it complete on tab instead.
    // Completing on tab is generally a superior option, as it removes
    // ambiguity between committing the completion and inserting a newline.
    auto_complete_commit_on_tab: bool,
}