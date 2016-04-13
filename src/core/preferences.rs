use super::settings::{Settings, ParseSettings, SettingsArray};
use std::str::FromStr;
use std::path::PathBuf;
use super::syntax::{ScopeSelector, ParseScopeError};
use super::regex::Regex;
use super::workspace::ExcludePatterns;

#[derive(Debug)]
pub enum WordWrap {
    Auto,
    Wrap,
    NoWrap
}

#[derive(Debug)]
pub enum CaretStyle {
    Smooth,
    Phase,
    Blink,
    Solid
}

#[derive(Debug)]
pub enum DrawWhiteSpace {
    None,
    Selection,
    All
}

#[derive(Debug)]
pub enum DefaultLineEnding {
    System,
    Windows,
    Unix
}

#[derive(Debug)]
pub enum GPUWindowBuffer {
    Auto,
    Enabled,
    Disabled
}

#[derive(Debug)]
pub enum OverlayScrollBars {
    System,
    Enabled,
    Disabled
}

#[derive(Debug)]
pub enum EnableTelemetry {
	Auto,
    Enabled,
    Disabled
}

#[derive(Debug)]
pub enum SettingsType {
    Int,
    UInt,
    Float,
    Boolean,
    String,
    Array
}

#[derive(Debug)]
pub enum ParsePreferencesError {
    PreferencesAreNotObject,
    PreferencesAreNotDefined(String),
    IncorrectTypeOfSettings(SettingsType, String),
}

#[derive(Debug)]
pub enum ParseTriggerError {
    IncorrectTypeOfTrigger,
    TriggerIsNotArrayOfObjects,
}


#[derive(Debug)]
pub struct Trigger {
   pub selector: ScopeSelector,
   pub characters: String
}

/*
impl ParseSettings for Preferences {
    type Error = ParsePreferencesError;
    fn parse_settings(settings: Settings) -> Result<Preferences, Self::Error> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(ParsePreferencesError::PreferencesAreNotObject),
        };
                            Vec
        Settings::Array(SettingsArray<Settings::Object>)


impl ParseSettings for Trigger {
    type Error = ParseTriggerError;
    fn parse_settings(settings: Settings) -> Results<Trigger, Self::Error> {
        let mut array = match settings {
            Settings::Array(SettingsArray)
        };
    }
}
*/

#[derive(Debug)]
pub struct Preferences {
	/// Sets the colors used within the text area
	pub color_scheme: PathBuf,

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
	pub word_separators: Regex,

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
	pub tab_size: u32,

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
    pub dictionary: PathBuf,

    /// Sets which scopes are checked for spelling errors
    pub spelling_selector: ScopeSelector,

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

    /// Set to false to not highlight curly brackets. This only takes effect if
    /// match_brackets is true
    pub match_brackets_braces: bool,

    /// Set to false to not highlight angle brackets. This only takes effect if
    /// match_brackets is true
    pub match_brackets_angle: bool,

    /// Enable visualization of the matching tag in HTML and XML
    pub match_tags: bool,

    /// Highlights other occurrences of the currently selected text
    pub match_selection: bool,

    /// Additional spacing at the top of each line, in pixels
    pub line_padding_top: i32,

    /// Additional spacing at the bottom of each line, in pixels
    pub line_padding_bottom: i32,

    /// Set to false to disable scrolling past the end of the buffer.
    /// On OS X, this value is overridden in the platform specific settings, so
    /// you'll need to place this line in your user settings to override it.
    pub scroll_past_end: bool,

    /// This controls what happens when pressing up or down when on the first
    /// or last line.
    /// On OS X, this value is overridden in the platform specific settings, so
    /// you'll need to place this line in your user settings to override it.
    pub move_to_limit_on_up_down: bool,

    /// Set to "none" to turn off drawing white space, "selection" to draw only the
    /// white space within the selection, and "all" to draw all white space
    pub draw_white_space: DrawWhiteSpace,

    /// Set to false to turn off the indentation guides.
    /// The color and width of the indent guides may be customized by editing
    /// the corresponding .tmTheme file, and specifying the colors "guide",
    /// "activeGuide" and "stackGuide"
    pub draw_indent_guides: bool,

    /// Controls how the indent guides are drawn, valid options are
    /// "draw_normal" and "draw_active". draw_active will draw the indent
    /// guides containing the caret in a different color.
    pub indent_guide_options: Vec<String>,

