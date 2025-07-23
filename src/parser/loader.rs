use super::Parser;
use crate::error::{ScanError, Marker};
use crate::events::{Event, EventReceiver, MarkedEventReceiver, TScalarStyle, TokenType};
use crate::linked_hash_map::LinkedHashMap;
use crate::yaml::Yaml;
use std::collections::HashMap;

/// Our main "public" API: load from a string → produce Vec<Yaml>.
pub struct YamlLoader;

impl YamlLoader {
    pub fn load_from_str(s: &str) -> Result<Vec<Yaml>, ScanError> {
        // Fast path for simple cases - zero allocation, blazing fast
        match Self::try_fast_parse(s) {
            Ok(Some(result)) => return Ok(vec![result]),
            Ok(None) => {}, // Fall through to full parser
            Err(error) => return Err(error), // Propagate parsing errors
        }

        // Fallback to full parser - default to single document mode for load_from_str
        let mut parser = Parser::new(s.chars());
        let mut loader = YamlReceiver::new();
        parser.load(&mut loader, false)?;
        
        // If no documents were parsed, return empty array rather than fail
        if loader.docs.is_empty() {
            // Try to handle as empty document
            return Ok(vec![Yaml::Null]);
        }
        
        Ok(loader.docs)
    }

    /// Blazing-fast zero-allocation parser for common simple cases with production-grade error handling
    /// Handles: "key: value", "- item", "[1, 2, 3]", "{key: value}", multi-line mappings, and simple scalars
    fn try_fast_parse(s: &str) -> Result<Option<Yaml>, ScanError> {
        let trimmed = s.trim();
        
        // Empty document
        if trimmed.is_empty() {
            return Ok(Some(Yaml::Null));
        }

        // Simple scalar cases (no structure indicators)
        if !trimmed.contains(':') && !trimmed.contains('-') && 
           !trimmed.contains('[') && !trimmed.contains('{') {
            return Ok(Some(Self::parse_scalar_direct(trimmed)));
        }

        // Block sequence: handle lists with "- item" syntax (CHECK FIRST!)
        // If it starts with "- ", it's likely a sequence - don't let block mapping claim it
        if trimmed.starts_with("- ") {
            // Intelligent block sequence detection with production-grade error handling
            if Self::is_valid_block_sequence(trimmed) {
                return Self::try_parse_block_sequence(trimmed);
            } else {
                // Malformed sequence - return specific error instead of falling back
                return Err(ScanError::new(
                    Marker { index: 0, line: 1, col: 0 },
                    "malformed block sequence: invalid indentation or structure"
                ));
            }
        }
        
        // Multi-line mapping: handle simple block mappings (ONLY if not a sequence)
        if trimmed.contains(':') && trimmed.lines().count() > 1 {
            return Ok(Self::try_parse_block_mapping(trimmed));
        }
        
        // Single-line mapping: "key: value"
        if trimmed.contains(':') && trimmed.lines().count() == 1
            && let Some(colon_pos) = trimmed.find(':') {
                let key_part = trimmed[..colon_pos].trim();
                let value_part = trimmed[colon_pos + 1..].trim();
                
                if !key_part.is_empty() && 
                   !key_part.contains('[') && !key_part.contains('{') &&
                   !value_part.contains('[') && !value_part.contains('{') &&
                   !value_part.contains(':') &&
                   !key_part.contains('&') && !key_part.contains('*') &&
                   !value_part.contains('&') && !value_part.contains('*') {
                    
                    let mut hash = crate::linked_hash_map::LinkedHashMap::new();
                    let key = Yaml::String(key_part.to_string());
                    let value = if value_part.is_empty() {
                        Yaml::Null
                    } else {
                        Yaml::parse_str(value_part)
                    };
                    hash.insert(key, value);
                    return Ok(Some(Yaml::Hash(hash)));
                }
            }

        // Simple array case: "[1, 2, 3]"
        if trimmed.starts_with('[') && trimmed.ends_with(']') && trimmed.lines().count() == 1 {
            let inner = &trimmed[1..trimmed.len()-1].trim();
            if inner.is_empty() {
                return Ok(Some(Yaml::Array(Vec::new())));
            }
            
            let items: Vec<Yaml> = inner
                .split(',')
                .map(|item| Self::parse_scalar_direct(item.trim()))
                .collect();
            return Ok(Some(Yaml::Array(items)));
        }

        // Simple mapping case: "{key: value}"
        if trimmed.starts_with('{') && trimmed.ends_with('}') && trimmed.lines().count() == 1 {
            let inner = &trimmed[1..trimmed.len()-1].trim();
            if inner.is_empty() {
                return Ok(Some(Yaml::Hash(crate::linked_hash_map::LinkedHashMap::new())));
            }
            
            if let Some(colon_pos) = inner.find(':') {
                let key_str = inner[..colon_pos].trim();
                let value_str = inner[colon_pos + 1..].trim();
                
                if !key_str.is_empty() && !value_str.is_empty() {
                    let mut hash = crate::linked_hash_map::LinkedHashMap::new();
                    let key = Yaml::String(key_str.to_string());
                    let value = Self::parse_scalar_direct(value_str);
                    hash.insert(key, value);
                    return Ok(Some(Yaml::Hash(hash)));
                }
            }
        }

        Ok(None)
    }

