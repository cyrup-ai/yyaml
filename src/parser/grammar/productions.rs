//! YAML 1.2 grammar rules and production definitions
//!
//! This module provides comprehensive grammar rules, production definitions,
//! and parsing utilities for YAML 1.2 specification compliance.

use crate::error::{Marker, ScanError};
use crate::lexer::{Position, TokenKind};

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
    SIndent(i32),   // s-indent(n)
    SIndentLt(i32), // s-indent(<n)
    SIndentLe(i32), // s-indent(≤n)

    // Line prefix productions [76-79]
    SLinePrefix(i32, YamlContext), // s-line-prefix(n,c)
    SBlockLinePrefix(i32),     // s-block-line-prefix(n)
    SFlowLinePrefix(i32),      // s-flow-line-prefix(n)

    // Separation productions [80-81]
    SSeparate(i32, YamlContext), // s-separate(n,c)
    SSeparateLines(i32),     // s-separate-lines(n)

    // Empty productions [70-73]
    LEmpty(i32, YamlContext),    // l-empty(n,c)
    BLTrimmed(i32, YamlContext), // b-l-trimmed(n,c)
    BLFolded(i32, YamlContext),  // b-l-folded(n,c)

    // Block scalar productions [162-182]
    CLBlockScalar(i32, ChompingMode), // c-l-block-scalar(n,t)
    CLLiteral(i32),                   // c-l+literal(n)
    CLFolded(i32),                    // c-l+folded(n)

    // Flow scalar productions [126-135]
    NSPlainFirst(YamlContext), // ns-plain-first(c)
    NSPlainSafe(YamlContext),  // ns-plain-safe(c)
    NSPlainChar(YamlContext),  // ns-plain-char(c)
    NSPlainOneLine(YamlContext), // ns-plain-one-line(c)
    NSPlainMultiLine(i32, YamlContext), // ns-plain-multi-line(n,c)

    // Flow collection productions [137-150]
    CFlowSequence(i32, YamlContext), // c-flow-sequence(n,c)
    CFlowMapping(i32, YamlContext),  // c-flow-mapping(n,c)
    NSFlowSeqEntry(i32, YamlContext), // ns-flow-seq-entry(n,c)
    NSSFlowSeqEntries(i32, YamlContext), // ns-s-flow-seq-entries(n,c)
    NSFlowNode(i32, YamlContext), // ns-flow-node(n,c)
    NSFlowPair(i32, YamlContext),    // ns-flow-pair(n,c)

    // Block collection productions [183-201]
    LBlockSequence(i32),    // l+block-sequence(n)
    LBlockMapping(i32),     // l+block-mapping(n)
    NSLBlockMapEntry(i32),  // ns-l-block-map-entry(n)
    NSLCompactMapping(i32), // ns-l-compact-mapping(n)

    // Additional block collection productions
    CLBlockMapExplicitEntry(i32), // c-l-block-map-explicit-entry(n)
    CLBlockMapImplicitEntry(i32), // c-l-block-map-implicit-entry(n)
    NSLBlockMapExplicitValue(i32), // ns-l-block-map-explicit-value(n)

    // Document productions
    LDocumentPrefix, // l-document-prefix
    CDirectivesEnd, // c-directives-end
    CDocumentEnd, // c-document-end
    LDocumentSuffix, // l-document-suffix

    // Directive productions
    NSReservedDirective, // ns-reserved-directive

    // Additional block scalar
    CBBlockHeader(i32, ChompingMode), // c-b-block-header(m,t)
}

impl Production {
    /// Check if this production matches with given parameters
    #[must_use]
    pub fn matches(&self, indent: i32, context: YamlContext) -> bool {
        match self {
            Self::SIndent(n) => indent == *n,
            Self::SIndentLt(n) => indent < *n,
            Self::SIndentLe(n) => indent <= *n,
            Self::SLinePrefix(n, c) => indent == *n && context == *c,
            Self::SSeparate(n, c) => indent == *n && context == *c,
            Self::LBlockSequence(n) => indent >= *n,
            Self::LBlockMapping(n) => indent >= *n,
            Self::NSPlainFirst(c)
            | Self::NSPlainSafe(c)
            | Self::NSPlainChar(c)
            | Self::NSPlainOneLine(c) => context == *c,
            Self::NSPlainMultiLine(n, c) => indent >= *n && context == *c,
            Self::CLBlockMapExplicitEntry(n) => indent >= *n,
            Self::CLBlockMapImplicitEntry(n) => indent >= *n,
            Self::NSLBlockMapExplicitValue(n) => indent >= *n,
            Self::CBBlockHeader(m, _) => indent == *m,
            Self::CFlowSequence(n, c) | Self::CFlowMapping(n, c) => {
                indent >= *n && context == *c
            }
            Self::NSFlowSeqEntry(n, c)
            | Self::NSSFlowSeqEntries(n, c)
            | Self::NSFlowNode(n, c)
            | Self::NSFlowPair(n, c) => indent >= *n && context == *c,
            // Non-parametric productions always match
            _ => true,
        }
    }

    /// Get the minimum indentation required by this production
    #[must_use]
    pub fn min_indent(&self) -> Option<i32> {
        match self {
            Self::SIndent(n) => Some(*n),
            Self::SIndentLt(_n) => Some(0), // Any indent less than n
            Self::SIndentLe(_n) => Some(0), // Any indent <= n
            Self::SBlockLinePrefix(n) => Some(*n),
            Self::LBlockSequence(n) => Some(n + 1), // Entries at n+1
            Self::LBlockMapping(n) => Some(n + 1),  // Keys at n+1
            Self::CLBlockMapExplicitEntry(n) => Some(*n),
            Self::CLBlockMapImplicitEntry(n) => Some(*n),
            Self::NSLBlockMapExplicitValue(n) => Some(*n),
            Self::CBBlockHeader(m, _) => Some(*m),
            _ => None,
        }
    }

    /// Check if production requires specific context
    #[must_use]
    pub const fn required_context(&self) -> Option<YamlContext> {
        match self {
            Self::SLinePrefix(_, c)
            | Self::SSeparate(_, c)
            | Self::LEmpty(_, c)
            | Self::BLTrimmed(_, c)
            | Self::BLFolded(_, c) => Some(*c),
            Self::NSPlainFirst(c)
            | Self::NSPlainSafe(c)
            | Self::NSPlainChar(c)
            | Self::NSPlainOneLine(c) => Some(*c),
            Self::NSPlainMultiLine(_, c) => Some(*c),
            Self::CFlowSequence(_, c)
            | Self::CFlowMapping(_, c)
            | Self::NSFlowSeqEntry(_, c)
            | Self::NSSFlowSeqEntries(_, c)
            | Self::NSFlowNode(_, c)
            | Self::NSFlowPair(_, c) => Some(*c),
            _ => None,
        }
    }
}
