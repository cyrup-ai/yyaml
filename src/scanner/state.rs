//! Scanner state management with zero-allocation design
//!
//! This module provides efficient state tracking for the YAML scanner including
//! position tracking, buffer management, and flow context handling.

use crate::error::{Marker, ScanError};
use crate::scanner::token::Token;
use std::collections::VecDeque;

/// Scanner configuration for customizable behavior
#[derive(Debug, Clone)]
pub struct ScannerConfig {
    /// Maximum recursion depth for nested structures
    pub max_depth: usize,
    /// Initial buffer capacity for performance
    pub initial_buffer_capacity: usize,
    /// Maximum anchor/alias name length
    pub max_anchor_length: usize,
    /// Enable strict YAML 1.2 compliance
    pub strict_yaml12: bool,
    /// Allow duplicate anchors (non-standard)
    pub allow_duplicate_anchors: bool,
}

impl Default for ScannerConfig {
    #[inline]
    fn default() -> Self {
        Self {
            max_depth: 1024,
            initial_buffer_capacity: 64,
            max_anchor_length: 1024,
            strict_yaml12: true,
            allow_duplicate_anchors: false,
        }
    }
}

/// Scanner state with efficient buffer and position management
pub struct ScannerState<T: Iterator<Item = char>> {
    /// Character source iterator
    source: T,
    /// Lookahead buffer for peeking
    buffer: VecDeque<char>,
    /// Current position marker
    mark: Marker,
    /// Source exhausted flag
    done: bool,
    /// Cached token for peeking
    cached_token: Option<Token>,
    /// Stream start produced flag
    stream_start_produced: bool,
    /// Stream end produced flag
    stream_end_produced: bool,
    /// Flow nesting level for context tracking
    flow_level: usize,
    /// Current indentation level
    indent: i32,
    /// Indentation stack for block context
    indent_stack: Vec<i32>,
    /// Simple key allowed flag
    simple_key_allowed: bool,
}

impl<T: Iterator<Item = char>> ScannerState<T> {
    /// Create new scanner state with optimized defaults
    #[inline]
    pub fn new(source: T) -> Self {
        Self {
            source,
            buffer: VecDeque::with_capacity(64),
            mark: Marker::new(),
            done: false,
            cached_token: None,
            stream_start_produced: false,
            stream_end_produced: false,
            flow_level: 0,
            indent: -1,
            indent_stack: Vec::with_capacity(16),
            simple_key_allowed: true,
        }
    }

    /// Get current position marker
    #[inline]
    pub fn mark(&self) -> Marker {
        self.mark
    }

    /// Check if source is exhausted
    #[inline]
    pub fn is_done(&self) -> bool {
        self.done && self.buffer.is_empty()
    }

    /// Check if stream start was produced
    #[inline]
    pub fn stream_started(&self) -> bool {
        self.stream_start_produced
    }

    /// Check if stream end was produced
    #[inline]
    pub fn stream_ended(&self) -> bool {
        self.stream_end_produced
    }

    /// Mark stream as started
    #[inline]
    pub fn mark_stream_started(&mut self) {
        self.stream_start_produced = true;
    }

    /// Mark stream as ended
    #[inline]
    pub fn mark_stream_ended(&mut self) {
        self.stream_end_produced = true;
    }

    /// Check if we're in flow context
    #[inline]
    pub fn in_flow_context(&self) -> bool {
        self.flow_level > 0
    }

    /// Enter flow context (for [ ] or { })
    #[inline]
    pub fn enter_flow_context(&mut self) {
        self.flow_level += 1;
        self.simple_key_allowed = true;
    }

    /// Exit flow context with validation
    #[inline]
    pub fn exit_flow_context(&mut self) -> Result<(), ScanError> {
        if self.flow_level == 0 {
            return Err(ScanError::new(self.mark, "unexpected flow collection end"));
        }
        self.flow_level -= 1;
        Ok(())
    }

    /// Get current flow level
    #[inline]
    pub fn flow_level(&self) -> usize {
        self.flow_level
    }

    /// Check if simple key is allowed
    #[inline]
    pub fn simple_key_allowed(&self) -> bool {
        self.simple_key_allowed
    }

    /// Set simple key allowed flag
    #[inline]
    pub fn set_simple_key_allowed(&mut self, allowed: bool) {
        self.simple_key_allowed = allowed;
    }

    /// Get current indentation
    #[inline]
    pub fn indent(&self) -> i32 {
        self.indent
    }

    /// Push indentation level
    #[inline]
    pub fn push_indent(&mut self, indent: i32) {
        self.indent_stack.push(self.indent);
        self.indent = indent;
    }

    /// Pop indentation level
    #[inline]
    pub fn pop_indent(&mut self) -> Option<i32> {
        if let Some(prev) = self.indent_stack.pop() {
            let current = self.indent;
            self.indent = prev;
            Some(current)
        } else {
            None
        }
    }