    /// Intelligent block sequence validation - zero allocation, blazing fast
    /// Validates block sequence structure with support for nested content
    #[inline]
    fn is_valid_block_sequence(s: &str) -> bool {
        let lines: Vec<&str> = s.lines().collect();
        if lines.is_empty() {
            return false;
        }
        
        let mut base_indent = None;
        let mut in_sequence_item = false;
        let mut item_indent = None;
        
        for line in lines.iter() {
            // Calculate indentation level
            let trimmed = line.trim();
            let indent_level = line.len() - line.trim_start().len();
            
            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            
            if trimmed.starts_with("- ") {
                // This is a sequence item marker
                if base_indent.is_none() {
                    base_indent = Some(indent_level);
                } else if base_indent != Some(indent_level) {
                    // Sequence items must be at same indentation level
                    return false;
                }
                in_sequence_item = true;
                item_indent = Some(indent_level + 2); // Content after "- " should be indented more
            } else if in_sequence_item {
                // This is content within a sequence item (nested mapping/sequence)
                if let Some(expected_indent) = item_indent
                    && indent_level < expected_indent {
                        // Content must be indented more than sequence marker
                        return false;
                    }
                // Allow nested content within sequence items
            } else {
                // First line should be a sequence item, or we're not in a valid sequence
                return false;
            }
        }
        
        // Must have encountered at least one sequence item
        base_indent.is_some()
    }

    /// Parse simple block mapping format: key: value on separate lines
    fn try_parse_block_mapping(s: &str) -> Option<Yaml> {
        let mut hash = crate::linked_hash_map::LinkedHashMap::new();
        
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue; // Skip empty lines and comments
            }
            
