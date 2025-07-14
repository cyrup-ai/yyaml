//! Production-quality YAML parser with complete grammar support
//!
//! This module provides comprehensive YAML 1.2 parsing with zero infinite recursion,
//! proper AST construction, and robust error handling.

// Core parsing components
pub mod ast;
pub mod blocks;
pub mod documents;
pub mod flows;
pub mod grammar;
pub mod indentation;
pub mod node_parser;
pub mod scalars;
pub mod state_machine;

// Legacy modules for backward compatibility
pub mod block;
pub mod document;
pub mod flow;
pub mod loader;

pub use ast::*;

use grammar::ParseContext;

// Export core parsing functionality
pub use indentation::{
    IndentationResult, calculate_block_entry_indent, validate_block_mapping_indentation,
    validate_block_sequence_indentation,
};
pub use loader::YamlLoader;
pub use node_parser::parse_node;
pub use state_machine::{State, execute_state_machine};

use crate::error::{Marker, ScanError};
use crate::events::{Event, MarkedEventReceiver, TokenType};
use crate::lexer::Token;
use crate::lexer::{LexError, Position, TokenKind, YamlLexer};
use crate::scanner::Scanner;
use std::collections::{HashMap, VecDeque};

/// Zero-allocation parsing context that eliminates multiple mutable borrows
/// while maintaining blazing-fast performance and elegant ergonomics
#[derive(Debug)]
pub struct ParsingContext<'a, 'input> {
    pub lexer: &'a mut YamlLexer<'input>,
    pub token_buffer: &'a mut VecDeque<Token<'input>>,
    pub recursion_depth: &'a mut usize,
    pub parse_state: &'a mut ParseState,
}

impl<'a, 'input> ParsingContext<'a, 'input> {
    /// Create a new parsing context from parser state components
    #[inline]
    pub fn new(
        lexer: &'a mut YamlLexer<'input>,
        token_buffer: &'a mut VecDeque<Token<'input>>,
        recursion_depth: &'a mut usize,
        parse_state: &'a mut ParseState,
    ) -> Self {
        Self {
            lexer,
            token_buffer,
            recursion_depth,
            parse_state,
        }
    }

    /// Peek at the next token without consuming it (zero-allocation)
    #[inline]
    pub fn peek_token(&mut self) -> Result<Option<&Token<'input>>, ParseError> {
        if self.token_buffer.is_empty() {
            match self.lexer.next_token() {
                Ok(token) => {
                    if matches!(token.kind, TokenKind::StreamEnd) {
                        return Ok(None);
                    }
                    self.token_buffer.push_back(token);
                }
                Err(e) => return Err(ParseError::from_lex_error(e)),
            }
        }
        Ok(self.token_buffer.front())
    }

    /// Consume the next token (zero-allocation)
    #[inline]
    pub fn consume_token(&mut self) -> Result<Token<'input>, ParseError> {
        if let Some(token) = self.token_buffer.pop_front() {
            return Ok(token);
        }

        match self.lexer.next_token() {
            Ok(token) => {
                if matches!(token.kind, TokenKind::StreamEnd) {
                    Err(ParseError::new(
                        ParseErrorKind::UnexpectedEndOfInput,
                        token.position,
                        "unexpected end of input",
                    ))
                } else {
                    Ok(token)
                }
            }
            Err(e) => Err(ParseError::from_lex_error(e)),
        }
    }

    /// Get current parsing position
    #[inline]
    pub fn current_position(&self) -> Position {
        self.token_buffer
            .front()
            .map(|t| t.position)
            .unwrap_or_else(|| self.lexer.position())
    }

    /// Check recursion depth to prevent stack overflow
    #[inline]
    pub fn check_recursion_depth(&self) -> Result<(), ParseError> {
        const MAX_RECURSION_DEPTH: usize = 1000;
        if *self.recursion_depth >= MAX_RECURSION_DEPTH {
            return Err(ParseError::new(
                ParseErrorKind::RecursionLimitExceeded,
                self.current_position(),
                format!("recursion depth exceeded: {}", MAX_RECURSION_DEPTH),
            ));
        }
        Ok(())
    }

    /// Skip insignificant tokens (whitespace, comments) with blazing-fast performance
    #[inline]
    pub fn skip_insignificant_tokens(&mut self) -> Result<(), ParseError> {
        loop {
            match self.peek_token()? {
                Some(token)
                    if matches!(token.kind, TokenKind::Whitespace(_) | TokenKind::Comment(_)) =>
                {
                    self.consume_token()?; // Skip insignificant token
                }
                _ => break,
            }
        }
        Ok(())
    }

    /// Check if at end of input
    #[inline]
    pub fn is_at_end(&mut self) -> Result<bool, ParseError> {
        self.skip_insignificant_tokens()?;
        Ok(self.peek_token()?.is_none())
    }
}

