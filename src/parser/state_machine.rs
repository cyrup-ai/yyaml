use crate::yaml::Yaml;
use crate::linked_hash_map::LinkedHashMap;
use crate::error::ScanError;
use crate::events::TokenType;
use crate::scanner::Scanner;
use crate::parser::grammar::{Context, ParametricContext};
use std::collections::HashMap;

/// YAML parsing state machine states
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
    BlockMappingFirstKey,
    BlockMappingKey,
    BlockMappingValue,
    FlowSequenceFirstEntry,
    FlowSequenceEntry,
    FlowMappingFirstKey,
    FlowMappingKey,
    FlowMappingValue,
    End,
}

/// State machine parser that builds Yaml AST directly
pub struct StateMachine<T: Iterator<Item = char>> {
    pub scanner: Scanner<T>,
    pub states: Vec<State>,
    pub state: State,
    pub anchors: HashMap<String, usize>,
    pub anchor_id: usize,
    pub indents: Vec<usize>,  // Keep for compatibility
    ast_stack: Vec<YamlBuilder>,
    pending_tag: Option<(String, String)>,

    // ADD:
    pub context: ParametricContext,
    recursion_depth: usize,  // From TASK1
    max_recursion_depth: usize,
}

/// Builder for constructing Yaml AST during parsing
#[derive(Debug)]
enum YamlBuilder {
    Sequence(Vec<Yaml>),
    Mapping(LinkedHashMap<Yaml, Yaml>, Option<Yaml>), // map, current_key
    Scalar(String),
}

impl<T: Iterator<Item = char>> StateMachine<T> {
    pub fn new(src: T) -> Self {
        StateMachine {
            scanner: Scanner::new(src),
            states: Vec::new(),
            state: State::StreamStart,
            anchors: HashMap::new(),
            anchor_id: 1,
            indents: Vec::new(),
            ast_stack: Vec::new(),
            pending_tag: None,

            // ADD:
            context: ParametricContext::new(),
            recursion_depth: 0,
            max_recursion_depth: 100,
        }
    }

    pub fn push_indent(&mut self, indent: usize) {
        self.indents.push(indent);
    }

    pub fn pop_indent(&mut self) {
        self.indents.pop();
    }

    pub fn pop_state(&mut self) {
        if let Some(state) = self.states.pop() {
            // Check if we're leaving a context scope
            match (self.state, state) {
                (State::FlowSequenceEntry, State::BlockNode) |
                (State::FlowMappingValue, State::BlockNode) |
                (State::BlockMappingValue, State::BlockMappingKey) => {
                    self.context.pop_context();
                }
                _ => {}
            }
            self.state = state;
        }
    }

    pub fn push_state(&mut self, st: State) {
        self.states.push(self.state);
        self.state = st;
    }

    pub fn register_anchor(&mut self, name: String) -> usize {
        let new_id = self.anchor_id;
        self.anchor_id += 1;
        self.anchors.insert(name, new_id);
        new_id
    }

    /// Execute the state machine and return the constructed Yaml AST
    pub fn parse(&mut self) -> Result<Yaml, ScanError> {
        while self.state != State::End {
            self.execute_state()?;
        }
        
        // Return the final constructed AST
        if let Some(builder) = self.ast_stack.pop() {
            Ok(self.finalize_builder(builder))
        } else {
            Ok(Yaml::Null)
        }
    }

