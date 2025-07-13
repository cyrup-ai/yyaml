use crate::error::{ScanError, Marker};
use crate::events::{Event, TScalarStyle, TokenType, MarkedEventReceiver};
use crate::scanner::Scanner;
use std::collections::HashMap;

mod loader;
mod block;
mod flow;
mod document;

pub use loader::YamlLoader;

/// Parser states
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum State {
    StreamStart,
    ImplicitDocumentStart,
    DocumentStart,
    DocumentContent,
    DocumentEnd,
    BlockNode,
    BlockSequenceFirstEntry,
    BlockSequenceEntry,
    IndentlessSequenceEntry,
    BlockMappingFirstKey,
    BlockMappingKey,
    BlockMappingValue,
    FlowSequenceFirstEntry,
    FlowSequenceEntry,
    FlowSequenceEntryMappingKey,
    FlowSequenceEntryMappingValue,
    FlowSequenceEntryMappingEnd,
    FlowMappingFirstKey,
    FlowMappingKey,
    FlowMappingValue,
    FlowMappingEmptyValue,
    End,
}

/// The parser struct, scanning char-by-char, producing tokens, then events.
pub struct Parser<T: Iterator<Item = char>> {
    pub scanner: Scanner<T>,
    pub states: Vec<State>,
    pub state: State,
    pub current: Option<(Event, Marker)>,
    pub anchors: HashMap<String, usize>,
    pub anchor_id: usize,
}

impl<T: Iterator<Item = char>> Parser<T> {
    pub fn new(src: T) -> Self {
        Parser {
            scanner: Scanner::new(src),
            states: Vec::new(),
            state: State::StreamStart,
            current: None,
            anchors: HashMap::new(),
            anchor_id: 1,
        }
    }
    
    pub fn pop_state(&mut self) {
        self.state = self.states.pop().unwrap();
    }
    
    pub fn push_state(&mut self, st: State) {
        self.states.push(st);
    }
    
    pub fn parse(&mut self) -> Result<(Event, Marker), ScanError> {
        if self.state == State::End {
            return Ok((Event::StreamEnd, self.scanner.mark()));
        }
        let (ev, mk) = self.state_machine()?;
        Ok((ev, mk))
    }
    
    fn state_machine(&mut self) -> Result<(Event, Marker), ScanError> {
        match self.state {
            State::StreamStart => self.stream_start(),
            State::ImplicitDocumentStart => document::document_start(self, true),
            State::DocumentStart => document::document_start(self, false),
            State::DocumentContent => document::document_content(self),
            State::DocumentEnd => document::document_end(self),
            State::BlockNode => self.parse_node(true, false),
            State::BlockSequenceFirstEntry => block::block_sequence_entry(self, true),
            State::BlockSequenceEntry => block::block_sequence_entry(self, false),
            State::IndentlessSequenceEntry => block::indentless_sequence_entry(self),
            State::BlockMappingFirstKey => block::block_mapping_key(self, true),
            State::BlockMappingKey => block::block_mapping_key(self, false),
            State::BlockMappingValue => block::block_mapping_value(self),
            State::FlowSequenceFirstEntry => flow::flow_sequence_entry(self, true),
            State::FlowSequenceEntry => flow::flow_sequence_entry(self, false),
            State::FlowSequenceEntryMappingKey => flow::flow_sequence_entry_mapping_key(self),
            State::FlowSequenceEntryMappingValue => flow::flow_sequence_entry_mapping_value(self),
            State::FlowSequenceEntryMappingEnd => flow::flow_sequence_entry_mapping_end(self),
            State::FlowMappingFirstKey => flow::flow_mapping_key(self, true),
            State::FlowMappingKey => flow::flow_mapping_key(self, false),
            State::FlowMappingValue => flow::flow_mapping_value(self, false),
            State::FlowMappingEmptyValue => flow::flow_mapping_value(self, true),
            State::End => unreachable!(),
        }
    }

    pub fn next(&mut self) -> Result<(Event, Marker), ScanError> {
        match self.current.take() {
            Some(x) => Ok(x),
            None => self.parse(),
        }
    }
    
    pub fn load<R: MarkedEventReceiver>(&mut self, recv: &mut R, multi: bool) -> Result<(), ScanError> {
        loader::load(self, recv, multi)
    }
    
    pub fn register_anchor(&mut self, name: String) -> usize {
        let new_id = self.anchor_id;
        self.anchor_id += 1;
        self.anchors.insert(name, new_id);
        new_id
    }