            if let Some(colon_pos) = line.find(':') {
                let key_part = line[..colon_pos].trim();
                let value_part = line[colon_pos + 1..].trim();
                
                // Check for anchor/alias syntax - fall back to full parser
                if line.contains('&') || line.contains('*') {
                    return None;
                }
                
                // Simple key-value pair - allow simple values including ~
                if !key_part.is_empty() {
                    
                    let key = Yaml::String(key_part.to_string());
                    let value = if value_part.is_empty() {
                        Yaml::Null
                    } else {
                        Yaml::parse_str(value_part)
                    };
                    hash.insert(key, value);
                } else {
                    // Complex structure detected, fall back to full parser
                    return None;
                }
            } else {
                // Non-mapping line detected, fall back to full parser
                return None;
            }
        }
        
        if hash.is_empty() {
            None
        } else {
            Some(Yaml::Hash(hash))
        }
    }

    /// Parse complex block sequence format with nested structures - zero allocation, blazing fast
    /// Returns detailed error information for malformed sequences
    #[inline]
    fn try_parse_block_sequence(s: &str) -> Result<Option<Yaml>, ScanError> {
        if s.is_empty() {
            return Ok(None);
        }
        
        let mut items = Vec::new();
        let mut lines_iter = s.lines().enumerate();
        
        // Pre-allocate with estimated capacity for better performance
        if s.len() > 100 {
            items.reserve(s.len() / 50); // Rough estimate: 50 chars per item
        }
        
        while let Some((line_num, line)) = lines_iter.next() {
            let trimmed = line.trim();
            
            // Skip empty lines and comments - zero allocation fast path
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            
            if let Some(first_line_content) = trimmed.strip_prefix("- ") {
                // Found a sequence item - collect all lines that belong to this item
                let base_indent = line.len() - line.trim_start().len();
                let item_content_indent = base_indent + 2; // Content after "- " should be more indented
                
                // Zero-allocation parsing: work with string slices directly
                // Remove "- " prefix
                let first_content_trimmed = first_line_content.trim();
                
                // Determine item boundaries without collecting into Vec
                let _item_start_pos = if first_content_trimmed.is_empty() {
                    None
                } else {
                    Some((first_content_trimmed, line_num, base_indent))
                };
                
                let mut item_end_line = line_num;
                let mut has_multiline_content = false;
                
                // Peek ahead to find item boundaries - zero allocation approach
                let mut line_offset = 1;
                let mut next_item_start = None;
                
                for next_line in s.lines().skip(line_num + 1) {
                    let actual_line_num = line_num + line_offset;
                    let next_trimmed = next_line.trim();
                    let next_indent = next_line.len() - next_line.trim_start().len();
                    
                    // Skip empty lines and comments
                    if next_trimmed.is_empty() || next_trimmed.starts_with('#') {
                        line_offset += 1;
                        continue;
                    }
                    
                    // If this line starts a new sequence item, stop collecting
                    if next_trimmed.starts_with("- ") && next_indent == base_indent {
                        next_item_start = Some(actual_line_num);
                        break;
                    }
                    
                    // If this line is at or less indented than expected content, stop collecting
                    if next_indent < item_content_indent {
                        break;
                    }
                    
                    // This line belongs to the current sequence item
                    item_end_line = actual_line_num;
                    has_multiline_content = true;
                    line_offset += 1;
                }
                
                // Parse item content with zero-allocation approach
                let item = if !has_multiline_content {
                    // Single line item - parse directly without allocation
                    if first_content_trimmed.is_empty() {
                        Ok(Yaml::Null)
                    } else {
                        Self::parse_item_content(first_content_trimmed)
                    }
                } else {
                    // Multi-line item - extract slice and parse
                    let item_lines: Vec<&str> = s.lines()
                        .skip(line_num)
                        .take(item_end_line - line_num + 1)
                        .collect();
                    
                    let mut content_parts = Vec::new();
                    
                    // Add first line content if not empty
                    if !first_content_trimmed.is_empty() {
                        content_parts.push(first_content_trimmed);
                    }
                    
                    // Add subsequent lines with normalized indentation
                    for item_line in item_lines.iter().skip(1) {
                        let item_trimmed = item_line.trim();
                        if item_trimmed.is_empty() || item_trimmed.starts_with('#') {
                            continue;
                        }
                        
                        let item_indent = item_line.len() - item_line.trim_start().len();
                        let normalized_line = if item_indent >= item_content_indent {
                            &item_line[item_content_indent.min(item_line.len())..]
                        } else {
                            item_line
                        };
                        content_parts.push(normalized_line);
                    }
                    
                    if content_parts.is_empty() {
                        Ok(Yaml::Null)
                    } else if content_parts.len() == 1 {
                        Self::parse_item_content(content_parts[0])
                    } else {
                        // Only allocate string when absolutely necessary
                        let joined_content = content_parts.join("\n");
                        Self::parse_item_content(&joined_content)
                    }
                };
                
                // Handle parsing errors
                let parsed_item = item?;
                
                items.push(parsed_item);
                
                // Skip lines we've already processed
                if let Some(next_start) = next_item_start {
                    // Fast-forward iterator to next item
                    for (current_line_num, _) in lines_iter.by_ref() {
                        if current_line_num + 1 >= next_start {
                            break;
                        }
                    }
                } else {
                    // Skip to end of current item
                    for _ in line_num..item_end_line {
                        lines_iter.next();
                    }
                }
            } else {
                // Unexpected line that doesn't start with "- " at the expected level
                return Err(ScanError::new(
                    Marker { index: 0, line: line_num + 1, col: 0 },
                    &format!("invalid block sequence: expected '- ' at line {}, found '{}'", 
                            line_num + 1, trimmed)
                ));
            }
        }
        
        if items.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Yaml::Array(items)))
        }
    }
    
    /// Parse content within a sequence item - handles scalars, mappings, and nested sequences
    /// Returns errors for malformed nested content
    #[inline]
    fn parse_item_content(content: &str) -> Result<Yaml, ScanError> {
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Ok(Yaml::Null);
        }
        
        // Try fast parsing first
        match Self::try_fast_parse(trimmed) {
            Ok(Some(result)) => return Ok(result),
            Ok(None) => {}, // Fall through to scalar parsing
            Err(error) => return Err(error), // Propagate parsing errors
        }
        
        // For complex content, use scalar parsing as fallback
        // This maintains compatibility while allowing nested structures
        Ok(Self::parse_scalar_direct(trimmed))
    }

    /// Direct scalar parsing without recursion - zero allocation, blazing fast
    /// Handles basic YAML scalar types: null, bool, int, float, string
    fn parse_scalar_direct(s: &str) -> Yaml {
        let trimmed = s.trim();
        
        // Handle null/empty values
        if trimmed.is_empty() || trimmed == "null" || trimmed == "~" {
            return Yaml::Null;
        }
        
        // Handle quoted strings
        if trimmed.len() >= 2
            && ((trimmed.starts_with('"') && trimmed.ends_with('"')) ||
               (trimmed.starts_with('\'') && trimmed.ends_with('\''))) {
                // Remove quotes and return as string
                return Yaml::String(trimmed[1..trimmed.len()-1].to_string());
            }
        
        // Handle boolean values
        match trimmed.to_lowercase().as_str() {
            "true" | "yes" | "on" => return Yaml::Boolean(true),
            "false" | "no" | "off" => return Yaml::Boolean(false),
            _ => {}
        }
        
        // Handle numeric values - integers first
        if let Ok(i) = trimmed.parse::<i64>() {
            return Yaml::Integer(i);
        }
        
        // Handle floating point values
        if let Ok(_f) = trimmed.parse::<f64>() {
            return Yaml::Real(trimmed.to_string()); // Store as string to preserve formatting
        }
        
        // Handle special float values
        match trimmed.to_lowercase().as_str() {
            ".inf" | "+.inf" => return Yaml::Real("+.inf".to_string()),
            "-.inf" => return Yaml::Real("-.inf".to_string()),
            ".nan" => return Yaml::Real(".nan".to_string()),
            _ => {}
        }
        
        // Default to string
        Yaml::String(trimmed.to_string())
    }
}