    /// Execute a single state transition
    pub fn execute_state(&mut self) -> Result<(), ScanError> {
        match self.state {
            State::StreamStart => self.handle_stream_start(),
            State::DocumentStart => self.handle_document_start(),
            State::DocumentContent => self.handle_document_content(),
            State::BlockNode => self.handle_block_node(),
            State::BlockSequenceFirstEntry => self.handle_block_sequence_first_entry(),
            State::BlockSequenceEntry => self.handle_block_sequence_entry(),
            State::BlockMappingFirstKey => self.handle_block_mapping_first_key(),
            State::BlockMappingKey => self.handle_block_mapping_key(),
            State::BlockMappingValue => self.handle_block_mapping_value(),
            State::FlowSequenceFirstEntry => self.handle_flow_sequence_first_entry(),
            State::FlowSequenceEntry => self.handle_flow_sequence_entry(),
            State::FlowMappingFirstKey => self.handle_flow_mapping_first_key(),
            State::FlowMappingKey => self.handle_flow_mapping_key(),
            State::FlowMappingValue => self.handle_flow_mapping_value(),
            _ => {
                self.state = State::End;
                Ok(())
            }
        }
    }

    fn handle_stream_start(&mut self) -> Result<(), ScanError> {
        let token = self.scanner.peek_token()?;
        match &token.1 {
            TokenType::StreamStart(_) => {
                self.scanner.fetch_token();
                self.state = State::DocumentStart;
                Ok(())
            }
            _ => Err(ScanError::new(token.0, "expected stream start")),
        }
    }

    fn handle_document_start(&mut self) -> Result<(), ScanError> {
        self.state = State::DocumentContent;
        Ok(())
    }

    fn handle_document_content(&mut self) -> Result<(), ScanError> {
        // Document content starts at indent 0, BLOCK-OUT context
        self.context.push_context(Context::BlockOut, 0);
        self.state = State::BlockNode;
        Ok(())
    }

    fn handle_block_node(&mut self) -> Result<(), ScanError> {
        // Keep parsing until we handle a non-tag token
        loop {
            let token = self.scanner.peek_token()?;

            // ADD context validation:
            if !self.can_accept_token(&token.1) {
                return Err(ScanError::new(
                    token.0,
                    &format!("Token {:?} not allowed in {:?} context",
                            token.1, self.context.current_context)
                ));
            }

            match &token.1 {
                TokenType::Scalar(_, value) => {
                    // Peek ahead to see if this is a mapping key
                    self.scanner.fetch_token(); // Consume the scalar
                    let next_token = self.scanner.peek_token()?;

                    if matches!(next_token.1, TokenType::Value) {
                        // This is a mapping key
                        let key = Yaml::parse_str(value);
                        self.ast_stack.push(YamlBuilder::Mapping(LinkedHashMap::new(), Some(key)));
                        self.state = State::BlockMappingValue;
                        return Ok(());
                    } else {
                        // Just a scalar value
                        let yaml = Yaml::parse_str(value);
                        self.push_yaml(yaml);
                        self.pop_state();
                        return Ok(());
                    }
                }
                TokenType::BlockEntry => {
                    self.scanner.fetch_token();
                    self.ast_stack.push(YamlBuilder::Sequence(Vec::new()));
                    // Don't push state - we're at root level
                    self.state = State::BlockSequenceFirstEntry;
                    return Ok(());
                }
                TokenType::Key => {
                    self.scanner.fetch_token();
                    self.ast_stack.push(YamlBuilder::Mapping(LinkedHashMap::new(), None));
                    self.state = State::BlockMappingFirstKey;
                    return Ok(());
                }
                TokenType::FlowSequenceStart => {
                    self.scanner.fetch_token();
                    self.ast_stack.push(YamlBuilder::Sequence(Vec::new()));
                    self.state = State::FlowSequenceFirstEntry;
                    return Ok(());
                }
                TokenType::FlowMappingStart => {
                    self.scanner.fetch_token();
                    self.ast_stack.push(YamlBuilder::Mapping(LinkedHashMap::new(), None));
                    self.state = State::FlowMappingFirstKey;
                    return Ok(());
                }
                TokenType::Tag(handle, suffix) => {
                    // Store the tag for the next value
                    self.pending_tag = Some((handle.clone(), suffix.clone()));
                    self.scanner.fetch_token();
                    // Continue looping to parse the value that follows the tag
                    continue;
                }
                _ => {
                    self.push_yaml(Yaml::Null);
                    self.pop_state();
                    return Ok(());
                }
            }
        }
    }