    fn stream_start(&mut self) -> Result<(Event, Marker), ScanError> {
        let t = self.scanner.peek_token()?;
        match t.1 {
            TokenType::StreamStart(_) => {
                self.state = State::ImplicitDocumentStart;
                let tok = self.scanner.fetch_token();
                Ok((Event::StreamStart, tok.0))
            }
            _ => Err(ScanError::new(
                t.0,
                "did not find expected <stream-start>"
            )),
        }
    }
    
    fn parse_node(&mut self, block: bool, indentless_seq: bool) -> Result<(Event, Marker), ScanError> {
        let mut anchor_id = 0;
        let mut tag = None;
        
        // parse optional anchor/tag
        loop {
            let token = self.scanner.peek_token()?;
            match &token.1 {
                TokenType::Alias(_) => {
                    self.pop_state();
                    let tok = self.scanner.fetch_token();
                    let name = match tok.1 {
                        TokenType::Alias(n) => n,
                        _ => unreachable!(),
                    };
                    if let Some(aid) = self.anchors.get(&name) {
                        return Ok((Event::Alias(*aid), tok.0));
                    } else {
                        return Ok((Event::Alias(9999999), tok.0));
                    }
                }
                TokenType::Anchor(_) => {
                    let tok = self.scanner.fetch_token();
                    let name = match tok.1 {
                        TokenType::Anchor(n) => n,
                        _ => unreachable!(),
                    };
                    anchor_id = self.register_anchor(name);
                }
                TokenType::Tag(..) => {
                    let tok = self.scanner.fetch_token();
                    tag = Some(tok.1);
                }
                _ => break,
            }
        }
        
        // Handle different node types
        let token = self.scanner.peek_token()?;
        match &token.1 {
            TokenType::BlockEntry if indentless_seq => {
                self.state = State::IndentlessSequenceEntry;
                let mk = self.scanner.mark();
                return Ok((Event::SequenceStart(anchor_id), mk));
            }
            TokenType::BlockEntry if block => {
                self.state = State::BlockSequenceFirstEntry;
                let mk = self.scanner.mark();
                return Ok((Event::SequenceStart(anchor_id), mk));
            }
            TokenType::Scalar(..) => {
                // Check if this is the start of a block mapping
                if block {
                    // Look ahead to see if there's a colon after this scalar
                    let scalar_tok = self.scanner.fetch_token();
                    let (style, val) = match scalar_tok.1 {
                        TokenType::Scalar(s, v) => (s, v),
                        _ => unreachable!(),
                    };
                    
                    if let Ok(next_tok) = self.scanner.peek_token() {
                        if matches!(next_tok.1, TokenType::Value) {
                            // This is a block mapping! 
                            self.push_state(self.state); // Push current state before changing
                            self.state = State::BlockMappingFirstKey;
                            let mk = scalar_tok.0;
                            // Store this first key for the mapping
                            self.current = Some((Event::Scalar(val, style, anchor_id, tag), mk));
                            return Ok((Event::MappingStart(anchor_id), mk));
                        }
                    }
                    // Not a mapping, treat as regular scalar
                    self.pop_state();
                    return Ok((Event::Scalar(val, style, anchor_id, tag), scalar_tok.0));
                } else {
                    // Not in block context, treat as regular scalar
                    let tok = self.scanner.fetch_token();
                    let (style, val) = match tok.1 {
                        TokenType::Scalar(s, v) => (s, v),
                        _ => unreachable!(),
                    };
                    self.pop_state();
                    return Ok((Event::Scalar(val, style, anchor_id, tag), tok.0));
                }
            }
            TokenType::FlowSequenceStart => {
                self.state = State::FlowSequenceFirstEntry;
                let tok = self.scanner.fetch_token();
                return Ok((Event::SequenceStart(anchor_id), tok.0));
            }
            TokenType::FlowMappingStart => {
                self.state = State::FlowMappingFirstKey;
                let tok = self.scanner.fetch_token();
                return Ok((Event::MappingStart(anchor_id), tok.0));
            }
            _ => {
                if anchor_id > 0 || tag.is_some() {
                    self.pop_state();
                    let mk = self.scanner.mark();
                    return Ok((Event::Scalar("".to_string(), TScalarStyle::Plain, anchor_id, tag), mk));
                }
                let mk = self.scanner.mark();
                Err(ScanError::new(mk, "did not find expected node content"))
            }
        }
    }
} 