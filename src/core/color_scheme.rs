use std::str::FromStr;
use std::io::Read;

use plist::{PlistEvent, ParserError};
use plist::xml::StreamingParser;

use core::settings::{FromPlist, ParsePlist};

use self::ParseColorSchemeError::*;

#[derive(Default)]
pub struct ColorScheme {
    settings: ColorSchemeSettings,
    scopes: Vec<ColorSchemeScope>
}

#[derive(Default)]
pub struct ColorSchemeSettings {
    /// Foreground color for the view.
    foreground: Color,
    /// Backgound color of the view.
    background: Color,
    /// Color of the caret.
    caret: Color,
    /// Color of the line the caret is in.
    /// Only used when the `higlight_line` setting is set to `true`.
    line_highlight: Color,

    /// Color of bracketed sections of text when the caret is in a bracketed section.
    /// Only applied when the `match_brackets` setting is set to `true`.
    bracket_contents_foreground: Color,
    /// Controls certain options when the caret is in a bracket section.
    /// Only applied when the `match_brackets` setting is set to `true`.
    bracket_contents_options: UnderlineOption,
    /// Foreground color of the brackets when the caret is next to a bracket.
    /// Only applied when the `match_brackets` setting is set to `true`.
    brackets_foreground: Color,
    /// Background color of the brackets when the caret is next to a bracket.
    /// Only applied when the `match_brackets` setting is set to `true`.
    brackets_background: Color,
    /// Controls certain options when the caret is next to a bracket.
    /// Only applied when the match_brackets setting is set to `true`.
    brackets_options: UnderlineOption,

    /// Color of tags when the caret is next to a tag.
    /// Only used when the `match_tags` setting is set to `true`.
    tags_foreground: Color,
    /// Controls certain options when the caret is next to a tag.
    /// Only applied when the match_tags setting is set to `true`.
    tagsOptions: UnderlineOption,

    /// Background color of regions matching the current search.
    find_highlight: Color,
    /// Background color of regions matching the current search.
    find_highlight_foreground: Color,

    /// Background color of the gutter.
    gutter: Color,
    /// Foreground color of the gutter.
    gutter_foreground: Color,

    /// Color of the selection regions.
    selection: Color,
    /// Background color of the selection regions.
    selection_background: Color,
    /// Color of the selection regions border.
    selection_border: Color,
    /// Color of inactive selections (inactive view).
    inactive_selection: Color,

    /// Color of the guides displayed to indicate nesting levels.
    guide: Color,
    /// Color of the guide lined up with the caret.
    /// Only applied if the `indent_guide_options` setting is set to `draw_active`.
    active_guide: Color,
    /// Color of the current guide’s parent guide level.
    /// Only used if the `indent_guide_options` setting is set to `draw_active`.
    stack_guide: Color,

    /// Background color for regions added via `sublime.add_regions()`
    /// with the `sublime.DRAW_OUTLINED` flag added.
    highlight: Color,
    /// Foreground color for regions added via `sublime.add_regions()`
    /// with the `sublime.DRAW_OUTLINED` flag added.
    highlight_foreground: Color
}

#[derive(Default)]
pub struct ColorSchemeScope {
    /// Target scope name.
    scope: String,
    /// Style of the font.
    font_style: FontStyle,
    /// Foreground color.
    foreground: Option<Color>,
    /// Background color.
    background: Option<Color>
}

#[derive(Default)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

bitflags! {
    flags FontStyle: u8 {
        const FONT_STYLE_BOLD = 1,
        const FONT_STYLE_UNDERLNINE = 2,
        const FONT_STYLE_ITALIC = 4,
    }
}

pub enum UnderlineOption {
    None,
    Underline,
    StippledUnderline,
    SquigglyUnderline
}

pub enum ParseColorSchemeError {
    IncorrectUnderlineOption,
    IncorrectFontStyle,
    IncorrectColor,
    IncorrectSyntax,
    UndefinedSettings,
    UndefinedScopeSettings,
    Parse(ParserError)
}

impl From<ParserError> for ParseColorSchemeError {
    fn from(error: ParserError) -> ParseColorSchemeError {
        Parse(error)
    }
}

