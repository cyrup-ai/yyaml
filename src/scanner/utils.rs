//! Utility functions for efficient scanner operations
//!
//! This module provides common scanning utilities including whitespace handling,
//! character classification, and buffer management with zero-allocation design.

use crate::error::{Marker, ScanError};
use crate::scanner::state::ScannerState;

/// Skip whitespace and comments efficiently
#[inline]
pub fn skip_whitespace_and_comments<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<(), ScanError> {
    loop {
        match state.peek_char() {
            Ok(' ') | Ok('\t') => {
                state.consume_char()?;
            }
            Ok('\n') | Ok('\r') => {
                consume_line_break(state)?;
            }
            Ok('#') => {
                skip_comment_line(state)?;
            }
            _ => break,
        }
    }
    Ok(())
}

/// Skip only whitespace (not comments)
#[inline]
pub fn skip_whitespace<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<(), ScanError> {
    while matches!(state.peek_char(), Ok(' ') | Ok('\t')) {
        state.consume_char()?;
    }
    Ok(())
}

/// Skip entire comment line including newline
#[inline]
pub fn skip_comment_line<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<(), ScanError> {
    // Consume the '#' if we're at it
    if matches!(state.peek_char(), Ok('#')) {
        state.consume_char()?;
    }

    // Skip to end of line
    while let Ok(ch) = state.peek_char() {
        if matches!(ch, '\n' | '\r') {
            consume_line_break(state)?;
            break;
        }
        state.consume_char()?;
    }

    Ok(())
}

/// Consume line break handling both \n and \r\n
#[inline]
pub fn consume_line_break<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<(), ScanError> {
    match state.consume_char()? {
        '\r' => {
            // Check for \r\n sequence
            if matches!(state.peek_char(), Ok('\n')) {
                state.consume_char()?;
            }
        }
        '\n' => {
            // Simple newline
        }
        ch => {
            return Err(ScanError::new(
                state.mark(),
                &format!("expected line break, found '{ch}'"),
            ));
        }
    }
    Ok(())
}

/// Skip to start of next line
#[inline]
pub fn skip_to_next_line<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<(), ScanError> {
    while let Ok(ch) = state.peek_char() {
        if matches!(ch, '\n' | '\r') {
            consume_line_break(state)?;
            break;
        }
        state.consume_char()?;
    }
    Ok(())
}

/// Check if character is YAML whitespace
#[inline]
pub fn is_whitespace(ch: char) -> bool {
    matches!(ch, ' ' | '\t')
}

/// Check if character is line break
#[inline]
pub fn is_line_break(ch: char) -> bool {
    matches!(ch, '\n' | '\r')
}

/// Check if character is blank (whitespace or line break)
#[inline]
pub fn is_blank(ch: char) -> bool {
    is_whitespace(ch) || is_line_break(ch)
}

/// Check if character can start a plain scalar
#[inline]
pub fn can_start_plain_scalar(ch: char) -> bool {
    !matches!(
        ch,
        // Flow indicators
        ',' | '[' | ']' | '{' | '}' |
        // Key/value indicators
        ':' | '?' |
        // Anchor/alias indicators
        '&' | '*' |
        // Tag indicator
        '!' |
        // Comment indicator
        '#' |
        // Block scalar indicators
        '|' | '>' |
        // Quoted scalar indicators
        '\'' | '"' |
        // Document markers (context-dependent)
        '-' | '.' |
        // Directive indicator
        '%' |
        // Reserved indicators
        '@' | '`' |
        // Whitespace
        ' ' | '\t' | '\n' | '\r'
    )
}

/// Check if character can continue a plain scalar
#[inline]
pub fn can_continue_plain_scalar(ch: char, in_flow: bool) -> bool {
    if is_blank(ch) {
        return false;
    }

    if ch == '#' {
        return false;
    }

    if in_flow && matches!(ch, ',' | '[' | ']' | '{' | '}') {
        return false;
    }

    true
}

/// Check if character needs escaping in double-quoted strings
#[inline]
pub fn needs_escaping_in_double_quoted(ch: char) -> bool {
    matches!(
        ch,
        '"' | '\\' | '\0'..='\x1F' | '\x7F' | '\u{85}' | '\u{A0}' | '\u{2028}' | '\u{2029}'
    )
}

/// Check if character is printable YAML character
#[inline]
pub fn is_printable(ch: char) -> bool {
    match ch {
        '\t' | '\n' | '\r' => true,
        ch if (' '..='\u{7E}').contains(&ch) => true,
        '\u{85}' => true,
        ch if ('\u{A0}'..='\u{D7FF}').contains(&ch) => true,
        ch if ('\u{E000}'..='\u{FFFD}').contains(&ch) => true,
        ch if ('\u{10000}'..='\u{10FFFF}').contains(&ch) => true,
        _ => false,
    }
}

