//! Complete YAML 1.2 grammar implementation
//!
//! This module provides comprehensive grammar rules, production definitions,
//! and parsing utilities for YAML 1.2 specification compliance.

use super::{ParseError, ParseErrorKind};
use crate::lexer::{Position, TokenKind};

/// YAML 1.2 grammar rules and production definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Production {
    // Document structure
    Stream,
    Document,
    ExplicitDocument,
    ImplicitDocument,
    DirectiveDocument,
    BareDocument,

    // Node productions
    Node,
    FlowNode,
    BlockNode,

    // Collection productions
    FlowSequence,
    FlowMapping,
    BlockSequence,
    BlockMapping,

    // Scalar productions
    PlainScalar,
    QuotedScalar,
    SingleQuotedScalar,
    DoubleQuotedScalar,
    LiteralScalar,
    FoldedScalar,

    // Property productions
    Properties,
    Tag,
    Anchor,

    // Flow productions
    FlowPair,
    FlowEntry,

    // Block productions
    BlockEntry,
    BlockPair,
    BlockKey,
    BlockValue,

    // Indicator productions
    Comment,
    Directive,
    Reserved,
}

/// Context information for grammar-driven parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseContext {
    /// Root document context
    Document,
    /// Block collection context with indentation level
    BlockIn(usize),
    /// Flow collection context with nesting level
    FlowIn(usize),
    /// Block key context
    BlockKey,
    /// Flow key context
    FlowKey,
    /// Block value context
    BlockValue,
    /// Flow value context  
    FlowValue,
}

/// Grammar validation and decision utilities
pub struct Grammar;

impl Grammar {
    /// Check if token can start a document
    #[inline]
    pub fn can_start_document(token: &TokenKind) -> bool {
        matches!(token, TokenKind::DocumentStart
            | TokenKind::YamlDirective { .. }
            | TokenKind::TagDirective { .. }
            | TokenKind::ReservedDirective { .. }
            | TokenKind::Scalar { .. }
            | TokenKind::FlowSequenceStart
            | TokenKind::FlowMappingStart
            | TokenKind::BlockEntry
            | TokenKind::Anchor(_)
            | TokenKind::Alias(_)
            | TokenKind::Tag { .. })
    }

    /// Check if token can start a node
    #[inline]
    pub fn can_start_node(token: &TokenKind) -> bool {
        matches!(token, TokenKind::Scalar { .. }
            | TokenKind::FlowSequenceStart
            | TokenKind::FlowMappingStart
            | TokenKind::BlockEntry
            | TokenKind::Anchor(_)
            | TokenKind::Alias(_)
            | TokenKind::Tag { .. })
    }

    /// Check if token can start a flow collection
    #[inline]
    pub fn can_start_flow_collection(token: &TokenKind) -> bool {
        matches!(
            token,
            TokenKind::FlowSequenceStart | TokenKind::FlowMappingStart
        )
    }

    /// Check if token can start a block collection
    #[inline]
    pub fn can_start_block_collection(token: &TokenKind) -> bool {
        matches!(token, TokenKind::BlockEntry | TokenKind::Key)
    }

    /// Check if token is a scalar
    #[inline]
    pub fn is_scalar(token: &TokenKind) -> bool {
        matches!(token, TokenKind::Scalar { .. })
    }

    /// Check if token is a property (anchor or tag)
    #[inline]
    pub fn is_property(token: &TokenKind) -> bool {
        matches!(token, TokenKind::Anchor(_) | TokenKind::Tag { .. })
    }

    /// Check if token can continue a plain scalar in given context
    pub fn can_continue_plain_scalar(token: &TokenKind, context: &ParseContext) -> bool {
        match context {
            ParseContext::FlowIn(_) | ParseContext::FlowKey | ParseContext::FlowValue => {
                // In flow context, plain scalars are more restricted
                match token {
                    TokenKind::FlowEntry
                    | TokenKind::FlowSequenceEnd
                    | TokenKind::FlowMappingEnd
                    | TokenKind::Value => false,
                    TokenKind::Scalar { .. } => true,
                    _ => false,
                }
            }
            ParseContext::BlockIn(_) | ParseContext::BlockKey | ParseContext::BlockValue => {
                // In block context, plain scalars can continue until certain indicators
                match token {
                    TokenKind::Value
                    | TokenKind::BlockEntry
                    | TokenKind::Comment(_)
                    | TokenKind::DocumentStart
                    | TokenKind::DocumentEnd => false,
                    TokenKind::Scalar { .. } => true,
                    _ => false,
                }
            }
            ParseContext::Document => {
                // At document level, very permissive
                !matches!(
                    token,
                    TokenKind::DocumentStart | TokenKind::DocumentEnd | TokenKind::StreamEnd
                )
            }
        }
    }