// RecursionGuard removed: Manual depth tracking provides zero-allocation performance
// RAII pattern would add overhead without functional benefit in this high-performance context

/// High-performance YAML parser with complete grammar support
#[derive(Debug)]
pub struct YamlParser<'input> {
    lexer: YamlLexer<'input>,
    token_buffer: VecDeque<Token<'input>>,
    _current_document: Option<Document<'input>>,
    parse_state: ParseState,
    recursion_depth: usize,
    max_recursion_depth: usize,
}

/// Maintain compatibility with existing Parser struct
pub struct Parser<T: Iterator<Item = char>> {
    pub scanner: Scanner<T>,
    pub states: Vec<State>,
    pub state: State,
    pub current: Option<(Event, Marker)>,
    pub first_mapping_key: Option<(Event, Marker)>,
    pub anchors: HashMap<String, usize>,
    pub anchor_id: usize,
    pub indents: Vec<usize>,
}

/// Parser state for robust state management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseState {
    StreamStart,
    DocumentStart,
    DocumentContent,
    DocumentEnd,
    StreamEnd,
    Error,
}

/// Parser configuration for customizable behavior
#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub max_recursion_depth: usize,
    pub allow_multiple_documents: bool,
    pub strict_mode: bool,
    pub preserve_source_locations: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            max_recursion_depth: 100,
            allow_multiple_documents: true,
            strict_mode: false,
            preserve_source_locations: true,
        }
    }
}

impl<T: Iterator<Item = char>> Parser<T> {
    pub fn new(src: T) -> Self {
        Parser {
            scanner: Scanner::new(src),
            states: Vec::new(),
            state: State::StreamStart,
            current: None,
            first_mapping_key: None,
            anchors: HashMap::new(),
            anchor_id: 1,
            indents: Vec::new(),
        }
    }

    pub fn push_indent(&mut self, indent: usize) {
        self.indents.push(indent);
    }

    pub fn pop_indent(&mut self) {
        self.indents.pop();
    }

    pub fn pop_state(&mut self) {
        if let Some(state) = self.states.pop() {
            self.state = state;
        }
    }

    pub fn push_state(&mut self, st: State) {
        self.states.push(self.state);
        self.state = st;
    }

    pub fn parse(&mut self) -> Result<(Event, Marker), ScanError> {
        if self.state == State::End {
            return Ok((Event::StreamEnd, self.scanner.mark()));
        }
        let (ev, mk) = self.state_machine()?;
        Ok((ev, mk))
    }

    fn state_machine(&mut self) -> Result<(Event, Marker), ScanError> {
        execute_state_machine(self)
    }

    pub fn next(&mut self) -> Result<(Event, Marker), ScanError> {
        match self.current.take() {
            Some(x) => Ok(x),
            None => self.parse(),
        }
    }

    pub fn load<R: MarkedEventReceiver>(
        &mut self,
        recv: &mut R,
        multi: bool,
    ) -> Result<(), ScanError> {
        loader::load(self, recv, multi)
    }

    pub fn register_anchor(&mut self, name: String) -> usize {
        let new_id = self.anchor_id;
        self.anchor_id += 1;
        self.anchors.insert(name, new_id);
        new_id
    }

