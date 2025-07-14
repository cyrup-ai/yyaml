//! Scalar scanning with zero-allocation optimizations
//!
//! This module provides efficient scanning of all YAML scalar types including
//! plain, quoted, and block scalars with proper escape handling.

use crate::error::ScanError;
use crate::scanner::ScannerConfig;
use crate::scanner::state::ScannerState;

/// Scan plain scalar with efficient character classification
#[inline]
pub fn scan_plain_scalar<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
    config: &ScannerConfig,
) -> Result<String, ScanError> {
    let mut result = String::with_capacity(32);
    let mut spaces = String::new();
    let start_col = state.column();
    let in_flow = state.in_flow_context();

    loop {
        // Check for end conditions efficiently
        if let Ok(ch) = state.peek_char() {
            // Flow context indicators
            if in_flow && matches!(ch, ',' | '[' | ']' | '{' | '}') {
                break;
            }

            // Common indicators
            if matches!(ch, ':' | '#') {
                // Check if ':' is followed by space (value indicator)
                if ch == ':' {
                    if let Some(next) = state.peek_char_at(1) {
                        if matches!(next, ' ' | '\t' | '\n' | '\r')
                            || (in_flow && matches!(next, ',' | '[' | ']' | '{' | '}'))
                        {
                            break;
                        }
                    } else {
                        // EOF after ':'
                        break;
                    }
                }
                // '#' preceded by space is comment
                if ch == '#' && !spaces.is_empty() {
                    break;
                }
            }

            // Whitespace handling
            if matches!(ch, ' ' | '\t') {
                spaces.push(state.consume_char()?);
                continue;
            }

            // Line breaks
            if matches!(ch, '\n' | '\r') {
                // Check if next line would change indentation in block context
                if !in_flow {
                    let current_mark = state.mark();
                    state.consume_char()?; // consume newline

                    // Skip any additional newlines
                    while matches!(state.peek_char(), Ok('\n') | Ok('\r')) {
                        state.consume_char()?;
                    }

                    // Check indentation of next line
                    let mut next_col = 0;
                    let mut temp_chars = Vec::new();
                    while let Ok(ch) = state.peek_char() {
                        if matches!(ch, ' ' | '\t') {
                            temp_chars.push(state.consume_char()?);
                            next_col += 1;
                        } else {
                            break;
                        }
                    }

                    // Put back the indentation chars if we continue
                    if next_col > 0 && next_col >= start_col as usize {
                        // Continue scalar
                        if !result.is_empty() && !spaces.is_empty() {
                            result.push_str(&spaces);
                        }
                        result.push(' '); // fold newline to space
                        spaces.clear();
                    } else {
                        // End scalar due to dedent
                        break;
                    }
                    continue;
                } else {
                    break; // newline ends scalar in flow
                }
            }

            // Document markers
            if ch == '-' || ch == '.' {
                if state.check_document_start()? || state.check_document_end()? {
                    break;
                }
            }

            // Regular character
            if !spaces.is_empty() {
                result.push_str(&spaces);
                spaces.clear();
            }
            result.push(state.consume_char()?);
        } else {
            // EOF
            break;
        }
    }

    // Trim trailing spaces
    result.truncate(result.trim_end().len());

    if result.is_empty() {
        return Err(ScanError::new(state.mark(), "empty plain scalar"));
    }

    Ok(result)
}

/// Scan single-quoted scalar with proper escape handling
#[inline]
pub fn scan_single_quoted<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<String, ScanError> {
    let mut result = String::with_capacity(32);
    let start_mark = state.mark();

    loop {
        match state.peek_char()? {
            '\'' => {
                state.consume_char()?;
                // Check for escaped quote ''
                if matches!(state.peek_char(), Ok('\'')) {
                    state.consume_char()?;
                    result.push('\'');
                } else {
                    // End of string
                    return Ok(result);
                }
            }
            '\n' | '\r' => {
                // Fold newlines to spaces
                state.consume_char()?;
                // Skip leading whitespace on next line
                while matches!(state.peek_char(), Ok(' ') | Ok('\t')) {
                    state.consume_char()?;
                }
                if !result.is_empty() && !result.ends_with(' ') {
                    result.push(' ');
                }
            }
            ch => {
                result.push(state.consume_char()?);
            }
        }
    }
}

