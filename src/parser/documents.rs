//! Multi-document stream parsing and document-level utilities
//!
//! This module provides comprehensive document parsing, stream handling,
//! and multi-document YAML processing according to YAML 1.2 specification.

use super::ast::{Document, Node, Stream};
use super::grammar::ParseContext;
use super::{ParseError, ParseErrorKind, YamlParser};
use crate::lexer::Position;
use crate::lexer::TokenKind;

/// Document parser with multi-document stream support
pub struct DocumentParser;

impl DocumentParser {
    /// Parse complete YAML stream containing multiple documents
    pub fn parse_stream<'input>(
        parser: &mut YamlParser<'input>,
    ) -> Result<Stream<'input>, ParseError> {
        let mut documents = Vec::new();

        // Parse stream start if present
        if let Some(token) = parser.peek_token()? {
            if matches!(token.kind, TokenKind::StreamStart) {
                parser.consume_token()?;
            }
        }

        // Parse documents until stream end
        while !parser.is_at_end()? {
            parser.skip_insignificant_tokens()?;

            // Check for stream end
            if let Some(token) = parser.peek_token()? {
                if matches!(token.kind, TokenKind::StreamEnd) {
                    break;
                }
            } else {
                break;
            }

            // Parse document
            if let Some(document) = Self::parse_document(parser)? {
                documents.push(document);
            }

            // Skip any trailing content between documents
            Self::skip_inter_document_content(parser)?;
        }

        Ok(Stream::new(documents))
    }

    /// Parse a single YAML document
    pub fn parse_document<'input>(
        parser: &mut YamlParser<'input>,
    ) -> Result<Option<Document<'input>>, ParseError> {
        parser.check_recursion_depth()?;

        let start_pos = parser.current_position();

        // Skip leading whitespace and comments
        parser.skip_insignificant_tokens()?;

        if parser.is_at_end()? {
            return Ok(None);
        }

        // Check for explicit document start
        let has_explicit_start = if let Some(token) = parser.peek_token()? {
            if matches!(token.kind, TokenKind::DocumentStart) {
                parser.consume_token()?; // consume ---
                true
            } else {
                false
            }
        } else {
            false
        };

        // Parse directives if present
        let _directives = Self::parse_directives(parser)?;

        // Parse document content
        parser.skip_insignificant_tokens()?;

        let content = if let Some(token) = parser.peek_token()? {
            match token.kind {
                TokenKind::DocumentEnd | TokenKind::DocumentStart | TokenKind::StreamEnd => {
                    // Empty document
                    None
                }
                _ => Some(Self::parse_document_content(parser)?),
            }
        } else {
            None
        };

        // Check for explicit document end
        let has_explicit_end = if let Some(token) = parser.peek_token()? {
            if matches!(token.kind, TokenKind::DocumentEnd) {
                parser.consume_token()?; // consume ...
                true
            } else {
                false
            }
        } else {
            false
        };

        Ok(Some(Document::new(
            content,
            has_explicit_start,
            has_explicit_end,
            start_pos,
        )))
    }

    /// Parse document content (the main YAML node) - blazing-fast with zero-allocation
    fn parse_document_content<'input>(
        parser: &mut YamlParser<'input>,
    ) -> Result<Node<'input>, ParseError> {
        // Use ParsingContext consistently to avoid borrowing conflicts
        let mut context = super::ParsingContext::new(
            &mut parser.lexer,
            &mut parser.token_buffer,
            &mut parser.recursion_depth,
            &mut parser.parse_state,
        );
        
        Self::parse_document_content_with_context(&mut context)
    }

    /// Parse YAML directives at document start
    /// Parse document content using ParsingContext for flow collections
    fn parse_document_content_with_context<'input>(
        context: &mut super::ParsingContext<'_, 'input>,
    ) -> Result<Node<'input>, ParseError> {
        context.skip_insignificant_tokens()?;

        let current_pos = context.current_position();
        let token = context.peek_token()?.ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::UnexpectedEndOfInput,
                current_pos,
                "expected node content",
            )
        })?;

        // Clone the data we need before consuming
        let token_kind = token.kind.clone();

        match token_kind {
            TokenKind::Scalar { .. } => {
                let scalar_token = context.consume_token()?;
                super::scalars::ScalarParser::parse_scalar_with_context(scalar_token, context, &ParseContext::Document)
            }

            TokenKind::FlowSequenceStart => {
                let start_token = context.consume_token()?;
                super::flows::FlowParser::parse_sequence(context, start_token, |ctx| {
                    Self::parse_document_content_with_context(ctx)
                })
            }

            TokenKind::FlowMappingStart => {
                let start_token = context.consume_token()?;
                super::flows::FlowParser::parse_mapping(context, start_token, |ctx| {
                    Self::parse_document_content_with_context(ctx)
                })
            }

            TokenKind::Anchor(name) => {
                let anchor_token = context.consume_token()?;
                let anchored_node = Self::parse_document_content_with_context(context)?;
                Ok(Node::Anchor(super::ast::AnchorNode::new(
                    name,
                    Box::new(anchored_node),
                    anchor_token.position,
                )))
            }

            TokenKind::Alias(name) => {
                let alias_token = context.consume_token()?;
                Ok(Node::Alias(super::ast::AliasNode::new(
                    name,
                    alias_token.position,
                )))
            }

            TokenKind::Tag { handle, suffix } => {
                let tag_token = context.consume_token()?;
                let tagged_node = Self::parse_document_content_with_context(context)?;
                Ok(Node::Tagged(super::ast::TaggedNode::new(
                    handle,
                    suffix,
                    Box::new(tagged_node),
                    tag_token.position,
                )))
            }

            _ => Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                token.position,
                format!("unexpected token at document level: {:?}", token_kind),
            )),
        }
    }

    fn parse_directives<'input>(
        parser: &mut YamlParser<'input>,
    ) -> Result<Vec<DirectiveInfo<'input>>, ParseError> {
        let mut context = super::ParsingContext::new(
            &mut parser.lexer,
            &mut parser.token_buffer,
            &mut parser.recursion_depth,
            &mut parser.parse_state,
        );
        Self::parse_directives_with_context(&mut context)
    }

    /// Parse directives using ParsingContext (zero-allocation bridge)
    fn parse_directives_with_context<'input>(
        context: &mut super::ParsingContext<'_, 'input>,
    ) -> Result<Vec<DirectiveInfo<'input>>, ParseError> {
        let mut directives = Vec::new();

        loop {
            if let Some(token) = context.peek_token()? {
                match token.kind {
                    TokenKind::YamlDirective { .. }
                    | TokenKind::TagDirective { .. }
                    | TokenKind::ReservedDirective { .. } => {
                        // This is a directive, consume it
                        let directive_token = context.consume_token()?;

                        match directive_token.kind {
                            TokenKind::YamlDirective { major, minor } => {
                                directives.push(DirectiveInfo::Yaml {
                                    major,
                                    minor,
                                    position: directive_token.position,
                                });
                            }

                            TokenKind::TagDirective { handle, prefix } => {
                                directives.push(DirectiveInfo::Tag {
                                    handle,
                                    prefix,
                                    position: directive_token.position,
                                });
                            }

                            TokenKind::ReservedDirective { name, value } => {
                                directives.push(DirectiveInfo::Reserved {
                                    name,
                                    value,
                                    position: directive_token.position,
                                });
                            }

                            _ => unreachable!("We already checked this is a directive token"),
                        }

                        context.skip_insignificant_tokens()?;
                    }
                    _ => break, // Not a directive
                }
            } else {
                break;
            }
        }

        Ok(directives)
    }

    /// Check if scalar token could be a document-level mapping key
    #[allow(dead_code)] // May be used for future document parsing extensions
    fn is_document_level_mapping_key<'input>(
        parser: &mut YamlParser<'input>,
    ) -> Result<bool, ParseError> {
        let mut context = super::ParsingContext::new(
            &mut parser.lexer,
            &mut parser.token_buffer,
            &mut parser.recursion_depth,
            &mut parser.parse_state,
        );
        Self::is_document_level_mapping_key_with_context(&mut context)
    }

    /// Check if scalar token could be a document-level mapping key using ParsingContext
    #[allow(dead_code)] // May be used for future document parsing extensions
    fn is_document_level_mapping_key_with_context<'input>(
        context: &mut super::ParsingContext<'_, 'input>,
    ) -> Result<bool, ParseError> {
        // Save token buffer state for lookahead
        let saved_buffer = context.token_buffer.clone();

        // Skip potential key token
        if context.peek_token()?.is_some() {
            context.consume_token()?;
        }

        // Skip whitespace
        context.skip_insignificant_tokens()?;

        // Look for value indicator
        let is_key = if let Some(token) = context.peek_token()? {
            matches!(token.kind, TokenKind::Value)
        } else {
            false
        };

        // Restore token buffer state
        *context.token_buffer = saved_buffer;

        Ok(is_key)
    }

    /// Skip content between documents (comments, whitespace)
    fn skip_inter_document_content<'input>(
        parser: &mut YamlParser<'input>,
    ) -> Result<(), ParseError> {
        let mut context = super::ParsingContext::new(
            &mut parser.lexer,
            &mut parser.token_buffer,
            &mut parser.recursion_depth,
            &mut parser.parse_state,
        );
        Self::skip_inter_document_content_with_context(&mut context)
    }

    /// Skip content between documents using ParsingContext (zero-allocation bridge)
    fn skip_inter_document_content_with_context<'input>(
        context: &mut super::ParsingContext<'_, 'input>,
    ) -> Result<(), ParseError> {
        while let Some(token) = context.peek_token()? {
            match token.kind {
                TokenKind::StreamStart | TokenKind::StreamEnd => {
                    context.consume_token()?;
                }
                TokenKind::DocumentStart | TokenKind::DocumentEnd => {
                    // Don't consume document markers
                    break;
                }
                _ => break,
            }
        }
        Ok(())
    }

    /// Validate document structure
    pub fn validate_document_structure<'input>(
        document: &Document<'input>,
    ) -> Result<Vec<String>, ParseError> {
        let mut warnings = Vec::new();

        // Check for empty documents
        if document.content.is_none() {
            warnings.push("document is empty".to_string());
        }

        // Validate document markers
        if document.has_explicit_start && document.has_explicit_end {
            // Both markers present - good practice for multi-document streams
        } else if !document.has_explicit_start && !document.has_explicit_end {
            // Neither marker - acceptable for single documents
        } else {
            warnings
                .push("inconsistent document markers (only one of start/end present)".to_string());
        }

        Ok(warnings)
    }

    /// Get document type information
    pub fn get_document_type<'input>(document: &Document<'input>) -> DocumentType {
        match &document.content {
            None => DocumentType::Empty,
            Some(node) => match node {
                Node::Scalar(_) => DocumentType::Scalar,
                Node::Sequence(_) => DocumentType::Sequence,
                Node::Mapping(_) => DocumentType::Mapping,
                Node::Anchor(anchor) => Self::get_document_type_from_node(&anchor.node),
                Node::Tagged(tagged) => Self::get_document_type_from_node(&tagged.node),
                Node::Alias(_) => DocumentType::Alias,
                Node::Null(_) => DocumentType::Null,
            },
        }
    }

    /// Get document type from nested node
    fn get_document_type_from_node<'input>(node: &Node<'input>) -> DocumentType {
        match node {
            Node::Scalar(_) => DocumentType::Scalar,
            Node::Sequence(_) => DocumentType::Sequence,
            Node::Mapping(_) => DocumentType::Mapping,
            Node::Anchor(anchor) => Self::get_document_type_from_node(&anchor.node),
            Node::Tagged(tagged) => Self::get_document_type_from_node(&tagged.node),
            Node::Alias(_) => DocumentType::Alias,
            Node::Null(_) => DocumentType::Null,
        }
    }

    /// Check if stream contains multiple documents
    #[inline]
    pub fn is_multi_document_stream<'input>(stream: &Stream<'input>) -> bool {
        stream.len() > 1
    }

    /// Extract all directive information from documents
    pub fn extract_directive_info<'input>(
        _documents: &[Document<'input>],
    ) -> DirectiveSummary<'input> {
        let yaml_versions = Vec::new();
        let tag_handles = Vec::new();
        let reserved_directives = Vec::new();

        // Note: This is a placeholder since we're not storing directives in Document yet
        // In a full implementation, directives would be stored in the Document struct

        DirectiveSummary {
            yaml_versions,
            tag_handles,
            reserved_directives,
        }
    }

    /// Merge compatible documents
    pub fn merge_documents<'input>(
        doc1: Document<'input>,
        doc2: Document<'input>,
    ) -> Result<Document<'input>, ParseError> {
        match (doc1.content, doc2.content) {
            (Some(Node::Mapping(mut map1)), Some(Node::Mapping(map2))) => {
                // Merge mapping documents
                map1.pairs.extend(map2.pairs);
                Ok(Document::new(
                    Some(Node::Mapping(map1)),
                    doc1.has_explicit_start || doc2.has_explicit_start,
                    doc1.has_explicit_end || doc2.has_explicit_end,
                    doc1.position,
                ))
            }
            (Some(Node::Sequence(mut seq1)), Some(Node::Sequence(seq2))) => {
                // Merge sequence documents
                seq1.items.extend(seq2.items);
                Ok(Document::new(
                    Some(Node::Sequence(seq1)),
                    doc1.has_explicit_start || doc2.has_explicit_start,
                    doc1.has_explicit_end || doc2.has_explicit_end,
                    doc1.position,
                ))
            }
            _ => Err(ParseError::new(
                ParseErrorKind::InternalError,
                doc1.position,
                "cannot merge incompatible document types",
            )),
        }
    }
}