impl Default for UnderlineOption {
    fn default() -> UnderlineOption {
        UnderlineOption::None
    }
}

impl Default for FontStyle {
    fn default() -> FontStyle {
        FontStyle::empty()
    }
}

impl FromStr for UnderlineOption {
    type Err = ParseColorSchemeError;

    fn from_str(s: &str) -> Result<UnderlineOption, Self::Err> {
        Ok(match s {
            "underline" => UnderlineOption::Underline,
            "stippled_underline" => UnderlineOption::StippledUnderline,
            "squiggly_underline" => UnderlineOption::SquigglyUnderline,
            _ => return Err(IncorrectUnderlineOption)
        })
    }
}

impl FromStr for FontStyle {
    type Err = ParseColorSchemeError;

    fn from_str(s: &str) -> Result<FontStyle, Self::Err> {
        let mut font_style = FontStyle::empty();
        for i in s.split(' ') {
            font_style.insert(match s {
                "bold" => FONT_STYLE_BOLD,
                "underline" => FONT_STYLE_UNDERLNINE,
                "italic" => FONT_STYLE_ITALIC,
                _ => return Err(IncorrectFontStyle),
            })
        }
        Ok(font_style)
    }
}

impl FromStr for Color {
    type Err = ParseColorSchemeError;

    fn from_str(s: &str) -> Result<Color, Self::Err> {
        let mut chars = s.chars();
        if chars.next() != Some('#') {
            return Err(IncorrectColor);
        }
        let mut d = Vec::new();
        for char in chars {
            d.push(try!(char.to_digit(16).ok_or(IncorrectColor)) as u8);
        }
        Ok(match d.len() {
            3 => Color { r: d[0], g: d[1], b: d[2], a: 255 },
            6 => Color { r: d[0]*16+d[1], g: d[2]*16+d[3], b: d[4]*16+d[5], a: 255 },
            8 => Color { r: d[0]*16+d[1], g: d[2]*16+d[3], b: d[4]*16+d[5], a: d[6]*16+d[7] },
            _ => return Err(IncorrectColor)
        })
    }
}

macro_rules! next {
    ($parser:expr) => {
        try!(try!($parser.next().ok_or(IncorrectSyntax)))
    }
}

macro_rules! eat {
    ($parser:expr, $pat:pat) => {
        match next!($parser) {
            $pat => (),
            _ => return Err(IncorrectSyntax)
        };
    }
}

macro_rules! parse_key {
    ($parser:expr => $expr:expr) => {
        match next!($parser) {
            PlistEvent::EndDictionary => $expr,
            PlistEvent::StringValue(key) => key,
            _ => return Err(IncorrectSyntax)
        }
    }
}

macro_rules! parse_str {
    ($parser:expr) => {
        match next!($parser) {
            PlistEvent::StringValue(value) => value,
            _ => return Err(IncorrectSyntax)
        }
    }
}

fn parse_dictionary<R: Read>(parser: &mut StreamingParser<R>)
    -> Result<Vec<(String, String)>, ParseColorSchemeError> {
    let mut r = Vec::new();
    loop {
        r.push((parse_key!(parser => break), parse_str!(parser)));
    };
    Ok(r)
}

impl<R: Read> ParsePlist<R> for ColorSchemeScope {
    type Error = ParseColorSchemeError;

    fn parse_plist(parser: &mut StreamingParser<R>) -> Result<ColorSchemeScope, Self::Error> {
        let mut scope_settings = ColorSchemeScope::default();
        for (key, value) in try!(parse_dictionary(parser)) {
            match &key[..] {
                "scope"      => scope_settings.scope      = value,
                "fontStyle"  => scope_settings.font_style = try!(FontStyle::from_str(&value)),
                "foreground" => scope_settings.foreground = Some(try!(Color::from_str(&value))),
                "background" => scope_settings.background = Some(try!(Color::from_str(&value))),
                _ => return Err(UndefinedScopeSettings)
            }
        }
        if scope_settings.scope.is_empty() {
            return Err(IncorrectSyntax)
        }
        Ok(scope_settings)
    }
}

impl<R: Read> ParsePlist<R> for ColorSchemeSettings {
    type Error = ParseColorSchemeError;