    /// Set to true to removing trailing white space on save
    pub trim_trailing_white_space_on_save: bool,

    /// Set to true to ensure the last line of the file ends in a newline
    /// character when saving
    pub ensure_newline_at_eof_on_save: bool,

    /// Set to true to automatically save files when switching to a different file
    /// or application
    pub save_on_focus_lost: bool,

    /// Save via writing to an alternate file, and then renaming it over the
    /// original file.
    pub atomic_save: bool,

    /// The encoding to use when the encoding can't be determined automatically.
    /// ASCII, UTF-8 and UTF-16 encodings will be automatically detected.
    pub fallback_encoding: String,

    /// Encoding used when saving new files, and files opened with an undefined
    /// encoding (e.g., plain ascii files). If a file is opened with a specific
    /// encoding (either detected or given explicitly), this setting will be
    /// ignored, and the file will be saved with the encoding it was opened
    /// with.
    pub default_encoding: String,

    /// Files containing null bytes are opened as hexadecimal by default
    pub enable_hexadecimal_encoding: bool,

    /// Determines what character(s) are used to terminate each line in new files.
    /// Valid values are 'system' (whatever the OS uses), 'windows' (CRLF) and
    /// 'unix' (LF only).
    pub default_line_ending: DefaultLineEnding,

    /// When enabled, pressing tab will insert the best matching completion.
    /// When disabled, tab will only trigger snippets or insert a tab.
    /// Shift+tab can be used to insert an explicit tab when tab_completion is
    /// enabled.
    pub tab_completion: bool,

    /// Enable auto complete to be triggered automatically when typing.
    pub auto_complete: bool,

    /// The maximum file size where auto complete will be automatically triggered.
    pub auto_complete_size_limit: i32,

    /// The delay, in ms, before the auto complete window is shown after typing
    pub auto_complete_delay: i32,

    /// Controls what scopes auto complete will be triggered in
    pub auto_complete_selector: ScopeSelector,

    /// Additional situations to trigger auto complete
    pub auto_complete_triggers: Vec<Trigger>,

    /// By default, auto complete will commit the current completion on enter.
    /// This setting can be used to make it complete on tab instead.
    /// Completing on tab is generally a superior option, as it removes
    /// ambiguity between committing the completion and inserting a newline.
    pub auto_complete_commit_on_tab: bool,

    /// Controls if auto complete is shown when snippet fields are active.
    /// Only relevant if auto_complete_commit_on_tab is true.
    pub auto_complete_with_fields: bool,

    /// Controls what happens when pressing the up key while the first item in
    /// the auto complete window is selected: if false, the window is hidden,
    /// otherwise the last item in the window is selected. Likewise for the
    /// down key when the last item is selected.
    pub auto_complete_cycle: bool,

    /// Automatically close HTML and XML tags when </ is entered.
    pub auto_close_tags: bool,

   	/// By default, shift+tab will only unindent if the selection spans
    /// multiple lines. When pressing shift+tab at other times, it'll insert a
    /// tab character - this allows tabs to be inserted when tab_completion is
    /// enabled. Set this to true to make shift+tab always unindent, instead of
    /// inserting tabs.
    pub shift_tab_unindent: bool,

    /// If true, the copy and cut commands will operate on the current line
    /// when the selection is empty, rather than doing nothing.
    pub copy_with_empty_selection: bool,

    /// If true, the selected text will be copied into the find panel when it's
    /// shown.
    /// On OS X, this value is overridden in the platform specific settings, so
    /// you'll need to place this line in your user settings to override it.
    pub find_selected_text: bool,

    /// When auto_find_in_selection is enabled, the "Find in Selection" flag
    /// will be enabled automatically when multiple lines of text are selected
    pub auto_find_in_selection: bool,

    /// When drag_text is enabled, clicking on selected text will begin a
    /// drag-drop operation. This is not currently implemented under Linux.
    pub drag_text: bool,

    ///
    /// User Interface Settings
    ///

    /// The theme controls the look of Sublime Text's UI (buttons, tabs, scroll bars, etc)
    pub theme: PathBuf,

    /// Set to 0 to disable smooth scrolling. Set to a value between 0 and 1 to
    /// scroll slower, or set to larger than 1 to scroll faster
    pub scroll_speed: f32,

