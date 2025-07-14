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
        let mut parser = Parser::new(s.chars());
        let mut loader = YamlReceiver::new();
        parser.load(&mut loader, true)?;
        Ok(loader.docs)
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
            let top = self.doc_stack.last_mut().unwrap();
            match top.0 {
                Yaml::Array(ref mut arr) => arr.push(node),
                Yaml::Hash(ref mut h) => {
                    let cur_key = self.key_stack.last_mut().unwrap();
                    if cur_key.is_badvalue() {
                        *cur_key = node;
                    } else {
                        let mut swap_key = Yaml::BadValue;
                        std::mem::swap(&mut swap_key, cur_key);
                        h.insert(swap_key, node);
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
                1 => self.docs.push(self.doc_stack.pop().unwrap().0),
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
                    Yaml::from_str(&s)
                };
                self.insert_new_node((node, aid));
            }
            Event::SequenceStart(aid) => {
                self.doc_stack.push((Yaml::Array(Vec::new()), aid));
            }
            Event::SequenceEnd => {
                let top = self.doc_stack.pop().unwrap();
                self.insert_new_node(top);
            }
            Event::MappingStart(aid) => {
                let h = LinkedHashMap::new();
                self.doc_stack.push((Yaml::Hash(h), aid));
                self.key_stack.push(Yaml::BadValue);
            }
            Event::MappingEnd => {
                self.key_stack.pop();
                let top = self.doc_stack.pop().unwrap();
                self.insert_new_node(top);
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
        assert_eq!(ev, Event::StreamStart);
        recv.on_event(ev, mark);
    }
    if parser.scanner.stream_ended() {
        recv.on_event(Event::StreamEnd, parser.scanner.mark());
        return Ok(());
    }
    loop {
        let (ev, mark) = parser.next()?;
        if ev == Event::StreamEnd {
            recv.on_event(ev, mark);
            return Ok(());
        }
        parser.anchors.clear();
        load_document(parser, ev, mark, recv)?;
        if !multi {
            break;
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
    assert_eq!(first_ev, Event::DocumentStart);
    recv.on_event(first_ev, mark);
    let (ev, mark2) = parser.next()?;
    load_node(parser, ev, mark2, recv)?;
    let (evend, mark3) = parser.next()?;
    assert_eq!(evend, Event::DocumentEnd);
    recv.on_event(evend, mark3);
    Ok(())
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
