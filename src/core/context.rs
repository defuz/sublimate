use std::convert::From;
use rustc_serialize::json::Json;
use ParseContextRuleError::*;

enum Operator<T> {
    /// Test for equality.
    Equal(T), NotEqual(T),
    /// Match against a regular expression (full match).
    RegexMatch(String), NotRegexMatch(String),
    /// Match against a regular expression (partial match).
    RegexContains(String), NotRegexContains(String)
}

enum ContextRule {
    /// Returns `true` if the autocomplete list is visible.
    AutoCompleteVisibleEqual(bool),
    /// Returns `true` if a next snippet field is available.
    HasNextFieldEqual(bool),
    /// Returns `true` if a previous snippet field is available.
    HasPrevFieldEqual(bool),
    /// Returns `true` if any overlay is visible.
    OverlayVisibleEqual(bool),
    /// Returns `true` if any panel is visible.
    PanelVisibleEqual(bool),
    /// Returns `true` if a panel has input focus.
    PanelHasFocusEqual(bool),
    /// Returns `true` if the panel given as operand is visible.
    PanelEqual(bool),
    /// Returns `true` if the selection is an empty region.
    SelectionEmptyEqual(bool, /* match_all = */ bool),
    /// Returns the number of selections.
    NumSelectionsEqual(uint),
    /// Returns the number of selections.
    NumSelectionsNotEqual(uint),
    /// Returns the name of the current scope.
    Selector(Operator<String>),
    /// Restricts the test to the text following the caret.
    FollowingText(Operator<String>),
    /// Restricts the test to the text preceding the caret.
    PrecedingText(Operator<String>),
    /// Restricts the test to the selected text.
    Text(Operator<String>),
    /// Returns the value of the setting.
    Setting(String, Operator<Settings>)
}

enum ParseContextRuleError {
    NotObject,
    KeyNameNotFound,
    UnknownKey,
    IncorrectOperatorForBooleanKey
}

impl From<Json> for ContextRule {
    fn from_settings(s: Settings) -> Result<Key, Self::Err> {
        if let Settings::Object(obj) = s {
            match obj.remove("key") {
                Some(Settings::String(key)) => {
                    if key.starts_with("setting.") {
                        let key =
                    } else {
                        let rule_builder = match key {
                            "auto_complete_visible" => ContextRule::AutoCompleteVisibleEqual,
                            "has_next_field"        => ContextRule::HasNextFieldEqual,
                            "has_prev_field"        => ContextRule::HasPrevFieldEqual,
                            "overlay_visible"       => ContextRule::OverlayVisibleEqual,
                            "panel_visible"         => ContextRule::PanelVisibleEqual,
                            "panel_has_focus"       => ContextRule::PanelHasFocusEqual,
                            "panel"                 => ContextRule::PanelEqual,
                            _                       => return Err(UnknownKey)
                        };
                        match (obj.remove("operator"), obj.remove("operand")) {
                            (Some(Settings::String("not_equal")), Some(Settings::Bool(false))) |
                            (Some(Settings::String("equal")), Some(Settings::Bool(true)))      |
                            (None, None)
                                => rule_builder(true),
                            (Some(Settings::String("not_equal")), Some(Settings::Bool(true)))  |
                            (Some(Settings::String("equal")), Some(Settings::Bool(false)))
                                => rule_builder(false),
                            _   => return Err(IncorrectOperatorForBooleanKey)
                        }
                    }
                },
                _ => Err(KeyNameNotFound)
            }
        } else {
            Err(NotObject)
        }
    }
}

struct Context {
    rules: Box<[ContextRule]>
}
