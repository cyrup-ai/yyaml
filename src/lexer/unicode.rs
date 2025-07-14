//! Unicode handling and escaping for YAML lexical analysis
//!
//! This module provides comprehensive Unicode support including normalization,
//! escape sequence processing, and character classification according to YAML 1.2.

use std::borrow::Cow;

/// Unicode escape sequence processor
pub struct UnicodeProcessor;

impl UnicodeProcessor {
    /// Process escape sequences in a string
    pub fn process_escapes(input: &str) -> Result<Cow<str>, EscapeError> {
        if !input.contains('\\') {
            return Ok(Cow::Borrowed(input));
        }

        let mut result = String::with_capacity(input.len());
        let mut chars = input.chars();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('0') => result.push('\0'),
                    Some('a') => result.push('\x07'),
                    Some('b') => result.push('\x08'),
                    Some('t') => result.push('\t'),
                    Some('n') => result.push('\n'),
                    Some('v') => result.push('\x0B'),
                    Some('f') => result.push('\x0C'),
                    Some('r') => result.push('\r'),
                    Some('e') => result.push('\x1B'),
                    Some(' ') => result.push(' '),
                    Some('"') => result.push('"'),
                    Some('/') => result.push('/'),
                    Some('\\') => result.push('\\'),
                    Some('N') => result.push('\u{85}'),
                    Some('_') => result.push('\u{A0}'),
                    Some('L') => result.push('\u{2028}'),
                    Some('P') => result.push('\u{2029}'),
                    Some('x') => {
                        let hex = Self::read_hex_digits(&mut chars, 2)?;
                        result.push(char::from(hex as u8));
                    }
                    Some('u') => {
                        let hex = Self::read_hex_digits(&mut chars, 4)?;
                        if let Some(ch) = char::from_u32(hex) {
                            result.push(ch);
                        } else {
                            return Err(EscapeError::InvalidUnicode(hex));
                        }
                    }
                    Some('U') => {
                        let hex = Self::read_hex_digits(&mut chars, 8)?;
                        if let Some(ch) = char::from_u32(hex) {
                            result.push(ch);
                        } else {
                            return Err(EscapeError::InvalidUnicode(hex));
                        }
                    }
                    Some(other) => return Err(EscapeError::InvalidEscape(other)),
                    None => return Err(EscapeError::UnterminatedEscape),
                }
            } else {
                result.push(ch);
            }
        }

        Ok(Cow::Owned(result))
    }

    /// Read a specific number of hexadecimal digits
    fn read_hex_digits<I: Iterator<Item = char>>(
        chars: &mut I,
        count: usize,
    ) -> Result<u32, EscapeError> {
        let mut result = 0;
        for _ in 0..count {
            match chars.next() {
                Some(ch) if ch.is_ascii_hexdigit() => {
                    result = result * 16 + ch.to_digit(16).unwrap();
                }
                Some(ch) => return Err(EscapeError::InvalidHexDigit(ch)),
                None => return Err(EscapeError::UnexpectedEndOfInput),
            }
        }
        Ok(result)
    }

    /// Escape a string for YAML output
    pub fn escape_string(input: &str, style: EscapeStyle) -> String {
        let mut result = String::with_capacity(input.len() + 2);

        match style {
            EscapeStyle::DoubleQuoted => {
                result.push('"');
                for ch in input.chars() {
                    match ch {
                        '\0' => result.push_str("\\0"),
                        '\x07' => result.push_str("\\a"),
                        '\x08' => result.push_str("\\b"),
                        '\t' => result.push_str("\\t"),
                        '\n' => result.push_str("\\n"),
                        '\x0B' => result.push_str("\\v"),
                        '\x0C' => result.push_str("\\f"),
                        '\r' => result.push_str("\\r"),
                        '\x1B' => result.push_str("\\e"),
                        '"' => result.push_str("\\\""),
                        '\\' => result.push_str("\\\\"),
                        '\u{85}' => result.push_str("\\N"),
                        '\u{A0}' => result.push_str("\\_"),
                        '\u{2028}' => result.push_str("\\L"),
                        '\u{2029}' => result.push_str("\\P"),
                        ch if ch.is_control() => {
                            if (ch as u32) <= 0xFF {
                                result.push_str(&format!("\\x{:02X}", ch as u32));
                            } else if (ch as u32) <= 0xFFFF {
                                result.push_str(&format!("\\u{:04X}", ch as u32));
                            } else {
                                result.push_str(&format!("\\U{:08X}", ch as u32));
                            }
                        }
                        ch => result.push(ch),
                    }
                }
                result.push('"');
            }
            EscapeStyle::SingleQuoted => {
                result.push('\'');
                for ch in input.chars() {
                    if ch == '\'' {
                        result.push_str("''");
                    } else {
                        result.push(ch);
                    }
                }
                result.push('\'');
            }
        }

        result
    }
}

