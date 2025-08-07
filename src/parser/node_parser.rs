use super::state_machine::{State, execute_state_machine};
use crate::error::{Marker, ScanError};
use crate::events::{Event, TScalarStyle, TokenType};
use crate::parser::Parser;

pub fn parse_node<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
    block: bool,
    indentless_seq: bool,
) -> Result<(Event, Marker), ScanError> {
    let mut anchor_id = 0;
    let mut tag = None;

    loop {
        let token = parser.scanner.peek_token()?;
        match &token.1 {
            TokenType::Alias(_) => {
                parser.pop_state();
                let tok = parser.scanner.fetch_token();
                let name = match tok.1 {
                    TokenType::Alias(n) => n,
                    _ => unreachable!(),
                };
                if let Some(aid) = parser.anchors.get(&name) {
                    return Ok((Event::Alias(*aid), tok.0));
                } else {
                    return Ok((Event::Alias(9999999), tok.0));
                }
            }
            TokenType::Anchor(_) => {
                let tok = parser.scanner.fetch_token();
                let name = match tok.1 {
                    TokenType::Anchor(n) => n,
                    _ => unreachable!(),
                };
                anchor_id = parser.register_anchor(name);
            }
            TokenType::Tag(..) => {
                let tok = parser.scanner.fetch_token();
                tag = Some(tok.1);
            }
            _ => break,
        }
    }

    let token = parser.scanner.peek_token()?;
    match &token.1 {
        TokenType::BlockEntry if indentless_seq => {
            let col = token.0.col;
            let mk = token.0;
            parser.push_indent(col);
            parser.state = State::IndentlessSequenceEntry;
            Ok((Event::SequenceStart(anchor_id), mk))
        }
        TokenType::BlockEntry if block => {
            let col = token.0.col;
            let mk = token.0;
            parser.push_indent(col);
            parser.state = State::BlockSequenceFirstEntry;
            Ok((Event::SequenceStart(anchor_id), mk))
        }
        TokenType::Scalar(..) => {
            if block {
                // Check if this scalar is followed by a colon - if so, it's a mapping
                let token = parser.scanner.fetch_token();
                let token_mark = token.0;
                let (style, val) = match token.1 {
                    TokenType::Scalar(s, v) => (s, v),
                    _ => unreachable!(),
                };

                // Check if next token is a colon
                if let Ok(next_tok) = parser.scanner.peek_token()
                    && matches!(next_tok.1, TokenType::Value) {
                        // Found colon - this is a mapping
                        // Store the scalar key for block_mapping_key to retrieve
                        let col = token_mark.col;
                        parser.first_mapping_key =
                            Some((Event::Scalar(val.clone(), style, anchor_id, tag), token_mark));
                        parser.push_indent(col);
                        parser.state = State::BlockMappingFirstKey;
                        return Ok((Event::MappingStart(anchor_id), token_mark));
                    }

                // No colon found - return scalar directly
                parser.pop_state();
                Ok((Event::Scalar(val, style, anchor_id, tag), token_mark))
            } else {
                let tok = parser.scanner.fetch_token();
                let (style, val) = match tok.1 {
                    TokenType::Scalar(s, v) => (s, v),
                    _ => unreachable!(),
                };
                parser.pop_state();
                Ok((Event::Scalar(val, style, anchor_id, tag), tok.0))
            }
        }
        TokenType::FlowSequenceStart => {
            parser.state = State::FlowSequenceFirstEntry;
            let tok = parser.scanner.fetch_token();
            Ok((Event::SequenceStart(anchor_id), tok.0))
        }
        TokenType::FlowMappingStart => {
            parser.state = State::FlowMappingFirstKey;
            let tok = parser.scanner.fetch_token();
            Ok((Event::MappingStart(anchor_id), tok.0))
        }
        TokenType::StreamEnd | TokenType::DocumentEnd | TokenType::DocumentStart => {
            // Document boundaries - treat as empty content
            if anchor_id > 0 || tag.is_some() {
                parser.pop_state();
                let mk = parser.scanner.mark();
                return Ok((
                    Event::Scalar("".to_string(), TScalarStyle::Plain, anchor_id, tag),
                    mk,
                ));
            }
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("".to_string(), TScalarStyle::Plain, 0, None), mk))
        }
        TokenType::BlockEnd => {
            // Block end - treat as empty content or end current structure
            if anchor_id > 0 || tag.is_some() {
                parser.pop_state();
                let mk = parser.scanner.mark();
                return Ok((
                    Event::Scalar("".to_string(), TScalarStyle::Plain, anchor_id, tag),
                    mk,
                ));
            }
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("".to_string(), TScalarStyle::Plain, 0, None), mk))
        }
        TokenType::Key => {
            // Unexpected key token - treat as implicit mapping start in block context
            if block {
                let mk = parser.scanner.mark();
                parser.push_indent(mk.col);
                parser.state = State::BlockMappingFirstKey;
                return Ok((Event::MappingStart(anchor_id), mk));
            } else {
                // In flow context, treat as empty scalar
                parser.pop_state();
                let mk = parser.scanner.mark();
                Ok((Event::Scalar("".to_string(), TScalarStyle::Plain, anchor_id, tag), mk))
            }
        }
        TokenType::Value => {
            // Colon without preceding key - treat as empty key in mapping
            if block {
                let mk = parser.scanner.mark();
                parser.push_indent(mk.col);
                parser.state = State::BlockMappingFirstKey;
                parser.first_mapping_key = Some((Event::Scalar("".to_string(), TScalarStyle::Plain, anchor_id, tag), mk));
                return Ok((Event::MappingStart(anchor_id), mk));
            } else {
                parser.pop_state();
                let mk = parser.scanner.mark();
                Ok((Event::Scalar("".to_string(), TScalarStyle::Plain, anchor_id, tag), mk))
            }
        }
        TokenType::FlowEntry => {
            // Flow entry without context - treat as empty scalar
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("".to_string(), TScalarStyle::Plain, anchor_id, tag), mk))
        }
        TokenType::FlowSequenceEnd | TokenType::FlowMappingEnd => {
            // Flow end markers - treat as empty content
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("".to_string(), TScalarStyle::Plain, anchor_id, tag), mk))
        }
        _ => {
            // For any remaining unhandled token types (directives, reserved, etc.)
            if anchor_id > 0 || tag.is_some() {
                parser.pop_state();
                let mk = parser.scanner.mark();
                return Ok((
                    Event::Scalar("".to_string(), TScalarStyle::Plain, anchor_id, tag),
                    mk,
                ));
            }
            // As a last resort, treat unknown tokens as empty scalars instead of erroring
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("".to_string(), TScalarStyle::Plain, 0, None), mk))
        }
    }
}

// Removed parse_node_block_context - it was causing infinite recursion
// The state machine already handles all the proper YAML 1.2 compliance