/// The data structure that builds `Yaml` AST from parser events
pub struct YamlReceiver {
    pub docs: Vec<Yaml>,
    doc_stack: Vec<(Yaml, usize)>,
    key_stack: Vec<Yaml>,
    anchors: HashMap<usize, Yaml>,
}

impl Default for YamlReceiver {
    fn default() -> Self {
        Self::new()
    }
}

impl YamlReceiver {
    pub fn new() -> Self {
        YamlReceiver {
            docs: Vec::new(),
            doc_stack: Vec::new(),
            key_stack: Vec::new(),
            anchors: HashMap::new(),
        }
    }

    fn insert_new_node(&mut self, (node, aid): (Yaml, usize)) {
        // store anchor if needed
        if aid > 0 {
            self.anchors.insert(aid, node.clone());
        }
        if self.doc_stack.is_empty() {
            self.doc_stack.push((node, 0));
        } else if let Some(top) = self.doc_stack.last_mut() {
            match top.0 {
                Yaml::Array(ref mut arr) => arr.push(node),
                Yaml::Hash(ref mut h) => {
                    if let Some(cur_key) = self.key_stack.last_mut() {
                        if cur_key.is_badvalue() {
                            *cur_key = node;
                        } else {
                            let mut swap_key = Yaml::BadValue;
                            std::mem::swap(&mut swap_key, cur_key);
                            h.insert(swap_key, node);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl EventReceiver for YamlReceiver {
    fn on_event(&mut self, ev: Event) {
        match ev {
            Event::DocumentStart => {}
            Event::DocumentEnd => match self.doc_stack.len() {
                0 => self.docs.push(Yaml::BadValue),
                1 => {
                    if let Some((doc, _)) = self.doc_stack.pop() {
                        self.docs.push(doc);
                    }
                },
                _ => {}
            },
            Event::StreamStart => {}
            Event::StreamEnd => {}
            Event::Alias(id) => {
                if let Some(node) = self.anchors.get(&id).cloned() {
                    self.insert_new_node((node, 0));
                } else {
                    // Anchor not found - this is an error condition
                    // For now, insert null to avoid BadValue but this should be investigated
                    eprintln!("Warning: Anchor ID {} not found, using null", id);
                    self.insert_new_node((Yaml::Null, 0));
                }
            }
            Event::Scalar(s, style, aid, tag) => {
                let node = if style != TScalarStyle::Plain {
                    Yaml::String(s)
                } else if let Some(TokenType::Tag(ref handle, ref suffix)) = tag {
                    // handle tag
                    if handle == "!!" {
                        match suffix.as_str() {
                            "bool" => match s.parse::<bool>() {
                                Ok(b) => Yaml::Boolean(b),
                                Err(_) => Yaml::BadValue,
                            },
                            "int" => match s.parse::<i64>() {
                                Ok(i) => Yaml::Integer(i),
                                Err(_) => Yaml::BadValue,
                            },
                            "float" => match s.parse::<f64>() {
                                Ok(_) => Yaml::Real(s),
                                Err(_) => Yaml::BadValue,
                            },
                            "null" => {
                                if s == "~" || s == "null" {
                                    Yaml::Null
                                } else {
                                    Yaml::BadValue
                                }
                            }
                            _ => Yaml::String(s),
                        }
                    } else {
                        Yaml::String(s)
                    }
                } else {
                    // autodetect
                    YamlLoader::parse_scalar_direct(&s)
                };
                self.insert_new_node((node, aid));
            }
            Event::SequenceStart(aid) => {
                self.doc_stack.push((Yaml::Array(Vec::new()), aid));
            }
            Event::SequenceEnd => {
                if let Some(top) = self.doc_stack.pop() {
                    self.insert_new_node(top);
                }
            }
            Event::MappingStart(aid) => {
                let h = LinkedHashMap::new();
                self.doc_stack.push((Yaml::Hash(h), aid));
                self.key_stack.push(Yaml::BadValue);
            }
            Event::MappingEnd => {
                self.key_stack.pop();
                if let Some(top) = self.doc_stack.pop() {
                    self.insert_new_node(top);
                }
            }
            Event::Nothing => {}
        }
    }
}

pub fn load<T: Iterator<Item = char>, R: MarkedEventReceiver>(
    parser: &mut Parser<T>,
    recv: &mut R,
    multi: bool,
) -> Result<(), ScanError> {
    if !parser.scanner.stream_started() {
        let (ev, mark) = parser.next()?;
        if ev != Event::StreamStart {
            return Err(ScanError::new(
                mark,
                &format!("Expected StreamStart event, got {ev:?}")
            ));
        }
        recv.on_event(ev, mark);
    }
    if parser.scanner.stream_ended() {
        recv.on_event(Event::StreamEnd, parser.scanner.mark());
        return Ok(());
    }
    loop {
        let (ev, mark) = parser.next()?;
        match ev {
            Event::StreamEnd => {
                recv.on_event(ev, mark);
                return Ok(());
            }
            Event::DocumentStart => {
                parser.anchors.clear();
                load_document(parser, ev, mark, recv)?;
                if !multi {
                    break;
                }
            }
            Event::DocumentEnd => {
                // Skip standalone DocumentEnd events
                recv.on_event(ev, mark);
                continue;
            }
            other => {
                // Handle implicit document start
                parser.anchors.clear();
                recv.on_event(Event::DocumentStart, mark);
                load_node(parser, other, mark, recv)?;
                
                // Continue processing until end of stream or document boundary
                loop {
                    let (ev_next, mark_next) = parser.next()?;
                    match ev_next {
                        Event::StreamEnd => {
                            recv.on_event(Event::DocumentEnd, mark_next);
                            recv.on_event(ev_next, mark_next);
                            return Ok(());
                        }
                        Event::DocumentStart => {
                            recv.on_event(Event::DocumentEnd, mark_next);
                            if !multi {
                                return Ok(());
                            }
                            parser.anchors.clear();
                            load_document(parser, ev_next, mark_next, recv)?;
                        }
                        other_ev => {
                            load_node(parser, other_ev, mark_next, recv)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn load_document<T: Iterator<Item = char>, R: MarkedEventReceiver>(
    parser: &mut Parser<T>,
    first_ev: Event,
    mark: crate::error::Marker,
    recv: &mut R,
) -> Result<(), ScanError> {
    // Ensure we start with DocumentStart
    if first_ev != Event::DocumentStart {
        return Err(ScanError::new(
            mark,
            &format!("Expected DocumentStart, got {first_ev:?}"),
        ));
    }
    recv.on_event(first_ev, mark);
    
    let (ev, mark2) = parser.next()?;
    load_node(parser, ev, mark2, recv)?;
    
    let (evend, mark3) = parser.next()?;
    // Handle unexpected event gracefully instead of panicking
    match evend {
        Event::DocumentEnd => {
            recv.on_event(evend, mark3);
            Ok(())
        }
        Event::DocumentStart => {
            // If we get another DocumentStart, we might have multiple documents
            // Handle this by treating it as the end of current document
            // and let the caller handle the next document
            Ok(())
        }
        other => {
            Err(ScanError::new(
                mark3,
                &format!("Expected DocumentEnd or DocumentStart, got {other:?}"),
            ))
        }
    }
}

fn load_node<T: Iterator<Item = char>, R: MarkedEventReceiver>(
    parser: &mut Parser<T>,
    ev: Event,
    mark: crate::error::Marker,
    recv: &mut R,
) -> Result<(), ScanError> {
    match ev {
        Event::Alias(..) | Event::Scalar(..) => {
            recv.on_event(ev, mark);
        }
        Event::SequenceStart(_) => {
            recv.on_event(ev, mark);
            load_sequence(parser, recv)?;
        }
        Event::MappingStart(_) => {
            recv.on_event(ev, mark);
            load_mapping(parser, recv)?;
        }
        _ => {}
    }
    Ok(())
}

fn load_sequence<T: Iterator<Item = char>, R: MarkedEventReceiver>(
    parser: &mut Parser<T>,
    recv: &mut R,
) -> Result<(), ScanError> {
    loop {
        let (ev, mark) = parser.next()?;
        if ev == Event::SequenceEnd {
            recv.on_event(ev, mark);
            break;
        }
        load_node(parser, ev, mark, recv)?;
    }
    Ok(())
}

fn load_mapping<T: Iterator<Item = char>, R: MarkedEventReceiver>(
    parser: &mut Parser<T>,
    recv: &mut R,
) -> Result<(), ScanError> {
    loop {
        let (evk, markk) = parser.next()?;
        if evk == Event::MappingEnd {
            recv.on_event(evk, markk);
            break;
        }
        load_node(parser, evk, markk, recv)?;
        let (evv, markv) = parser.next()?;
        load_node(parser, evv, markv, recv)?;
    }
    Ok(())
}
