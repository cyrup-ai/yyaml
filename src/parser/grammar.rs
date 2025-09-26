//! Complete YAML 1.2 grammar implementation
//!
//! This module provides comprehensive grammar rules, production definitions,
//! and parsing utilities for YAML 1.2 specification compliance.

use crate::lexer::{Position, TokenKind, LexError};

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
}

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

/// YAML 1.2 grammar rules and production definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Production {
    // Keep existing non-parametric productions
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

    // ADD parametric productions (grouped by category):

    // Indentation productions [YAML 1.2 spec productions 63-74]
    SIndent(i32),                           // s-indent(n)
    SIndentLt(i32),                         // s-indent(<n)
    SIndentLe(i32),                         // s-indent(≤n)

    // Line prefix productions [76-79]
    SLinePrefix(i32, Context),              // s-line-prefix(n,c)
    SBlockLinePrefix(i32),                  // s-block-line-prefix(n)
    SFlowLinePrefix(i32),                   // s-flow-line-prefix(n)

    // Separation productions [80-81]
    SSeparate(i32, Context),                // s-separate(n,c)
    SSeparateLines(i32),                    // s-separate-lines(n)

    // Empty productions [70-73]
    LEmpty(i32, Context),                   // l-empty(n,c)
    BLTrimmed(i32, Context),                // b-l-trimmed(n,c)
    BLFolded(i32, Context),                 // b-l-folded(n,c)

    // Block scalar productions [162-182]
    CLBlockScalar(i32, ChompingMode),       // c-l-block-scalar(n,t)
    CLLiteral(i32),                         // c-l+literal(n)
    CLFolded(i32),                          // c-l+folded(n)

    // Flow scalar productions [126-135]
    NSPlainFirst(Context),                  // ns-plain-first(c)
    NSPlainSafe(Context),                   // ns-plain-safe(c)
    NSPlainChar(Context),                   // ns-plain-char(c)

    // Block collection productions [183-201]
    LBlockSequence(i32),                    // l+block-sequence(n)
    LBlockMapping(i32),                     // l+block-mapping(n)
    NSLBlockMapEntry(i32),                  // ns-l-block-map-entry(n)
    NSLCompactMapping(i32),                 // ns-l-compact-mapping(n)

    // Flow collection productions [137-150]
    CFlowSequence(i32, Context),            // c-flow-sequence(n,c)
    CFlowMapping(i32, Context),             // c-flow-mapping(n,c)
    NSFlowPair(i32, Context),               // ns-flow-pair(n,c)
}

impl Production {
    /// Check if this production matches with given parameters
    pub fn matches(&self, indent: i32, context: Context) -> bool {
        match self {
            Production::SIndent(n) => indent == *n,
            Production::SIndentLt(n) => indent < *n,
            Production::SIndentLe(n) => indent <= *n,
            Production::SLinePrefix(n, c) => indent == *n && context == *c,
            Production::SSeparate(n, c) => indent == *n && context == *c,
            Production::LBlockSequence(n) => indent >= *n,
            Production::LBlockMapping(n) => indent >= *n,
            Production::NSPlainFirst(c) | Production::NSPlainSafe(c) | Production::NSPlainChar(c) => {
                context == *c
            }
            Production::CFlowSequence(n, c) | Production::CFlowMapping(n, c) => {
                indent >= *n && context == *c
            }
            // Non-parametric productions always match
            _ => true,
        }
    }

    /// Get the minimum indentation required by this production
    pub fn min_indent(&self) -> Option<i32> {
        match self {
            Production::SIndent(n) => Some(*n),
            Production::SIndentLt(n) => Some(0),  // Any indent less than n
            Production::SIndentLe(n) => Some(0),  // Any indent <= n
            Production::SBlockLinePrefix(n) => Some(*n),
            Production::LBlockSequence(n) => Some(n + 1),  // Entries at n+1
            Production::LBlockMapping(n) => Some(n + 1),   // Keys at n+1
            Production::CLBlockScalar(n, _) => Some(*n),
            Production::CLLiteral(n) => Some(*n),
            Production::CLFolded(n) => Some(*n),
            _ => None,
        }
    }

    /// Check if production requires specific context
    pub fn required_context(&self) -> Option<Context> {
        match self {
            Production::SLinePrefix(_, c) | Production::SSeparate(_, c)
            | Production::LEmpty(_, c) | Production::BLTrimmed(_, c)
            | Production::BLFolded(_, c) => Some(*c),
            Production::NSPlainFirst(c) | Production::NSPlainSafe(c)
            | Production::NSPlainChar(c) => Some(*c),
            Production::CFlowSequence(_, c) | Production::CFlowMapping(_, c)
            | Production::NSFlowPair(_, c) => Some(*c),
            _ => None,
        }
    }
}

/// YAML 1.2 parsing contexts for parametric productions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    BlockIn,   // BLOCK-IN
    BlockOut,  // BLOCK-OUT
    BlockKey,  // BLOCK-KEY
    FlowIn,    // FLOW-IN
    FlowOut,   // FLOW-OUT
    FlowKey,   // FLOW-KEY
}

