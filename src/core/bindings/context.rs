use core::regex::{Regex, RegexError};
use core::settings::{Settings, ParseSettings};
use core::Core;

use self::ParseContextError::*;

pub type Context = Vec<ContextRule>;

#[derive(Debug)]
pub enum ContextRule {
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
    /// Returns the number of selections.
    NumSelectionsEqual(u64),
    /// Returns the number of selections.
    NumSelectionsNotEqual(u64),
    /// Returns the name of the current scope.
    Selector(Operator<String>, bool),
    /// Restricts the test to the selected text.
    Text(Operator<String>, bool),
    /// Restricts the test to the text following the caret.
    FollowingText(Operator<String>, bool),
    /// Restricts the test to the text preceding the caret.
    PrecedingText(Operator<String>, bool),
    /// Returns the value of the setting.
    Setting(String, Operator<Settings>),
}

#[derive(Debug)]
pub enum Operator<T> {
    /// Test for equality.
    Equal(T),
    NotEqual(T),
    /// Match against a regular expression.
    RegexContains(Regex),
    NotRegexContains(Regex),
}

#[derive(Debug)]
pub enum ParseContextError {
    ContextIsNotArray,
    ContextRuleIsNotObject,
    IncorrectKey,
    IncorrectOperatorOrOperand,
    IncorrectMatchAllValue,
    RegexParse(RegexError),
}

impl ParseSettings for ContextRule {
    type Error = ParseContextError;
    fn parse_settings(settings: Settings) -> Result<ContextRule, Self::Error> {
        let mut obj = match settings {
            Settings::Object(obj) => obj,
            _ => return Err(ContextRuleIsNotObject),
        };

        let key_string = match obj.remove("key") {
            Some(Settings::String(key)) => key,
            _ => return Err(IncorrectKey),
        };

        let (operator_string, operand) = match (obj.remove("operator"), obj.remove("operand")) {
            (Some(Settings::String(operator)), Some(operand)) => (operator, operand),
            (None, None) => ("equal".to_owned(), Settings::Boolean(true)),
            _ => return Err(IncorrectOperatorOrOperand),
        };

        let (key, operator) = if &key_string[..] == "selection_empty" {
            // Convert rule like {"key": "selection_empty", "operator": "equal", "operand":
            // true}
            //    into equialent {"key": "text", "operator": "equal", "operand": ""}
            ("text",
             match (&operator_string[..], operand) {
                ("not_equal", Settings::Boolean(false)) | ("equal", Settings::Boolean(true)) =>
                    ("equal", Settings::String("".to_owned())),
                ("not_equal", Settings::Boolean(true)) | ("equal", Settings::Boolean(false)) =>
                    ("not_equal", Settings::String("".to_owned())),
                _ => return Err(IncorrectOperatorOrOperand),
            })
        } else {
            // Convert rule like {"key": "foo", "operator": "regex_match", "operand": "bar"}
            // into equialent {"key": "foo", "operator": "regex_contains", "operand":
            // "^bar$"}
            (&key_string[..],
             match (&operator_string[..], operand) {
                ("regex_match", Settings::String(s)) =>
                    ("regex_contains",
                     Settings::String("^".to_owned() + &s + "$")),
                ("not_regex_match", Settings::String(s)) =>
                    ("not_regex_contains",
                     Settings::String("^".to_owned() + &s + "$")),
                (operator, operand) => (operator, operand),
            })
        };

        let context_rule = if key.starts_with("setting.") {
            let key = key[8..].to_owned();
            let operator = match operator {
                ("equal", operand) => Operator::Equal(operand),
                ("not_equal", operand) => Operator::NotEqual(operand),
                (op, Settings::String(pattern)) => {
                    let operator_builder: fn(Regex) -> Operator<Settings> = match op {
                        "regex_contains" => Operator::RegexContains,
                        "not_regex_contains" => Operator::NotRegexContains,
                        _ => return Err(IncorrectOperatorOrOperand),
                    };
                    let regex = match Regex::new(&pattern) {
                        Ok(regex) => regex,
                        Err(err) => return Err(RegexParse(err)),
                    };
                    operator_builder(regex)
                }
                _ => return Err(IncorrectOperatorOrOperand),
            };
            ContextRule::Setting(key, operator)
        } else if key == "num_selections" {
            match operator {
                ("equal", Settings::U64(num)) => ContextRule::NumSelectionsEqual(num),
                ("not_equal", Settings::U64(num)) => ContextRule::NumSelectionsNotEqual(num),
                _ => return Err(IncorrectOperatorOrOperand),
            }
        } else {
            let bool_rule_builder: Option<fn(bool) -> ContextRule> = match key {
                "auto_complete_visible" => Some(ContextRule::AutoCompleteVisibleEqual),
                "has_next_field" => Some(ContextRule::HasNextFieldEqual),
                "has_prev_field" => Some(ContextRule::HasPrevFieldEqual),
                "overlay_visible" => Some(ContextRule::OverlayVisibleEqual),
                "panel_visible" => Some(ContextRule::PanelVisibleEqual),
                "panel_has_focus" => Some(ContextRule::PanelHasFocusEqual),
                "panel" => Some(ContextRule::PanelEqual),
                _ => None,
            };
            match bool_rule_builder {
                Some(rule_builder) => match operator {
                    ("not_equal", Settings::Boolean(false)) |
                    ("equal", Settings::Boolean(true)) => rule_builder(true),
                    ("not_equal", Settings::Boolean(true)) |
                    ("equal", Settings::Boolean(false)) => rule_builder(false),
                    _ => return Err(IncorrectOperatorOrOperand),
                },
                None => {
                    let rule_builder: fn(Operator<String>, bool) -> ContextRule = match key {
                        "selector" => ContextRule::Selector,
                        "text" => ContextRule::Text,
                        "following_text" => ContextRule::FollowingText,
                        "preceding_text" => ContextRule::PrecedingText,
                        _ => return Err(IncorrectKey),
                    };
                    let operator = match operator {
                        ("equal", Settings::String(operand)) => Operator::Equal(operand),
                        ("not_equal", Settings::String(operand)) => Operator::NotEqual(operand),
                        (op, Settings::String(pattern)) => {
                            let operator_builder: fn(Regex) -> Operator<String> = match op {
                                "regex_contains" => Operator::RegexContains,
                                "not_regex_contains" => Operator::NotRegexContains,
                                _ => return Err(IncorrectOperatorOrOperand),
                            };
                            let regex = match Regex::new(&pattern) {
                                Ok(regex) => regex,
                                Err(err) => return Err(RegexParse(err)),
                            };
                            operator_builder(regex)
                        }
                        _ => return Err(IncorrectOperatorOrOperand),
                    };
                    let match_all = match obj.remove("match_all") {
                        Some(Settings::Boolean(match_all)) => match_all,
                        None => false,
                        _ => return Err(IncorrectMatchAllValue),
                    };
                    rule_builder(operator, match_all)
                }
            }
        };
        // TODO: check that obj is empty
        Ok(context_rule)
    }
}

