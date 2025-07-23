use super::{Parser, State};
use crate::error::ScanError;
use crate::events::{Event, TScalarStyle, TokenType};

pub fn flow_sequence_entry<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
    first: bool,
) -> Result<(Event, crate::error::Marker), ScanError> {
    if first {
        if parser.scanner.peek_token()?.1 == TokenType::FlowSequenceEnd {
            parser.pop_state();
            let tok = parser.scanner.fetch_token();
            return Ok((Event::SequenceEnd, tok.0));
        }
    } else {
        match parser.scanner.peek_token()?.1 {
            TokenType::FlowEntry => {
                parser.scanner.skip();
            }
            TokenType::FlowSequenceEnd => {
                parser.pop_state();
                let tok = parser.scanner.fetch_token();
                return Ok((Event::SequenceEnd, tok.0));
            }
            _ => {
                let mk = parser.scanner.mark();
                return Err(ScanError::new(mk, "did not find expected ',' or ']'"));
            }
        }
    }

    match parser.scanner.peek_token()?.1 {
        TokenType::FlowSequenceEnd => {
            parser.pop_state();
            let tok = parser.scanner.fetch_token();
            Ok((Event::SequenceEnd, tok.0))
        }
        TokenType::Key => {
            parser.state = State::FlowSequenceEntryMappingKey;
            let tok = parser.scanner.fetch_token();
            Ok((Event::MappingStart(0), tok.0))
        }
        _ => {
            parser.push_state(State::FlowSequenceEntry);
            parser.parse_node(false, false)
        }
    }
}

pub fn flow_sequence_entry_mapping_key<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
) -> Result<(Event, crate::error::Marker), ScanError> {
    match parser.scanner.peek_token()?.1 {
        TokenType::Value | TokenType::FlowEntry | TokenType::FlowSequenceEnd => {
            parser.state = State::FlowSequenceEntryMappingValue;
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk))
        }
        _ => {
            parser.push_state(State::FlowSequenceEntryMappingValue);
            parser.parse_node(false, false)
        }
    }
}

pub fn flow_sequence_entry_mapping_value<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
) -> Result<(Event, crate::error::Marker), ScanError> {
    match parser.scanner.peek_token()?.1 {
        TokenType::Value => {
            parser.scanner.skip();
            match parser.scanner.peek_token()?.1 {
                TokenType::FlowEntry | TokenType::FlowSequenceEnd => {
                    parser.state = State::FlowSequenceEntryMappingEnd;
                    let mk = parser.scanner.mark();
                    Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk))
                }
                _ => {
                    parser.push_state(State::FlowSequenceEntryMappingEnd);
                    parser.parse_node(false, false)
                }
            }
        }
        _ => {
            parser.state = State::FlowSequenceEntryMappingEnd;
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk))
        }
    }
}

pub fn flow_sequence_entry_mapping_end<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
) -> Result<(Event, crate::error::Marker), ScanError> {
    parser.state = State::FlowSequenceEntry;
    let mk = parser.scanner.mark();
    Ok((Event::MappingEnd, mk))
}

pub fn flow_mapping_key<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
    first: bool,
) -> Result<(Event, crate::error::Marker), ScanError> {
    if first {
        if parser.scanner.peek_token()?.1 == TokenType::FlowMappingEnd {
            parser.pop_state();
            let tok = parser.scanner.fetch_token();
            return Ok((Event::MappingEnd, tok.0));
        }
    } else {
        match parser.scanner.peek_token()?.1 {
            TokenType::FlowEntry => {
                parser.scanner.skip();
            }
            TokenType::FlowMappingEnd => {
                parser.pop_state();
                let tok = parser.scanner.fetch_token();
                return Ok((Event::MappingEnd, tok.0));
            }
            _ => {
                let mk = parser.scanner.mark();
                return Err(ScanError::new(mk, "did not find expected ',' or '}'"));
            }
        }
    }

    match parser.scanner.peek_token()?.1 {
        TokenType::FlowMappingEnd => {
            parser.pop_state();
            let tok = parser.scanner.fetch_token();
            Ok((Event::MappingEnd, tok.0))
        }
        TokenType::Key => {
            parser.scanner.skip();
            match parser.scanner.peek_token()?.1 {
                TokenType::Value | TokenType::FlowEntry | TokenType::FlowMappingEnd => {
                    parser.state = State::FlowMappingValue;
                    let mk = parser.scanner.mark();
                    Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk))
                }
                _ => {
                    parser.push_state(State::FlowMappingValue);
                    parser.parse_node(false, false)
                }
            }
        }
        TokenType::Value => {
            parser.state = State::FlowMappingEmptyValue;
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk))
        }
        _ => {
            parser.push_state(State::FlowMappingValue);
            parser.parse_node(false, false)
        }
    }
}

pub fn flow_mapping_value<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
    empty: bool,
) -> Result<(Event, crate::error::Marker), ScanError> {
    if empty {
        parser.state = State::FlowMappingKey;
        let mk = parser.scanner.mark();
        return Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk));
    }

    match parser.scanner.peek_token()?.1 {
        TokenType::Value => {
            parser.scanner.skip();
            match parser.scanner.peek_token()?.1 {
                TokenType::FlowEntry | TokenType::FlowMappingEnd => {
                    parser.state = State::FlowMappingKey;
                    let mk = parser.scanner.mark();
                    Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk))
                }
                _ => {
                    parser.push_state(State::FlowMappingKey);
                    parser.parse_node(false, false)
                }
            }
        }
        _ => {
            parser.state = State::FlowMappingKey;
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk))
        }
    }
}
