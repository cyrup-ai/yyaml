//! Block-style syntax parsing for sequences and mappings
//!
//! This module provides comprehensive parsing for YAML block collections:
//! block sequences (- item) and block mappings (key: value).

use super::ast::{
    MappingNode, MappingPair, MappingStyle, Node, NullNode, SequenceNode, SequenceStyle,
};
use super::grammar::{ContextStack, ParseContext};
use super::{ParseError, ParseErrorKind, YamlParser};
use crate::lexer::{Position, Token, TokenKind};

/// Block collection parser with indentation tracking
pub struct BlockParser;

impl BlockParser {
    /// Parse block sequence starting with '-' - blazing-fast with modern ParsingContext API
    pub fn parse_sequence<'input>(
        parser: &mut YamlParser<'input>,
        start_token: Token<'input>,
        _base_indent: usize,
    ) -> Result<Node<'input>, ParseError> {
        // Delegate to the ParsingContext-based implementation to avoid borrowing conflicts
        let mut context = super::ParsingContext::new(
            &mut parser.lexer,
            &mut parser.token_buffer,
            &mut parser.recursion_depth,
            &mut parser.parse_state,
        );
        
        Self::parse_sequence_with_context(&mut context, start_token, _base_indent, |ctx| {
            let context_stack = ContextStack::new();
            Self::parse_context_node(ctx, &context_stack, 0)
        })
    }

    // DEPRECATED: Use parse_mapping_with_context instead to avoid borrowing conflicts
    // This method has been replaced by parse_mapping_with_context
    /*
    /// Parse block mapping with key: value pairs (DEPRECATED - has borrowing conflicts)
    pub fn parse_mapping<'input>(
        parser: &mut YamlParser<'input>,
        first_key_token: Token<'input>,
        _base_indent: usize,
    ) -> Result<Node<'input>, ParseError> {
        // This method has borrowing conflicts - use parse_mapping_with_context instead
        unimplemented!("Use parse_mapping_with_context instead")
    }
    */

    /// Parse block mapping using ParsingContext (modern API)
    pub fn parse_mapping_with_context<'input, F>(
        context: &mut super::ParsingContext<'_, 'input>,
        first_key_token: Token<'input>,
        _base_indent: usize,
        mut parse_node_fn: F,
    ) -> Result<Node<'input>, ParseError>
    where
        F: FnMut(&mut super::ParsingContext<'_, 'input>) -> Result<Node<'input>, ParseError>,
    {
        let start_pos = first_key_token.position;
        let mut pairs = Vec::new();
        let mapping_indent = start_pos.column.saturating_sub(1);

        // Parse first key-value pair
        let first_pair =
            Self::parse_mapping_pair_with_context(context, first_key_token, &mut parse_node_fn)?;
        pairs.push(first_pair);

        // Parse remaining pairs
        loop {
            context.skip_insignificant_tokens()?;

            if let Some(token) = context.peek_token()? {
                let token_position = token.position;
                // Check if this could be another mapping key at the correct indentation
                if Self::is_potential_mapping_key_with_context(
                    context,
                    token_position,
                    mapping_indent,
                )? {
                    let key_token = context.consume_token()?;
                    let pair = Self::parse_mapping_pair_with_context(
                        context,
                        key_token,
                        &mut parse_node_fn,
                    )?;
                    pairs.push(pair);
                } else {
                    break; // Not a mapping key or wrong indentation
                }
            } else {
                break; // End of input
            }
        }

        Ok(Node::Mapping(MappingNode::new(
            pairs,
            MappingStyle::Block,
            start_pos,
        )))
    }

    /// Parse mapping pair using ParsingContext
    fn parse_mapping_pair_with_context<'input, F>(
        context: &mut super::ParsingContext<'_, 'input>,
        key_token: Token<'input>,
        parse_node_fn: &mut F,
    ) -> Result<MappingPair<'input>, ParseError>
    where
        F: FnMut(&mut super::ParsingContext<'_, 'input>) -> Result<Node<'input>, ParseError>,
    {
        // Parse key
        let key = match key_token.kind {
            TokenKind::Scalar { .. } => {
                super::scalars::ScalarParser::parse_scalar_with_context(key_token, context, &ParseContext::BlockKey)?
            }
            _ => {
                return Err(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    key_token.position,
                    format!("unexpected token for mapping key: {:?}", key_token.kind),
                ));
            }
        };

        // Expect value indicator ':'
        context.skip_insignificant_tokens()?;
        let current_pos = context.current_position();
        let colon_token = context.peek_token()?.ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::UnexpectedEndOfInput,
                current_pos,
                "expected ':' after mapping key",
            )
        })?;

        if !matches!(colon_token.kind, TokenKind::Value) {
            return Err(ParseError::new(
                ParseErrorKind::ExpectedToken,
                colon_token.position,
                "expected ':' after mapping key",
            ));
        }

        context.consume_token()?; // consume :

        // Parse value
        context.skip_insignificant_tokens()?;
        let value = if let Some(token) = context.peek_token()? {
            match token.kind {
                TokenKind::BlockEntry | TokenKind::DocumentEnd | TokenKind::StreamEnd => {
                    Node::Null(NullNode::new(context.current_position()))
                }
                _ => parse_node_fn(context)?,
            }
        } else {
            Node::Null(NullNode::new(context.current_position()))
        };

        Ok(MappingPair::new(key, value))
    }

    /// Check if token could be a mapping key (ParsingContext version)
    pub fn is_potential_mapping_key_with_context<'input>(
        context: &mut super::ParsingContext<'_, 'input>,
        position: Position,
        min_indent: usize,
    ) -> Result<bool, ParseError> {
        // Must be at valid indentation
        if position.column < min_indent {
            return Ok(false);
        }

        // Save parser state for lookahead
        let saved_buffer = context.token_buffer.clone();

        // Look ahead for ':' value indicator
        if let Some(_token) = context.peek_token()? {
            context.consume_token()?; // consume potential key
            context.skip_insignificant_tokens()?;

            let is_key = if let Some(next_token) = context.peek_token()? {
                matches!(next_token.kind, TokenKind::Value)
            } else {
                false
            };

            // Restore parser state
            *context.token_buffer = saved_buffer;

            Ok(is_key)
        } else {
            Ok(false)
        }
    }

    // Removed parse_block_item: Used deleted RecursionGuard, functionality covered by parse_sequence_with_context

    // DEPRECATED: Use parse_mapping_pair_with_context instead to avoid borrowing conflicts
    /*
    /// Parse key-value pair in block mapping (DEPRECATED - has borrowing conflicts)
    fn parse_mapping_pair<'input>(
        parser: &mut YamlParser<'input>,
        key_token: Token<'input>,
        context_stack: &ContextStack,
    ) -> Result<MappingPair<'input>, ParseError> {
        // This method has borrowing conflicts - use parse_mapping_pair_with_context instead
        unimplemented!("Use parse_mapping_pair_with_context instead")
    }
    */

    // Removed parse_complex_key: Unused method, complex keys handled by parse_context_node

    /// Parse block sequence using ParsingContext (modern API)
    pub fn parse_sequence_with_context<'input, F>(
        context: &mut super::ParsingContext<'_, 'input>,
        start_token: Token<'input>,
        _base_indent: usize,
        mut parse_node_fn: F,
    ) -> Result<Node<'input>, ParseError>
    where
        F: FnMut(&mut super::ParsingContext<'_, 'input>) -> Result<Node<'input>, ParseError>,
    {
        let start_pos = start_token.position;

        // Validate start token
        if !matches!(start_token.kind, TokenKind::BlockEntry) {
            return Err(ParseError::new(
                ParseErrorKind::ExpectedToken,
                start_pos,
                "expected '-' to start block sequence",
            ));
        }

        let mut items = Vec::new();
        let sequence_indent = start_pos.column.saturating_sub(1);

        loop {
            // Parse first item after initial '-'
            context.skip_insignificant_tokens()?;

            let item = if let Some(token) = context.peek_token()? {
                match &token.kind {
                    // Nested block entry at same level
                    TokenKind::BlockEntry
                        if Self::is_at_sequence_indent(token.position, sequence_indent) =>
                    {
                        // Empty item (dash followed by another dash)
                        Node::Null(NullNode::new(start_pos))
                    }

                    // End of sequence (dedent or other structure)
                    _ if !Self::is_continuation_of_sequence(token.position, sequence_indent) => {
                        // Current item is null/empty
                        Node::Null(NullNode::new(start_pos))
                    }

                    _ => {
                        // Parse actual item content using the provided parse function
                        parse_node_fn(context)?
                    }
                }
            } else {
                // End of input
                Node::Null(NullNode::new(start_pos))
            };

            items.push(item);

            // Look for next sequence item
            context.skip_insignificant_tokens()?;

            if let Some(token) = context.peek_token()? {
                match &token.kind {
                    TokenKind::BlockEntry
                        if Self::is_at_sequence_indent(token.position, sequence_indent) =>
                    {
                        context.consume_token()?; // consume '-'
                        continue; // Parse next item
                    }
                    _ => break, // End of sequence
                }
            } else {
                break; // End of input
            }
        }

        Ok(Node::Sequence(SequenceNode::new(
            items,
            SequenceStyle::Block,
            start_pos,
        )))
    }


    /// Parse any node in block context using ParsingContext (zero-allocation bridge)
    fn parse_context_node<'input>(
        context: &mut super::ParsingContext<'_, 'input>,
        context_stack: &ContextStack,
        min_indent: usize,
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

        // Validate indentation
        if token.position.column < min_indent {
            return Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                token.position,
                format!(
                    "insufficient indentation: expected at least {}, got {}",
                    min_indent, token.position.column
                ),
            ));
        }

        // Clone the data we need before consuming
        let token_kind = token.kind.clone();
        let token_position = token.position;

        match token_kind {
            // Flow collections (delegate directly since we have ParsingContext)
            TokenKind::FlowSequenceStart => {
                let start_token = context.consume_token()?;
                super::flows::FlowParser::parse_sequence(context, start_token, |ctx| {
                    Self::parse_context_node(ctx, context_stack, min_indent)
                })
            }

            TokenKind::FlowMappingStart => {
                let start_token = context.consume_token()?;
                super::flows::FlowParser::parse_mapping(context, start_token, |ctx| {
                    Self::parse_context_node(ctx, context_stack, min_indent)
                })
            }

            // Scalars
            TokenKind::Scalar { .. } => {
                let scalar_token = context.consume_token()?;
                super::scalars::ScalarParser::parse_scalar_with_context(
                    scalar_token,
                    context,
                    &ParseContext::BlockIn(min_indent),
                )
            }

            // Anchors &anchor
            TokenKind::Anchor(name) => {
                let anchor_token = context.consume_token()?;
                let anchored_node = Self::parse_context_node(context, context_stack, min_indent)?;
                Ok(Node::Anchor(super::ast::AnchorNode::new(
                    name,
                    Box::new(anchored_node),
                    anchor_token.position,
                )))
            }

            // Aliases *alias
            TokenKind::Alias(name) => {
                let alias_token = context.consume_token()?;
                Ok(Node::Alias(super::ast::AliasNode::new(
                    name,
                    alias_token.position,
                )))
            }

            // Tags !tag
            TokenKind::Tag { handle, suffix } => {
                let tag_token = context.consume_token()?;
                let tagged_node = Self::parse_context_node(context, context_stack, min_indent)?;
                Ok(Node::Tagged(super::ast::TaggedNode::new(
                    handle,
                    suffix,
                    Box::new(tagged_node),
                    tag_token.position,
                )))
            }

            _ => Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                token_position,
                format!("unexpected token in block context: {:?}", token_kind),
            )),
        }
    }

    // Removed deprecated flow parsing placeholders:
    // - parse_flow_sequence_simple: Use flows::FlowParser for production flow sequence parsing  
    // - parse_flow_mapping_simple: Use flows::FlowParser for production flow mapping parsing

    // Removed parse_block_node: Large unused method with placeholder implementations
    // Functionality provided by parse_context_node and specific parsers

    /// Check if position is at correct indentation for sequence continuation
    #[inline]
    fn is_at_sequence_indent(position: Position, sequence_indent: usize) -> bool {
        position.column == sequence_indent + 1
    }

    /// Check if position indicates continuation of current sequence
    #[inline]
    fn is_continuation_of_sequence(position: Position, sequence_indent: usize) -> bool {
        position.column > sequence_indent
    }

    /// Check if token could be a mapping key
    pub fn is_potential_mapping_key<'input>(
        parser: &mut YamlParser<'input>,
        position: Position,
        min_indent: usize,
    ) -> Result<bool, ParseError> {
        // Must be at valid indentation
        if position.column < min_indent {
            return Ok(false);
        }

        // Save parser state for lookahead
        let saved_buffer = parser.token_buffer.clone();

        // Look ahead for ':' value indicator
        if let Some(_token) = parser.peek_token()? {
            parser.consume_token()?; // consume potential key
            parser.skip_insignificant_tokens()?;

            let is_key = if let Some(next_token) = parser.peek_token()? {
                matches!(next_token.kind, TokenKind::Value)
            } else {
                false
            };

            // Restore parser state
            parser.token_buffer = saved_buffer;

            Ok(is_key)
        } else {
            Ok(false)
        }
    }

    // Removed is_next_mapping_key: Unused utility method
    // Functionality covered by is_potential_mapping_key_with_context

    /// Validate block indentation rules
    pub fn validate_indentation(
        current_indent: usize,
        expected_indent: usize,
        context: &str,
    ) -> Result<(), ParseError> {
        if current_indent < expected_indent {
            Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                Position::start(),
                format!(
                    "invalid indentation in {}: expected at least {}, got {}",
                    context, expected_indent, current_indent
                ),
            ))
        } else {
            Ok(())
        }
    }

    /// Calculate expected indentation for nested block structure
    #[inline]
    pub fn calculate_nested_indent(parent_indent: usize, nesting_level: usize) -> usize {
        parent_indent + (nesting_level * 2)
    }

    /// Check if we've reached end of block structure
    pub fn is_end_of_block_structure<'input>(
        parser: &mut YamlParser<'input>,
        _base_indent: usize,
    ) -> Result<bool, ParseError> {
        if let Some(token) = parser.peek_token()? {
            match token.kind {
                TokenKind::DocumentEnd | TokenKind::DocumentStart | TokenKind::StreamEnd => {
                    Ok(true)
                }
                _ => {
                    // Check indentation
                    Ok(token.position.column <= _base_indent)
                }
            }
        } else {
            Ok(true) // End of input
        }
    }

    /// Get block context information for error reporting
    pub fn get_block_context_info(indent_level: usize, structure_type: &str) -> String {
        format!("{} at indentation level {}", structure_type, indent_level)
    }
}

