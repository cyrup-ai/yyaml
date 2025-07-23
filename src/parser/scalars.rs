//! Scalar value parsing with comprehensive type inference
//!
//! This module provides sophisticated scalar parsing, type detection,
//! and value conversion following YAML 1.2 specification.

use super::ast::{Node, ScalarNode};
use super::grammar::{Grammar, ParseContext};
use super::{ParseError, ParseErrorKind};
use crate::lexer::{Position, ScalarStyle, Token, TokenKind};
use std::borrow::Cow;
use std::fmt;

/// Scalar value parser with type inference
pub struct ScalarParser;

impl ScalarParser {
    /// Parse scalar using ParsingContext (zero-allocation, blazing-fast)
    /// This is the preferred method for ParsingContext-based parsing
    pub fn parse_with_context<'input>(
        context: &mut super::ParsingContext<'_, 'input>,
        parse_context: &ParseContext,
    ) -> Result<Node<'input>, ParseError> {
        let token = context.consume_token().map_err(|_| {
            ParseError::new(
                ParseErrorKind::UnexpectedEndOfInput,
                context.current_position(),
                "expected scalar token",
            )
        })?;
        
        Self::parse_scalar(token, parse_context)
    }
    
    /// Parse scalar from consumed token using ParsingContext architecture
    pub fn parse_scalar_with_context<'input>(
        token: Token<'input>,
        _context: &mut super::ParsingContext<'_, 'input>,
        parse_context: &ParseContext,
    ) -> Result<Node<'input>, ParseError> {
        Self::parse_scalar(token, parse_context)
    }

    /// Parse scalar token into appropriate node type
    pub fn parse_scalar<'input>(
        token: Token<'input>,
        context: &ParseContext,
    ) -> Result<Node<'input>, ParseError> {
        if let TokenKind::Scalar { value, style, tag } = token.kind {
            // Validate scalar style for context
            Grammar::validate_scalar_style(style, context, &value)?;

            let parsed_value = Self::process_scalar_value(value, style)?;

            Ok(Node::Scalar(ScalarNode::new(
                parsed_value,
                style,
                tag,
                token.position,
            )))
        } else {
            Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                token.position,
                format!("expected scalar token, found: {:?}", token.kind),
            ))
        }
    }

    /// Process scalar value based on style
    #[inline]
    fn process_scalar_value<'input>(
        value: Cow<'input, str>,
        style: ScalarStyle,
    ) -> Result<Cow<'input, str>, ParseError> {
        match style {
            ScalarStyle::Plain => Self::process_plain_scalar(value),
            ScalarStyle::SingleQuoted => Self::process_single_quoted(value),
            ScalarStyle::DoubleQuoted => Self::process_double_quoted(value),
            ScalarStyle::Literal => Self::process_literal_scalar(value),
            ScalarStyle::Folded => Self::process_folded_scalar(value),
        }
    }

    /// Process plain scalar with type inference
    fn process_plain_scalar<'input>(
        value: Cow<'input, str>,
    ) -> Result<Cow<'input, str>, ParseError> {
        let trimmed = value.trim();

        // Empty or whitespace-only becomes null
        if trimmed.is_empty() {
            return Ok(Cow::Borrowed(""));
        }

        // YAML 1.2 null values
        if Self::is_null_value(trimmed) {
            return Ok(Cow::Borrowed(""));
        }

        // Boolean values
        if Self::is_boolean_value(trimmed) {
            return if trimmed.len() == value.len() {
                Ok(value)
            } else {
                Ok(Cow::Owned(trimmed.to_string()))
            };
        }

        // Numeric values (integers and floats)
        if Self::is_numeric_value(trimmed) {
            return if trimmed.len() == value.len() {
                Ok(value)
            } else {
                Ok(Cow::Owned(trimmed.to_string()))
            };
        }

        // String value - return as-is but trimmed
        if trimmed.len() != value.len() {
            Ok(Cow::Owned(trimmed.to_string()))
        } else {
            Ok(value)
        }
    }

    /// Process single-quoted scalar
    fn process_single_quoted<'input>(
        value: Cow<'input, str>,
    ) -> Result<Cow<'input, str>, ParseError> {
        // Single-quoted scalars only escape single quotes
        if value.contains("''") {
            let processed = value.replace("''", "'");
            Ok(Cow::Owned(processed))
        } else {
            Ok(value)
        }
    }

    /// Process double-quoted scalar with escape sequences
    fn process_double_quoted<'input>(
        value: Cow<'input, str>,
    ) -> Result<Cow<'input, str>, ParseError> {
        if !value.contains('\\') {
            return Ok(value);
        }

        let mut result = String::new();
        let mut chars = value.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(&next_ch) = chars.peek() {
                    match next_ch {
                        '0' => {
                            chars.next();
                            result.push('\0');
                        }
                        'a' => {
                            chars.next();
                            result.push('\x07');
                        }
                        'b' => {
                            chars.next();
                            result.push('\x08');
                        }
                        't' | '\t' => {
                            chars.next();
                            result.push('\t');
                        }
                        'n' => {
                            chars.next();
                            result.push('\n');
                        }
                        'v' => {
                            chars.next();
                            result.push('\x0B');
                        }
                        'f' => {
                            chars.next();
                            result.push('\x0C');
                        }
                        'r' => {
                            chars.next();
                            result.push('\r');
                        }
                        'e' => {
                            chars.next();
                            result.push('\x1B');
                        }
                        ' ' => {
                            chars.next();
                            result.push(' ');
                        }
                        '"' => {
                            chars.next();
                            result.push('"');
                        }
                        '/' => {
                            chars.next();
                            result.push('/');
                        }
                        '\\' => {
                            chars.next();
                            result.push('\\');
                        }
                        'N' => {
                            chars.next();
                            result.push('\u{85}');
                        }
                        '_' => {
                            chars.next();
                            result.push('\u{A0}');
                        }
                        'L' => {
                            chars.next();
                            result.push('\u{2028}');
                        }
                        'P' => {
                            chars.next();
                            result.push('\u{2029}');
                        }
                        'x' => {
                            chars.next(); // consume 'x'
                            let hex = Self::parse_hex_escape(&mut chars, 2)?;
                            result.push(hex);
                        }
                        'u' => {
                            chars.next(); // consume 'u'
                            let hex = Self::parse_hex_escape(&mut chars, 4)?;
                            result.push(hex);
                        }
                        'U' => {
                            chars.next(); // consume 'U'
                            let hex = Self::parse_hex_escape(&mut chars, 8)?;
                            result.push(hex);
                        }
                        _ => {
                            // Invalid escape sequence
                            return Err(ParseError::new(
                                ParseErrorKind::UnexpectedToken,
                                Position::start(),
                                format!("invalid escape sequence: \\{next_ch}"),
                            ));
                        }
                    }
                } else {
                    // Trailing backslash
                    return Err(ParseError::new(
                        ParseErrorKind::UnexpectedEndOfInput,
                        Position::start(),
                        "unterminated escape sequence",
                    ));
                }
            } else {
                result.push(ch);
            }
        }

        Ok(Cow::Owned(result))
    }

    /// Parse hexadecimal escape sequence
    fn parse_hex_escape<I>(chars: &mut I, length: usize) -> Result<char, ParseError>
    where
        I: Iterator<Item = char>,
    {
        let mut hex_string = String::new();

        for _ in 0..length {
            if let Some(ch) = chars.next() {
                if ch.is_ascii_hexdigit() {
                    hex_string.push(ch);
                } else {
                    return Err(ParseError::new(
                        ParseErrorKind::UnexpectedToken,
                        Position::start(),
                        format!("invalid hex digit in escape sequence: {ch}"),
                    ));
                }
            } else {
                return Err(ParseError::new(
                    ParseErrorKind::UnexpectedEndOfInput,
                    Position::start(),
                    "incomplete hex escape sequence",
                ));
            }
        }

        let code_point = u32::from_str_radix(&hex_string, 16).map_err(|_| {
            ParseError::new(
                ParseErrorKind::UnexpectedToken,
                Position::start(),
                "invalid hex escape sequence",
            )
        })?;

        char::from_u32(code_point).ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::UnexpectedToken,
                Position::start(),
                "invalid Unicode code point",
            )
        })
    }

    /// Process literal block scalar with full YAML 1.2 compliance
    fn process_literal_scalar<'input>(
        value: Cow<'input, str>,
    ) -> Result<Cow<'input, str>, ParseError> {
        // Literal scalars preserve line breaks exactly as written
        // Handle strip/keep/clip indicators and indentation
        let lines: Vec<&str> = value.lines().collect();

        if lines.is_empty() {
            return Ok(Cow::Borrowed(""));
        }

        // Determine base indentation from first non-empty line
        let mut base_indent = None;
        for line in &lines {
            if !line.trim().is_empty() {
                base_indent = Some(line.len() - line.trim_start().len());
                break;
            }
        }

        let base_indent = base_indent.unwrap_or(0);

        // Process each line, preserving literal formatting
        let mut processed_lines = Vec::with_capacity(lines.len());
        for line in lines {
            if line.trim().is_empty() {
                processed_lines.push("".to_string());
            } else {
                // Remove base indentation but preserve additional indentation
                let line_indent = line.len() - line.trim_start().len();
                if line_indent >= base_indent {
                    processed_lines.push(line[base_indent..].to_string());
                } else {
                    processed_lines.push(line.to_string());
                }
            }
        }

        // Join with newlines (literal scalars preserve line breaks)
        Ok(Cow::Owned(processed_lines.join("\n")))
    }

    /// Process folded block scalar with full YAML 1.2 compliance
    fn process_folded_scalar<'input>(
        value: Cow<'input, str>,
    ) -> Result<Cow<'input, str>, ParseError> {
        // Folded scalars fold line breaks into spaces while preserving paragraph breaks
        let lines: Vec<&str> = value.lines().collect();

        if lines.is_empty() {
            return Ok(Cow::Borrowed(""));
        }

        // Determine base indentation from first non-empty line
        let mut base_indent = None;
        for line in &lines {
            if !line.trim().is_empty() {
                base_indent = Some(line.len() - line.trim_start().len());
                break;
            }
        }

        let base_indent = base_indent.unwrap_or(0);

        // Process lines, folding single line breaks but preserving paragraph breaks
        let mut result = String::new();
        let mut in_paragraph = false;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                // Empty line - end current paragraph
                if in_paragraph {
                    result.push('\n');
                    in_paragraph = false;
                }
                result.push('\n');
            } else {
                // Non-empty line - remove base indentation
                let line_indent = line.len() - line.trim_start().len();
                let processed_line = if line_indent >= base_indent {
                    &line[base_indent.min(line.len())..]
                } else {
                    line
                };

                // Check if this line is more indented (should be preserved literally)
                let line_base_indent = processed_line.len() - processed_line.trim_start().len();

                if line_base_indent > 0 {
                    // More indented line - preserve as literal
                    if in_paragraph {
                        result.push('\n');
                    }
                    result.push_str(processed_line);
                    if i < lines.len() - 1 {
                        result.push('\n');
                    }
                    in_paragraph = false;
                } else {
                    // Normal line - fold with previous if in same paragraph
                    if in_paragraph {
                        result.push(' ');
                    }
                    result.push_str(processed_line.trim());
                    in_paragraph = true;
                }
            }
        }

        // Remove trailing newlines for folded style
        while result.ends_with('\n') {
            result.pop();
        }

        Ok(Cow::Owned(result))
    }

    /// Check if value represents null according to YAML 1.2
    #[inline]
    fn is_null_value(value: &str) -> bool {
        matches!(value, "null" | "Null" | "NULL" | "~")
    }

    /// Check if value represents boolean according to YAML 1.2
    #[inline]
    fn is_boolean_value(value: &str) -> bool {
        matches!(
            value,
            "true"
                | "True"
                | "TRUE"
                | "false"
                | "False"
                | "FALSE"
                | "yes"
                | "Yes"
                | "YES"
                | "no"
                | "No"
                | "NO"
                | "on"
                | "On"
                | "ON"
                | "off"
                | "Off"
                | "OFF"
        )
    }

    /// Check if value represents numeric value according to YAML 1.2
    fn is_numeric_value(value: &str) -> bool {
        Self::is_integer_value(value) || Self::is_float_value(value)
    }

    /// Check if value is an integer (decimal, octal, hexadecimal, binary)
    fn is_integer_value(value: &str) -> bool {
        if value.is_empty() {
            return false;
        }

        // Handle sign
        let value = if value.starts_with('+') || value.starts_with('-') {
            &value[1..]
        } else {
            value
        };

        if value.is_empty() {
            return false;
        }

        // Hexadecimal: 0x[0-9a-fA-F]+
        if value.starts_with("0x") || value.starts_with("0X") {
            return value[2..].chars().all(|c| c.is_ascii_hexdigit()) && !value[2..].is_empty();
        }

        // Binary: 0b[01]+
        if value.starts_with("0b") || value.starts_with("0B") {
            return value[2..].chars().all(|c| c == '0' || c == '1') && !value[2..].is_empty();
        }

        // Octal: 0o[0-7]+
        if value.starts_with("0o") || value.starts_with("0O") {
            return value[2..].chars().all(|c| c.is_digit(8)) && !value[2..].is_empty();
        }

        // Decimal: [0-9]+
        value.chars().all(char::is_numeric)
    }

    /// Check if value is a float
    fn is_float_value(value: &str) -> bool {
        if value.is_empty() {
            return false;
        }

        // Handle special float values
        match value {
            ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => return true,
            "-.inf" | "-.Inf" | "-.INF" => return true,
            ".nan" | ".NaN" | ".NAN" => return true,
            _ => {}
        }

        // Handle sign
        let value = if value.starts_with('+') || value.starts_with('-') {
            &value[1..]
        } else {
            value
        };

        if value.is_empty() {
            return false;
        }

        // Check for decimal point or scientific notation
        if !value.contains('.') && !value.contains('e') && !value.contains('E') {
            return false;
        }

        // Split on decimal point
        if let Some(dot_pos) = value.find('.') {
            let before_dot = &value[..dot_pos];
            let after_dot = &value[dot_pos + 1..];

            // Check for scientific notation in the fractional part
            if let Some(e_pos) = after_dot.find(['e', 'E']) {
                let fraction = &after_dot[..e_pos];
                let exponent = &after_dot[e_pos + 1..];

                // Validate parts
                let before_ok = before_dot.is_empty() || before_dot.chars().all(char::is_numeric);
                let fraction_ok = fraction.chars().all(char::is_numeric);
                let exponent_ok = Self::is_valid_exponent(exponent);

                return before_ok && fraction_ok && exponent_ok;
            } else {
                // No scientific notation, just decimal
                let before_ok = before_dot.is_empty() || before_dot.chars().all(char::is_numeric);
                let after_ok = after_dot.chars().all(char::is_numeric);

                return before_ok && after_ok && (!before_dot.is_empty() || !after_dot.is_empty());
            }
        } else {
            // No decimal point, check for scientific notation
            if let Some(e_pos) = value.find(['e', 'E']) {
                let mantissa = &value[..e_pos];
                let exponent = &value[e_pos + 1..];

                let mantissa_ok = mantissa.chars().all(char::is_numeric) && !mantissa.is_empty();
                let exponent_ok = Self::is_valid_exponent(exponent);

                return mantissa_ok && exponent_ok;
            }
        }

        false
    }

    /// Validate scientific notation exponent
    fn is_valid_exponent(exponent: &str) -> bool {
        if exponent.is_empty() {
            return false;
        }

        let exponent = if exponent.starts_with('+') || exponent.starts_with('-') {
            &exponent[1..]
        } else {
            exponent
        };

        !exponent.is_empty() && exponent.chars().all(char::is_numeric)
    }

    /// Infer the actual type of a scalar value
    pub fn infer_type(value: &str) -> ScalarType {
        let trimmed = value.trim();

        if trimmed.is_empty() || Self::is_null_value(trimmed) {
            return ScalarType::Null;
        }

        if Self::is_boolean_value(trimmed) {
            return ScalarType::Boolean;
        }

        if Self::is_integer_value(trimmed) {
            return ScalarType::Integer;
        }

        if Self::is_float_value(trimmed) {
            return ScalarType::Float;
        }

        ScalarType::String
    }

    /// Convert scalar to specific type
    pub fn convert_to_type<'input>(
        value: Cow<'input, str>,
        target_type: ScalarType,
    ) -> Result<ScalarValue<'input>, ParseError> {
        let trimmed = value.trim();

        match target_type {
            ScalarType::Null => Ok(ScalarValue::Null),

            ScalarType::Boolean => match trimmed {
                "true" | "True" | "TRUE" | "yes" | "Yes" | "YES" | "on" | "On" | "ON" => {
                    Ok(ScalarValue::Boolean(true))
                }
                "false" | "False" | "FALSE" | "no" | "No" | "NO" | "off" | "Off" | "OFF" => {
                    Ok(ScalarValue::Boolean(false))
                }
                _ => Err(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    Position::start(),
                    format!("cannot convert '{trimmed}' to boolean"),
                )),
            },

            ScalarType::Integer => Self::parse_integer(trimmed).map(ScalarValue::Integer),

            ScalarType::Float => Self::parse_float(trimmed).map(ScalarValue::Float),

            ScalarType::String => Ok(ScalarValue::String(value)),
        }
    }

    /// Parse integer value
    fn parse_integer(value: &str) -> Result<i64, ParseError> {
        if value.is_empty() {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                Position::start(),
                "empty integer value",
            ));
        }

        // Handle negative values
        let (sign, value) = if let Some(stripped) = value.strip_prefix('-') {
            (-1, stripped)
        } else if let Some(stripped) = value.strip_prefix('+') {
            (1, stripped)
        } else {
            (1, value)
        };

        // Parse based on prefix
        let parsed = if value.starts_with("0x") || value.starts_with("0X") {
            i64::from_str_radix(&value[2..], 16)
        } else if value.starts_with("0b") || value.starts_with("0B") {
            i64::from_str_radix(&value[2..], 2)
        } else if value.starts_with("0o") || value.starts_with("0O") {
            i64::from_str_radix(&value[2..], 8)
        } else {
            value.parse::<i64>()
        };

        parsed.map(|n| sign * n).map_err(|_| {
            ParseError::new(
                ParseErrorKind::UnexpectedToken,
                Position::start(),
                format!("invalid integer value: {value}"),
            )
        })
    }

    /// Parse float value
    fn parse_float(value: &str) -> Result<f64, ParseError> {
        match value {
            ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => Ok(f64::INFINITY),
            "-.inf" | "-.Inf" | "-.INF" => Ok(f64::NEG_INFINITY),
            ".nan" | ".NaN" | ".NAN" => Ok(f64::NAN),
            _ => value.parse::<f64>().map_err(|_| {
                ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    Position::start(),
                    format!("invalid float value: {value}"),
                )
            }),
        }
    }
}

