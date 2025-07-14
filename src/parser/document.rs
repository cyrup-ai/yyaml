use super::{Parser, State};
use crate::error::ScanError;
use crate::events::{Event, TScalarStyle, TokenType};

pub fn document_start<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
    implicit: bool,
) -> Result<(Event, crate::error::Marker), ScanError> {
    if !implicit {
        while let TokenType::DocumentEnd = parser.scanner.peek_token()?.1 {
            parser.scanner.skip();
        }
    }
    match parser.scanner.peek_token()?.1 {
        TokenType::StreamEnd => {
            parser.state = State::End;
            let tok = parser.scanner.fetch_token();
            Ok((Event::StreamEnd, tok.0))
        }
        TokenType::VersionDirective(..)
        | TokenType::TagDirective(..)
        | TokenType::DocumentStart => explicit_document_start(parser),
        _ if implicit => {
            // For implicit documents, don't look for directives, just start parsing content
            parser.push_state(State::DocumentEnd);
            parser.state = State::BlockNode;
            let mk = parser.scanner.mark();
            Ok((Event::DocumentStart, mk))
        }
        _ => explicit_document_start(parser),
    }
}

fn explicit_document_start<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
) -> Result<(Event, crate::error::Marker), ScanError> {
    process_directives(parser)?;
    match parser.scanner.peek_token()?.1 {
        TokenType::DocumentStart => {
            parser.push_state(State::DocumentEnd);
            parser.state = State::DocumentContent;
            let tok = parser.scanner.fetch_token();
            Ok((Event::DocumentStart, tok.0))
        }
        _ => {
            let mk = parser.scanner.mark();
            Err(ScanError::new(mk, "did not find expected <document start>"))
        }
    }
}

fn process_directives<T: Iterator<Item = char>>(parser: &mut Parser<T>) -> Result<(), ScanError> {
    loop {
        match parser.scanner.peek_token()?.1 {
            TokenType::VersionDirective(..) => {
                // skip
            }
            TokenType::TagDirective(..) => {
                // skip
            }
            _ => break,
        }
        parser.scanner.skip();
    }
    Ok(())
}

pub fn document_content<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
) -> Result<(Event, crate::error::Marker), ScanError> {
    match parser.scanner.peek_token()?.1 {
        TokenType::VersionDirective(..)
        | TokenType::TagDirective(..)
        | TokenType::DocumentStart
        | TokenType::DocumentEnd
        | TokenType::StreamEnd => {
            parser.pop_state();
            let mk = parser.scanner.mark();
            Ok((Event::Scalar("~".into(), TScalarStyle::Plain, 0, None), mk))
        }
        _ => parser.parse_node(true, false),
    }
}

pub fn document_end<T: Iterator<Item = char>>(
    parser: &mut Parser<T>,
) -> Result<(Event, crate::error::Marker), ScanError> {
    let mk = parser.scanner.mark();
    while let TokenType::DocumentEnd = parser.scanner.peek_token()?.1 {
        parser.scanner.skip();
    }

    // Check if there's more content - if so, we need to continue with implicit documents
    match parser.scanner.peek_token()?.1 {
        TokenType::StreamEnd => {
            parser.state = State::End;
        }
        TokenType::DocumentStart => {
            parser.state = State::DocumentStart;
        }
        _ => {
            // There's more content, so continue with implicit document parsing
            parser.state = State::ImplicitDocumentStart;
        }
    }

    Ok((Event::DocumentEnd, mk))
}