    fn handle_block_sequence_first_entry(&mut self) -> Result<(), ScanError> {
        // The first BlockEntry was already consumed when we transitioned to this state
        // Now we need to handle the content of this first sequence item
        self.state = State::BlockSequenceEntry;
        self.handle_sequence_content()
    }

    fn handle_block_sequence_entry(&mut self) -> Result<(), ScanError> {
        let token = self.scanner.peek_token()?;
        match &token.1 {
            TokenType::BlockEntry => {
                self.scanner.fetch_token();
                self.handle_sequence_content()
            }
            _ => {
                // End of sequence
                if let Some(YamlBuilder::Sequence(items)) = self.ast_stack.pop() {
                    self.push_yaml(Yaml::Array(items));
                }
                self.pop_state();
                Ok(())
            }
        }
    }

    fn handle_sequence_content(&mut self) -> Result<(), ScanError> {
        let token = self.scanner.peek_token()?;
        match &token.1 {
            TokenType::Scalar(_, value) => {
                self.scanner.fetch_token();
                let yaml = Yaml::parse_str(value);
                if let Some(YamlBuilder::Sequence(items)) = self.ast_stack.last_mut() {
                    items.push(yaml);
                }
                Ok(())
            }
            TokenType::BlockEntry | TokenType::Key |
            TokenType::FlowSequenceStart | TokenType::FlowMappingStart => {
                // These indicate nested structures in the sequence
                // Push the current state and transition to handle the nested structure
                self.push_state(State::BlockSequenceEntry);
                self.state = State::BlockNode;
                Ok(())
            }
            _ => {
                // Empty sequence item - add null
                if let Some(YamlBuilder::Sequence(items)) = self.ast_stack.last_mut() {
                    items.push(Yaml::Null);
                }
                Ok(())
            }
        }
    }

    fn handle_block_mapping_first_key(&mut self) -> Result<(), ScanError> {
        // Block mapping key uses BLOCK-KEY context at n+1
        let n = self.context.current_indent();
        self.context.push_context(Context::BlockKey, n + 1);

        self.state = State::BlockMappingKey;
        self.handle_mapping_key()
    }

    fn handle_block_mapping_key(&mut self) -> Result<(), ScanError> {
        self.handle_mapping_key()
    }

    fn handle_mapping_key(&mut self) -> Result<(), ScanError> {
        let token = self.scanner.peek_token()?;
        match &token.1 {
            TokenType::Scalar(_, value) => {
                self.scanner.fetch_token();
                let key = Yaml::parse_str(value);
                if let Some(YamlBuilder::Mapping(_, current_key)) = self.ast_stack.last_mut() {
                    *current_key = Some(key);
                }
                self.state = State::BlockMappingValue;
                Ok(())
            }
            _ => {
                // End of mapping
                if let Some(YamlBuilder::Mapping(map, _)) = self.ast_stack.pop() {
                    self.push_yaml(Yaml::Hash(map));
                }

                // Check if we're at the root level
                if self.states.is_empty() {
                    self.state = State::End;
                } else {
                    self.pop_state();
                }
                Ok(())
            }
        }
    }

