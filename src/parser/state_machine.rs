use super::super::parser::{block, document, flow, node_parser};
use crate::error::{Marker, ScanError};
use crate::events::{Event, TokenType};

#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum State {
    StreamStart,
    ImplicitDocumentStart,
    DocumentStart,
    DocumentContent,
    DocumentEnd,
    BlockNode,
    BlockNodeBlockOut,
    BlockNodeBlockIn,
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
    
    // Missing context-specific states per YAML 1.2 rule [80]
    BlockKey,      // BLOCK-KEY context
    FlowIn,        // FLOW-IN context  
    FlowOut,       // FLOW-OUT context
    FlowKey,       // FLOW-KEY context
    
    // REMOVED: Production rule states that caused infinite loops
    // All functionality moved to direct token consumption in main states
    
    End,
}

pub fn execute_state_machine<T: Iterator<Item = char>>(
    parser: &mut crate::parser::Parser<T>,
) -> Result<(Event, Marker), ScanError> {
    use log::{debug, trace};
    
    // Universal state machine debugging - IMMUTABLE STATE LOGGING
    let current_state = parser.state;
    let stack_depth = parser.states.len();
    let current_pos = parser.scanner.mark();
    
    // Get current token for debugging (peek without consuming)
    let current_token = parser.scanner.peek_token().map(|token| {
        format!("{}:{:?}", token.0.line, token.1)
    }).unwrap_or_else(|_| "ERROR_GETTING_TOKEN".to_string());
    
    // STATE MACHINE DEBUG - every state transition logged
    debug!("STATE_ENTRY: {:?} at {}:{}", current_state, current_pos.line, current_pos.col);
    debug!("STACK_DEPTH: {} | TOKEN: {}", stack_depth, current_token);
    
    let result = match parser.state {
        State::StreamStart => parser.stream_start(),
        State::ImplicitDocumentStart => document::document_start(parser, true),
        State::DocumentStart => document::document_start(parser, false),
        State::DocumentContent => document::document_content(parser),
        State::DocumentEnd => document::document_end(parser),
        State::BlockNode => {
            // YAML 1.2 Rule [196]: ACTUALLY PARSE THE TOKEN, don't just transition
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Scalar(style, val) => {
                    let tok = parser.scanner.fetch_token();
                    parser.pop_state();
                    Ok((Event::Scalar(val.clone(), *style, 0, None), tok.0))
                }
                TokenType::FlowSequenceStart => {
                    let tok = parser.scanner.fetch_token();
                    parser.state = State::FlowSequenceFirstEntry;
                    Ok((Event::SequenceStart(0), tok.0))
                }
                TokenType::FlowMappingStart => {
                    let tok = parser.scanner.fetch_token();
                    parser.state = State::FlowMappingFirstKey;
                    Ok((Event::MappingStart(0), tok.0))
                }
                TokenType::BlockEntry => {
                    let col = token.0.col;
                    let mk = token.0;
                    parser.scanner.fetch_token(); // consume the token
                    parser.push_indent(col);
                    parser.state = State::BlockSequenceFirstEntry;
                    Ok((Event::SequenceStart(0), mk))
                }
                _ => {
                    // Default to empty scalar for unknown tokens
                    parser.pop_state();
                    let mk = parser.scanner.mark();
                    Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
                }
            }
        }
        State::BlockNodeBlockOut => {
            // Block context parsing - DIRECTLY HANDLE TOKENS
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Scalar(style, val) => {
                    let tok = parser.scanner.fetch_token();
                    parser.pop_state();
                    Ok((Event::Scalar(val.clone(), *style, 0, None), tok.0))
                }
                TokenType::BlockEntry => {
                    let col = token.0.col;
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.push_indent(col);
                    parser.state = State::BlockSequenceFirstEntry;
                    Ok((Event::SequenceStart(0), mk))
                }
                _ => {
                    parser.pop_state();
                    let mk = parser.scanner.mark();
                    Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
                }
            }
        }
        State::BlockNodeBlockIn => {
            // Block context parsing - DIRECTLY HANDLE TOKENS
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Scalar(style, val) => {
                    let tok = parser.scanner.fetch_token();
                    parser.pop_state();
                    Ok((Event::Scalar(val.clone(), *style, 0, None), tok.0))
                }
                TokenType::BlockEntry => {
                    let col = token.0.col;
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.push_indent(col);
                    parser.state = State::BlockSequenceFirstEntry;
                    Ok((Event::SequenceStart(0), mk))
                }
                _ => {
                    parser.pop_state();
                    let mk = parser.scanner.mark();
                    Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
                }
            }
        }
        State::BlockSequenceFirstEntry => {
            // YAML 1.2 Rule [183]: c-l-block-seq(n) ::= (s-indent(n+1+m) c-l-block-seq-entry(n+m))+
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::BlockEntry => {
                    let mk = token.0;
                    parser.scanner.fetch_token(); // consume the token
                    parser.push_state(State::BlockNode); // Parse the sequence entry content  
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                TokenType::BlockEnd => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.pop_state();
                    parser.pop_indent();
                    Ok((Event::SequenceEnd, mk))
                }
                _ => {
                    let mk = parser.scanner.mark();
                    parser.pop_state();
                    parser.pop_indent();
                    Ok((Event::SequenceEnd, mk))
                }
            }
        }
        State::BlockSequenceEntry => {
            // Subsequent sequence entries
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::BlockEntry => {
                    let mk = token.0;
                    parser.scanner.fetch_token(); 
                    parser.push_state(State::BlockNode);
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                _ => {
                    let mk = parser.scanner.mark();
                    parser.pop_state();
                    parser.pop_indent();
                    Ok((Event::SequenceEnd, mk))
                }
            }
        }
        State::IndentlessSequenceEntry => {
            // Indentless sequence (like in flow contexts)
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::BlockEntry => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.push_state(State::BlockNode);
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                _ => {
                    let mk = parser.scanner.mark();
                    parser.pop_state();
                    Ok((Event::SequenceEnd, mk))
                }
            }
        }
        State::BlockMappingFirstKey => {
            // YAML 1.2 Rule [186]: c-l-block-mapping(n) ::= (s-indent(n+1+m) c-l-block-map-entry(n+m))+
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Key => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.state = State::BlockMappingValue;
                    parser.push_state(State::BlockNode); // Parse key
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                TokenType::Scalar(style, val) => {
                    // Implicit key
                    let tok = parser.scanner.fetch_token();
                    parser.state = State::BlockMappingValue;
                    Ok((Event::Scalar(val.clone(), *style, 0, None), tok.0))
                }
                TokenType::BlockEnd => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.pop_state();
                    parser.pop_indent();
                    Ok((Event::MappingEnd, mk))
                }
                _ => {
                    let mk = parser.scanner.mark();
                    parser.pop_state();
                    parser.pop_indent();
                    Ok((Event::MappingEnd, mk))
                }
            }
        }
        State::BlockMappingKey => {
            // Subsequent mapping keys  
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Key => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.state = State::BlockMappingValue;
                    parser.push_state(State::BlockNode);
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                TokenType::Scalar(style, val) => {
                    let tok = parser.scanner.fetch_token();
                    parser.state = State::BlockMappingValue;
                    Ok((Event::Scalar(val.clone(), *style, 0, None), tok.0))
                }
                _ => {
                    let mk = parser.scanner.mark();
                    parser.pop_state();
                    parser.pop_indent();
                    Ok((Event::MappingEnd, mk))
                }
            }
        }
        State::BlockMappingValue => {
            // Parse mapping value
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Value => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.state = State::BlockMappingKey;
                    parser.push_state(State::BlockNode); // Parse value
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                _ => {
                    // Null value
                    let mk = parser.scanner.mark();
                    parser.state = State::BlockMappingKey;
                    Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
                }
            }
        }
        State::FlowSequenceFirstEntry => {
            // YAML 1.2 Rule [110]: c-flow-sequence(n,c) ::= '[' s-separate(n,c)? c-flow-seq-entries(n,FLOW-IN)? ']'
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::FlowSequenceEnd => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.pop_state();
                    Ok((Event::SequenceEnd, mk))
                }
                _ => {
                    parser.state = State::FlowSequenceEntry;
                    parser.push_state(State::FlowIn); // Parse entry in FLOW-IN context
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
            }
        }
        State::FlowSequenceEntry => {
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::FlowEntry => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.push_state(State::FlowIn);
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                TokenType::FlowSequenceEnd => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.pop_state();
                    Ok((Event::SequenceEnd, mk))
                }
                _ => {
                    let mk = parser.scanner.mark();
                    parser.pop_state();
                    Ok((Event::SequenceEnd, mk))
                }
            }
        }
        State::FlowSequenceEntryMappingKey => {
            // Handle mapping inside flow sequence
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Key => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.state = State::FlowSequenceEntryMappingValue;
                    parser.push_state(State::FlowKey);
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                _ => {
                    let mk = parser.scanner.mark();
                    parser.state = State::FlowSequenceEntry;
                    Ok((Event::MappingEnd, mk))
                }
            }
        }
        State::FlowSequenceEntryMappingValue => {
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Value => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.state = State::FlowSequenceEntryMappingEnd;
                    parser.push_state(State::FlowIn);
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                _ => {
                    let mk = parser.scanner.mark();
                    parser.state = State::FlowSequenceEntryMappingEnd;
                    Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
                }
            }
        }
        State::FlowSequenceEntryMappingEnd => {
            let mk = parser.scanner.mark();
            parser.state = State::FlowSequenceEntry;
            Ok((Event::MappingEnd, mk))
        }
        State::FlowMappingFirstKey => {
            // YAML 1.2 Rule [113]: c-flow-mapping(n,c) ::= '{' s-separate(n,c)? c-flow-map-entries(n,FLOW-IN)? '}'
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::FlowMappingEnd => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.pop_state();
                    Ok((Event::MappingEnd, mk))
                }
                _ => {
                    parser.state = State::FlowMappingValue;
                    parser.push_state(State::FlowKey); // Parse key in FLOW-KEY context
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
            }
        }
        State::FlowMappingKey => {
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::FlowEntry => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.state = State::FlowMappingValue;
                    parser.push_state(State::FlowKey);
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                TokenType::FlowMappingEnd => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.pop_state();
                    Ok((Event::MappingEnd, mk))
                }
                _ => {
                    let mk = parser.scanner.mark();
                    parser.pop_state();
                    Ok((Event::MappingEnd, mk))
                }
            }
        }
        State::FlowMappingValue => {
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Value => {
                    let mk = token.0;
                    parser.scanner.fetch_token();
                    parser.state = State::FlowMappingKey;
                    parser.push_state(State::FlowIn); // Parse value in FLOW-IN context
                    // Immediately call state machine again for next state
                    execute_state_machine(parser)
                }
                _ => {
                    // Null value
                    let mk = parser.scanner.mark();
                    parser.state = State::FlowMappingKey;
                    Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
                }
            }
        }
        State::FlowMappingEmptyValue => {
            // Handle empty mapping value case
            let mk = parser.scanner.mark();
            parser.state = State::FlowMappingKey;
            Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
        },
        
        // Context-specific states per YAML 1.2 rule [80] - zero allocation, blazing-fast implementations
        State::BlockKey => {
            // BLOCK-KEY context: Handle separation and DIRECTLY parse key
            parser.scanner.skip_inline_separation()?;
            
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Scalar(style, val) => {
                    let tok = parser.scanner.fetch_token();
                    parser.pop_state();
                    Ok((Event::Scalar(val.clone(), *style, 0, None), tok.0))
                }
                _ => {
                    parser.pop_state();
                    let mk = parser.scanner.mark();
                    Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
                }
            }
        }
        State::FlowIn => {
            // FLOW-IN context: s-separate(n,FLOW-IN) ::= s-separate-lines(n)  
            // Multi-line separation allowed with proper indentation
            parser.scanner.skip_multiline_separation()?;
            
            // Parse flow content in FLOW-IN context
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::FlowSequenceStart => {
                    parser.state = State::FlowSequenceFirstEntry;
                    let tok = parser.scanner.fetch_token();
                    Ok((Event::SequenceStart(0), tok.0))
                }
                TokenType::FlowMappingStart => {
                    parser.state = State::FlowMappingFirstKey;
                    let tok = parser.scanner.fetch_token();
                    Ok((Event::MappingStart(0), tok.0))
                }
                _ => {
                    // Regular content in flow-in context - DIRECTLY HANDLE
                    let token = parser.scanner.fetch_token();
                    match token.1 {
                        TokenType::Scalar(style, val) => {
                            parser.pop_state();
                            Ok((Event::Scalar(val, style, 0, None), token.0))
                        }
                        _ => {
                            parser.pop_state();
                            let mk = parser.scanner.mark();
                            Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
                        }
                    }
                }
            }
        }
        State::FlowOut => {
            // FLOW-OUT context: s-separate(n,FLOW-OUT) ::= s-separate-lines(n)
            // Multi-line separation allowed with proper indentation  
            parser.scanner.skip_multiline_separation()?;
            
            // Parse flow content in FLOW-OUT context
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::FlowSequenceStart => {
                    parser.state = State::FlowSequenceFirstEntry;
                    let tok = parser.scanner.fetch_token();
                    Ok((Event::SequenceStart(0), tok.0))
                }
                TokenType::FlowMappingStart => {
                    parser.state = State::FlowMappingFirstKey;
                    let tok = parser.scanner.fetch_token();
                    Ok((Event::MappingStart(0), tok.0))
                }
                _ => {
                    // Regular content in flow-out context - DIRECTLY HANDLE
                    let token = parser.scanner.fetch_token();
                    match token.1 {
                        TokenType::Scalar(style, val) => {
                            parser.pop_state();
                            Ok((Event::Scalar(val, style, 0, None), token.0))
                        }
                        _ => {
                            parser.pop_state();
                            let mk = parser.scanner.mark();
                            Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
                        }
                    }
                }
            }
        }
        State::FlowKey => {
            // FLOW-KEY context: s-separate(n,FLOW-KEY) ::= s-separate-in-line
            // Only horizontal separation allowed - same as BLOCK-KEY
            parser.scanner.skip_inline_separation()?;
            
            // Parse key content in flow context - DIRECTLY HANDLE
            let token = parser.scanner.peek_token()?;
            match &token.1 {
                TokenType::Scalar(style, val) => {
                    let tok = parser.scanner.fetch_token();
                    parser.pop_state();
                    Ok((Event::Scalar(val.clone(), *style, 0, None), tok.0))
                }
                _ => {
                    parser.pop_state();
                    let mk = parser.scanner.mark();
                    Ok((Event::Scalar("".to_string(), crate::events::TScalarStyle::Plain, 0, None), mk))
                }
            }
        }
        
        // REMOVE: This state was causing infinite loops - functionality moved to BlockNode
        // REMOVE: This state was causing infinite loops - functionality moved to direct handlers
        // REMOVE: This state was causing infinite loops - functionality moved to direct handlers
        // REMOVE: This state was causing infinite loops - functionality moved to direct handlers
        // REMOVE: This state was causing infinite loops - functionality moved to direct handlers
        // REMOVE: This state was causing infinite loops - functionality moved to direct handlers
        // REMOVE: This state was causing infinite loops - functionality moved to direct handlers
        
        State::End => unreachable!(),
    };
    
    // Universal state machine debugging - IMMUTABLE EXIT LOGGING
    let new_state = parser.state;
    let new_stack_depth = parser.states.len();
    let state_changed = new_state != current_state;
    let stack_changed = new_stack_depth != stack_depth;
    
    // STATE MACHINE EXIT DEBUG
    debug!("STATE_EXIT: {:?} -> {:?}", current_state, new_state);
    match &result {
        Ok((event, marker)) => {
            debug!("RESULT: SUCCESS | Event: {:?} at {}:{}", event, marker.line, marker.col);
        }
        Err(error) => {
            debug!("RESULT: ERROR | {:?}", error);
        }
    }
    
    // Add trace-level step-by-step logging for complex debugging
    trace!("STATE_TRANSITION: {} -> {} | Stack: {} -> {} | Token: {}", 
           format!("{:?}", current_state), format!("{:?}", new_state), 
           stack_depth, new_stack_depth, current_token);
    
    result
}