    /// Determine if a mapping key-value pair is implicit (no explicit value indicator)
    pub fn is_implicit_mapping_value(context: &ParseContext) -> bool {
        matches!(context, ParseContext::BlockIn(_))
    }

    /// Check if indentation is valid for block context
    #[inline]
    pub fn is_valid_block_indentation(
        current_indent: usize,
        context_indent: usize,
        strict: bool,
    ) -> bool {
        if strict {
            current_indent > context_indent
        } else {
            current_indent >= context_indent
        }
    }

    /// Determine production rule for current parsing state
    pub fn determine_production(
        token: &TokenKind,
        context: &ParseContext,
        lookahead: Option<&TokenKind>,
    ) -> Result<Production, ParseError> {
        match context {
            ParseContext::Document => match token {
                TokenKind::DocumentStart => Ok(Production::ExplicitDocument),
                TokenKind::YamlDirective { .. }
                | TokenKind::TagDirective { .. }
                | TokenKind::ReservedDirective { .. } => Ok(Production::DirectiveDocument),
                _ if Self::can_start_node(token) => Ok(Production::BareDocument),
                _ => Err(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    Position::start(),
                    format!("unexpected token in document context: {token:?}"),
                )),
            },

            ParseContext::FlowIn(_) => {
                match token {
                    TokenKind::FlowSequenceStart => Ok(Production::FlowSequence),
                    TokenKind::FlowMappingStart => Ok(Production::FlowMapping),
                    TokenKind::Scalar { .. } => {
                        // Check if this could be a flow mapping key
                        if matches!(lookahead, Some(TokenKind::Value)) {
                            Ok(Production::FlowPair)
                        } else {
                            Ok(Production::PlainScalar)
                        }
                    }
                    TokenKind::Anchor(_) | TokenKind::Tag { .. } => Ok(Production::Properties),
                    TokenKind::Alias(_) => Ok(Production::Node),
                    _ => Ok(Production::FlowNode),
                }
            }

            ParseContext::BlockIn(_) => {
                match token {
                    TokenKind::BlockEntry => Ok(Production::BlockSequence),
                    TokenKind::Key => Ok(Production::BlockMapping),
                    TokenKind::Scalar { .. } => {
                        // Check if this could be a block mapping key
                        if matches!(lookahead, Some(TokenKind::Value)) {
                            Ok(Production::BlockPair)
                        } else {
                            Ok(Production::PlainScalar)
                        }
                    }
                    TokenKind::Anchor(_) | TokenKind::Tag { .. } => Ok(Production::Properties),
                    TokenKind::Alias(_) => Ok(Production::Node),
                    _ => Ok(Production::BlockNode),
                }
            }

            ParseContext::BlockKey | ParseContext::FlowKey => match token {
                TokenKind::Scalar { .. } => Ok(Production::PlainScalar),
                TokenKind::FlowSequenceStart => Ok(Production::FlowSequence),
                TokenKind::FlowMappingStart => Ok(Production::FlowMapping),
                TokenKind::Anchor(_) | TokenKind::Tag { .. } => Ok(Production::Properties),
                TokenKind::Alias(_) => Ok(Production::Node),
                _ => Err(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    Position::start(),
                    format!("unexpected token in key context: {token:?}"),
                )),
            },

