//! High-performance YAML scanner with zero-allocation design
//!
//! This module provides the core scanning functionality for YAML tokenization,
//! with optimized character processing and minimal memory allocation.

use std::borrow::Cow;
use std::iter::Peekable;
use std::str::Chars;

use super::position::*;
use super::tokens::*;
use super::unicode::{UnicodeProcessor, chars, normalization};
use super::{LexError, LexErrorKind};

/// High-performance YAML scanner
#[derive(Debug, Clone)]
pub struct Scanner<'input> {
    input: &'input str,
    chars: Peekable<Chars<'input>>,
    current_offset: usize,
    peeked_token: Option<Token<'input>>,
    indent_stack: Vec<usize>,
    flow_level: usize,
    _simple_key_allowed: bool,
    document_start: bool,
}

impl<'input> Scanner<'input> {
    /// Create a new scanner for the given input
    #[inline]
    pub fn new(input: &'input str) -> Self {
        let normalized = normalization::remove_bom(input);
        Self {
            input: normalized,
            chars: normalized.chars().peekable(),
            current_offset: 0,
            peeked_token: None,
            indent_stack: vec![0],
            flow_level: 0,
            _simple_key_allowed: true,
            document_start: true,
        }
    }

    /// Check if we've reached the end of input
    #[inline]
    pub fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    /// Scan the next token
    pub fn scan_token(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        if let Some(token) = self.peeked_token.take() {
            return Ok(token);
        }

        self.scan_token_impl(position)
    }

    /// Peek at the next token without consuming it
    pub fn peek_token(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        if self.peeked_token.is_none() {
            self.peeked_token = Some(self.scan_token_impl(position)?);
        }
        // Safe to unwrap here as we just ensured it's Some, but use pattern matching instead
        match &self.peeked_token {
            Some(token) => Ok(token.clone()),
            None => {
                // This should never happen due to the logic above, but handle gracefully
                Err(LexError::new(
                    LexErrorKind::UnexpectedCharacter('\0'.to_string()),
                    position.current(),
                ))
            }
        }
    }