/// Scan double-quoted scalar with comprehensive escape sequences
#[inline]
pub fn scan_double_quoted<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<String, ScanError> {
    let mut result = String::with_capacity(32);
    let start_mark = state.mark();

    loop {
        match state.peek_char()? {
            '"' => {
                state.consume_char()?;
                return Ok(result);
            }
            '\\' => {
                state.consume_char()?;
                let escaped = process_escape_sequence(state)?;
                result.push(escaped);
            }
            '\n' | '\r' => {
                // Fold newlines to spaces
                state.consume_char()?;
                // Skip leading whitespace on next line
                while matches!(state.peek_char(), Ok(' ') | Ok('\t')) {
                    state.consume_char()?;
                }
                if !result.is_empty() && !result.ends_with(' ') {
                    result.push(' ');
                }
            }
            ch => {
                result.push(state.consume_char()?);
            }
        }
    }
}

/// Process escape sequence in double-quoted strings
#[inline]
fn process_escape_sequence<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<char, ScanError> {
    match state.consume_char()? {
        '0' => Ok('\0'),
        'a' => Ok('\x07'),
        'b' => Ok('\x08'),
        't' => Ok('\t'),
        'n' => Ok('\n'),
        'v' => Ok('\x0b'),
        'f' => Ok('\x0c'),
        'r' => Ok('\r'),
        'e' => Ok('\x1b'),
        ' ' => Ok(' '),
        '"' => Ok('"'),
        '/' => Ok('/'),
        '\\' => Ok('\\'),
        'N' => Ok('\u{0085}'), // NEL (Next Line)
        '_' => Ok('\u{00A0}'), // NBSP (Non-breaking space)
        'L' => Ok('\u{2028}'), // Line Separator
        'P' => Ok('\u{2029}'), // Paragraph Separator
        'x' => read_hex_escape(state, 2),
        'u' => read_hex_escape(state, 4),
        'U' => read_hex_escape(state, 8),
        '\n' | '\r' => {
            // Escaped newline - skip whitespace on next line
            while matches!(state.peek_char(), Ok(' ') | Ok('\t')) {
                state.consume_char()?;
            }
            Ok(' ') // Fold to space
        }
        ch => Err(ScanError::new(
            state.mark(),
            &format!("invalid escape sequence '\\{}'", ch),
        )),
    }
}

/// Read hexadecimal escape sequence
#[inline]
fn read_hex_escape<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
    digits: usize,
) -> Result<char, ScanError> {
    let mut value = 0u32;

    for i in 0..digits {
        match state.consume_char()? {
            ch @ '0'..='9' => value = value * 16 + (ch as u32 - '0' as u32),
            ch @ 'a'..='f' => value = value * 16 + (ch as u32 - 'a' as u32 + 10),
            ch @ 'A'..='F' => value = value * 16 + (ch as u32 - 'A' as u32 + 10),
            ch => {
                return Err(ScanError::new(
                    state.mark(),
                    &format!("invalid hex digit '{}' in escape sequence", ch),
                ));
            }
        }
    }

    char::from_u32(value).ok_or_else(|| {
        ScanError::new(
            state.mark(),
            &format!("invalid Unicode code point U+{:04X}", value),
        )
    })
}

