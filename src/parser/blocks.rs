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
    /// Parse block sequence starting with '-'
    pub fn parse_sequence<'input>(
        parser: &mut YamlParser<'input>,
        start_token: Token<'input>,
        base_indent: usize,
    ) -> Result<Node<'input>, ParseError> {
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
        let mut context_stack = ContextStack::new();
        context_stack.push(ParseContext::BlockIn(base_indent));

        // Calculate sequence indentation
        let sequence_indent = start_pos.column.saturating_sub(1);

        loop {
            // Parse first item after initial '-'
            parser.skip_insignificant_tokens()?;

            let item = if let Some(token) = parser.peek_token()? {
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
                        // Parse actual item content
                        Self::parse_block_item(parser, &context_stack, sequence_indent)?
                    }
                }
            } else {
                // End of input
                Node::Null(NullNode::new(start_pos))
            };

            items.push(item);

            // Look for next sequence item
            parser.skip_insignificant_tokens()?;

            if let Some(token) = parser.peek_token()? {
                match &token.kind {
                    TokenKind::BlockEntry
                        if Self::is_at_sequence_indent(token.position, sequence_indent) =>
                    {
                        parser.consume_token()?; // consume '-'
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

    /// Parse block mapping with key: value pairs
    pub fn parse_mapping<'input>(
        parser: &mut YamlParser<'input>,
        first_key_token: Token<'input>,
        base_indent: usize,
    ) -> Result<Node<'input>, ParseError> {
        let start_pos = first_key_token.position;
        let mut pairs = Vec::new();
        let mut context_stack = ContextStack::new();
        context_stack.push(ParseContext::BlockIn(base_indent));

        // Calculate mapping indentation from first key
        let mapping_indent = start_pos.column.saturating_sub(1);

        // Parse first key-value pair
        let first_pair = Self::parse_mapping_pair(parser, first_key_token, &context_stack)?;
        pairs.push(first_pair);

        // Parse remaining pairs
        loop {
            parser.skip_insignificant_tokens()?;

            if let Some(token) = parser.peek_token()? {
                let token_pos = token.position;
                // Check if this could be another mapping key at the correct indentation
                if Self::is_potential_mapping_key(parser, token_pos, mapping_indent)? {
                    let key_token = parser.consume_token()?;
                    let pair = Self::parse_mapping_pair(parser, key_token, &context_stack)?;
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

    /// Parse a single item in block sequence
    fn parse_block_item<'input>(
        parser: &mut YamlParser<'input>,
        context_stack: &ContextStack,
        sequence_indent: usize,
    ) -> Result<Node<'input>, ParseError> {
        parser.check_recursion_depth()?;
        parser.recursion_depth += 1;

        let result = Self::parse_block_node(parser, context_stack, sequence_indent + 2);

        parser.recursion_depth -= 1;
        result
    }

    /// Parse key-value pair in block mapping
    fn parse_mapping_pair<'input>(
        parser: &mut YamlParser<'input>,
        key_token: Token<'input>,
        context_stack: &ContextStack,
    ) -> Result<MappingPair<'input>, ParseError> {
        // Parse key
        let mut key_context = context_stack.clone();
        key_context.push(ParseContext::BlockKey);

        let key = match key_token.kind {
            TokenKind::Scalar { .. } => {
                super::scalars::ScalarParser::parse_scalar(key_token, key_context.current())?
            }
            TokenKind::Key => {
                // Explicit key indicator '?'
                parser.skip_insignificant_tokens()?;
                if let Some(_token) = parser.peek_token()? {
                    let next_token = parser.consume_token()?;
                    Self::parse_complex_key(next_token, &key_context)?
                } else {
                    return Err(ParseError::new(
                        ParseErrorKind::UnexpectedEndOfInput,
                        key_token.position,
                        "expected key after '?'",
                    ));
                }
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
        parser.skip_insignificant_tokens()?;
        let current_pos = parser.current_position();
        let colon_token = parser.peek_token()?.ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::UnexpectedEndOfInput,
                current_pos,
                "expected ':' after mapping key",
            )
        })?;

        let colon_position = colon_token.position;

        if !matches!(colon_token.kind, TokenKind::Value) {
            return Err(ParseError::new(
                ParseErrorKind::ExpectedToken,
                colon_position,
                "expected ':' after mapping key",
            ));
        }

        parser.consume_token()?; // consume :

        // Parse value
        parser.skip_insignificant_tokens()?;

        let mut value_context = context_stack.clone();
        value_context.push(ParseContext::BlockValue);

        let value_indent = colon_position.column;

        let value = if let Some(token) = parser.peek_token()? {
            let token_kind = token.kind.clone();
            let token_position = token.position;

            match token_kind {
                // Check for null/empty value cases
                TokenKind::BlockEntry | TokenKind::DocumentEnd | TokenKind::StreamEnd => {
                    Node::Null(NullNode::new(parser.current_position()))
                }
                _ if Self::is_next_mapping_key(parser, token_position, value_indent)? => {
                    // Next line is another mapping key, this value is null
                    Node::Null(NullNode::new(parser.current_position()))
                }
                _ => Self::parse_block_node(parser, &value_context, value_indent)?,
            }
        } else {
            // End of input, empty value
            Node::Null(NullNode::new(parser.current_position()))
        };

        Ok(MappingPair::new(key, value))
    }

    /// Parse complex key (flow collection or tagged/anchored key)
    fn parse_complex_key<'input>(
        token: Token<'input>,
        context_stack: &ContextStack,
    ) -> Result<Node<'input>, ParseError> {
        match token.kind {
            TokenKind::Scalar { .. } => {
                super::scalars::ScalarParser::parse_scalar(token, context_stack.current())
            }
            TokenKind::FlowSequenceStart => {
                // Flow sequence as key
                Err(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    token.position,
                    "flow sequences as mapping keys not yet supported",
                ))
            }
            TokenKind::FlowMappingStart => {
                // Flow mapping as key
                Err(ParseError::new(
                    ParseErrorKind::UnexpectedToken,
                    token.position,
                    "flow mappings as mapping keys not yet supported",
                ))
            }
            _ => Err(ParseError::new(
                ParseErrorKind::UnexpectedToken,
                token.position,
                format!("invalid complex key token: {:?}", token.kind),
            )),
        }
    }

    /// Parse any node in block context
    fn parse_block_node<'input>(
        parser: &mut YamlParser<'input>,
        context_stack: &ContextStack,
        min_indent: usize,
    ) -> Result<Node<'input>, ParseError> {
        parser.skip_insignificant_tokens()?;

        let current_pos = parser.current_position();
        let token = parser.peek_token()?.ok_or_else(|| {
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
            // Nested block sequence
            TokenKind::BlockEntry => {
                let entry_token = parser.consume_token()?;
                Self::parse_sequence(parser, entry_token, token_position.column)
            }

            // Flow collections
            TokenKind::FlowSequenceStart => {
                let start_token = parser.consume_token()?;
                super::flows::FlowParser::parse_sequence(parser, start_token)
            }

            TokenKind::FlowMappingStart => {
                let start_token = parser.consume_token()?;
                super::flows::FlowParser::parse_mapping(parser, start_token)
            }

            // Scalars
            TokenKind::Scalar { .. } => {
                // Check if this could be a block mapping key
                if Self::is_potential_mapping_key(parser, token_position, min_indent)? {
                    let key_token = parser.consume_token()?;
                    Self::parse_mapping(parser, key_token, min_indent)
                } else {
                    let scalar_token = parser.consume_token()?;
                    super::scalars::ScalarParser::parse_scalar(
                        scalar_token,
                        context_stack.current(),
                    )
                }
            }

            // Explicit key indicator
            TokenKind::Key => {
                let key_token = parser.consume_token()?;
                Self::parse_mapping(parser, key_token, min_indent)
            }

            // Anchors
            TokenKind::Anchor(name) => {
                let anchor_token = parser.consume_token()?;
                let anchored_node = Self::parse_block_node(parser, context_stack, min_indent)?;
                Ok(Node::Anchor(super::ast::AnchorNode::new(
                    name,
                    Box::new(anchored_node),
                    anchor_token.position,
                )))
            }

            // Aliases
            TokenKind::Alias(name) => {
                let alias_token = parser.consume_token()?;
                Ok(Node::Alias(super::ast::AliasNode::new(
                    name,
                    alias_token.position,
                )))
            }

            // Tags
            TokenKind::Tag { handle, suffix } => {
                let tag_token = parser.consume_token()?;
                let tagged_node = Self::parse_block_node(parser, context_stack, min_indent)?;
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

    /// Check if next token is another mapping key
    fn is_next_mapping_key<'input>(
        parser: &mut YamlParser<'input>,
        position: Position,
        current_indent: usize,
    ) -> Result<bool, ParseError> {
        // Must be at same or lesser indentation as current mapping
        if position.column > current_indent {
            return Ok(false);
        }

        // Check if it's a potential key
        Self::is_potential_mapping_key(parser, position, position.column)
    }

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
        base_indent: usize,
    ) -> Result<bool, ParseError> {
        if let Some(token) = parser.peek_token()? {
            match token.kind {
                TokenKind::DocumentEnd | TokenKind::DocumentStart | TokenKind::StreamEnd => {
                    Ok(true)
                }
                _ => {
                    // Check indentation
                    Ok(token.position.column <= base_indent)
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
                let token_pos = token.position;
                let token_kind = token.kind.clone();

                if token_pos.column == expected_indent {
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
                                token_pos,
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