    fn handle_block_mapping_value(&mut self) -> Result<(), ScanError> {
        let token = self.scanner.peek_token()?;
        match &token.1 {
            TokenType::Value => {
                self.scanner.fetch_token();

                // Block mapping value uses BLOCK-IN context at n+1+m
                let n = self.context.current_indent();
                self.context.push_context(Context::BlockIn, n + 2);  // n+1+1

                // Handle tags and other tokens after the colon
                loop {
                    let value_token = self.scanner.peek_token()?;
                    match &value_token.1 {
                        TokenType::Tag(handle, suffix) => {
                            // Store the tag for the value
                            self.pending_tag = Some((handle.clone(), suffix.clone()));
                            self.scanner.fetch_token();
                            // Continue to get the actual value
                            continue;
                        }
                        TokenType::Scalar(_, value) => {
                            self.scanner.fetch_token();
                            let yaml_value = Yaml::parse_str(value);
                            self.add_mapping_pair(yaml_value);
                            self.state = State::BlockMappingKey;
                            return Ok(());
                        }
                        _ => {
                            // Handle nested structures as the value
                            // Push current state so when nested structure completes,
                            // we return to BlockMappingKey to look for the next key
                            self.push_state(State::BlockMappingKey);
                            self.state = State::BlockNode;
                            return Ok(());
                        }
                    }
                }
            }
            _ => {
                // Null value
                self.add_mapping_pair(Yaml::Null);
                self.state = State::BlockMappingKey;
                Ok(())
            }
        }
    }

    fn handle_flow_sequence_first_entry(&mut self) -> Result<(), ScanError> {
        // Flow sequence switches to FLOW-IN context
        let current_indent = self.context.current_indent();
        self.context.push_context(Context::FlowIn, current_indent);

        self.state = State::FlowSequenceEntry;
        Ok(())
    }

    fn handle_flow_sequence_entry(&mut self) -> Result<(), ScanError> {
        let token = self.scanner.peek_token()?;
        match &token.1 {
            TokenType::FlowSequenceEnd => {
                self.scanner.fetch_token();
                if let Some(YamlBuilder::Sequence(items)) = self.ast_stack.pop() {
                    self.push_yaml(Yaml::Array(items));
                }
                self.pop_state();
                Ok(())
            }
            TokenType::FlowEntry => {
                self.scanner.fetch_token();
                Ok(())
            }
            TokenType::Scalar(_, value) => {
                self.scanner.fetch_token();
                let yaml = Yaml::parse_str(value);
                if let Some(YamlBuilder::Sequence(items)) = self.ast_stack.last_mut() {
                    items.push(yaml);
                }
                Ok(())
            }
            _ => Ok(())
        }
    }

    fn handle_flow_mapping_first_key(&mut self) -> Result<(), ScanError> {
        self.state = State::FlowMappingKey;
        Ok(())
    }

    fn handle_flow_mapping_key(&mut self) -> Result<(), ScanError> {
        let token = self.scanner.peek_token()?;
        match &token.1 {
            TokenType::FlowMappingEnd => {
                self.scanner.fetch_token();
                if let Some(YamlBuilder::Mapping(map, _)) = self.ast_stack.pop() {
                    self.push_yaml(Yaml::Hash(map));
                }
                self.pop_state();
                Ok(())
            }
            TokenType::Scalar(_, value) => {
                self.scanner.fetch_token();
                let key = Yaml::parse_str(value);
                if let Some(YamlBuilder::Mapping(_, current_key)) = self.ast_stack.last_mut() {
                    *current_key = Some(key);
                }
                self.state = State::FlowMappingValue;
                Ok(())
            }
            _ => Ok(())
        }
    }

    fn handle_flow_mapping_value(&mut self) -> Result<(), ScanError> {
        let token = self.scanner.peek_token()?;
        match &token.1 {
            TokenType::Value => {
                self.scanner.fetch_token();
                let value_token = self.scanner.peek_token()?;
                match &value_token.1 {
                    TokenType::Scalar(_, value) => {
                        self.scanner.fetch_token();
                        let yaml_value = Yaml::parse_str(value);
                        self.add_mapping_pair(yaml_value);
                        self.state = State::FlowMappingKey;
                        Ok(())
                    }
                    _ => Ok(())
                }
            }
            _ => Ok(())
        }
    }

