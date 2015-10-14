enum Operand {
    /// Returns `true` if the autocomplete list is visible.
    AutoCompleteVisible,
    /// Returns `true` if a next snippet field is available.
    HasNextField,
    /// Returns `true` if a previous snippet field is available.
    HasPrevField,
    /// Returns `true` if any overlay is visible.
    OverlayVisible,
    /// Returns `true` if any panel is visible.
    PanelVisible,
    /// Returns `true` if the selection is an empty region.
    SelectionEmpty,
    /// Returns `true` if a panel has input focus.
    PanelHasFocus,
    /// Returns `true` if the panel given as operand is visible.
    Panel,
    /// Returns the number of selections.
    NumSelections,
    /// Returns the name of the current scope.
    Selector,
    /// Restricts the test to the text following the caret.
    FollowingText,
    /// Restricts the test to the text preceding the caret.
    PrecedingText,
    /// Restricts the test to the selected text.
    Text,
    /// Returns the value of the setting.
    Setting(String)
}

enum Operator {
    /// Test for equality.
    Equal, NotEqual,
    /// Match against a regular expression (full match).
    RegexMatch, NotRegexMatch,
    /// Match against a regular expression (partial match).
    RegexContains, NotRegexContains
}

struct ContextRule {

}

struct Context {
    rules: Box<[ContextRule]>
}