/// Block scalar chomping modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChompingMode {
    Strip,  // Remove all trailing newlines
    Clip,   // Keep first trailing newline only
    Keep,   // Keep all trailing newlines
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

    /// Check if production is valid in current parametric context
    pub fn is_valid_parametric_production(
        production: &Production,
        context: &ParametricContext,
    ) -> bool {
        production.matches(context.current_indent(), context.current_context)
    }

    /// Get applicable productions for current parametric context
    pub fn applicable_parametric_productions(
        context: &ParametricContext,
    ) -> Vec<Production> {
        let mut productions = Vec::new();
        let indent = context.current_indent();

        match context.current_context {
            Context::BlockOut | Context::BlockIn => {
                productions.push(Production::LBlockSequence(indent));
                productions.push(Production::LBlockMapping(indent));
                productions.push(Production::SIndent(indent));
                productions.push(Production::SBlockLinePrefix(indent));
            }
            Context::FlowOut | Context::FlowIn => {
                productions.push(Production::CFlowSequence(indent, context.current_context));
                productions.push(Production::CFlowMapping(indent, context.current_context));
                productions.push(Production::SFlowLinePrefix(indent));
            }
            Context::BlockKey | Context::FlowKey => {
                productions.push(Production::NSPlainFirst(context.current_context));
                productions.push(Production::NSPlainSafe(context.current_context));
            }
        }

        productions
    }

    /// Convert between parametric context and existing parse context for compatibility
    pub fn parametric_to_parse_context(parametric: &ParametricContext) -> ParseContext {
        parametric.to_parse_context()
    }

    /// Determine next parametric context after parsing a production
    pub fn next_parametric_context(
        current: &ParametricContext,
        production: Production,
        new_indent: Option<i32>,
    ) -> Context {
        match (current.current_context, production) {
            (Context::BlockOut, Production::Document) => Context::BlockIn,
            (Context::BlockIn, Production::LBlockSequence(_)) => Context::BlockIn,
            (Context::BlockIn, Production::LBlockMapping(_)) => Context::BlockIn,
            (Context::BlockIn, Production::CFlowSequence(_, _)) => Context::FlowIn,
            (Context::BlockIn, Production::CFlowMapping(_, _)) => Context::FlowIn,
            (Context::FlowIn, Production::CFlowSequence(_, _)) => Context::FlowIn,
            (Context::FlowIn, Production::CFlowMapping(_, _)) => Context::FlowIn,
            (Context::BlockIn, Production::BlockKey) => Context::BlockKey,
            (Context::BlockKey, Production::BlockValue) => Context::BlockIn,
            (Context::FlowIn, Production::FlowPair) => Context::FlowKey,
            (Context::FlowKey, Production::FlowEntry) => Context::FlowIn,
            // Default: maintain current context
            _ => current.current_context,
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

/// Tracks parametric context during parsing - integrates with existing indentation system
#[derive(Debug, Clone)]
pub struct ParametricContext {
    /// Context stack for YAML 1.2 parametric productions
    pub context_stack: Vec<Context>,
    pub current_context: Context,
    pub chomping_mode: Option<ChompingMode>,
    // REUSE existing indentation system - DO NOT DUPLICATE
    pub indentation: crate::parser::indentation::IndentationStateMachine,
}

impl ParametricContext {
    pub fn new() -> Self {
        Self {
            context_stack: vec![Context::BlockOut], // Document level starts BLOCK-OUT
            current_context: Context::BlockOut,
            chomping_mode: None,
            // REUSE existing IndentationStateMachine from parser/indentation.rs
            indentation: crate::parser::indentation::IndentationStateMachine::new(),
        }
    }

    pub fn push_context(&mut self, context: Context, indent: i32) {
        self.context_stack.push(self.current_context);
        self.current_context = context;

        // Use existing indentation system
        let is_sequence = matches!(context, Context::BlockIn);
        self.indentation.push_indent(indent as usize, is_sequence);
    }

    pub fn pop_context(&mut self) {
        if let Some(context) = self.context_stack.pop() {
            self.current_context = context;
            self.indentation.pop_indent();
        }
    }

    /// Get current indentation from existing system
    pub fn current_indent(&self) -> i32 {
        self.indentation.current_indent() as i32
    }

    /// Calculate n+m indentation for block collections per YAML 1.2 spec
    pub fn calculate_block_indent(&self, base: i32, offset: i32) -> i32 {
        base + offset
    }

    /// Set chomping mode for block scalars
    pub fn set_chomping_mode(&mut self, mode: ChompingMode) {
        self.chomping_mode = Some(mode);
    }

    /// Clear chomping mode after processing block scalar
    pub fn clear_chomping_mode(&mut self) {
        self.chomping_mode = None;
    }

    /// Convert YAML 1.2 Context to existing ParseContext for backward compatibility
    pub fn to_parse_context(&self) -> ParseContext {
        match self.current_context {
            Context::BlockIn => ParseContext::BlockIn(self.current_indent() as usize),
            Context::BlockOut => ParseContext::Document,
            Context::BlockKey => ParseContext::BlockKey,
            Context::FlowIn => ParseContext::FlowIn(self.context_stack.len()),
            Context::FlowOut => ParseContext::FlowValue,
            Context::FlowKey => ParseContext::FlowKey,
        }
    }
}

impl Default for ParametricContext {
    fn default() -> Self {
        Self::new()
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
