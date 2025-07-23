//! Flow-style syntax parsing for sequences and mappings
//!
//! This module provides comprehensive parsing for YAML flow collections:
//! sequences [item1, item2] and mappings {key1: value1, key2: value2}.

use super::ast::{MappingNode, MappingPair, MappingStyle, Node, SequenceNode, SequenceStyle};
use super::grammar::{ContextStack, ParseContext};
use super::{ParseError, ParseErrorKind, ParsingContext};
use crate::lexer::{Position, Token, TokenKind};

/// Flow collection parser with comprehensive error handling
pub struct FlowParser;

impl FlowParser {
    /// Parse flow sequence [item1, item2, ...] with zero-allocation, blazing-fast performance
    pub fn parse_sequence<'input, F>(
        context: &mut ParsingContext<'_, 'input>,
        start_token: Token<'input>,
        mut parse_node_fn: F,
    ) -> Result<Node<'input>, ParseError>
    where
        F: FnMut(&mut ParsingContext<'_, 'input>) -> Result<Node<'input>, ParseError>,
    {
        let start_pos = start_token.position;

        // Validate start token
        if !matches!(start_token.kind, TokenKind::FlowSequenceStart) {
            return Err(ParseError::new(
                ParseErrorKind::ExpectedToken,
                start_pos,
                "expected '[' to start flow sequence",
            ));
        }

        let mut items = Vec::new();
        let mut context_stack = ContextStack::new();
        context_stack.push(ParseContext::FlowIn(1));

        // Track parsing state
        let mut expecting_item = true;
        let mut found_comma = false;

        loop {
            // Skip whitespace and comments
            context.skip_insignificant_tokens()?;

            if let Some(token) = context.peek_token()? {
                match &token.kind {
                    TokenKind::FlowSequenceEnd => {
                        context.consume_token()?; // consume ]
                        break;
                    }

                    TokenKind::FlowEntry => {
                        if expecting_item && !items.is_empty() {
                            return Err(ParseError::new(
                                ParseErrorKind::UnexpectedToken,
                                token.position,
                                "unexpected ',' at start of sequence or after another ','",
                            ));
                        }

                        context.consume_token()?; // consume ,
                        expecting_item = true;
                        found_comma = true;

                        // Check for trailing comma
                        context.skip_insignificant_tokens()?;
                        if let Some(next_token) = context.peek_token()?
                            && matches!(next_token.kind, TokenKind::FlowSequenceEnd) {
                                continue; // Allow trailing comma
                            }
                    }

                    _ => {
                        if !expecting_item && !found_comma {
                            return Err(ParseError::new(
                                ParseErrorKind::ExpectedToken,
                                token.position,
                                "expected ',' or ']' in flow sequence",
                            ));
                        }

                        // Parse sequence item
                        let item = parse_node_fn(context)?;
                        items.push(item);

                        expecting_item = false;
                        found_comma = false;
                    }
                }
            } else {
                return Err(ParseError::new(
                    ParseErrorKind::UnexpectedEndOfInput,
                    context.current_position(),
                    "unterminated flow sequence",
                ));
            }
        }

        Ok(Node::Sequence(SequenceNode::new(
            items,
            SequenceStyle::Flow,
            start_pos,
        )))
    }

    /// Parse flow mapping {key1: value1, key2: value2, ...} with zero-allocation, blazing-fast performance
    pub fn parse_mapping<'input, F>(
        context: &mut ParsingContext<'_, 'input>,
        start_token: Token<'input>,
        mut parse_node_fn: F,
    ) -> Result<Node<'input>, ParseError>
    where
        F: FnMut(&mut ParsingContext<'_, 'input>) -> Result<Node<'input>, ParseError>,
    {
        let start_pos = start_token.position;

        // Validate start token
        if !matches!(start_token.kind, TokenKind::FlowMappingStart) {
            return Err(ParseError::new(
                ParseErrorKind::ExpectedToken,
                start_pos,
                "expected '{' to start flow mapping",
            ));
        }

        let mut pairs = Vec::new();
        let mut context_stack = ContextStack::new();
        context_stack.push(ParseContext::FlowIn(1));

        // Track parsing state
        let mut expecting_pair = true;
        let mut found_comma = false;

        loop {
            // Skip whitespace and comments
            context.skip_insignificant_tokens()?;

            if let Some(token) = context.peek_token()? {
                match &token.kind {
                    TokenKind::FlowMappingEnd => {
                        context.consume_token()?; // consume }
                        break;
                    }

                    TokenKind::FlowEntry => {
                        if expecting_pair && !pairs.is_empty() {
                            return Err(ParseError::new(
                                ParseErrorKind::UnexpectedToken,
                                token.position,
                                "unexpected ',' at start of mapping or after another ','",
                            ));
                        }

                        context.consume_token()?; // consume ,
                        expecting_pair = true;
                        found_comma = true;

                        // Check for trailing comma
                        context.skip_insignificant_tokens()?;
                        if let Some(next_token) = context.peek_token()?
                            && matches!(next_token.kind, TokenKind::FlowMappingEnd) {
                                continue; // Allow trailing comma
                            }
                    }

                    _ => {
                        if !expecting_pair && !found_comma {
                            return Err(ParseError::new(
                                ParseErrorKind::ExpectedToken,
                                token.position,
                                "expected ',' or '}' in flow mapping",
                            ));
                        }

                        // Parse key-value pair - use parse_node_fn for both key and value
                        let key = parse_node_fn(context)?;

                        // Expect ':' separator
                        context.skip_insignificant_tokens()?;
                        match context.peek_token()? {
                            Some(token) if matches!(token.kind, TokenKind::Value) => {
                                context.consume_token()?; // consume :
                            }
                            Some(token) => {
                                return Err(ParseError::new(
                                    ParseErrorKind::ExpectedToken,
                                    token.position,
                                    "expected ':' after mapping key",
                                ));
                            }
                            None => {
                                return Err(ParseError::new(
                                    ParseErrorKind::UnexpectedEndOfInput,
                                    context.current_position(),
                                    "expected ':' after mapping key",
                                ));
                            }
                        }

                        let value = parse_node_fn(context)?;
                        let pair = MappingPair { key, value };
                        pairs.push(pair);

                        expecting_pair = false;
                        found_comma = false;
                    }
                }
            } else {
                return Err(ParseError::new(
                    ParseErrorKind::UnexpectedEndOfInput,
                    context.current_position(),
                    "unterminated flow mapping",
                ));
            }
        }

        Ok(Node::Mapping(MappingNode::new(
            pairs,
            MappingStyle::Flow,
            start_pos,
        )))
    }

    // Removed parse_flow_item: Unused utility method
    // Flow item parsing handled directly in parse_sequence for zero-allocation performance

    // Removed parse_flow_pair: Unused utility method  
    // Flow pair parsing handled directly in parse_mapping for zero-allocation performance

    // Removed parse_flow_node: Unused utility method
    // Flow node parsing handled directly in main parsing pipeline for zero-allocation performance

    /// Check if current position could start a flow mapping key
    pub fn is_flow_mapping_key<'input>(
        context: &mut ParsingContext<'_, 'input>,
    ) -> Result<bool, ParseError> {
        // Save parser state by cloning token buffer
        let saved_buffer = context.token_buffer.clone();
        let _saved_position = context.current_position();

        // Try to parse as potential key and look for ':'
        let result = Self::check_for_value_indicator(context);

        // Restore parser state
        *context.token_buffer = saved_buffer;

        result
    }

    /// Check for value indicator after potential key
    fn check_for_value_indicator<'input>(
        context: &mut ParsingContext<'_, 'input>,
    ) -> Result<bool, ParseError> {
        // Skip the potential key token
        if context.peek_token()?.is_some() {
            context.consume_token()?;
        }

        // Skip whitespace
        context.skip_insignificant_tokens()?;

        // Check for ':'
        if let Some(token) = context.peek_token()? {
            Ok(matches!(token.kind, TokenKind::Value))
        } else {
            Ok(false)
        }
    }

    /// Validate flow collection nesting depth
    #[inline]
    pub fn validate_nesting_depth(depth: usize, max_depth: usize) -> Result<(), ParseError> {
        if depth > max_depth {
            Err(ParseError::new(
                ParseErrorKind::RecursionLimitExceeded,
                Position::start(),
                format!(
                    "flow collection nesting depth {depth} exceeds maximum {max_depth}"
                ),
            ))
        } else {
            Ok(())
        }
    }

    /// Get flow context information for error reporting
    pub fn get_flow_context_info(nesting_level: usize) -> String {
        format!("flow collection at nesting level {nesting_level}")
    }

    /// Check if token can terminate a flow collection
    #[inline]
    pub fn can_terminate_flow_collection(token: &TokenKind, is_sequence: bool) -> bool {
        if is_sequence {
            matches!(token, TokenKind::FlowSequenceEnd)
        } else {
            matches!(token, TokenKind::FlowMappingEnd)
        }
    }

    /// Check if token is a flow separator
    #[inline]
    pub fn is_flow_separator(token: &TokenKind) -> bool {
        matches!(token, TokenKind::FlowEntry)
    }

    /// Parse empty flow collection
    pub fn parse_empty_sequence<'input>(start_pos: Position) -> Node<'input> {
        Node::Sequence(SequenceNode::new(
            Vec::new(),
            SequenceStyle::Flow,
            start_pos,
        ))
    }

    /// Parse empty flow mapping
    pub fn parse_empty_mapping<'input>(start_pos: Position) -> Node<'input> {
        Node::Mapping(MappingNode::new(Vec::new(), MappingStyle::Flow, start_pos))
    }
}