    fn parse_plist(parser: &mut StreamingParser<R>) -> Result<ColorSchemeSettings, Self::Error> {
        let mut settings = ColorSchemeSettings::default();
        for (key, value) in try!(parse_dictionary(parser)) {
            match &key[..] {
                "foreground" =>
                    settings.foreground = try!(Color::from_str(&value)),
                "background" =>
                    settings.background = try!(Color::from_str(&value)),
                "caret" =>
                    settings.caret = try!(Color::from_str(&value)),
                "lineHighlight" =>
                    settings.line_highlight = try!(Color::from_str(&value)),
                "bracketContentsForeground" =>
                    settings.bracket_contents_foreground = try!(Color::from_str(&value)),
                "bracketContentsOptions" =>
                    settings.bracket_contents_options = try!(UnderlineOption::from_str(&value)),
                "bracketsForeground" =>
                    settings.brackets_foreground = try!(Color::from_str(&value)),
                "bracketsBackground" =>
                    settings.brackets_background = try!(Color::from_str(&value)),
                "bracketsOptions" =>
                    settings.brackets_options = try!(UnderlineOption::from_str(&value)),
                "tagsForeground" =>
                    settings.tags_foreground = try!(Color::from_str(&value)),
                "tagsOptions" =>
                    settings.tagsOptions = try!(UnderlineOption::from_str(&value)),
                "findHighlight" =>
                    settings.find_highlight = try!(Color::from_str(&value)),
                "findHighlightForeground" =>
                    settings.find_highlight_foreground = try!(Color::from_str(&value)),
                "gutter" =>
                    settings.gutter = try!(Color::from_str(&value)),
                "gutterForeground" =>
                    settings.gutter_foreground = try!(Color::from_str(&value)),
                "selection" =>
                    settings.selection = try!(Color::from_str(&value)),
                "selectionBackground" =>
                    settings.selection_background = try!(Color::from_str(&value)),
                "selectionBorder" =>
                    settings.selection_border = try!(Color::from_str(&value)),
                "inactiveSelection" =>
                    settings.inactive_selection = try!(Color::from_str(&value)),
                "guide" =>
                    settings.guide = try!(Color::from_str(&value)),
                "activeGuide" =>
                    settings.active_guide = try!(Color::from_str(&value)),
                "stackGuide" =>
                    settings.stack_guide = try!(Color::from_str(&value)),
                "highlight" =>
                    settings.highlight = try!(Color::from_str(&value)),
                "highlightForeground" =>
                    settings.highlight_foreground = try!(Color::from_str(&value)),
                _ => return Err(UndefinedScopeSettings)
            }
        };
        Ok(settings)
    }
}

impl<R: Read> ParsePlist<R> for ColorScheme {
    type Error = ParseColorSchemeError;

    fn parse_plist(parser: &mut StreamingParser<R>) -> Result<ColorScheme, Self::Error> {
        eat!(parser, PlistEvent::StartPlist);
        eat!(parser, PlistEvent::StartDictionary(..));
        let mut scheme = None;
        loop {
            let key = parse_key!(parser => break);
            if &key[..] == "settings" {
                if scheme.is_some() {
                    return Err(IncorrectSyntax);
                }
                eat!(parser, PlistEvent::StartArray(..));
                // parse settings item
                eat!(parser, PlistEvent::StartDictionary(..));
                let key = parse_key!(parser => break);
                if &key[..] != "settings" {
                    return Err(IncorrectSyntax);
                }
                eat!(parser, PlistEvent::StartDictionary(..));
                let settings = try!(ColorSchemeSettings::parse_plist(parser));
                let mut scopes = Vec::new();
                loop {
                    match next!(parser) {
                        PlistEvent::EndDictionary => break,
                        PlistEvent::StartDictionary(..) => {
                            scopes.push(try!(ColorSchemeScope::parse_plist(parser)))
                        }
                        _ => return Err(IncorrectSyntax)
                    }
                }
                scheme = Some(ColorScheme { settings: settings, scopes: scopes });
            }
        }
        eat!(parser, PlistEvent::EndPlist);
        match scheme {
            Some(scheme) => Ok(scheme),
            None => Err(IncorrectSyntax)
        }
    }
}