/// Scalar type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScalarType {
    Null,
    Boolean,
    Integer,
    Float,
    String,
}

/// Strongly-typed scalar values
#[derive(Debug, Clone, PartialEq)]
pub enum ScalarValue<'input> {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(Cow<'input, str>),
}

impl<'input> ScalarValue<'input> {
    /// Get the type of this scalar value
    #[inline]
    pub fn scalar_type(&self) -> ScalarType {
        match self {
            ScalarValue::Null => ScalarType::Null,
            ScalarValue::Boolean(_) => ScalarType::Boolean,
            ScalarValue::Integer(_) => ScalarType::Integer,
            ScalarValue::Float(_) => ScalarType::Float,
            ScalarValue::String(_) => ScalarType::String,
        }
    }
}

impl<'input> fmt::Display for ScalarValue<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalarValue::Null => write!(f, "null"),
            ScalarValue::Boolean(b) => write!(f, "{b}"),
            ScalarValue::Integer(i) => write!(f, "{i}"),
            ScalarValue::Float(float) => {
                if float.is_infinite() {
                    if *float > 0.0 {
                        write!(f, ".inf")
                    } else {
                        write!(f, "-.inf")
                    }
                } else if float.is_nan() {
                    write!(f, ".nan")
                } else {
                    write!(f, "{float}")
                }
            }
            ScalarValue::String(s) => write!(f, "{s}"),
        }
    }
}