    /// Add a key-value pair to the current mapping
    fn add_mapping_pair(&mut self, mut value: Yaml) {
        // Apply pending tag if present
        if let Some((handle, suffix)) = self.pending_tag.take() {
            let tag_uri = match handle.as_str() {
                "!!" => format!("tag:yaml.org,2002:{}", suffix),
                "!" => suffix,
                _ => format!("{}{}", handle, suffix),
            };
            value = Yaml::Tagged(tag_uri, Box::new(value));
        }

        if let Some(YamlBuilder::Mapping(map, current_key)) = self.ast_stack.last_mut() {
            if let Some(key) = current_key.take() {
                map.insert(key, value);
            }
        }
    }

    /// Push a constructed Yaml value onto the AST stack
    fn push_yaml(&mut self, mut yaml: Yaml) {
        // Apply pending tag if present
        if let Some((handle, suffix)) = self.pending_tag.take() {
            let tag_uri = match handle.as_str() {
                "!!" => format!("tag:yaml.org,2002:{}", suffix),
                "!" => suffix,
                _ => format!("{}{}", handle, suffix),
            };
            yaml = Yaml::Tagged(tag_uri, Box::new(yaml));
        }

        // If we have a container being built, add to it
        if let Some(builder) = self.ast_stack.last_mut() {
            match builder {
                YamlBuilder::Sequence(items) => {
                    items.push(yaml);
                }
                YamlBuilder::Mapping(map, current_key) => {
                    if let Some(key) = current_key.take() {
                        // We have a key waiting for a value
                        map.insert(key, yaml);
                    } else {
                        // No key yet, this must be a key
                        *current_key = Some(yaml);
                    }
                }
                _ => {}
            }
        } else {
            // This is the root document
            self.ast_stack.push(YamlBuilder::Scalar(format!("{:?}", yaml)));
        }
    }

    fn can_accept_token(&self, token: &TokenType) -> bool {
        match self.context.current_context {
            Context::FlowIn | Context::FlowOut => {
                // Flow contexts cannot have block entries
                !matches!(token, TokenType::BlockEntry)
            }
            Context::BlockKey => {
                // Block key context has restricted character set
                !matches!(token, TokenType::FlowSequenceStart | TokenType::FlowMappingStart)
            }
            _ => true,
        }
    }

    fn check_indentation(&mut self, token_indent: usize) -> Result<(), ScanError> {
        let required_indent = match self.context.current_context {
            Context::BlockIn => self.context.current_indent(),
            Context::BlockKey => self.context.current_indent(),  // Keys at n+1
            Context::BlockOut => 0,  // No restriction
            Context::FlowIn | Context::FlowOut | Context::FlowKey => {
                // Flow contexts ignore indentation
                return Ok(());
            }
        };

        if token_indent < required_indent as usize {
            return Err(ScanError::new(
                self.scanner.mark(),
                &format!("Insufficient indentation: expected {}, got {}",
                        required_indent, token_indent)
            ));
        }

        Ok(())
    }

    /// Finalize a YamlBuilder into a Yaml value
    fn finalize_builder(&self, builder: YamlBuilder) -> Yaml {
        match builder {
            YamlBuilder::Sequence(items) => Yaml::Array(items),
            YamlBuilder::Mapping(map, _) => Yaml::Hash(map),
            YamlBuilder::Scalar(s) => Yaml::String(s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_state_machine() {
        let yaml = r#"
outer:
  - item1  # BLOCK-IN at indent 2
  - key: value  # BLOCK-KEY at 4, BLOCK-IN at 7
    flow: [a, b]  # FLOW-IN inside []
"#;

        let mut sm = StateMachine::new(yaml.chars());

        // Parse and check context at various points
        // This would require adding debug hooks or inspection methods
        let result = sm.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_validation() {
        // Test context initialization
        let yaml = "key: value";
        let mut sm = StateMachine::new(yaml.chars());

        // Verify ParametricContext is initialized properly
        assert_eq!(sm.context.current_context, Context::BlockOut);
        assert_eq!(sm.context.current_indent(), 0);

        let result = sm.parse();
        assert!(result.is_ok());
    }
}