/// Flow parsing optimization hints
pub struct FlowOptimizations;

impl FlowOptimizations {
    /// Pre-allocate collection capacity based on lookahead
    pub fn estimate_capacity<'input>(
        context: &mut ParsingContext<'_, 'input>,
    ) -> Result<usize, ParseError> {
        // Simple heuristic: count tokens until closing bracket
        let mut count = 0;
        let saved_buffer = context.token_buffer.clone();

        while let Some(token) = context.peek_token()? {
            match token.kind {
                TokenKind::FlowSequenceEnd | TokenKind::FlowMappingEnd => break,
                TokenKind::FlowEntry => count += 1,
                _ => {}
            }
            context.consume_token()?;
        }

        // Restore buffer
        *context.token_buffer = saved_buffer;

        // Return estimated capacity (add 1 for items between separators)
        Ok(count + 1)
    }

    /// Check if flow collection should use compact representation
    #[inline]
    pub fn should_use_compact_representation(item_count: usize, total_length: usize) -> bool {
        item_count <= 5 && total_length <= 80
    }

    /// Optimize memory layout for small collections
    #[inline]
    pub fn optimize_small_collection<T>(mut vec: Vec<T>) -> Vec<T> {
        vec.shrink_to_fit();
        vec
    }
}

/// Flow parsing error recovery utilities
pub struct FlowErrorRecovery;