impl ParseSettings for Context {
    type Error = ParseContextError;
    fn parse_settings(settings: Settings) -> Result<Context, Self::Error> {
        let arr = match settings {
            Settings::Array(arr) => arr,
            _ => return Err(ContextIsNotArray),
        };
        let mut context = Context::new();
        for settings in arr {
            context.push(try!(ContextRule::parse_settings(settings)));
        }
        Ok(context)
    }
}

impl Operator<String> {
    fn evaluate(&self, v: &str) -> bool {
        match *self {
            Operator::Equal(ref operand) => v == operand,
            Operator::NotEqual(ref operand) => v != operand,
            Operator::RegexContains(ref operand) => operand.is_match(v),
            Operator::NotRegexContains(ref operand) => !operand.is_match(v),
        }
    }
}

impl Operator<Settings> {
    fn evaluate(&self, v: &Settings) -> bool {
        match *self {
            Operator::Equal(ref operand) => v == operand,
            Operator::NotEqual(ref operand) => v != operand,
            Operator::RegexContains(ref operand) => match *v {
                Settings::String(ref v) => operand.is_match(v),
                _ => false,
            },
            Operator::NotRegexContains(ref operand) => match *v {
                Settings::String(ref v) => !operand.is_match(v),
                _ => false,
            },
        }
    }
}

pub trait Evaluate {
    fn evaluate(&self, core: &Core) -> bool;
}

impl Evaluate for ContextRule {
    fn evaluate(&self, _: &Core) -> bool {
        // TODO: implement this
        false
    }
}

impl Evaluate for Context {
    fn evaluate(&self, core: &Core) -> bool {
        self.iter().all(|rule| rule.evaluate(core))
    }
}