    /// Controls side bar animation when expanding or collapsing folders
    pub tree_animation_enabled: bool,

    /// Controls animation throughout the application
    pub animation_enabled: bool,

    /// Makes tabs with modified files more visible
    pub highlight_modified_tabs: bool,

    pub show_tab_close_buttons: bool,

    /// Show folders in the side bar in bold
    pub bold_folder_labels: bool,

    /// OS X only: Set to true to disable Lion style full screen support.
    /// Sublime Text must be restarted for this to take effect.
    pub use_simple_full_screen: bool,

    /// OS X only. Valid values are true, false, and "auto". Auto will enable
    /// the setting when running on a screen 2560 pixels or wider (i.e., a
    /// Retina display). When this setting is enabled, OpenGL is used to
    /// accelerate drawing. Sublime Text must be restarted for changes to take
    /// effect.
    pub gpu_window_buffer: GPUWindowBuffer,

    /// Valid values are "system", "enabled" and "disabled"
    pub overlay_scroll_bars: OverlayScrollBars,

    /// Allows tabs to scroll left and right, instead of simply shrinking
    pub enable_tab_scrolling: bool,

    /// Display file encoding in the status bar
    pub show_encoding: bool,

    /// Display line endings in the status bar
    pub show_line_endings: bool,

    ///
    /// Application Behavior Settings
    ///

    /// Exiting the application with hot_exit enabled will cause it to close
    /// immediately without prompting. Unsaved modifications and open files will
    /// be preserved and restored when next starting.
    ///
    /// Closing a window with an associated project will also close the window
    /// without prompting, preserving unsaved changes in the workspace file
    /// alongside the project.
    pub hot_exit: bool,

    /// remember_full_screen will allow Sublime Text to start in full screen
    /// mode if it was exited in full screen mode. When set to false, Sublime
    /// Text will never start in full screen mode.
    pub remember_full_screen: bool,

    /// Always prompt before reloading a file, even if the file hasn't been
    /// modified. The default behavior is to automatically reload a file if it
    /// hasn't been edited. If a file has unsaved changes, a prompt will always
    /// be shown.
    pub always_prompt_for_file_reload: bool,

    /// OS X only: When files are opened from finder, or by dragging onto the
    /// dock icon, this controls if a new window is created or not.
    pub open_files_in_new_window: bool,

    /// OS X only: This controls if an empty window is created at startup or not.
    pub create_window_at_startup: bool,

    /// Set to true to close windows as soon as the last file is closed, unless
    /// there's a folder open within the window.
    /// On OS X, this value is overridden in the platform specific settings, so
    /// you'll need to place this line in your user settings to override it.
    pub close_windows_when_empty: bool,

    /// Show the full path to files in the title bar.
    /// On OS X, this value is overridden in the platform specific settings, so
    /// you'll need to place this line in your user settings to override it.
    pub show_full_path: bool,

    /// Shows the Build Results panel when building. If set to false, the Build
    /// Results can be shown via the Tools/Build Results menu.
    pub show_panel_on_build: bool,

    /// Preview file contents when clicking on a file in the side bar. Double
    /// clicking or editing the preview will open the file and assign it a tab.
    pub preview_on_click: bool,

    /// folder_exclude_patterns and file_exclude_patterns control which files
    /// are listed in folders on the side bar. These can also be set on a per-
    /// project basis.
    pub folder_exclude_patterns: ExcludePatterns,
    pub file_exclude_patterns: ExcludePatterns,
    /// These files will still show up in the side bar, but won't be included in
    /// Goto Anything or Find in Files
    pub binary_file_patterns: ExcludePatterns,

    /// File indexing parses all files in the side bar, and builds an index of
    /// their symbols. This is required for Goto Definition to work.
    pub index_files: bool,

    /// Set the number threads to use for indexing. A value of 0 will make
    /// Sublime Text guess based on the number of cores. Use the index_files
    /// setting to disable all workers.
    pub index_workers: i32,

    /// index_exclude_patterns indicate which files won't be indexed.
    pub index_exclude_patterns: Vec<String>,