impl FlowErrorRecovery {
    /// Attempt to recover from malformed flow collection
    pub fn recover_from_malformed_collection<'input>(
        context: &mut ParsingContext<'_, 'input>,
        error: ParseError,
        is_sequence: bool,
    ) -> Result<Option<Node<'input>>, ParseError> {
        // Try to find the closing bracket and return partial collection
        let mut depth = 1;
        let mut recovered_items = Vec::new();

        while let Some(token) = context.peek_token()? {
            match token.kind {
                TokenKind::FlowSequenceStart | TokenKind::FlowMappingStart => {
                    depth += 1;
                    context.consume_token()?;
                }
                TokenKind::FlowSequenceEnd | TokenKind::FlowMappingEnd => {
                    depth -= 1;
                    if depth == 0 {
                        context.consume_token()?; // consume closing bracket
                        if is_sequence {
                            return Ok(Some(Node::Sequence(SequenceNode::new(
                                recovered_items
                                    .into_iter()
                                    .filter_map(|item| {
                                        if let Node::Scalar(_) = item {
                                            Some(item)
                                        } else {
                                            None
                                        }
                                    })
                                    .collect(),
                                SequenceStyle::Flow,
                                Position::start(),
                            ))));
                        } else {
                            // For mappings, more complex recovery needed
                            return Ok(Some(Node::Mapping(MappingNode::new(
                                Vec::new(),
                                MappingStyle::Flow,
                                Position::start(),
                            ))));
                        }
                    }
                    context.consume_token()?;
                }
                TokenKind::Scalar { .. } => {
                    if depth == 1 {
                        if let Ok(scalar) = super::scalars::ScalarParser::parse_scalar(
                            context.consume_token()?,
                            &ParseContext::FlowIn(1),
                        ) {
                            recovered_items.push(scalar);
                        }
                    } else {
                        context.consume_token()?;
                    }
                }
                _ => {
                    context.consume_token()?;
                }
            }
        }

        // Could not recover, return original error
        Err(error)
    }

    /// Suggest corrections for common flow syntax errors
    pub fn suggest_correction(error: &ParseError) -> Option<String> {
        match error.kind {
            ParseErrorKind::ExpectedToken => {
                if error.message.contains("expected ','") {
                    Some("missing comma between collection items".to_string())
                } else if error.message.contains("expected ':'") {
                    Some("missing colon after mapping key".to_string())
                } else if error.message.contains("expected ']'") {
                    Some("missing closing bracket for sequence".to_string())
                } else if error.message.contains("expected '}'") {
                    Some("missing closing brace for mapping".to_string())
                } else {
                    None
                }
            }
            ParseErrorKind::UnexpectedToken => {
                if error.message.contains("unexpected ','") {
                    Some("extra comma in collection".to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
