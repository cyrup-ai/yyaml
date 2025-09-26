use super::State;
// execute_state_machine removed - functionality moved to StateMachine::parse()
use crate::error::ScanError;
use crate::events::{Event, TScalarStyle, TokenType};

// REMOVED: All event-driven flow parsing functions - replaced by state machine
// The old event-driven parser architecture has been completely eliminated
// Flow parsing is now handled directly by the StateMachine in state_machine.rs

// These functions were part of the broken event-driven architecture:
// - flow_sequence_entry
// - flow_sequence_entry_mapping_key
// - flow_sequence_entry_mapping_value
// - flow_sequence_entry_mapping_end
// - flow_mapping_key
// - flow_mapping_value

// Flow parsing is now implemented in StateMachine::handle_flow_* methods