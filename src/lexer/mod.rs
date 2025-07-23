//! Comprehensive lexical analysis module for YAML 1.2 tokenization
//!
//! This module provides a complete tokenization system with zero-allocation design
//! and precise source location tracking for production-quality YAML parsing.

pub mod position;
pub mod scanner;
pub mod tokens;
pub mod unicode;

pub use position::*;
pub use scanner::*;
pub use tokens::*;

/// High-performance lexer interface for YAML tokenization
#[derive(Debug, Clone)]
pub struct YamlLexer<'input> {
    scanner: Scanner<'input>,
    position: PositionTracker,
}

impl<'input> YamlLexer<'input> {
    /// Create a new lexer for the given input
    #[inline]
    pub fn new(input: &'input str) -> Self {
        Self {
            scanner: Scanner::new(input),
            position: PositionTracker::new(),
        }
    }

    /// Get the next token from the input
    #[inline]
    pub fn next_token(&mut self) -> Result<Token<'input>, LexError> {
        self.scanner.scan_token(&mut self.position)
    }

    /// Peek at the next token without consuming it
    #[inline]
    pub fn peek_token(&mut self) -> Result<Token<'input>, LexError> {
        self.scanner.peek_token(&mut self.position)
    }

    /// Get the current position in the input
    #[inline]
    pub fn position(&self) -> Position {
        self.position.current()
    }

    /// Check if we've reached the end of input
    #[inline]
    pub fn is_at_end(&mut self) -> bool {
        self.scanner.is_at_end()
    }

    /// Create an iterator over all tokens
    pub fn tokens(self) -> TokenIterator<'input> {
        TokenIterator::new(self)
    }
}

/// Iterator over tokens produced by the lexer
pub struct TokenIterator<'input> {
    lexer: YamlLexer<'input>,
    finished: bool,
}

impl<'input> TokenIterator<'input> {
    #[inline]
    fn new(lexer: YamlLexer<'input>) -> Self {
        Self {
            lexer,
            finished: false,
        }
    }
}

impl<'input> Iterator for TokenIterator<'input> {
    type Item = Result<Token<'input>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        match self.lexer.next_token() {
            Ok(token) => {
                if matches!(token.kind, TokenKind::StreamEnd) {
                    self.finished = true;
                }
                Some(Ok(token))
            }
            Err(e) => {
                self.finished = true;
                Some(Err(e))
            }
        }
    }
}

/// Lexical error type with precise source location
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub position: Position,
}

impl LexError {
    #[inline]
    pub fn new(kind: LexErrorKind, position: Position) -> Self {
        Self { kind, position }
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {}:{}",
            self.kind, self.position.line, self.position.column
        )
    }
}

impl std::error::Error for LexError {}

/// Types of lexical errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    UnexpectedCharacter(String),
    InvalidEscape(String),
    UnterminatedString,
    InvalidUnicode,
    InvalidUnicodeEscape,
    InvalidNumber,
    InvalidTag(String),
    InvalidAnchor(String),
    InvalidAlias(String),
    InvalidDirective(String),
    InvalidIndentation(String),
    UnexpectedEndOfInput,
    EmptyScalar,
}

impl std::fmt::Display for LexErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexErrorKind::UnexpectedCharacter(msg) => write!(f, "unexpected character: {msg}"),
            LexErrorKind::InvalidEscape(msg) => write!(f, "invalid escape sequence: {msg}"),
            LexErrorKind::UnterminatedString => write!(f, "unterminated string"),
            LexErrorKind::InvalidUnicode => write!(f, "invalid unicode sequence"),
            LexErrorKind::InvalidUnicodeEscape => write!(f, "invalid unicode escape"),
            LexErrorKind::InvalidNumber => write!(f, "invalid number format"),
            LexErrorKind::InvalidTag(msg) => write!(f, "invalid tag: {msg}"),
            LexErrorKind::InvalidAnchor(msg) => write!(f, "invalid anchor: {msg}"),
            LexErrorKind::InvalidAlias(msg) => write!(f, "invalid alias: {msg}"),
            LexErrorKind::InvalidDirective(msg) => write!(f, "invalid directive: {msg}"),
            LexErrorKind::InvalidIndentation(msg) => write!(f, "invalid indentation: {msg}"),
            LexErrorKind::UnexpectedEndOfInput => write!(f, "unexpected end of input"),
            LexErrorKind::EmptyScalar => write!(f, "empty scalar"),
        }
    }
}
