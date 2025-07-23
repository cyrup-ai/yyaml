use super::{
    IndentationResult, calculate_block_entry_indent, validate_block_mapping_indentation,
    validate_block_sequence_indentation,
};
use super::{Parser, State};
use crate::error::ScanError;
use crate::events::{Event, TScalarStyle, TokenType};
use log::{debug, trace};

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
                    // Empty sequence item followed by another BlockEntry
                    parser.state = State::BlockSequenceEntry;
                    Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk))
                }
                _ => {
                    // Parse the content after the dash
                    parser.push_state(State::BlockSequenceEntry);
                    parser.parse_node(true, false)
                }
            }
        }
        TokenType::StreamEnd | TokenType::DocumentEnd | TokenType::DocumentStart => {
            parser.pop_indent();
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::SequenceEnd, mk))
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
            crate::error::Marker::new(),
            "Expected first mapping key to be present",
        ));
    }

    // Look for the next key or end of mapping
    let token = parser.scanner.peek_token()?;
    let current_indent = *parser.indents.last().unwrap_or(&0);
    
    // High-performance mapping key validation
    let indent_result = validate_block_mapping_indentation(&token, current_indent, true);
    
    match indent_result {
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
                    // Keys typically don't have anchors in YAML, but handle consistently
                    Ok((Event::Scalar(val, style, 0, None), tok.0))
                }
                _ => {
                    // Scalar not followed by colon - end of mapping
                    parser.pop_state();
                    // Put the scalar back for next parsing
                    parser.current = Some((Event::Scalar(val, style, 0, None), tok.0));
                    let mk = parser.scanner.mark();
                    Ok((Event::MappingEnd, mk))
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
                    let next_token = parser.scanner.peek_token()?;
                    match next_token.1 {
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
    let token = parser.scanner.peek_token()?;
    
    match token.1 {
        TokenType::Value => {
            let _tok = parser.scanner.fetch_token(); // consume the ':'
            
            // Parse anchors, aliases, and tags before the actual value (like parse_node does)
            let mut anchor_id = 0;
            let mut tag = None;
            
            loop {
                let token = parser.scanner.peek_token()?;
                trace!("Processing token in anchor loop: {:?}", token.1);
                match &token.1 {
                    TokenType::Alias(_) => {
                        let tok = parser.scanner.fetch_token();
                        let name = match tok.1 {
                            TokenType::Alias(n) => n,
                            _ => unreachable!(),
                        };
                        debug!("Found alias: {name}");
                        if let Some(aid) = parser.anchors.get(&name) {
                            parser.state = State::BlockMappingKey;
                            debug!("Resolved alias {name} to id {aid}, transitioning to BlockMappingKey");
                            return Ok((Event::Alias(*aid), tok.0));
                        } else {
                            parser.state = State::BlockMappingKey;
                            debug!("Unresolved alias {name}, using fallback id, transitioning to BlockMappingKey");
                            return Ok((Event::Alias(9999999), tok.0));
                        }
                    }
                    TokenType::Anchor(_) => {
                        let tok = parser.scanner.fetch_token();
                        let name = match tok.1 {
                            TokenType::Anchor(n) => n,
                            _ => unreachable!(),
                        };
                        anchor_id = parser.register_anchor(name.clone());
                        debug!("Registered anchor {name} with id {anchor_id}");
                    }
                    TokenType::Tag(..) => {
                        let tok = parser.scanner.fetch_token();
                        tag = Some(tok.1);
                        debug!("Found tag: {tag:?}");
                    }
                    _ => {
                        trace!("Exiting anchor loop");
                        break;
                    }
                }
            }
            
            // Now parse the actual value - expect a scalar after anchor processing
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Scalar(..) => {
                    let tok = parser.scanner.fetch_token();
                    let (style, val) = match tok.1 {
                        TokenType::Scalar(s, v) => (s, v),
                        _ => unreachable!(),
                    };
                    parser.state = State::BlockMappingKey;
                    Ok((Event::Scalar(val, style, anchor_id, tag), tok.0))
                }
                _ => {
                    // For complex values (mappings, sequences), delegate to parse_node
                    parser.push_state(State::BlockMappingKey);
                    match parser.parse_node(true, false) {
                        Ok((mut event, mark)) => {
                            // Apply the collected anchor and tag to whatever parse_node returned
                            match &mut event {
                                Event::MappingStart(aid) => {
                                    *aid = anchor_id;
                                    debug!("Applied anchor_id={anchor_id} to MappingStart");
                                }
                                Event::SequenceStart(aid) => {
                                    *aid = anchor_id;
                                    debug!("Applied anchor_id={anchor_id} to SequenceStart");
                                }
                                Event::Scalar(_, _, aid, etag) => {
                                    *aid = anchor_id;
                                    *etag = tag;
                                    debug!("Applied anchor_id={anchor_id} and tag to Scalar");
                                }
                                _ => {
                                    debug!("parse_node returned {event:?}, no anchor/tag to apply");
                                }
                            }
                            Ok((event, mark))
                        }
                        Err(err) => {
                            debug!("parse_node failed: {err:?}");
                            Err(err)
                        }
                    }
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
