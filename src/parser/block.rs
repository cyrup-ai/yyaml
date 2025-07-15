use super::{
    IndentationResult, calculate_block_entry_indent, validate_block_mapping_indentation,
    validate_block_sequence_indentation,
};
use super::{Parser, State};
use crate::error::ScanError;
use crate::events::{Event, TScalarStyle, TokenType};

pub fn block_sequence_entry<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
    first: bool,
) -> Result<(Event, crate::error::Marker), ScanError> {
    if first {
        parser.scanner.skip();
    }

    let current_indent = *parser.indents.last().unwrap_or(&0);
    let token = parser.scanner.peek_token()?;
    let token_col = token.0.col;
    let _token_marker = token.0;

    match &token.1 {
        TokenType::BlockEntry => {
            // Ultra-fast indentation validation
            match validate_block_sequence_indentation(&token, current_indent, first) {
                IndentationResult::Continue => {}
                IndentationResult::EndSequence(marker) => {
                    parser.pop_indent();
                    parser.pop_state();
                    return Ok((Event::SequenceEnd, marker));
                }
                IndentationResult::EndMapping(marker) => {
                    parser.pop_indent();
                    parser.pop_state();
                    return Ok((Event::SequenceEnd, marker));
                }
                IndentationResult::InvalidIndentation {
                    found: _,
                    expected: _,
                    marker,
                } => {
                    return Err(ScanError::new(marker, "invalid indentation"));
                }
            }

            let mk = parser.scanner.mark();
            parser.scanner.skip();

            // Calculate next indent position
            let mark_after_dash = parser.scanner.mark();
            let next_token = parser.scanner.peek_token()?;
            let next_token_type = next_token.1.clone();

            if let Some(new_indent) = calculate_block_entry_indent(
                mark_after_dash.line,
                next_token.0.line,
                next_token.0.col,
                current_indent,
                token_col,
            ) {
                parser.push_indent(new_indent);
            }

            match next_token_type {
                TokenType::BlockEntry => {
                    parser.state = State::BlockSequenceEntry;
                    return Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk));
                }
                _ => {
                    parser.push_state(State::BlockSequenceEntry);
                    return parser.parse_node(true, false);
                }
            }
        }
        _ => {
            parser.pop_indent();
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::SequenceEnd, mk))
        }
    }
}

pub fn indentless_sequence_entry<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
) -> Result<(Event, crate::error::Marker), ScanError> {
    let token = parser.scanner.peek_token()?;
    let current_indent = *parser.indents.last().unwrap_or(&0);

    match &token.1 {
        TokenType::BlockEntry => {
            // Fast indentless sequence validation
            match validate_block_sequence_indentation(&token, current_indent, false) {
                IndentationResult::Continue => {}
                IndentationResult::EndSequence(marker) => {
                    parser.pop_indent();
                    parser.pop_state();
                    return Ok((Event::SequenceEnd, marker));
                }
                IndentationResult::EndMapping(marker) => {
                    parser.pop_indent();
                    parser.pop_state();
                    return Ok((Event::SequenceEnd, marker));
                }
                IndentationResult::InvalidIndentation {
                    found: _,
                    expected: _,
                    marker,
                } => {
                    return Err(ScanError::new(marker, "invalid indentation"));
                }
            }

            let mk = parser.scanner.mark();
            parser.scanner.skip();
            match parser.scanner.peek_token()?.1 {
                TokenType::BlockEntry => {
                    parser.state = State::IndentlessSequenceEntry;
                    Ok((
                        Event::Scalar("~".to_string(), TScalarStyle::Plain, 0, None),
                        mk,
                    ))
                }
                _ => {
                    parser.push_state(State::IndentlessSequenceEntry);
                    parser.parse_node(true, false)
                }
            }
        }
        _ => {
            parser.pop_indent();
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::SequenceEnd, mk))
        }
    }
}