    /// When enabled, anonymised usage data is sent back, assisting Sublime HQ
    /// in making informed decisions about improving Sublime Text. File names
    /// and file contents are never included, but data such as computer
    /// specifications, startup time, installed packages, and edited file types
    /// are. When disabled, telemetry is neither recorded or sent.
    /// A setting of auto will enable telemetry in dev builds, and disable
    /// telemetry in regular builds.
    pub enable_telemetry: EnableTelemetry,

    /// List any packages to ignore here. When removing entries from this list,
    /// a restart may be required if the package contains plugins.
    pub ignored_packages: Vec<String>
}

impl ParseSettings for Preferences {
    type Error = ParsePreferencesError;
    fn parse_settings(settings: Settings) -> Result<Preferences, Self::Error> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(ParsePreferencesError::PreferencesAreNotObject),
        };

        let color_scheme = match obj.remove("color_scheme") {
            Some(Settings::String(s)) => PathBuf::from(s),
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("color_scheme".to_string())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "color_scheme".to_string()))
        };

        let font_face = match obj.remove("font_face") {
            Some(Settings::String(s)) => s,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("font_face".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "font_face".to_owned()))
        };

        let font_size = match obj.remove("font_size") {
            Some(Settings::I64(i)) => i as i32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("font_size".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Int, "font_size".to_owned()))
        };

        let font_options = match obj.remove("font_options") {
            Some(Settings::Array(arr)) => arr,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("font_options".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Array, "font_options".to_owned()))
        };

        let word_separators = match obj.remove("word_separators") {
            Some(Settings::String(s)) => Regex::new(&*s),
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("word_separators".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "word_separators".to_owned()))
        };

        let line_numbers = match obj.remove("line_numbers") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("line_numbers".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "line_numbers".to_owned()))
        };

        let gutter = match obj.remove("gutter") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("gutter".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "gutter".to_owned()))
        };

        let margin = match obj.remove("margin") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("margin".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "margin".to_owned()))
        };

        let fold_buttons = match obj.remove("fold_buttons") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("fold_buttons".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "fold_buttons".to_owned()))
        };

        let fade_fold_buttons = match obj.remove("fade_fold_buttons") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("fade_fold_buttons".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "fade_fold_buttons".to_owned()))
        };

        let rulers = match obj.remove("rulers") {
            Some(Settings::Array(arr)) => arr,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("rulers".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Array, "rulers".to_owned()))
        };

        let spell_check = match obj.remove("spell_check") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("spell_check".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "spell_check".to_owned()))
        };

        let tab_size = match obj.remove("tab_size") {
            Some(Settings::U64(u)) => u as u32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("tab_size".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::UInt, "tab_size".to_owned()))
        };

        let translate_tabs_to_spaces = match obj.remove("translate_tabs_to_spaces") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("translate_tabs_to_spaces".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "translate_tabs_to_spaces".to_owned()))
        };

        let use_tab_stops = match obj.remove("use_tab_stops") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("use_tab_stops".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "use_tab_stops".to_owned()))
        };

        let detect_indentation = match obj.remove("detect_indentation") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("detect_indentation".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "detect_indentation".to_owned()))
        };

        let auto_indent = match obj.remove("auto_indent") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_indent".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "auto_indent".to_owned()))
        };

        let smart_indent = match obj.remove("smart_indent") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("smart_indent".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "smart_indent".to_owned()))
        };

        let indent_to_bracket = match obj.remove("indent_to_bracket") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("indent_to_bracket".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "indent_to_bracket".to_owned()))
        };

        let trim_automatic_white_space = match obj.remove("trim_automatic_white_space") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("trim_automatic_white_space".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "trim_automatic_white_space".to_owned()))
        };

        let word_wrap: WordWrap = match obj.remove("word_wrap") {
            Some(Settings::String(s)) => match &*s {
                "auto" => WordWrap::Auto,
                "true" => WordWrap::Wrap,
                "false" => WordWrap::NoWrap,
                _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "word_wrap".to_owned()))
            },
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("word_wrap".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "word_wrap".to_owned()))
        };

        let wrap_width = match obj.remove("wrap_width") {
            Some(Settings::I64(num)) => num as i32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("wrap_width".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Int, "wrap_width".to_owned()))
        };

        let indent_subsequent_lines = match obj.remove("indent_subsequent_lines") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("indent_subsequent_lines".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "indent_subsequent_lines".to_owned()))
        };

        let draw_centered = match obj.remove("draw_centered") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("draw_centered".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "draw_centered".to_owned()))
        };

        let auto_match_enabled = match obj.remove("auto_match_enabled") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_match_enabled".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "auto_match_enabled".to_owned()))
        };

        let dictionary = match obj.remove("dictionary") {
            Some(Settings::String(s)) => PathBuf::from(s),
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("dictionary".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "dictionary".to_owned()))
        };

        let spelling_selector = match obj.remove("spelling_selector") {
            Some(Settings::String(s)) => match ScopeSelector::from_str(&*s){
                Ok(x) => x,
                Err(e) => return Err(ParsePreferencesError::PreferencesAreNotDefined("spelling_selector".to_owned())),
            },
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("spelling_selector".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "spelling_selector".to_owned()))
        };

        let draw_minimap_border = match obj.remove("draw_minimap_border") {
            Some(Settings::String(s)) => PathBuf::from(s),
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("draw_minimap_border".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "draw_minimap_border".to_owned()))
        };

        let always_show_minimap_viewport = match obj.remove("always_show_minimap_viewport") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("always_show_minimap_viewport".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "always_show_minimap_viewport".to_owned()))
        };

        let highlight_line = match obj.remove("highlight_line") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("highlight_line".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "highlight_line".to_owned()))
        };

        let caret_style: CaretStyle = match obj.remove("caret_style") {
            Some(Settings::String(s)) => match &*s {
                "smooth" => CaretStyle::Smooth,
                "phase" => CaretStyle::Phase,
                "blink" => CaretStyle::Blink,
                "solid" => CaretStyle::Solid,
                _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "caret_style".to_owned()))
            },
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("caret_style".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "caret_style".to_owned()))
        };

        let caret_extra_top = match obj.remove("caret_extra_top") {
            Some(Settings::I64(num)) => num as i32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("caret_extra_top".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Int, "caret_extra_top".to_owned()))
        };

        let caret_extra_bottom = match obj.remove("caret_extra_bottom") {
            Some(Settings::I64(num)) => num as i32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("caret_extra_bottom".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Int, "caret_extra_bottom".to_owned()))
        };

        let caret_extra_width = match obj.remove("caret_extra_width") {
            Some(Settings::I64(num)) => num as i32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("caret_extra_width".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Int, "caret_extra_width".to_owned()))
        };

        let match_brackets = match obj.remove("match_brackets") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("match_brackets".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "match_brackets".to_owned()))
        };

        let match_brackets_content = match obj.remove("match_brackets_content") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("match_brackets_content".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "match_brackets_content".to_owned()))
        };

        let match_brackets_square = match obj.remove("match_brackets_square") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("match_brackets_square".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "match_brackets_square".to_owned()))
        };

        let match_brackets_braces = match obj.remove("match_brackets_braces") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("match_brackets_braces".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "match_brackets_braces".to_owned()))
        };

        let match_brackets_angle = match obj.remove("match_brackets_angle") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("match_brackets_angle".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "match_brackets_angle".to_owned()))
        };

        let match_tags = match obj.remove("match_tags") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("match_tags".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "match_tags".to_owned()))
        };

        let match_selection = match obj.remove("match_selection") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("match_selection".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "match_selection".to_owned()))
        };

        let line_padding_top = match obj.remove("line_padding_top") {
            Some(Settings::I64(num)) => num as i32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("line_padding_top".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Int, "line_padding_top".to_owned()))
        };

        let line_padding_bottom = match obj.remove("line_padding_bottom") {
            Some(Settings::I64(num)) => num as i32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("line_padding_bottom".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Int, "line_padding_bottom".to_owned()))
        };

        let scroll_past_end = match obj.remove("scroll_past_end") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("scroll_past_end".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "scroll_past_end".to_owned()))
        };

        let move_to_limit_on_up_down = match obj.remove("move_to_limit_on_up_down") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("move_to_limit_on_up_down".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "move_to_limit_on_up_down".to_owned()))
        };

        let draw_white_space: DrawWhiteSpace = match obj.remove("draw_white_space") {
            Some(Settings::String(s)) => match &*s {
                "none" => DrawWhiteSpace::None,
                "all" => DrawWhiteSpace::All,
                "selection" => DrawWhiteSpace::Selection,
                _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "draw_white_space".to_owned()))
            },
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("draw_white_space".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "draw_white_space".to_owned()))
        };

        let draw_indent_guides = match obj.remove("draw_indent_guides") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("draw_indent_guides".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "draw_indent_guides".to_owned()))
        };

        let indent_guide_options = match obj.remove("indent_guide_options") {
            Some(Settings::Array(arr)) => arr,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("indent_guide_options".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Array, "indent_guide_options".to_owned()))
        };

        let trim_trailing_white_space_on_save = match obj.remove("trim_trailing_white_space_on_save") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("trim_trailing_white_space_on_save".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "trim_trailing_white_space_on_save".to_owned()))
        };


        let ensure_newline_at_eof_on_save = match obj.remove("ensure_newline_at_eof_on_save") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("ensure_newline_at_eof_on_save".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "ensure_newline_at_eof_on_save".to_owned()))
        };


        let save_on_focus_lost = match obj.remove("save_on_focus_lost") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("save_on_focus_lost".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "save_on_focus_lost".to_owned()))
        };


        let atomic_save = match obj.remove("atomic_save") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("atomic_save".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "atomic_save".to_owned()))
        };

        let fallback_encoding = match obj.remove("fallback_encoding") {
            Some(Settings::String(s)) => s,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("fallback_encoding".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "fallback_encoding".to_owned()))
        };

        let default_encoding = match obj.remove("default_encoding") {
            Some(Settings::String(s)) => s,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("default_encoding".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "default_encoding".to_owned()))
        };

        let enable_hexadecimal_encoding = match obj.remove("enable_hexadecimal_encoding") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("enable_hexadecimal_encoding".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "enable_hexadecimal_encoding".to_owned()))
        };

        let default_line_ending: DefaultLineEnding = match obj.remove("default_line_ending") {
            Some(Settings::String(s)) => match &*s {
                "windows" => DefaultLineEnding::Windows,
                "unix" => DefaultLineEnding::Unix,
                "system" => DefaultLineEnding::System,
                _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "default_line_ending".to_owned()))
            },
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("default_line_ending".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "default_line_ending".to_owned()))
        };

        let tab_completion = match obj.remove("tab_completion") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("tab_completion".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "tab_completion".to_owned()))
        };

        let auto_complete = match obj.remove("auto_complete") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_complete".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "auto_complete".to_owned()))
        };

        let auto_complete_size_limit = match obj.remove("auto_complete_size_limit") {
            Some(Settings::I64(num)) => num as i32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_complete_size_limit".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Int, "auto_complete_size_limit".to_owned()))
        };

        let auto_complete_delay = match obj.remove("auto_complete_delay") {
            Some(Settings::I64(num)) => num as i32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_complete_delay".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Int, "auto_complete_delay".to_owned()))
        };

        let auto_complete_selector = match obj.remove("auto_complete_selector") {
            Some(Settings::String(s)) => match ScopeSelector::from_str(&*s){
                Ok(x) => x,
                Err(_) => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_complete_selector".to_owned())),
            },
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_complete_selector".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "auto_complete_selector".to_owned())),
        };

        let auto_complete_commit_on_tab = match obj.remove("auto_complete_commit_on_tab") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_complete_commit_on_tab".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "auto_complete_commit_on_tab".to_owned()))
        };

        let auto_complete_with_fields = match obj.remove("auto_complete_with_fields") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_complete_with_fields".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "auto_complete_with_fields".to_owned()))
        };

        let auto_complete_cycle = match obj.remove("auto_complete_cycle") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_complete_cycle".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "auto_complete_cycle".to_owned()))
        };

        let auto_close_tags = match obj.remove("auto_close_tags") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_close_tags".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "auto_close_tags".to_owned()))
        };

        let shift_tab_unindent = match obj.remove("shift_tab_unindent") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("shift_tab_unindent".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "shift_tab_unindent".to_owned()))
        };

        let copy_with_empty_selection = match obj.remove("copy_with_empty_selection") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("copy_with_empty_selection".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "copy_with_empty_selection".to_owned()))
        };

        let find_selected_text = match obj.remove("find_selected_text") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("find_selected_text".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "find_selected_text".to_owned()))
        };

        let auto_find_in_selection = match obj.remove("auto_find_in_selection") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("auto_find_in_selection".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "auto_find_in_selection".to_owned()))
        };

        let drag_text = match obj.remove("drag_text") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("drag_text".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "drag_text".to_owned()))
        };

        let theme = match obj.remove("theme") {
            Some(Settings::String(s)) => PathBuf::from(s),
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("theme".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "theme".to_owned()))
        };

        let scroll_speed = match obj.remove("scroll_speed") {
            Some(Settings::F64(f)) => f as f32,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("scroll_speed".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Float, "scroll_speed".to_owned()))
        };

        let tree_animation_enabled = match obj.remove("tree_animation_enabled") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("tree_animation_enabled".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "tree_animation_enabled".to_owned()))
        };

        let animation_enabled = match obj.remove("animation_enabled") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("animation_enabled".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "animation_enabled".to_owned()))
        };

        let highlight_modified_tabs = match obj.remove("highlight_modified_tabs") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("highlight_modified_tabs".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "highlight_modified_tabs".to_owned()))
        };

        let show_tab_close_buttons = match obj.remove("show_tab_close_buttons") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("show_tab_close_buttons".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "show_tab_close_buttons".to_owned()))
        };

        let bold_folder_labels = match obj.remove("bold_folder_labels") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("bold_folder_labels".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "bold_folder_labels".to_owned()))
        };

        let use_simple_full_screen = match obj.remove("use_simple_full_screen") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("use_simple_full_screen".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "use_simple_full_screen".to_owned()))
        };

        let gpu_window_buffer: GPUWindowBuffer = match obj.remove("gpu_window_buffer") {
            Some(Settings::String(s)) => match &*s {
                "true" => GPUWindowBuffer::Enabled,
                "false" => GPUWindowBuffer::Disabled,
                "auto" => GPUWindowBuffer::Auto,
                _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "gpu_window_buffer".to_owned()))
            },
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("gpu_window_buffer".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "gpu_window_buffer".to_owned()))
        };

        let overlay_scroll_bars: OverlayScrollBars = match obj.remove("overlay_scroll_bars") {
            Some(Settings::String(s)) => match &*s {
                "system" => OverlayScrollBars::System,
                "enabled" => OverlayScrollBars::Enabled,
                "disabled" => OverlayScrollBars::Disabled,
                _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "overlay_scroll_bars".to_owned()))
            },
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("overlay_scroll_bars".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::String, "overlay_scroll_bars".to_owned()))
        };

        let enable_tab_scrolling = match obj.remove("enable_tab_scrolling") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("enable_tab_scrolling".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "enable_tab_scrolling".to_owned()))
        };

        let show_encoding = match obj.remove("show_encoding") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("show_encoding".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "show_encoding".to_owned()))
        };

        let show_line_endings = match obj.remove("show_line_endings") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("show_line_endings".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "show_line_endings".to_owned()))
        };

        let hot_exit = match obj.remove("hot_exit") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("hot_exit".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "hot_exit".to_owned()))
        };

        let remember_full_screen = match obj.remove("remember_full_screen") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("remember_full_screen".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "remember_full_screen".to_owned()))
        };

        let always_prompt_for_file_reload = match obj.remove("always_prompt_for_file_reload") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("always_prompt_for_file_reload".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "always_prompt_for_file_reload".to_owned()))
        };

        let open_files_in_new_window = match obj.remove("open_files_in_new_window") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("open_files_in_new_window".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "open_files_in_new_window".to_owned()))
        };

        let create_window_at_startup = match obj.remove("create_window_at_startup") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("create_window_at_startup".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "create_window_at_startup".to_owned()))
        };

        let close_windows_when_empty = match obj.remove("close_windows_when_empty") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("close_windows_when_empty".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "close_windows_when_empty".to_owned()))
        };

        let show_full_path = match obj.remove("show_full_path") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("show_full_path".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "show_full_path".to_owned()))
        };

        let show_panel_on_build = match obj.remove("show_panel_on_build") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("show_panel_on_build".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "show_panel_on_build".to_owned()))
        };

        let preview_on_click = match obj.remove("preview_on_click") {
            Some(Settings::Boolean(b)) => b,
            None => return Err(ParsePreferencesError::PreferencesAreNotDefined("preview_on_click".to_owned())),
            _ => return Err(ParsePreferencesError::IncorrectTypeOfSettings(SettingsType::Boolean, "preview_on_click".to_owned()))
        };

    }
}