    /// Check if has cached token
    #[inline]
    pub fn has_cached_token(&self) -> bool {
        self.cached_token.is_some()
    }

    /// Cache a token for peeking
    #[inline]
    pub fn cache_token(&mut self, token: Token) {
        self.cached_token = Some(token);
    }

    /// Peek at cached token
    #[inline]
    pub fn peek_cached_token(&self) -> Option<&Token> {
        self.cached_token.as_ref()
    }

    /// Take cached token
    #[inline]
    pub fn take_cached_token(&mut self) -> Option<Token> {
        self.cached_token.take()
    }

    /// Clear cached token
    #[inline]
    pub fn clear_cached_token(&mut self) {
        self.cached_token = None;
    }

    /// Fill buffer to ensure at least n characters
    #[inline]
    pub fn ensure_buffer(&mut self, n: usize) {
        while self.buffer.len() < n && !self.done {
            if let Some(ch) = self.source.next() {
                self.buffer.push_back(ch);
            } else {
                self.done = true;
            }
        }
    }

    /// Peek at next character without consuming
    #[inline]
    pub fn peek_char(&mut self) -> Result<char, ScanError> {
        self.ensure_buffer(1);
        self.buffer
            .front()
            .copied()
            .ok_or_else(|| ScanError::new(self.mark, "unexpected end of input"))
    }

    /// Peek at character at offset without consuming
    #[inline]
    pub fn peek_char_at(&mut self, offset: usize) -> Option<char> {
        self.ensure_buffer(offset + 1);
        self.buffer.get(offset).copied()
    }

    /// Check if next characters match a pattern
    #[inline]
    pub fn check_chars(&mut self, pattern: &[char]) -> bool {
        self.ensure_buffer(pattern.len());
        if self.buffer.len() < pattern.len() {
            return false;
        }
        self.buffer.iter().take(pattern.len()).eq(pattern.iter())
    }

    /// Consume next character and update position
    #[inline]
    pub fn consume_char(&mut self) -> Result<char, ScanError> {
        if let Some(ch) = self.buffer.pop_front() {
            self.mark.index += 1;
            if ch == '\n' {
                self.mark.line += 1;
                self.mark.col = 0;
            } else {
                self.mark.col += 1;
            }
            Ok(ch)
        } else {
            Err(ScanError::new(self.mark, "unexpected end of input"))
        }
    }

    /// Consume multiple characters
    #[inline]
    pub fn consume_chars(&mut self, n: usize) -> Result<(), ScanError> {
        for _ in 0..n {
            self.consume_char()?;
        }
        Ok(())
    }

    /// Check for document start marker (---)
    #[inline]
    pub fn check_document_start(&mut self) -> Result<bool, ScanError> {
        Ok(self.check_chars(&['-', '-', '-']) && self.check_boundary_after(3))
    }

    /// Check for document end marker (...)
    #[inline]
    pub fn check_document_end(&mut self) -> Result<bool, ScanError> {
        Ok(self.check_chars(&['.', '.', '.']) && self.check_boundary_after(3))
    }

    /// Check for block entry (- followed by space/newline)
    #[inline]
    pub fn check_block_entry(&mut self) -> Result<bool, ScanError> {
        self.ensure_buffer(2);
        if self.buffer.front() == Some(&'-') {
            match self.buffer.get(1) {
                Some(&' ') | Some(&'\t') | Some(&'\n') | Some(&'\r') => Ok(true),
                None => Ok(true), // EOF after -
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    /// Check if character at offset is a boundary (space, newline, EOF, etc)
    #[inline]
    fn check_boundary_after(&mut self, offset: usize) -> bool {
        match self.peek_char_at(offset) {
            Some(ch) => matches!(ch, ' ' | '\t' | '\n' | '\r' | ',' | ']' | '}'),
            None => true, // EOF is a boundary
        }
    }

    /// Peek ahead multiple characters as slice
    #[inline]
    pub fn peek_slice(&mut self, n: usize) -> Vec<char> {
        self.ensure_buffer(n);
        self.buffer.iter().take(n).copied().collect()
    }

    /// Check if at beginning of line
    #[inline]
    pub fn at_line_start(&self) -> bool {
        self.mark.col == 0
    }

    /// Get current column position
    #[inline]
    pub fn column(&self) -> usize {
        self.mark.col
    }
}

/// Marker extensions for efficient position tracking
impl Marker {
    /// Create default marker at start of stream
    #[inline]
    pub fn new() -> Self {
        Self {
            index: 0,
            line: 1,
            col: 0,
        }
    }

    /// Create marker at specific position
    #[inline]
    pub fn at(index: usize, line: usize, col: usize) -> Self {
        Self { index, line, col }
    }
}