/// Document type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentType {
    Empty,
    Scalar,
    Sequence,
    Mapping,
    Alias,
    Null,
}

/// Directive information extracted during parsing
#[derive(Debug, Clone, PartialEq)]
pub enum DirectiveInfo<'input> {
    Yaml {
        major: u32,
        minor: u32,
        position: Position,
    },
    Tag {
        handle: std::borrow::Cow<'input, str>,
        prefix: std::borrow::Cow<'input, str>,
        position: Position,
    },
    Reserved {
        name: std::borrow::Cow<'input, str>,
        value: std::borrow::Cow<'input, str>,
        position: Position,
    },
}

/// Summary of all directives found in a document stream
#[derive(Debug, Clone)]
pub struct DirectiveSummary<'input> {
    pub yaml_versions: Vec<(u32, u32)>,
    pub tag_handles: Vec<(std::borrow::Cow<'input, str>, std::borrow::Cow<'input, str>)>,
    pub reserved_directives: Vec<(std::borrow::Cow<'input, str>, std::borrow::Cow<'input, str>)>,
}

/// Document parsing optimization utilities
pub struct DocumentOptimizations;

impl DocumentOptimizations {
    /// Estimate document complexity for optimization decisions
    pub fn estimate_document_complexity<'input>(document: &Document<'input>) -> DocumentComplexity {
        match &document.content {
            None => DocumentComplexity::Trivial,
            Some(node) => Self::estimate_node_complexity(node),
        }
    }

    /// Estimate complexity of a node tree
    fn estimate_node_complexity<'input>(node: &Node<'input>) -> DocumentComplexity {
        match node {
            Node::Scalar(_) | Node::Null(_) | Node::Alias(_) => DocumentComplexity::Simple,

            Node::Sequence(seq) => {
                if seq.items.len() <= 10 {
                    DocumentComplexity::Simple
                } else if seq.items.len() <= 100 {
                    DocumentComplexity::Medium
                } else {
                    DocumentComplexity::Complex
                }
            }

            Node::Mapping(map) => {
                if map.pairs.len() <= 5 {
                    DocumentComplexity::Simple
                } else if map.pairs.len() <= 50 {
                    DocumentComplexity::Medium
                } else {
                    DocumentComplexity::Complex
                }
            }

            Node::Anchor(anchor) => Self::estimate_node_complexity(&anchor.node),
            Node::Tagged(tagged) => Self::estimate_node_complexity(&tagged.node),
        }
    }

    /// Suggest parsing strategy based on document size
    pub fn suggest_parsing_strategy<'input>(estimated_size: usize) -> ParsingStrategy {
        if estimated_size < 1024 {
            ParsingStrategy::InMemory
        } else if estimated_size < 1024 * 1024 {
            ParsingStrategy::Buffered
        } else {
            ParsingStrategy::Streaming
        }
    }

    /// Pre-allocate collections based on document type
    pub fn pre_allocate_for_document_type(doc_type: DocumentType) -> AllocationHints {
        match doc_type {
            DocumentType::Sequence => AllocationHints {
                sequence_capacity: 16,
                mapping_capacity: 0,
                string_capacity: 0,
            },
            DocumentType::Mapping => AllocationHints {
                sequence_capacity: 0,
                mapping_capacity: 8,
                string_capacity: 0,
            },
            DocumentType::Scalar => AllocationHints {
                sequence_capacity: 0,
                mapping_capacity: 0,
                string_capacity: 256,
            },
            _ => AllocationHints::default(),
        }
    }
}