/// Check if character is valid YAML identifier character
#[inline]
pub fn is_yaml_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-')
}

/// Read while predicate is true, with length limit
#[inline]
pub fn read_while<T: Iterator<Item = char>, F>(
    state: &mut ScannerState<T>,
    mut predicate: F,
    max_length: usize,
    context: &str,
) -> Result<String, ScanError>
where
    F: FnMut(char) -> bool,
{
    let mut result = String::with_capacity(32);
    let start_mark = state.mark();

    while let Ok(ch) = state.peek_char() {
        if !predicate(ch) {
            break;
        }

        if result.len() >= max_length {
            return Err(ScanError::new(
                start_mark,
                &format!("{context} too long (max {max_length} characters)"),
            ));
        }

        result.push(state.consume_char()?);
    }

    Ok(result)
}

/// Read exactly n characters
#[inline]
pub fn read_exact_chars<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
    n: usize,
    context: &str,
) -> Result<String, ScanError> {
    let mut result = String::with_capacity(n);

    for i in 0..n {
        match state.consume_char() {
            Ok(ch) => result.push(ch),
            Err(_) => {
                return Err(ScanError::new(
                    state.mark(),
                    &format!(
                        "unexpected end of input in {context} (expected {n} characters, got {i})"
                    ),
                ));
            }
        }
    }

    Ok(result)
}

/// Consume specific character or error
#[inline]
pub fn expect_char<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
    expected: char,
    context: &str,
) -> Result<(), ScanError> {
    match state.consume_char()? {
        ch if ch == expected => Ok(()),
        ch => Err(ScanError::new(
            state.mark(),
            &format!("expected '{expected}' in {context}, found '{ch}'"),
        )),
    }
}

/// Consume specific string or error
#[inline]
pub fn expect_string<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
    expected: &str,
    context: &str,
) -> Result<(), ScanError> {
    for (i, expected_char) in expected.chars().enumerate() {
        match state.consume_char() {
            Ok(ch) if ch == expected_char => continue,
            Ok(ch) => {
                return Err(ScanError::new(
                    state.mark(),
                    &format!(
                        "expected '{expected}' in {context} at position {i}, found '{ch}'"
                    ),
                ));
            }
            Err(_) => {
                return Err(ScanError::new(
                    state.mark(),
                    &format!(
                        "unexpected end of input in {context} (expected '{expected}', got {i} characters)"
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// Peek ahead for pattern without consuming
#[inline]
pub fn peek_pattern<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
    pattern: &[char],
) -> bool {
    state.check_chars(pattern)
}

/// Check if at end of input
#[inline]
pub fn at_end<T: Iterator<Item = char>>(state: &mut ScannerState<T>) -> bool {
    state.is_done()
}

/// Get current position info as string
pub fn position_info(marker: Marker) -> String {
    format!("line {}, column {}", marker.line, marker.col + 1)
}

/// Validate character is in allowed set
#[inline]
pub fn validate_char_in_set(
    ch: char,
    allowed: &[char],
    position: Marker,
    context: &str,
) -> Result<(), ScanError> {
    if allowed.contains(&ch) {
        Ok(())
    } else {
        Err(ScanError::new(
            position,
            &format!("invalid character '{ch}' in {context}"),
        ))
    }
}

/// Normalize line endings to LF
pub fn normalize_line_endings(input: &str) -> String {
    if !input.contains('\r') {
        return input.to_string();
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

    result
}

/// Count UTF-8 byte length of string slice
#[inline]
pub fn byte_length(s: &str) -> usize {
    s.len()
}

/// Count Unicode grapheme clusters (user-perceived characters)
pub fn grapheme_count(s: &str) -> usize {
    // Simplified grapheme counting - in production would use unicode-segmentation crate
    s.chars().count()
}

/// Efficient string builder for scanner operations
pub struct StringBuilder {
    buffer: String,
    _capacity_hint: usize,
}

impl StringBuilder {
    /// Create new string builder with capacity hint
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
            _capacity_hint: capacity,
        }
    }

    /// Push character to buffer
    #[inline]
    pub fn push(&mut self, ch: char) {
        self.buffer.push(ch);
    }

    /// Push string slice to buffer
    #[inline]
    pub fn push_str(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    /// Get current length
    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Clear buffer
    #[inline]
    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    /// Convert to final string
    #[inline]
    pub fn into_string(self) -> String {
        self.buffer
    }

    /// Get string slice view
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.buffer
    }

    /// Reserve additional capacity
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.buffer.reserve(additional);
    }
}

impl Default for StringBuilder {
    fn default() -> Self {
        Self::with_capacity(32)
    }
}