/// String escaping styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscapeStyle {
    DoubleQuoted,
    SingleQuoted,
}

/// Errors that can occur during escape processing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscapeError {
    InvalidEscape(char),
    InvalidHexDigit(char),
    InvalidUnicode(u32),
    UnterminatedEscape,
    UnexpectedEndOfInput,
}

impl std::fmt::Display for EscapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EscapeError::InvalidEscape(ch) => write!(f, "invalid escape sequence '\\{}'", ch),
            EscapeError::InvalidHexDigit(ch) => write!(f, "invalid hexadecimal digit '{}'", ch),
            EscapeError::InvalidUnicode(code) => {
                write!(f, "invalid unicode code point U+{:X}", code)
            }
            EscapeError::UnterminatedEscape => write!(f, "unterminated escape sequence"),
            EscapeError::UnexpectedEndOfInput => {
                write!(f, "unexpected end of input in escape sequence")
            }
        }
    }
}

impl std::error::Error for EscapeError {}

/// YAML character classification functions
pub mod chars {
    /// Check if a character is a YAML line break
    #[inline]
    pub fn is_break(ch: char) -> bool {
        matches!(ch, '\n' | '\r')
    }

    /// Check if a character is YAML whitespace (space or tab)
    #[inline]
    pub fn is_space(ch: char) -> bool {
        matches!(ch, ' ' | '\t')
    }

    /// Check if a character is YAML blank (whitespace or line break)
    #[inline]
    pub fn is_blank(ch: char) -> bool {
        is_space(ch) || is_break(ch)
    }

    /// Check if a character can start a plain scalar
    #[inline]
    pub fn can_start_plain_scalar(ch: char) -> bool {
        !matches!(
            ch,
            '-' | '?'
                | ':'
                | ','
                | '['
                | ']'
                | '{'
                | '}'
                | '#'
                | '&'
                | '*'
                | '!'
                | '|'
                | '>'
                | '\''
                | '"'
                | '%'
                | '@'
                | '`'
                | ' '
                | '\t'
                | '\n'
                | '\r'
        )
    }

    /// Check if a character can continue a plain scalar
    #[inline]
    pub fn can_continue_plain_scalar(ch: char) -> bool {
        !is_blank(ch) && ch != '#'
    }

    /// Check if a character is a valid YAML identifier character
    #[inline]
    pub fn is_yaml_identifier(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-')
    }

    /// Check if a character is a valid anchor/alias character
    #[inline]
    pub fn is_anchor_char(ch: char) -> bool {
        !is_blank(ch) && !matches!(ch, ',' | '[' | ']' | '{' | '}')
    }

    /// Check if a character is a valid tag character
    #[inline]
    pub fn is_tag_char(ch: char) -> bool {
        ch.is_ascii_alphanumeric()
            || matches!(
                ch,
                '-' | '_'
                    | '.'
                    | '~'
                    | ':'
                    | '/'
                    | '?'
                    | '#'
                    | '['
                    | ']'
                    | '@'
                    | '!'
                    | '$'
                    | '&'
                    | '\''
                    | '('
                    | ')'
                    | '*'
                    | '+'
                    | ','
                    | ';'
                    | '='
            )
    }