    pub fn stream_start(&mut self) -> Result<(Event, Marker), ScanError> {
        let t = self.scanner.peek_token()?;
        match t.1 {
            TokenType::StreamStart(_) => {
                self.state = State::ImplicitDocumentStart;
                let tok = self.scanner.fetch_token();
                Ok((Event::StreamStart, tok.0))
            }
            _ => Err(ScanError::new(t.0, "did not find expected <stream-start>")),
        }
    }

    pub fn parse_node(
        &mut self,
        block: bool,
        indentless_seq: bool,
    ) -> Result<(Event, Marker), ScanError> {
        parse_node(self, block, indentless_seq)
    }
}

impl<'input> YamlParser<'input> {
    /// Create a new parser with default configuration
    #[inline]
    pub fn new(input: &'input str) -> Self {
        Self::with_config(input, ParserConfig::default())
    }

    /// Create a new parser with custom configuration
    #[inline]
    pub fn with_config(input: &'input str, config: ParserConfig) -> Self {
        Self {
            lexer: YamlLexer::new(input),
            token_buffer: VecDeque::new(),
            _current_document: None,
            parse_state: ParseState::StreamStart,
            recursion_depth: 0,
            max_recursion_depth: config.max_recursion_depth,
        }
    }

    /// Parse a complete YAML stream
    pub fn parse_stream(&mut self) -> Result<Stream<'input>, ParseError> {
        let mut documents = Vec::new();

        while !self.is_at_end()? {
            match self.parse_state {
                ParseState::StreamStart => {
                    self.expect_stream_start()?;
                    self.parse_state = ParseState::DocumentStart;
                }
                ParseState::DocumentStart => {
                    if let Some(doc) = self.parse_document()? {
                        documents.push(doc);
                    }
                }
                ParseState::DocumentEnd => {
                    self.parse_state = ParseState::DocumentStart;
                }
                ParseState::StreamEnd => break,
                ParseState::Error => {
                    return Err(ParseError::new(
                        ParseErrorKind::InternalError,
                        self.current_position(),
                        "parser in error state",
                    ));
                }
                _ => {
                    return Err(ParseError::new(
                        ParseErrorKind::UnexpectedState,
                        self.current_position(),
                        format!("unexpected parse state: {:?}", self.parse_state),
                    ));
                }
            }
        }