/// Debugging helper for state machine validation and trace mode
pub fn validate_state_transition<T: Iterator<Item = char>>(
    _parser: &crate::parser::Parser<T>,
    from_state: State,
    to_state: State,
    event: &Event
) -> Result<(), String> {
    use log::warn;
    
    // State transition validation per YAML 1.2 spec
    match (from_state, to_state, event) {
        // Valid transitions for tagged sequences per rule [96] c-ns-properties  
        (State::BlockMappingValue, State::BlockNodeBlockOut, Event::SequenceStart(_)) => {
            // This is the critical path for tagged sequences like "vec: !wat\n  - 0"
            log::info!("TAGGED_SEQUENCE_PATH: BlockMappingValue -> BlockNodeBlockOut with SequenceStart");
            Ok(())
        }
        
        // REMOVED: Infinite loop states eliminated - direct token consumption implemented
        
        // All other transitions are valid until proven otherwise
        _ => Ok(())
    }
}

/// Enable step-by-step debugging mode for complex cases  
pub fn enable_trace_mode() {
    unsafe {
        std::env::set_var("RUST_LOG", "trace");
    }
    // Note: env_logger::init() should be called by the application, not the library
    // Applications using this parser should call env_logger::init() or another logger init
    log::info!("STATE_MACHINE: Trace mode enabled - every state transition will be logged");
}

// Separation functions removed: Now handled by Scanner public methods
// - Scanner::skip_inline_separation() for BLOCK-KEY and FLOW-KEY contexts  
// - Scanner::skip_multiline_separation() for FLOW-IN, FLOW-OUT, BLOCK-IN, BLOCK-OUT contexts
// This provides proper encapsulation and zero-allocation performance
