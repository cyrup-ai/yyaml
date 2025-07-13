use crate::error::ScanError;
use crate::events::{Event, TScalarStyle, TokenType};
use super::{Parser, State};

pub fn block_sequence_entry<T: Iterator<Item = char>>(
    parser: &mut Parser<T>, 
    first: bool
) -> Result<(Event, crate::error::Marker), ScanError> {
    if first {
        parser.scanner.skip();
    }
    match parser.scanner.peek_token()?.1 {
        TokenType::BlockEntry => {
            let mk = parser.scanner.mark();
            parser.scanner.skip();
            match parser.scanner.peek_token()?.1 {
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
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::SequenceEnd, mk))
        }
    }
}

pub fn indentless_sequence_entry<T: Iterator<Item = char>>(
    parser: &mut Parser<T>
) -> Result<(Event, crate::error::Marker), ScanError> {
    match parser.scanner.peek_token()?.1 {
        TokenType::BlockEntry => {
            let mk = parser.scanner.mark();
            parser.scanner.skip();
            match parser.scanner.peek_token()?.1 {
                TokenType::BlockEntry => {
                    parser.state = State::IndentlessSequenceEntry;
                    Ok((Event::Scalar("~".to_string(), TScalarStyle::Plain, 0, None), mk))
                }
                _ => {
                    parser.push_state(State::IndentlessSequenceEntry);
                    parser.parse_node(true, false)
                }
            }
        }
        _ => {
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::SequenceEnd, mk))
        }
    }
}

pub fn block_mapping_key<T: Iterator<Item = char>>(
    parser: &mut Parser<T>, 
    first: bool
) -> Result<(Event, crate::error::Marker), ScanError> {
    println!("DEBUG: block_mapping_key called with first={}, current.is_some()={}", first, parser.current.is_some());
    
    if first && parser.current.is_some() {
        // We have a pre-loaded key from parse_node
        let (event, mark) = parser.current.take().unwrap();
        println!("DEBUG: Returning stored key, setting state to BlockMappingValue");
        parser.state = State::BlockMappingValue;
        return Ok((event, mark));
    }
    
    println!("DEBUG: Looking for next key, peeking at token");
    // Look for the next key or end of mapping
    let token = parser.scanner.peek_token()?;
    println!("DEBUG: Peeked token: {:?}", token);
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
                    println!("DEBUG: Found scalar+colon, setting state to BlockMappingValue");
                    parser.state = State::BlockMappingValue;
                    return Ok((Event::Scalar(val, style, 0, None), tok.0));
                }
                _ => {
                    // Scalar not followed by colon - end of mapping
                    println!("DEBUG: Scalar not followed by colon, ending mapping");
                    parser.pop_state();
                    // Put the scalar back for next parsing
                    parser.current = Some((Event::Scalar(val, style, 0, None), tok.0));
                    let mk = parser.scanner.mark();
                    return Ok((Event::MappingEnd, mk));
                }
            }
        }
        TokenType::StreamEnd | TokenType::DocumentEnd | TokenType::DocumentStart => {
            println!("DEBUG: Found document/stream token, ending mapping");
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::MappingEnd, mk))
        }
        _ => {
            println!("DEBUG: Found other token, ending mapping");
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::MappingEnd, mk))
        }
    }
}

pub fn block_mapping_value<T: Iterator<Item = char>>(
    parser: &mut Parser<T>
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