pub fn block_mapping_key<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
    first: bool,
) -> Result<(Event, crate::error::Marker), ScanError> {
    if first && parser.first_mapping_key.is_some() {
        // We have a pre-loaded key from parse_node
        if let Some((event, mark)) = parser.first_mapping_key.take() {
            parser.state = State::BlockMappingValue;
            return Ok((event, mark));
        }
        // This should never happen since we checked is_some() above, but handle gracefully
        return Err(ScanError::new(
            crate::error::Marker::default(),
            "Expected first mapping key to be present",
        ));
    }

    // Look for the next key or end of mapping
    let token = parser.scanner.peek_token()?;
    let current_indent = *parser.indents.last().unwrap_or(&0);

    // High-performance mapping key validation
    match validate_block_mapping_indentation(&token, current_indent, true) {
        IndentationResult::Continue => {}
        IndentationResult::EndMapping(marker) => {
            parser.pop_indent();
            parser.pop_state();
            return Ok((Event::MappingEnd, marker));
        }
        IndentationResult::EndSequence(marker) => {
            parser.pop_indent();
            parser.pop_state();
            return Ok((Event::MappingEnd, marker));
        }
        IndentationResult::InvalidIndentation {
            found: _,
            expected: _,
            marker,
        } => {
            return Err(ScanError::new(marker, "invalid indentation"));
        }
    }

    match &token.1 {
        TokenType::Scalar(..) => {
            // Check if this scalar is followed by a colon (making it a key)
            let tok = parser.scanner.fetch_token();
            let (style, val) = match tok.1 {
                TokenType::Scalar(s, v) => (s, v),
                _ => unreachable!(),
            };

            match parser.scanner.peek_token()?.1 {
                TokenType::Value => {
                    parser.state = State::BlockMappingValue;
                    return Ok((Event::Scalar(val, style, 0, None), tok.0));
                }
                _ => {
                    // Scalar not followed by colon - end of mapping
                    parser.pop_state();
                    // Put the scalar back for next parsing
                    parser.current = Some((Event::Scalar(val, style, 0, None), tok.0));
                    let mk = parser.scanner.mark();
                    return Ok((Event::MappingEnd, mk));
                }
            }
        }
        TokenType::StreamEnd | TokenType::DocumentEnd | TokenType::DocumentStart => {
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::MappingEnd, mk))
        }
        _ => {
            // Try to parse a node as the next key
            let saved_state = parser.state;
            parser.push_state(saved_state);
            match parser.parse_node(true, false) {
                Ok((event, mark)) => {
                    // Check if the next token is a colon
                    match parser.scanner.peek_token()?.1 {
                        TokenType::Value => {
                            parser.state = State::BlockMappingValue;
                            Ok((event, mark))
                        }
                        _ => {
                            // Not a mapping key, end the mapping
                            parser.pop_state();
                            parser.current = Some((event, mark));
                            let mk = parser.scanner.mark();
                            Ok((Event::MappingEnd, mk))
                        }
                    }
                }
                Err(_) => {
                    // Failed to parse node, end mapping
                    parser.pop_state();
                    let mk = parser.scanner.mark();
                    Ok((Event::MappingEnd, mk))
                }
            }
        }
    }
}

pub fn block_mapping_value<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
) -> Result<(Event, crate::error::Marker), ScanError> {
    match parser.scanner.peek_token()?.1 {
        TokenType::Value => {
            let _tok = parser.scanner.fetch_token(); // consume the ':'
            // Check what comes after the colon
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Scalar(..) => {
                    let tok = parser.scanner.fetch_token();
                    let (style, val) = match tok.1 {
                        TokenType::Scalar(s, v) => (s, v),
                        _ => unreachable!(),
                    };
                    parser.state = State::BlockMappingKey;
                    return Ok((Event::Scalar(val, style, 0, None), tok.0));
                }
                TokenType::FlowSequenceStart => {
                    parser.push_state(State::BlockMappingKey);
                    return parser.parse_node(false, false);
                }
                TokenType::FlowMappingStart => {
                    parser.push_state(State::BlockMappingKey);
                    return parser.parse_node(false, false);
                }
                _ => {
                    // Empty value - continue to next key
                    parser.state = State::BlockMappingKey;
                    let mk = parser.scanner.mark();
                    return Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk));
                }
            }
        }
        _ => {
            // No colon found - this shouldn't happen in well-formed YAML
            parser.state = State::BlockMappingKey;
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk))
        }
    }
}