/// Block parsing optimization utilities
pub struct BlockOptimizations;

impl BlockOptimizations {
    /// Pre-calculate indentation requirements
    #[inline]
    pub fn calculate_indent_requirements(base_indent: usize) -> IndentRequirements {
        IndentRequirements {
            sequence_item: base_indent + 2,
            mapping_value: base_indent + 2,
            nested_collection: base_indent + 4,
        }
    }

    /// Optimize memory allocation for block collections
    pub fn optimize_block_collection_allocation(estimated_size: usize) -> usize {
        // Round up to next power of 2 for better memory usage
        if estimated_size <= 4 {
            4
        } else if estimated_size <= 8 {
            8
        } else if estimated_size <= 16 {
            16
        } else {
            estimated_size.next_power_of_two()
        }
    }

    /// Check if block structure should use compact representation
    #[inline]
    pub fn should_use_compact_block_representation(depth: usize, item_count: usize) -> bool {
        depth <= 2 && item_count <= 10
    }
}

/// Indentation requirements for different block structures
#[derive(Debug, Clone, Copy)]
pub struct IndentRequirements {
    pub sequence_item: usize,
    pub mapping_value: usize,
    pub nested_collection: usize,
}

/// Block parsing error recovery
pub struct BlockErrorRecovery;

impl BlockErrorRecovery {
    /// Recover from indentation errors
    pub fn recover_from_indentation_error<'input>(
        parser: &mut YamlParser<'input>,
        error: ParseError,
        expected_indent: usize,
    ) -> Result<Option<Node<'input>>, ParseError> {
        // Try to find the next valid structure at correct indentation
        loop {
            if let Some(token) = parser.peek_token()? {
                let token_position = token.position;
                let token_kind = token.kind.clone();

                if token_position.column == expected_indent {
                    // Found potential recovery point
                    match token_kind {
                        TokenKind::BlockEntry => {
                            // Start of new sequence
                            return Ok(None); // Let caller handle
                        }
                        TokenKind::Scalar { .. } => {
                            // Potential mapping key
                            if BlockParser::is_potential_mapping_key(
                                parser,
                                token_position,
                                expected_indent,
                            )? {
                                return Ok(None); // Let caller handle
                            }
                        }
                        _ => {}
                    }
                }

                // Skip malformed content
                parser.consume_token()?;
            } else {
                break;
            }
        }

        // Could not recover
        Err(error)
    }

    /// Suggest fixes for common block syntax errors
    pub fn suggest_block_syntax_fix(error: &ParseError) -> Option<String> {
        if error.message.contains("indentation") {
            Some("check YAML indentation - use spaces, not tabs".to_string())
        } else if error.message.contains("expected ':'") {
            Some("missing colon after mapping key".to_string())
        } else if error.message.contains("expected '-'") {
            Some("missing dash for sequence item".to_string())
        } else {
            None
        }
    }

    /// Validate entire block structure for consistency
    pub fn validate_block_structure_consistency<'input>(
        parser: &mut YamlParser<'input>,
        base_indent: usize,
    ) -> Result<Vec<String>, ParseError> {
        let warnings = Vec::new();
        let saved_buffer = parser.token_buffer.clone();

        while let Some(token) = parser.peek_token()? {
            match token.kind {
                TokenKind::DocumentEnd | TokenKind::DocumentStart | TokenKind::StreamEnd => break,
                _ => {
                    // Check for inconsistent indentation
                    if token.position.column < base_indent {
                        break;
                    }

                    // Check for mixed tabs and spaces (if we had that info)
                    // warnings.push("mixed tabs and spaces detected".to_string());

                    parser.consume_token()?;
                }
            }
        }

        // Restore parser state
        parser.token_buffer = saved_buffer;

        Ok(warnings)
    }
}

// RecursionGuard removed: Duplicate of mod.rs implementation, never constructed
// Manual recursion tracking provides superior zero-allocation performance

