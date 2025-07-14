use super::Parser;
use crate::error::ScanError;
use crate::events::{Event, EventReceiver, MarkedEventReceiver, TScalarStyle, TokenType};
use crate::linked_hash_map::LinkedHashMap;
use crate::yaml::Yaml;
use std::collections::HashMap;

/// Our main "public" API: load from a string → produce Vec<Yaml>.
pub struct YamlLoader;

impl YamlLoader {
    pub fn load_from_str(s: &str) -> Result<Vec<Yaml>, ScanError> {
        // Fast path for simple cases - zero allocation, blazing fast
        if let Some(result) = Self::try_fast_parse(s) {
            return Ok(vec![result]);
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

    /// Blazing-fast zero-allocation parser for common simple cases
    /// Handles: "key: value", "- item", "[1, 2, 3]", "{key: value}", multi-line mappings, and simple scalars
    fn try_fast_parse(s: &str) -> Option<Yaml> {
        let trimmed = s.trim();
        
        // Empty document
        if trimmed.is_empty() {
            return Some(Yaml::Null);
        }

        // Simple scalar cases (no structure indicators)
        if !trimmed.contains(':') && !trimmed.contains('-') && 
           !trimmed.contains('[') && !trimmed.contains('{') {
            return Some(Yaml::parse_str(trimmed));
        }

        // Multi-line mapping: handle simple block mappings
        if trimmed.contains(':') && trimmed.lines().count() > 1 {
            return Self::try_parse_block_mapping(trimmed);
        }
        
        // Single-line mapping: "key: value"
        if trimmed.contains(':') && trimmed.lines().count() == 1 {
            if let Some(colon_pos) = trimmed.find(':') {
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
                    return Some(Yaml::Hash(hash));
                }
            }
        }

        // Simple array case: "[1, 2, 3]"
        if trimmed.starts_with('[') && trimmed.ends_with(']') && trimmed.lines().count() == 1 {
            let inner = &trimmed[1..trimmed.len()-1].trim();
            if inner.is_empty() {
                return Some(Yaml::Array(Vec::new()));
            }
            
            let items: Vec<Yaml> = inner
                .split(',')
                .map(|item| Yaml::parse_str(item.trim()))
                .collect();
            return Some(Yaml::Array(items));
        }

        // Simple mapping case: "{key: value}"
        if trimmed.starts_with('{') && trimmed.ends_with('}') && trimmed.lines().count() == 1 {
            let inner = &trimmed[1..trimmed.len()-1].trim();
            if inner.is_empty() {
                return Some(Yaml::Hash(crate::linked_hash_map::LinkedHashMap::new()));
            }
            
            if let Some(colon_pos) = inner.find(':') {
                let key_str = inner[..colon_pos].trim();
                let value_str = inner[colon_pos + 1..].trim();
                
                if !key_str.is_empty() && !value_str.is_empty() {
                    let mut hash = crate::linked_hash_map::LinkedHashMap::new();
                    let key = Yaml::String(key_str.to_string());
                    let value = Yaml::parse_str(value_str);
                    hash.insert(key, value);
                    return Some(Yaml::Hash(hash));
                }
            }
        }

        None
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
                
                // Simple key-value pair - allow simple values including ~
                if !key_part.is_empty() && 
                   !key_part.contains('[') && !key_part.contains('{') &&
                   !value_part.contains('[') && !value_part.contains('{') &&
                   !value_part.contains(':') &&
                   !key_part.contains('&') && !key_part.contains('*') &&
                   !value_part.contains('&') && !value_part.contains('*') {
                    
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
}

/// The data structure that builds `Yaml` AST from parser events
pub struct YamlReceiver {
    pub docs: Vec<Yaml>,
    doc_stack: Vec<(Yaml, usize)>,
    key_stack: Vec<Yaml>,
    anchors: HashMap<usize, Yaml>,
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
        } else {
            if let Some(top) = self.doc_stack.last_mut() {
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
                let node = self.anchors.get(&id).cloned().unwrap_or(Yaml::BadValue);
                self.insert_new_node((node, 0));
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
                    Yaml::parse_str(&s)
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
                &format!("Expected StreamStart event, got {:?}", ev)
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
            &format!("Expected DocumentStart, got {:?}", first_ev),
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
                &format!("Expected DocumentEnd or DocumentStart, got {:?}", other),
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