        Ok(Stream::new(documents))
    }

    /// Parse a single document
    pub fn parse_document(&mut self) -> Result<Option<Document<'input>>, ParseError> {
        self.check_recursion_depth()?;

        let start_pos = self.current_position();

        // Skip any leading comments or whitespace
        self.skip_insignificant_tokens()?;

        if self.is_at_end()? {
            return Ok(None);
        }

        // Check for document start marker
        let has_explicit_start = if let Some(token) = self.peek_token()? {
            matches!(token.kind, TokenKind::DocumentStart)
        } else {
            false
        };

        if has_explicit_start {
            self.consume_token()?; // consume ---
        }

        // Parse document content
        let content = if let Some(token) = self.peek_token()? {
            match token.kind {
                TokenKind::DocumentEnd | TokenKind::StreamEnd => {
                    // Empty document
                    None
                }
                _ => Some(self.parse_node()?),
            }
        } else {
            None
        };

        // Check for document end marker
        let has_explicit_end = if let Some(token) = self.peek_token()? {
            if matches!(token.kind, TokenKind::DocumentEnd) {
                self.consume_token()?; // consume ...
                true
            } else {
                false
            }
        } else {
            false
        };

        self.parse_state = ParseState::DocumentEnd;

        Ok(Some(Document::new(
            content,
            has_explicit_start,
            has_explicit_end,
            start_pos,
        )))
    }

    /// Parse a YAML node (recursive entry point)
    #[inline]
    pub fn parse_node(&mut self) -> Result<Node<'input>, ParseError> {
        self.check_recursion_depth()?;
        self.recursion_depth += 1;

        let result = self.parse_node_impl();

        self.recursion_depth -= 1;
        result
    }

    /// Internal node parsing implementation - delegates to specialized parsers
    fn parse_node_impl(&mut self) -> Result<Node<'input>, ParseError> {
        // Check recursion depth before parsing
        if self.recursion_depth >= self.max_recursion_depth {
            return Err(ParseError::new(
                ParseErrorKind::RecursionLimitExceeded,
                Position::start(),
                format!(
                    "recursion depth exceeded maximum of {}",
                    self.max_recursion_depth
                ),
            ));
        }
        
        // Increment recursion depth
        self.recursion_depth += 1;
        
        // Parse node and ensure decrement happens regardless of outcome
        let result = self.parse_node_impl_inner();
        
        // Decrement recursion depth
        self.recursion_depth -= 1;
        
        result
    }
    
    fn parse_node_impl_inner(&mut self) -> Result<Node<'input>, ParseError> {
        // Create parsing context from parser state components
        let mut context = ParsingContext::new(
            &mut self.lexer,
            &mut self.token_buffer,
            &mut self.recursion_depth,
            &mut self.parse_state,
        );

        Self::parse_node_with_context_static(&mut context)
    }

    /// Static version of parse_node_with_context for use without self borrowing
    fn parse_node_with_context_static(
        context: &mut ParsingContext<'_, 'input>,
    ) -> Result<Node<'input>, ParseError> {
        // Check recursion depth before parsing
        context.check_recursion_depth()?;
        
        // Increment recursion depth
        *context.recursion_depth += 1;
        
        // Parse node and ensure decrement happens regardless of outcome
        let result = Self::parse_node_with_context_static_inner(context);
        
        // Decrement recursion depth
        *context.recursion_depth -= 1;
        
        result
    }
    
    fn parse_node_with_context_static_inner(
        context: &mut ParsingContext<'_, 'input>,
    ) -> Result<Node<'input>, ParseError> {
        // Skip whitespace and comments
        context.skip_insignificant_tokens()?;

        // Get current position before consuming token
        let current_pos = context.current_position();

        // Consume the token to avoid borrowing conflicts
        let token = context.consume_token().map_err(|_| {
            ParseError::new(
                ParseErrorKind::UnexpectedEndOfInput,
                current_pos,
                "expected node content",
            )
        })?;

        match token.kind {
            // Scalar values - delegate to ScalarParser
            TokenKind::Scalar { .. } => {
                scalars::ScalarParser::parse_scalar(token, &ParseContext::Document)
            }

            // Flow collections
            TokenKind::FlowSequenceStart => {
                flows::FlowParser::parse_sequence(context, token, |ctx| {
                    Self::parse_node_with_context_static(ctx)
                })
            }

            TokenKind::FlowMappingStart => {
                flows::FlowParser::parse_mapping(context, token, |ctx| {
                    Self::parse_node_with_context_static(ctx)
                })
            }

            // Block sequences
            TokenKind::BlockEntry => {
                blocks::BlockParser::parse_sequence_with_context(context, token, 0, |ctx| {
                    Self::parse_node_with_context_static(ctx)
                })
            }

            // Anchors
            TokenKind::Anchor(name) => {
                let position = token.position;
                let anchored_node = Self::parse_node_with_context_static(context)?;
                Ok(Node::Anchor(ast::AnchorNode::new(
                    name,
                    Box::new(anchored_node),
                    position,
                )))
            }

            // Aliases
            TokenKind::Alias(name) => Ok(Node::Alias(ast::AliasNode::new(name, token.position))),

            // Tags
            TokenKind::Tag { handle, suffix } => {
                let position = token.position;
                let tagged_node = Self::parse_node_with_context_static(context)?;
                Ok(Node::Tagged(ast::TaggedNode::new(
                    handle,
                    suffix,
                    Box::new(tagged_node),
                    position,
                )))
            }

            // Plain scalars that might be mapping keys
            _ => {
                // Check if this could be a mapping key
                let token_position = token.position;
                let is_mapping_key =
                    blocks::BlockParser::is_potential_mapping_key_with_context(context, token_position, 0)?;

                if is_mapping_key {
                    blocks::BlockParser::parse_mapping_with_context(
                        context,
                        token,
                        0,
                        |ctx| Self::parse_node_with_context_static(ctx),
                    )
                } else {
                    Err(ParseError::new(
                        ParseErrorKind::UnexpectedToken,
                        token.position,
                        format!("unexpected token: {:?}", token.kind),
                    ))
                }
            }
        }
    }

    // Removed parse_node_with_context and parse_node_with_context_inner methods:
    // These duplicated functionality already provided by parse_node_with_context_static
    // Eliminating code bloat for zero-allocation, blazing-fast performance

    /// Utility methods
    #[inline]
    fn peek_token(&mut self) -> Result<Option<&Token<'input>>, ParseError> {
        if self.token_buffer.is_empty() {
            match self.lexer.next_token() {
                Ok(token) => {
                    if matches!(token.kind, TokenKind::StreamEnd) {
                        return Ok(None);
                    }
                    self.token_buffer.push_back(token);
                }
                Err(e) => return Err(ParseError::from_lex_error(e)),
            }
        }
        Ok(self.token_buffer.front())
    }

    #[inline]
    fn consume_token(&mut self) -> Result<Token<'input>, ParseError> {
        if let Some(token) = self.token_buffer.pop_front() {
            Ok(token)
        } else {
            match self.lexer.next_token() {
                Ok(token) => Ok(token),
                Err(e) => Err(ParseError::from_lex_error(e)),
            }
        }
    }

    // Removed expect_token method: Unused utility adding binary bloat
    // Token validation handled inline for blazing-fast performance

    fn expect_stream_start(&mut self) -> Result<(), ParseError> {
        if let Some(token) = self.peek_token()? {
            if matches!(token.kind, TokenKind::StreamStart) {
                self.consume_token()?;
                Ok(())
            } else {
                // Implicit stream start
                Ok(())
            }
        } else {
            Ok(())
        }
    }

    #[inline]
    fn skip_insignificant_tokens(&mut self) -> Result<(), ParseError> {
        while let Some(token) = self.peek_token()? {
            match token.kind {
                TokenKind::Whitespace(_) | TokenKind::LineBreak | TokenKind::Comment(_) => {
                    self.consume_token()?;
                }
                _ => break,
            }
        }
        Ok(())
    }

    #[inline]
    fn is_at_end(&mut self) -> Result<bool, ParseError> {
        Ok(self.peek_token()?.is_none())
    }

    #[inline]
    fn current_position(&mut self) -> Position {
        self.lexer.position()
    }

    #[inline]
    fn check_recursion_depth(&self) -> Result<(), ParseError> {
        if self.recursion_depth >= self.max_recursion_depth {
            Err(ParseError::new(
                ParseErrorKind::RecursionLimitExceeded,
                Position::start(), // We don't have current position in this context
                format!(
                    "recursion depth exceeded maximum of {}",
                    self.max_recursion_depth
                ),
            ))
        } else {
            Ok(())
        }
    }
}

