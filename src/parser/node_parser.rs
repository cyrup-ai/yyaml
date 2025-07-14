use crate::error::{Marker, ScanError};
use crate::events::{Event, TScalarStyle, TokenType};
use crate::parser::Parser;
use super::state_machine::State;

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
            return Ok((Event::SequenceStart(anchor_id), mk));
        }
        TokenType::BlockEntry if block => {
            let col = token.0.col;
            let mk = token.0;
            parser.push_indent(col);
            parser.state = State::BlockSequenceFirstEntry;
            return Ok((Event::SequenceStart(anchor_id), mk));
        }
        TokenType::Scalar(..) => {
            if block {
                // Check if this scalar is followed by a colon - if so, it's a mapping
                let current_mark = parser.scanner.mark();
                let token = parser.scanner.fetch_token();
                let (style, val) = match token.1 {
                    TokenType::Scalar(s, v) => (s, v),
                    _ => unreachable!(),
                };

                // Check if next token is a colon
                if let Ok(next_tok) = parser.scanner.peek_token() {
                    if matches!(next_tok.1, TokenType::Value) {
                        // Found colon - this is a mapping
                        // Store the scalar key for block_mapping_key to retrieve
                        parser.first_mapping_key =
                            Some((Event::Scalar(val, style, anchor_id, tag), current_mark));
                        let col = current_mark.col;
                        parser.push_indent(col);
                        parser.state = State::BlockMappingFirstKey;
                        return Ok((Event::MappingStart(anchor_id), current_mark));
                    }
                }

                // No colon found - return scalar directly
                parser.pop_state();
                return Ok((Event::Scalar(val, style, anchor_id, tag), current_mark));
            } else {
                let tok = parser.scanner.fetch_token();
                let (style, val) = match tok.1 {
                    TokenType::Scalar(s, v) => (s, v),
                    _ => unreachable!(),
                };
                parser.pop_state();
                return Ok((Event::Scalar(val, style, anchor_id, tag), tok.0));
            }
        }
        TokenType::FlowSequenceStart => {
            parser.state = State::FlowSequenceFirstEntry;
            let tok = parser.scanner.fetch_token();
            return Ok((Event::SequenceStart(anchor_id), tok.0));
        }
        TokenType::FlowMappingStart => {
            parser.state = State::FlowMappingFirstKey;
            let tok = parser.scanner.fetch_token();
            return Ok((Event::MappingStart(anchor_id), tok.0));
        }
        _ => {
            if anchor_id > 0 || tag.is_some() {
                parser.pop_state();
                let mk = parser.scanner.mark();
                return Ok((
                    Event::Scalar("".to_string(), TScalarStyle::Plain, anchor_id, tag),
                    mk,
                ));
            }
            let mk = parser.scanner.mark();
            Err(ScanError::new(mk, "did not find expected node content"))
        }
    }
}