/// Scan block scalar (literal | or folded >)
#[inline]
pub fn scan_block_scalar<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
    literal: bool,
) -> Result<String, ScanError> {
    let mut result = String::with_capacity(128);

    // Parse block scalar header
    let (chomping, explicit_indent) = parse_block_scalar_header(state)?;

    // Skip to next line
    skip_to_next_line(state)?;

    // Determine base indentation
    let base_indent = if let Some(indent) = explicit_indent {
        indent
    } else {
        detect_block_scalar_indent(state)?
    };

    // Read block scalar content
    let mut trailing_breaks = String::new();
    let mut first_line = true;

    loop {
        // Check for document markers
        if state.at_line_start() {
            if state.check_document_start()? || state.check_document_end()? {
                break;
            }
        }

        // Read indentation
        let line_indent = count_indentation(state)?;

        if line_indent < base_indent {
            // Less indented line ends the scalar
            break;
        }

        // Check for blank line
        if matches!(state.peek_char(), Ok('\n') | Ok('\r') | Err(_)) {
            trailing_breaks.push('\n');
            if let Ok(_) = state.peek_char() {
                consume_line_break(state)?;
            }
            continue;
        }

        // Add any accumulated breaks
        if !trailing_breaks.is_empty() {
            if literal || trailing_breaks.len() > 1 {
                // In literal mode or multiple breaks, preserve them
                result.push_str(&trailing_breaks);
            } else if !result.is_empty() {
                // In folded mode with single break, convert to space
                result.push(' ');
            }
            trailing_breaks.clear();
        }

        // Add line content
        if !first_line && literal {
            result.push('\n');
        } else if !first_line && !literal && !result.is_empty() {
            result.push(' ');
        }
        first_line = false;

        // Skip extra indentation beyond base
        for _ in base_indent..line_indent {
            if matches!(state.peek_char(), Ok(' ')) {
                state.consume_char()?;
            }
        }

        // Read line content
        while let Ok(ch) = state.peek_char() {
            if matches!(ch, '\n' | '\r') {
                break;
            }
            result.push(state.consume_char()?);
        }

        // Consume line break if present
        if matches!(state.peek_char(), Ok('\n') | Ok('\r')) {
            consume_line_break(state)?;
        }
    }

    // Apply chomping indicator
    match chomping {
        Chomping::Strip => {
            // Remove all trailing newlines
            result.truncate(result.trim_end_matches('\n').len());
        }
        Chomping::Keep => {
            // Keep one trailing newline
            if !trailing_breaks.is_empty() {
                result.push('\n');
            }
        }
        Chomping::Clip => {
            // Keep all trailing newlines
            result.push_str(&trailing_breaks);
        }
    }

    Ok(result)
}

/// Block scalar chomping modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Chomping {
    Strip, // - indicator
    Clip,  // default
    Keep,  // + indicator
}

/// Parse block scalar header for chomping and indentation
#[inline]
fn parse_block_scalar_header<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<(Chomping, Option<usize>), ScanError> {
    let mut chomping = Chomping::Clip;
    let mut indent = None;

    // Skip optional indicators (can be in any order)
    for _ in 0..2 {
        match state.peek_char() {
            Ok('+') => {
                state.consume_char()?;
                chomping = Chomping::Keep;
            }
            Ok('-') => {
                state.consume_char()?;
                chomping = Chomping::Strip;
            }
            Ok(ch @ '1'..='9') => {
                state.consume_char()?;
                indent = Some((ch as usize) - ('0' as usize));
            }
            _ => break,
        }
    }

    Ok((chomping, indent))
}

/// Skip to next line, consuming any comments
#[inline]
fn skip_to_next_line<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<(), ScanError> {
    // Skip any remaining content on current line
    while let Ok(ch) = state.peek_char() {
        if matches!(ch, '\n' | '\r') {
            consume_line_break(state)?;
            break;
        }
        state.consume_char()?;
    }
    Ok(())
}

/// Detect indentation of block scalar content
#[inline]
fn detect_block_scalar_indent<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<usize, ScanError> {
    let mut min_indent = usize::MAX;

    // Look ahead to find first non-empty line
    let saved_mark = state.mark();
    let mut chars_consumed = Vec::new();

    loop {
        let indent = count_indentation(state)?;

        // Check if line is non-empty
        match state.peek_char() {
            Ok('\n') | Ok('\r') => {
                // Empty line, continue
                consume_line_break(state)?;
            }
            Ok(_) => {
                // Non-empty line found
                min_indent = indent;
                break;
            }
            Err(_) => {
                // EOF
                break;
            }
        }
    }

    if min_indent == usize::MAX {
        min_indent = 0;
    }

    Ok(min_indent)
}

/// Count indentation at current position
#[inline]
fn count_indentation<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<usize, ScanError> {
    let mut count = 0;

    while matches!(state.peek_char(), Ok(' ')) {
        state.consume_char()?;
        count += 1;
    }

    Ok(count)
}

/// Consume a line break (handling \r\n as single break)
#[inline]
fn consume_line_break<T: Iterator<Item = char>>(
    state: &mut ScannerState<T>,
) -> Result<(), ScanError> {
    match state.consume_char()? {
        '\r' => {
            // Check for \r\n
            if matches!(state.peek_char(), Ok('\n')) {
                state.consume_char()?;
            }
        }
        '\n' => {}
        ch => {
            return Err(ScanError::new(
                state.mark(),
                &format!("expected line break, found '{}'", ch),
            ));
        }
    }
    Ok(())
}