/// Parse error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub position: Position,
    pub message: String,
}

impl ParseError {
    pub fn new(kind: ParseErrorKind, position: Position, message: impl Into<String>) -> Self {
        Self {
            kind,
            position,
            message: message.into(),
        }
    }

    pub fn from_lex_error(error: LexError) -> Self {
        Self {
            kind: ParseErrorKind::LexicalError,
            position: error.position,
            message: error.kind.to_string().into(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at {}:{}: {}",
            self.kind, self.position.line, self.position.column, self.message
        )
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    LexicalError,
    UnexpectedToken,
    ExpectedToken,
    UnexpectedEndOfInput,
    RecursionLimitExceeded,
    UnexpectedState,
    InternalError,
}

impl std::fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::LexicalError => write!(f, "lexical error"),
            ParseErrorKind::UnexpectedToken => write!(f, "unexpected token"),
            ParseErrorKind::ExpectedToken => write!(f, "expected token"),
            ParseErrorKind::UnexpectedEndOfInput => write!(f, "unexpected end of input"),
            ParseErrorKind::RecursionLimitExceeded => write!(f, "recursion limit exceeded"),
            ParseErrorKind::UnexpectedState => write!(f, "unexpected parser state"),
            ParseErrorKind::InternalError => write!(f, "internal parser error"),
        }
    }
}