    /// Check if a character needs URI encoding in a tag
    #[inline]
    pub fn needs_uri_encoding(ch: char) -> bool {
        !is_tag_char(ch)
    }

    /// Check if a character is a white space (space or tab)
    #[inline]
    pub fn is_white(ch: char) -> bool {
        ch == ' ' || ch == '\t'
    }

    /// Check if a character is a word character (alphanumeric or hyphen)
    #[inline]
    pub fn is_word_char(ch: char) -> bool {
        ch.is_ascii_alphanumeric() || ch == '-'
    }

    /// Check if a character is a URI character
    #[inline]
    pub fn is_uri_char(ch: char) -> bool {
        ch.is_ascii_alphanumeric()
            || matches!(
                ch,
                '-' | '_'
                    | '.'
                    | '~'
                    | ':'
                    | '/'
                    | '?'
                    | '#'
                    | '['
                    | ']'
                    | '@'
                    | '!'
                    | '$'
                    | '&'
                    | '\''
                    | '('
                    | ')'
                    | '*'
                    | '+'
                    | ','
                    | ';'
                    | '='
            )
    }

    /// Check if a character is printable for YAML
    #[inline]
    pub fn is_printable(ch: char) -> bool {
        ch == '\t'
            || ch == '\n'
            || ch == '\r'
            || (ch >= ' ' && ch <= '\u{7E}')
            || ch == '\u{85}'
            || (ch >= '\u{A0}' && ch <= '\u{D7FF}')
            || (ch >= '\u{E000}' && ch <= '\u{FFFD}')
            || (ch >= '\u{10000}' && ch <= '\u{10FFFF}')
    }
}

/// Unicode normalization utilities
pub mod normalization {
    use std::borrow::Cow;

    /// Normalize line endings to LF
    pub fn normalize_line_endings(input: &str) -> Cow<str> {
        if !input.contains('\r') {
            return Cow::Borrowed(input);
        }

        let mut result = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next(); // Skip the \n
                    }
                    result.push('\n');
                }
                ch => result.push(ch),
            }
        }

        Cow::Owned(result)
    }

    /// Check if text contains only valid YAML characters
    pub fn is_valid_yaml_text(text: &str) -> bool {
        text.chars().all(super::chars::is_printable)
    }

    /// Remove BOM if present
    pub fn remove_bom(input: &str) -> &str {
        input.strip_prefix('\u{FEFF}').unwrap_or(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_processing() {
        assert_eq!(
            UnicodeProcessor::process_escapes("hello\\nworld").unwrap(),
            Cow::<str>::Owned("hello\nworld".to_string())
        );

        assert_eq!(
            UnicodeProcessor::process_escapes("no escapes").unwrap(),
            Cow::Borrowed("no escapes")
        );

        assert_eq!(
            UnicodeProcessor::process_escapes("\\u0041").unwrap(),
            Cow::<str>::Owned("A".to_string())
        );
    }

    #[test]
    fn test_character_classification() {
        assert!(chars::is_break('\n'));
        assert!(chars::is_break('\r'));
        assert!(!chars::is_break(' '));

        assert!(chars::is_space(' '));
        assert!(chars::is_space('\t'));
        assert!(!chars::is_space('\n'));

        assert!(chars::can_start_plain_scalar('a'));
        assert!(!chars::can_start_plain_scalar('['));
    }

    #[test]
    fn test_line_ending_normalization() {
        assert_eq!(
            normalization::normalize_line_endings("line1\r\nline2\r\nline3"),
            Cow::<str>::Owned("line1\nline2\nline3".to_string())
        );

        assert_eq!(
            normalization::normalize_line_endings("line1\nline2\nline3"),
            Cow::Borrowed("line1\nline2\nline3")
        );
    }
}