    /// Internal token scanning implementation
    fn scan_token_impl(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        self.skip_whitespace_and_comments(position);

        let start_pos = position.current();

        if self.is_at_end() {
            return Ok(Token::new(TokenKind::StreamEnd, start_pos, 0));
        }

        if self.document_start {
            self.document_start = false;
            return Ok(Token::new(TokenKind::StreamStart, start_pos, 0));
        }

        let ch = match self.chars.peek() {
            Some(&ch) => ch,
            None => {
                return Err(LexError::new(
                    LexErrorKind::UnexpectedCharacter('\0'.to_string()),
                    position.current(),
                ));
            }
        };

        // Handle indentation at start of line
        if start_pos.column == 1 && !chars::is_break(ch) {
            return self.scan_indentation(position);
        }

        match ch {
            // Document markers
            '-' => self.scan_dash_or_document_start(position),
            '.' => self.scan_dot_or_document_end(position),

            // Flow collection delimiters
            '[' => self.scan_single_char(TokenKind::FlowSequenceStart, position),
            ']' => self.scan_single_char(TokenKind::FlowSequenceEnd, position),
            '{' => self.scan_single_char(TokenKind::FlowMappingStart, position),
            '}' => self.scan_single_char(TokenKind::FlowMappingEnd, position),
            ',' => self.scan_single_char(TokenKind::FlowEntry, position),

            // Key-value indicators
            '?' => self.scan_single_char(TokenKind::Key, position),
            ':' => self.scan_colon(position),

            // Anchors and aliases
            '&' => self.scan_anchor(position),
            '*' => self.scan_alias(position),

            // Tags
            '!' => self.scan_tag(position),

            // Directives
            '%' => self.scan_directive(position),

            // Quoted strings
            '\'' => self.scan_single_quoted_string(position),
            '"' => self.scan_double_quoted_string(position),

            // Block scalars
            '|' => self.scan_literal_scalar(position),
            '>' => self.scan_folded_scalar(position),

            // Line breaks
            '\n' | '\r' => self.scan_line_break(position),

            // Plain scalars and everything else
            _ => self.scan_plain_scalar(position),
        }
    }

    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&mut self, position: &mut PositionTracker) {
        while let Some(&ch) = self.chars.peek() {
            match ch {
                ' ' | '\t' => {
                    self.consume_char(position);
                }
                '#' => {
                    // Skip comment to end of line
                    while let Some(&ch) = self.chars.peek() {
                        if chars::is_break(ch) {
                            break;
                        }
                        self.consume_char(position);
                    }
                }
                _ => break,
            }
        }
    }

    /// Scan indentation tokens
    fn scan_indentation(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        let mut indent = 0;

        while let Some(&ch) = self.chars.peek() {
            if ch == ' ' {
                indent += 1;
                self.consume_char(position);
            } else {
                break;
            }
        }

        let current_indent = match self.indent_stack.last() {
            Some(&indent) => indent,
            None => {
                // This should never happen as indent_stack is initialized with [0]
                // but handle gracefully by treating as zero indentation
                0
            }
        };

        if indent > current_indent {
            self.indent_stack.push(indent);
            Ok(Token::new(TokenKind::Indent(indent), start_pos, indent))
        } else if indent < current_indent {
            // Pop indentation levels until we match or go below
            let mut dedent_count = 0;
            while let Some(&stack_indent) = self.indent_stack.last() {
                if stack_indent <= indent {
                    break;
                }
                self.indent_stack.pop();
                dedent_count += 1;
            }

            if self.indent_stack.last() != Some(&indent) {
                return Err(LexError::new(
                    LexErrorKind::UnexpectedCharacter("invalid indentation level".to_string()),
                    start_pos,
                ));
            }

            Ok(Token::new(TokenKind::Dedent(dedent_count), start_pos, 0))
        } else {
            // Same indentation, continue scanning
            self.scan_token_impl(position)
        }
    }

    /// Scan dash or document start marker
    fn scan_dash_or_document_start(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();

        if self.check_sequence("---") {
            self.consume_char(position); // -
            self.consume_char(position); // -
            self.consume_char(position); // -
            Ok(Token::new(TokenKind::DocumentStart, start_pos, 3))
        } else {
            // Single dash, could be block entry
            self.consume_char(position);
            if chars::is_blank(*self.chars.peek().unwrap_or(&'\0')) {
                Ok(Token::new(TokenKind::BlockEntry, start_pos, 1))
            } else {
                // Part of a plain scalar
                self.scan_plain_scalar_from_dash(position, start_pos)
            }
        }
    }

    /// Scan dot or document end marker
    fn scan_dot_or_document_end(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();

        if self.check_sequence("...") {
            self.consume_char(position); // .
            self.consume_char(position); // .
            self.consume_char(position); // .
            Ok(Token::new(TokenKind::DocumentEnd, start_pos, 3))
        } else {
            // Single dot, part of plain scalar
            self.scan_plain_scalar(position)
        }
    }

    /// Scan a single character token
    fn scan_single_char(
        &mut self,
        kind: TokenKind<'input>,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        self.consume_char(position);

        // Update flow level for flow collection delimiters
        match kind {
            TokenKind::FlowSequenceStart | TokenKind::FlowMappingStart => {
                self.flow_level += 1;
            }
            TokenKind::FlowSequenceEnd | TokenKind::FlowMappingEnd => {
                if self.flow_level > 0 {
                    self.flow_level -= 1;
                }
            }
            _ => {}
        }

        Ok(Token::new(kind, start_pos, 1))
    }

    /// Scan colon (value indicator)
    fn scan_colon(&mut self, position: &mut PositionTracker) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        self.consume_char(position);

        // Check if this is followed by whitespace (required for value indicator)
        if chars::is_blank(*self.chars.peek().unwrap_or(&'\0')) || self.is_at_end() {
            Ok(Token::new(TokenKind::Value, start_pos, 1))
        } else {
            // Part of a plain scalar
            self.scan_plain_scalar_from_colon(position, start_pos)
        }
    }

    /// Scan anchor
    fn scan_anchor(&mut self, position: &mut PositionTracker) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        self.consume_char(position); // &

        let name = self.scan_anchor_name(position)?;
        let length = position.current().byte_offset - start_pos.byte_offset;

        Ok(Token::new(
            TokenKind::Anchor(Cow::Borrowed(name)),
            start_pos,
            length,
        ))
    }

    /// Scan alias
    fn scan_alias(&mut self, position: &mut PositionTracker) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        self.consume_char(position); // *

        let name = self.scan_anchor_name(position)?;
        let length = position.current().byte_offset - start_pos.byte_offset;

        Ok(Token::new(
            TokenKind::Alias(Cow::Borrowed(name)),
            start_pos,
            length,
        ))
    }

    /// Scan anchor or alias name
    fn scan_anchor_name(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<&'input str, LexError> {
        let start_offset = self.current_offset;

        if !self
            .chars
            .peek()
            .map_or(false, |&ch| chars::is_anchor_char(ch))
        {
            return Err(LexError::new(
                LexErrorKind::InvalidAnchor("anchor name cannot be empty".to_string()),
                position.current(),
            ));
        }

        while let Some(&ch) = self.chars.peek() {
            if chars::is_anchor_char(ch) {
                self.consume_char(position);
            } else {
                break;
            }
        }

        let end_offset = self.current_offset;
        Ok(&self.input[start_offset..end_offset])
    }

    /// Scan tag
    fn scan_tag(&mut self, position: &mut PositionTracker) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        self.consume_char(position); // !

        // Handle different tag formats: !, !tag, !<tag>, !handle!tag
        let (handle, suffix) = if let Some(&'<') = self.chars.peek() {
            // Verbatim tag: !<tag>
            self.consume_char(position); // <
            let tag = self.scan_verbatim_tag(position)?;
            (None, Cow::Borrowed(tag))
        } else {
            // Named tag or shorthand
            self.scan_named_tag(position)?
        };

        let length = position.current().byte_offset - start_pos.byte_offset;

        Ok(Token::new(
            TokenKind::Tag { handle, suffix },
            start_pos,
            length,
        ))
    }

    /// Scan verbatim tag content
    fn scan_verbatim_tag(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<&'input str, LexError> {
        let start_offset = self.current_offset;

        while let Some(&ch) = self.chars.peek() {
            if ch == '>' {
                let end_offset = self.current_offset;
                self.consume_char(position); // >
                return Ok(&self.input[start_offset..end_offset]);
            } else if chars::is_tag_char(ch) {
                self.consume_char(position);
            } else {
                return Err(LexError::new(
                    LexErrorKind::InvalidTag("invalid character in verbatim tag".to_string()),
                    position.current(),
                ));
            }
        }

        Err(LexError::new(
            LexErrorKind::UnexpectedEndOfInput,
            position.current(),
        ))
    }

    /// Scan named tag
    fn scan_named_tag(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<(Option<Cow<'input, str>>, Cow<'input, str>), LexError> {
        let start_offset = self.current_offset;

        // Scan tag characters
        while let Some(&ch) = self.chars.peek() {
            if chars::is_tag_char(ch) {
                self.consume_char(position);
            } else {
                break;
            }
        }

        let tag_content = &self.input[start_offset..self.current_offset];

        // Check if this is a handle!suffix format
        if let Some(exclamation_pos) = tag_content.find('!') {
            let handle = &tag_content[..exclamation_pos + 1];
            let suffix = &tag_content[exclamation_pos + 1..];
            Ok((Some(Cow::Borrowed(handle)), Cow::Borrowed(suffix)))
        } else {
            // Simple tag or secondary tag
            if tag_content.is_empty() {
                Ok((None, Cow::Borrowed("!")))
            } else {
                Ok((None, Cow::Borrowed(tag_content)))
            }
        }
    }

    /// Scan directive
    fn scan_directive(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        self.consume_char(position); // %

        let name = self.scan_directive_name(position)?;

        match name {
            "YAML" => {
                self.skip_spaces(position);
                let (major, minor) = self.scan_yaml_version(position)?;
                let length = position.current().byte_offset - start_pos.byte_offset;

                Ok(Token::new(
                    TokenKind::YamlDirective { major, minor },
                    start_pos,
                    length,
                ))
            }
            "TAG" => {
                self.skip_spaces(position);
                let (handle, prefix) = self.scan_tag_directive(position)?;
                let length = position.current().byte_offset - start_pos.byte_offset;

                Ok(Token::new(
                    TokenKind::TagDirective {
                        handle: Cow::Borrowed(handle),
                        prefix: Cow::Borrowed(prefix),
                    },
                    start_pos,
                    length,
                ))
            }
            _ => {
                self.skip_spaces(position);
                let value = self.scan_directive_value(position)?;
                let length = position.current().byte_offset - start_pos.byte_offset;

                Ok(Token::new(
                    TokenKind::ReservedDirective {
                        name: Cow::Borrowed(name),
                        value: Cow::Borrowed(value),
                    },
                    start_pos,
                    length,
                ))
            }
        }
    }

    /// Scan directive name
    fn scan_directive_name(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<&'input str, LexError> {
        let start_offset = self.current_offset;

        while let Some(&ch) = self.chars.peek() {
            if chars::is_yaml_identifier(ch) {
                self.consume_char(position);
            } else {
                break;
            }
        }

        let end_offset = self.current_offset;
        if start_offset == end_offset {
            return Err(LexError::new(
                LexErrorKind::InvalidDirective("directive name cannot be empty".to_string()),
                position.current(),
            ));
        }

        Ok(&self.input[start_offset..end_offset])
    }

    /// Scan YAML version
    fn scan_yaml_version(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<(u32, u32), LexError> {
        let major = self.scan_version_number(position)?;

        if !self.check_char('.') {
            return Err(LexError::new(
                LexErrorKind::InvalidDirective("expected '.' in YAML version".to_string()),
                position.current(),
            ));
        }
        self.consume_char(position); // .

        let minor = self.scan_version_number(position)?;

        Ok((major, minor))
    }

    /// Scan version number
    fn scan_version_number(&mut self, position: &mut PositionTracker) -> Result<u32, LexError> {
        let start_offset = self.current_offset;

        while let Some(&ch) = self.chars.peek() {
            if ch.is_ascii_digit() {
                self.consume_char(position);
            } else {
                break;
            }
        }

        let end_offset = self.current_offset;
        if start_offset == end_offset {
            return Err(LexError::new(
                LexErrorKind::InvalidDirective("expected version number".to_string()),
                position.current(),
            ));
        }

        self.input[start_offset..end_offset]
            .parse()
            .map_err(|_| LexError::new(LexErrorKind::InvalidNumber, position.current()))
    }

    /// Scan TAG directive parameters
    fn scan_tag_directive(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<(&'input str, &'input str), LexError> {
        let handle = self.scan_tag_handle(position)?;
        self.skip_spaces(position);
        let prefix = self.scan_tag_prefix(position)?;
        Ok((handle, prefix))
    }

    /// Scan tag handle
    fn scan_tag_handle(&mut self, position: &mut PositionTracker) -> Result<&'input str, LexError> {
        let start_offset = self.current_offset;

        if !self.check_char('!') {
            return Err(LexError::new(
                LexErrorKind::InvalidTag("tag handle must start with '!'".to_string()),
                position.current(),
            ));
        }
        self.consume_char(position); // !

        while let Some(&ch) = self.chars.peek() {
            if chars::is_yaml_identifier(ch) {
                self.consume_char(position);
            } else {
                break;
            }
        }

        if self.check_char('!') {
            self.consume_char(position); // !
        }

        let end_offset = self.current_offset;
        Ok(&self.input[start_offset..end_offset])
    }

    /// Scan tag prefix
    fn scan_tag_prefix(&mut self, position: &mut PositionTracker) -> Result<&'input str, LexError> {
        let start_offset = self.current_offset;

        while let Some(&ch) = self.chars.peek() {
            if chars::is_tag_char(ch) {
                self.consume_char(position);
            } else {
                break;
            }
        }

        let end_offset = self.current_offset;
        Ok(&self.input[start_offset..end_offset])
    }

    /// Scan directive value
    fn scan_directive_value(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<&'input str, LexError> {
        let start_offset = self.current_offset;

        while let Some(&ch) = self.chars.peek() {
            if chars::is_break(ch) {
                break;
            }
            self.consume_char(position);
        }

        let end_offset = self.current_offset;
        Ok(&self.input[start_offset..end_offset])
    }

    /// Scan single-quoted string
    fn scan_single_quoted_string(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        self.consume_char(position); // '

        let mut result = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch == '\'' {
                self.consume_char(position);

                // Check for escaped quote ''
                if self.check_char('\'') {
                    self.consume_char(position);
                    result.push('\'');
                } else {
                    // End of string
                    let length = position.current().byte_offset - start_pos.byte_offset;
                    return Ok(Token::new(
                        TokenKind::Scalar {
                            value: if result.is_empty() {
                                Cow::Borrowed("")
                            } else {
                                Cow::Owned(result)
                            },
                            style: ScalarStyle::SingleQuoted,
                            tag: None,
                        },
                        start_pos,
                        length,
                    ));
                }
            } else {
                result.push(ch);
                self.consume_char(position);
            }
        }

        Err(LexError::new(LexErrorKind::UnterminatedString, start_pos))
    }

    /// Scan double-quoted string
    fn scan_double_quoted_string(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        self.consume_char(position); // "

        let start_offset = self.current_offset;
        let mut has_escapes = false;

        while let Some(&ch) = self.chars.peek() {
            if ch == '"' {
                let end_offset = self.current_offset;
                self.consume_char(position); // "

                let raw_value = &self.input[start_offset..end_offset];
                let value = if has_escapes {
                    match UnicodeProcessor::process_escapes(raw_value) {
                        Ok(processed) => processed,
                        Err(e) => {
                            return Err(LexError::new(
                                LexErrorKind::InvalidEscape(format!("escape error: {}", e)),
                                position.current(),
                            ));
                        }
                    }
                } else {
                    Cow::Borrowed(raw_value)
                };

                let length = position.current().byte_offset - start_pos.byte_offset;
                return Ok(Token::new(
                    TokenKind::Scalar {
                        value,
                        style: ScalarStyle::DoubleQuoted,
                        tag: None,
                    },
                    start_pos,
                    length,
                ));
            } else if ch == '\\' {
                has_escapes = true;
                self.consume_char(position); // \
                if let Some(&_escaped) = self.chars.peek() {
                    self.consume_char(position); // escaped character
                }
            } else {
                self.consume_char(position);
            }
        }

        Err(LexError::new(LexErrorKind::UnterminatedString, start_pos))
    }

    /// Scan literal block scalar
    fn scan_literal_scalar(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        self.consume_char(position); // |

        // TODO: Implement full literal scalar parsing with strip/keep/clip indicators
        let length = position.current().byte_offset - start_pos.byte_offset;
        Ok(Token::new(
            TokenKind::Scalar {
                value: Cow::Borrowed(""),
                style: ScalarStyle::Literal,
                tag: None,
            },
            start_pos,
            length,
        ))
    }

    /// Scan folded block scalar
    fn scan_folded_scalar(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        self.consume_char(position); // >

        // TODO: Implement full folded scalar parsing with strip/keep/clip indicators
        let length = position.current().byte_offset - start_pos.byte_offset;
        Ok(Token::new(
            TokenKind::Scalar {
                value: Cow::Borrowed(""),
                style: ScalarStyle::Folded,
                tag: None,
            },
            start_pos,
            length,
        ))
    }

    /// Scan line break
    fn scan_line_break(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();

        if self.check_char('\r') {
            self.consume_char(position); // \r
            if self.check_char('\n') {
                self.consume_char(position); // \n
            }
        } else {
            self.consume_char(position); // \n
        }

        Ok(Token::new(TokenKind::LineBreak, start_pos, 1))
    }

    /// Scan plain scalar
    fn scan_plain_scalar(
        &mut self,
        position: &mut PositionTracker,
    ) -> Result<Token<'input>, LexError> {
        let start_pos = position.current();
        let start_offset = self.current_offset;

        // Check if first character can start a plain scalar
        if let Some(&ch) = self.chars.peek() {
            if !chars::can_start_plain_scalar(ch) {
                return Err(LexError::new(
                    LexErrorKind::UnexpectedCharacter(format!("unexpected character '{}'", ch)),
                    start_pos,
                ));
            }
        }

        // Scan plain scalar content
        while let Some(&ch) = self.chars.peek() {
            if chars::can_continue_plain_scalar(ch) {
                self.consume_char(position);
            } else {
                break;
            }
        }

        let end_offset = self.current_offset;
        if start_offset == end_offset {
            return Err(LexError::new(
                LexErrorKind::UnexpectedCharacter("empty plain scalar".to_string()),
                start_pos,
            ));
        }

        let value = &self.input[start_offset..end_offset];
        let length = end_offset - start_offset;

        Ok(Token::new(
            TokenKind::Scalar {
                value: Cow::Borrowed(value),
                style: ScalarStyle::Plain,
                tag: None,
            },
            start_pos,
            length,
        ))
    }

    /// Scan plain scalar starting with dash
    fn scan_plain_scalar_from_dash(
        &mut self,
        position: &mut PositionTracker,
        start_pos: Position,
    ) -> Result<Token<'input>, LexError> {
        let start_offset = start_pos.byte_offset;

        // Continue scanning the rest of the plain scalar
        while let Some(&ch) = self.chars.peek() {
            if chars::can_continue_plain_scalar(ch) {
                self.consume_char(position);
            } else {
                break;
            }
        }

        let end_offset = self.current_offset;
        let value = &self.input[start_offset..end_offset];
        let length = end_offset - start_offset;

        Ok(Token::new(
            TokenKind::Scalar {
                value: Cow::Borrowed(value),
                style: ScalarStyle::Plain,
                tag: None,
            },
            start_pos,
            length,
        ))
    }

    /// Scan plain scalar starting with colon
    fn scan_plain_scalar_from_colon(
        &mut self,
        position: &mut PositionTracker,
        start_pos: Position,
    ) -> Result<Token<'input>, LexError> {
        let start_offset = start_pos.byte_offset;

        // Continue scanning the rest of the plain scalar
        while let Some(&ch) = self.chars.peek() {
            if chars::can_continue_plain_scalar(ch) {
                self.consume_char(position);
            } else {
                break;
            }
        }

        let end_offset = self.current_offset;
        let value = &self.input[start_offset..end_offset];
        let length = end_offset - start_offset;

        Ok(Token::new(
            TokenKind::Scalar {
                value: Cow::Borrowed(value),
                style: ScalarStyle::Plain,
                tag: None,
            },
            start_pos,
            length,
        ))
    }

    /// Consume one character and update position
    #[inline]
    fn consume_char(&mut self, position: &mut PositionTracker) {
        if let Some(ch) = self.chars.next() {
            position.advance_char(ch);
            self.current_offset += ch.len_utf8();
        }
    }

    /// Check if the next character matches
    #[inline]
    fn check_char(&mut self, expected: char) -> bool {
        self.chars.peek() == Some(&expected)
    }

    /// Check if the next characters match a sequence
    fn check_sequence(&self, expected: &str) -> bool {
        let remaining_input = &self.input[self.current_offset..];
        remaining_input.starts_with(expected)
    }

    /// Skip space characters
    fn skip_spaces(&mut self, position: &mut PositionTracker) {
        while let Some(&ch) = self.chars.peek() {
            if ch == ' ' {
                self.consume_char(position);
            } else {
                break;
            }
        }
    }
}