            ParseContext::BlockValue | ParseContext::FlowValue => match token {
                TokenKind::Scalar { .. } => Ok(Production::PlainScalar),
                TokenKind::FlowSequenceStart => Ok(Production::FlowSequence),
                TokenKind::FlowMappingStart => Ok(Production::FlowMapping),
                TokenKind::BlockEntry => Ok(Production::BlockSequence),
                TokenKind::Anchor(_) | TokenKind::Tag { .. } => Ok(Production::Properties),
                TokenKind::Alias(_) => Ok(Production::Node),
                _ => Ok(Production::Node),
            },
        }
    }

    /// Validate scalar style is appropriate for context
    pub fn validate_scalar_style(
        style: crate::lexer::ScalarStyle,
        context: &ParseContext,
        value: &str,
    ) -> Result<(), ParseError> {
        use crate::lexer::ScalarStyle;

        match style {
            ScalarStyle::Plain => {
                // Plain scalars have context-dependent restrictions
                match context {
                    ParseContext::FlowIn(_) | ParseContext::FlowKey | ParseContext::FlowValue => {
                        // Check for flow indicators in plain scalar
                        if value.contains(&[',', '[', ']', '{', '}'][..]) {
                            return Err(ParseError::new(
                                ParseErrorKind::UnexpectedToken,
                                Position::start(),
                                "plain scalar contains flow indicators in flow context",
                            ));
                        }
                    }
                    _ => {
                        // Block context restrictions
                        if value.starts_with(&['-', '?', ':', '|', '>', '@', '`'][..]) {
                            return Err(ParseError::new(
                                ParseErrorKind::UnexpectedToken,
                                Position::start(),
                                "plain scalar starts with reserved indicator",
                            ));
                        }
                    }
                }
            }
            ScalarStyle::SingleQuoted | ScalarStyle::DoubleQuoted => {
                // Quoted scalars are generally safe
            }
            ScalarStyle::Literal | ScalarStyle::Folded => {
                // Block scalars only allowed in block context
                match context {
                    ParseContext::FlowIn(_) | ParseContext::FlowKey | ParseContext::FlowValue => {
                        return Err(ParseError::new(
                            ParseErrorKind::UnexpectedToken,
                            Position::start(),
                            "block scalar not allowed in flow context",
                        ));
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// Check if context allows simple keys
    #[inline]
    pub fn allows_simple_keys(context: &ParseContext) -> bool {
        matches!(context, ParseContext::FlowIn(_) | ParseContext::BlockIn(_))
    }

    /// Check if context requires explicit key indicators
    #[inline]
    pub fn requires_explicit_key(context: &ParseContext) -> bool {
        matches!(context, ParseContext::FlowIn(_))
    }

    /// Determine next context after parsing a production
    pub fn next_context(
        current: &ParseContext,
        production: Production,
        indent: Option<usize>,
    ) -> ParseContext {
        match (current, production) {
            (ParseContext::Document, Production::ExplicitDocument) => ParseContext::BlockIn(0),
            (ParseContext::Document, Production::BareDocument) => ParseContext::BlockIn(0),

            (ParseContext::BlockIn(level), Production::FlowSequence) => {
                ParseContext::FlowIn(*level + 1)
            }
            (ParseContext::BlockIn(level), Production::FlowMapping) => {
                ParseContext::FlowIn(*level + 1)
            }
            (ParseContext::BlockIn(_), Production::BlockSequence) => {
                ParseContext::BlockIn(indent.unwrap_or(0))
            }
            (ParseContext::BlockIn(_), Production::BlockMapping) => {
                ParseContext::BlockIn(indent.unwrap_or(0))
            }

            (ParseContext::FlowIn(level), Production::FlowSequence) => {
                ParseContext::FlowIn(*level + 1)
            }
            (ParseContext::FlowIn(level), Production::FlowMapping) => {
                ParseContext::FlowIn(*level + 1)
            }

            (ParseContext::BlockIn(_level), Production::BlockKey) => ParseContext::BlockKey,
            (ParseContext::BlockIn(_level), Production::BlockValue) => ParseContext::BlockValue,
            (ParseContext::FlowIn(_level), Production::FlowPair) => ParseContext::FlowKey,

            // Default: maintain current context
            _ => current.clone(),
        }
    }
}

/// Context stack for tracking nested parsing contexts
#[derive(Debug, Clone)]
pub struct ContextStack {
    stack: Vec<ParseContext>,
}

impl Default for ContextStack {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextStack {
    /// Create new context stack starting with document context
    #[inline]
    pub fn new() -> Self {
        Self {
            stack: vec![ParseContext::Document],
        }
    }

    /// Get current context
    #[inline]
    pub fn current(&self) -> &ParseContext {
        self.stack.last().unwrap_or(&ParseContext::Document)
    }

    /// Push new context
    #[inline]
    pub fn push(&mut self, context: ParseContext) {
        self.stack.push(context);
    }

    /// Pop context
    pub fn pop(&mut self) -> Option<ParseContext> {
        if self.stack.len() > 1 {
            self.stack.pop()
        } else {
            None
        }
    }

    /// Get nesting depth
    #[inline]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    /// Check if we're in a flow context
    #[inline]
    pub fn in_flow_context(&self) -> bool {
        matches!(
            self.current(),
            ParseContext::FlowIn(_) | ParseContext::FlowKey | ParseContext::FlowValue
        )
    }

    /// Check if we're in a block context
    #[inline]
    pub fn in_block_context(&self) -> bool {
        matches!(
            self.current(),
            ParseContext::BlockIn(_) | ParseContext::BlockKey | ParseContext::BlockValue
        )
    }

    /// Get current indentation level (for block contexts)
    pub fn current_indent(&self) -> Option<usize> {
        match self.current() {
            ParseContext::BlockIn(indent) => Some(*indent),
            _ => None,
        }
    }

    /// Get current flow level (for flow contexts)
    pub fn current_flow_level(&self) -> Option<usize> {
        match self.current() {
            ParseContext::FlowIn(level) => Some(*level),
            _ => None,
        }
    }
}

/// Production-specific parsing hints and optimizations
pub struct ProductionHints;

impl ProductionHints {
    /// Get performance hints for production
    pub fn get_hints(production: Production) -> ProductionOptimization {
        match production {
            Production::PlainScalar => ProductionOptimization {
                can_inline: true,
                zero_allocation: true,
                needs_lookahead: true,
                complexity: Complexity::Low,
            },
            Production::FlowSequence | Production::FlowMapping => ProductionOptimization {
                can_inline: false,
                zero_allocation: false,
                needs_lookahead: true,
                complexity: Complexity::Medium,
            },
            Production::BlockSequence | Production::BlockMapping => ProductionOptimization {
                can_inline: false,
                zero_allocation: false,
                needs_lookahead: true,
                complexity: Complexity::High,
            },
            _ => ProductionOptimization {
                can_inline: true,
                zero_allocation: true,
                needs_lookahead: false,
                complexity: Complexity::Low,
            },
        }
    }
}

/// Performance optimization information for productions
#[derive(Debug, Clone, Copy)]
pub struct ProductionOptimization {
    pub can_inline: bool,
    pub zero_allocation: bool,
    pub needs_lookahead: bool,
    pub complexity: Complexity,
}

/// Complexity classification for productions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Complexity {
    Low,
    Medium,
    High,
}

/// YAML 1.2 indicator classification
pub struct Indicators;

impl Indicators {
    /// Structure indicators
    pub const STRUCTURE: &'static [char] = &['-', '?', ':', ',', '[', ']', '{', '}'];

    /// Quoted scalar indicators
    pub const QUOTED: &'static [char] = &['\'', '"'];

    /// Block scalar indicators
    pub const BLOCK_SCALAR: &'static [char] = &['|', '>'];

    /// Directive indicators
    pub const DIRECTIVE: &'static [char] = &['%', '!'];

    /// Node property indicators
    pub const NODE_PROPERTY: &'static [char] = &['&', '*'];

    /// Reserved indicators
    pub const RESERVED: &'static [char] = &['@', '`'];

    /// Check if character is a YAML indicator
    #[inline]
    pub fn is_indicator(ch: char) -> bool {
        Self::STRUCTURE.contains(&ch)
            || Self::QUOTED.contains(&ch)
            || Self::BLOCK_SCALAR.contains(&ch)
            || Self::DIRECTIVE.contains(&ch)
            || Self::NODE_PROPERTY.contains(&ch)
            || Self::RESERVED.contains(&ch)
    }

    /// Check if character is a flow indicator
    #[inline]
    pub fn is_flow_indicator(ch: char) -> bool {
        matches!(ch, ',' | '[' | ']' | '{' | '}')
    }

    /// Check if character is a block indicator
    #[inline]
    pub fn is_block_indicator(ch: char) -> bool {
        matches!(ch, '-' | '?' | ':')
    }
}