/// Document complexity classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentComplexity {
    Trivial,
    Simple,
    Medium,
    Complex,
}

/// Parsing strategy recommendations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingStrategy {
    InMemory,
    Buffered,
    Streaming,
}

/// Memory allocation hints for optimization
#[derive(Debug, Clone, Copy)]
pub struct AllocationHints {
    pub sequence_capacity: usize,
    pub mapping_capacity: usize,
    pub string_capacity: usize,
}

impl Default for AllocationHints {
    fn default() -> Self {
        Self {
            sequence_capacity: 4,
            mapping_capacity: 4,
            string_capacity: 64,
        }
    }
}

/// Multi-document stream utilities
pub struct StreamUtilities;

impl StreamUtilities {
    /// Split single document into multiple based on content
    pub fn split_document<'input>(
        document: Document<'input>,
        split_strategy: SplitStrategy,
    ) -> Result<Vec<Document<'input>>, ParseError> {
        match split_strategy {
            SplitStrategy::ByTopLevelItems => Self::split_by_top_level_items(document),
            SplitStrategy::BySize(max_size) => Self::split_by_size(document, max_size),
            SplitStrategy::NoSplit => Ok(vec![document]),
        }
    }

    /// Split document by top-level items
    fn split_by_top_level_items<'input>(
        document: Document<'input>,
    ) -> Result<Vec<Document<'input>>, ParseError> {
        match document.content {
            Some(Node::Sequence(seq)) => {
                // Split sequence into individual documents
                let documents = seq
                    .items
                    .into_iter()
                    .map(|item| Document::new(Some(item), false, false, document.position))
                    .collect();
                Ok(documents)
            }
            Some(Node::Mapping(map)) => {
                // Split mapping into individual key-value documents
                let documents = map
                    .pairs
                    .into_iter()
                    .map(|pair| {
                        let single_pair_map = super::ast::MappingNode::new(
                            vec![pair],
                            super::ast::MappingStyle::Block,
                            document.position,
                        );
                        Document::new(
                            Some(Node::Mapping(single_pair_map)),
                            false,
                            false,
                            document.position,
                        )
                    })
                    .collect();
                Ok(documents)
            }
            _ => Ok(vec![document]), // Cannot split scalar/null documents
        }
    }

    /// Split document by estimated size
    fn split_by_size<'input>(
        document: Document<'input>,
        _max_size: usize,
    ) -> Result<Vec<Document<'input>>, ParseError> {
        // Placeholder implementation - would need actual size calculation
        Ok(vec![document])
    }

    /// Combine multiple documents into a single stream
    pub fn combine_into_stream<'input>(documents: Vec<Document<'input>>) -> Stream<'input> {
        Stream::new(documents)
    }

    /// Filter documents based on criteria
    pub fn filter_documents<'input, F>(stream: Stream<'input>, predicate: F) -> Stream<'input>
    where
        F: Fn(&Document<'input>) -> bool,
    {
        let filtered_docs = stream.documents.into_iter().filter(predicate).collect();
        Stream::new(filtered_docs)
    }
}

/// Document splitting strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitStrategy {
    NoSplit,
    ByTopLevelItems,
    BySize(usize),
}
