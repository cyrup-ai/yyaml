use super::super::parser::{block, document, flow};
use crate::error::{Marker, ScanError};
use crate::events::Event;

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

pub fn execute_state_machine<T: Iterator<Item = char>>(
    parser: &mut crate::parser::Parser<T>,
) -> Result<(Event, Marker), ScanError> {
    match parser.state {
        State::StreamStart => parser.stream_start(),
        State::ImplicitDocumentStart => document::document_start(parser, true),
        State::DocumentStart => document::document_start(parser, false),
        State::DocumentContent => document::document_content(parser),
        State::DocumentEnd => document::document_end(parser),
        State::BlockNode => super::parse_node(parser, true, false),
        State::BlockSequenceFirstEntry => block::block_sequence_entry(parser, true),
        State::BlockSequenceEntry => block::block_sequence_entry(parser, false),
        State::IndentlessSequenceEntry => block::indentless_sequence_entry(parser),
        State::BlockMappingFirstKey => block::block_mapping_key(parser, true),
        State::BlockMappingKey => block::block_mapping_key(parser, false),
        State::BlockMappingValue => block::block_mapping_value(parser),
        State::FlowSequenceFirstEntry => flow::flow_sequence_entry(parser, true),
        State::FlowSequenceEntry => flow::flow_sequence_entry(parser, false),
        State::FlowSequenceEntryMappingKey => flow::flow_sequence_entry_mapping_key(parser),
        State::FlowSequenceEntryMappingValue => flow::flow_sequence_entry_mapping_value(parser),
        State::FlowSequenceEntryMappingEnd => flow::flow_sequence_entry_mapping_end(parser),
        State::FlowMappingFirstKey => flow::flow_mapping_key(parser, true),
        State::FlowMappingKey => flow::flow_mapping_key(parser, false),
        State::FlowMappingValue => flow::flow_mapping_value(parser, false),
        State::FlowMappingEmptyValue => flow::flow_mapping_value(parser, true),
        State::End => unreachable!(),
    }
}
