use crate::error::{ScanError, Marker};
use crate::events::{TokenType, TScalarStyle, TEncoding};
use std::collections::VecDeque;

/// A single token: (Marker, TokenType).
#[derive(Clone, Debug)]
pub struct Token(pub Marker, pub TokenType);

/// The scanning logic. Adapted from the original, but inlined for performance.  
/// (We keep a small token queue, do minimal-lifetime allocations, etc.)
pub struct Scanner<T: Iterator<Item = char>> {
    rdr: T,
    buffer: VecDeque<char>,
    done: bool,
    mark: Marker,
    token: Option<Token>,
    stream_start_produced: bool,
    stream_end_produced: bool,
}

impl<T: Iterator<Item = char>> Scanner<T> {
    pub fn new(src: T) -> Self {
        Scanner {
            rdr: src,
            buffer: VecDeque::new(),
            done: false,
            mark: Marker { index: 0, line: 1, col: 0 },
            token: None,
            stream_start_produced: false,
            stream_end_produced: false,
        }
    }
    
    pub fn mark(&self) -> Marker {
        self.mark
    }
    
    pub fn peek_token(&mut self) -> Result<&Token, ScanError> {
        if self.token.is_none() {
            self.token = Some(self.fetch_next_token()?);
        }
        Ok(self.token.as_ref().unwrap())
    }
    
    pub fn fetch_token(&mut self) -> Token {
        self.token.take().unwrap()
    }
    
    pub fn skip(&mut self) {
        self.token = None;
    }
    
    pub fn stream_started(&self) -> bool {
        self.stream_start_produced
    }
    
    pub fn stream_ended(&self) -> bool {
        self.stream_end_produced
    }
    
    fn fetch_next_token(&mut self) -> Result<Token, ScanError> {
        // inline the logic for scanning the next token
        if !self.stream_start_produced {
            self.stream_start_produced = true;
            return Ok(Token(self.mark, TokenType::StreamStart(TEncoding::Utf8)));
        }
        self.skip_ws_and_comments();
        if self.done {
            if !self.stream_end_produced {
                self.stream_end_produced = true;
                return Ok(Token(self.mark, TokenType::StreamEnd));
            }
            return Ok(Token(self.mark, TokenType::NoToken));
        }
        // sniff the next char
        self.fill_buffer(1);
        let c = match self.buffer.front() {
            Some(c) => *c,
            None => {
                self.done = true;
                if !self.stream_end_produced {
                    self.stream_end_produced = true;
                    return Ok(Token(self.mark, TokenType::StreamEnd));
                }
                return Ok(Token(self.mark, TokenType::NoToken));
            }
        };
        let start_mark = self.mark;
        
        match c {
            // handle doc start ---
            '-' => {
                // might be doc start or block entry or negative number
                self.fill_buffer(3);
                if self.buffer.len() >= 3 {
                    let b: Vec<char> = self.buffer.iter().take(3).copied().collect();
                    if b == vec!['-', '-', '-'] {
                        // doc start
                        for _ in 0..3 {
                            self.next_char();
                        }
                        return Ok(Token(start_mark, TokenType::DocumentStart));
                    }
                }
                // Check if this is a block entry (- followed by space)
                self.fill_buffer(2);
                if self.buffer.len() >= 2 && (self.buffer[1] == ' ' || self.buffer[1] == '\t') {
                    self.next_char(); // consume the '-'
                    return Ok(Token(start_mark, TokenType::BlockEntry));
                }
                // otherwise treat as plain scalar
                let scalar = self.scan_plain_scalar()?;
                return Ok(Token(start_mark, TokenType::Scalar(TScalarStyle::Plain, scalar)));
            }
            '.' => {
                // might be doc end ...
                self.fill_buffer(3);
                if self.buffer.len() >= 3 {
                    let b: Vec<char> = self.buffer.iter().take(3).copied().collect();
                    if b == vec!['.', '.', '.'] {
                        for _ in 0..3 {
                            self.next_char();
                        }
                        return Ok(Token(start_mark, TokenType::DocumentEnd));
                    }
                }
                // else parse as scalar
                let scalar = self.scan_plain_scalar()?;
                return Ok(Token(start_mark, TokenType::Scalar(TScalarStyle::Plain, scalar)));
            }
            '[' => {
                self.next_char();
                return Ok(Token(start_mark, TokenType::FlowSequenceStart));
            }
            ']' => {
                self.next_char();
                return Ok(Token(start_mark, TokenType::FlowSequenceEnd));
            }
            '{' => {
                self.next_char();
                return Ok(Token(start_mark, TokenType::FlowMappingStart));
            }
            '}' => {
                self.next_char();
                return Ok(Token(start_mark, TokenType::FlowMappingEnd));
            }
            ',' => {
                self.next_char();
                return Ok(Token(start_mark, TokenType::FlowEntry));
            }
            ':' => {
                self.next_char();
                return Ok(Token(start_mark, TokenType::Value));
            }
            '?' => {
                // key indicator
                self.next_char();
                return Ok(Token(start_mark, TokenType::Key));
            }
            '&' => {
                // anchor
                self.next_char();
                let anchor = self.scan_anchor_name()?;
                return Ok(Token(start_mark, TokenType::Anchor(anchor)));
            }
            '*' => {
                // alias
                self.next_char();
                let alias = self.scan_anchor_name()?;
                return Ok(Token(start_mark, TokenType::Alias(alias)));
            }
            '|' => {
                // block scalar literal
                self.next_char();
                let block_str = self.scan_block_scalar(/*literal=*/true)?;
                return Ok(Token(
                    start_mark,
                    TokenType::Scalar(TScalarStyle::Literal, block_str),
                ));
            }
            '>' => {
                // block scalar folded
                self.next_char();
                let block_str = self.scan_block_scalar(/*literal=*/false)?;
                return Ok(Token(
                    start_mark,
                    TokenType::Scalar(TScalarStyle::Folded, block_str),
                ));
            }
            '\'' => {
                // single-quoted
                self.next_char();
                let s = self.scan_quoted('\'')?;
                return Ok(Token(
                    start_mark,
                    TokenType::Scalar(TScalarStyle::SingleQuoted, s),
                ));
            }
            '"' => {
                self.next_char();
                let s = self.scan_quoted('"')?;
                return Ok(Token(
                    start_mark,
                    TokenType::Scalar(TScalarStyle::DoubleQuoted, s),
                ));
            }
            '#' => {
                // comment? skip entire line
                while let Some(cc) = self.buffer.front() {
                    if *cc == '\n' {
                        break;
                    }
                    self.next_char();
                }
                // produce next token
                return self.fetch_next_token();
            }
            _ => {
                // fallback: plain scalar
                let s = self.scan_plain_scalar()?;
                return Ok(Token(start_mark, TokenType::Scalar(TScalarStyle::Plain, s)));
            }
        }
    }

    fn scan_anchor_name(&mut self) -> Result<String, ScanError> {
        let mut name = String::new();
        loop {
            self.fill_buffer(1);
            let c = match self.buffer.front() {
                Some(cc) => *cc,
                None => break,
            };
            if c.is_whitespace() || "#,[]{}-?:\r\n".contains(c) {
                break;
            }
            name.push(c);
            self.next_char();
        }
        Ok(name)
    }

    fn scan_quoted(&mut self, quote: char) -> Result<String, ScanError> {
        let mut out = String::new();
        loop {
            self.fill_buffer(1);
            let c = match self.buffer.front() {
                Some(cc) => *cc,
                None => return Err(ScanError::new(self.mark, "unterminated quoted string")),
            };
            if c == quote {
                self.next_char();
                break;
            }
            if quote == '"' && c == '\\' {
                // handle escapes
                self.next_char();
                self.fill_buffer(1);
                let e = match self.buffer.front() {
                    Some(cc) => *cc,
                    None => return Err(ScanError::new(self.mark, "unterminated escape")),
                };
                self.next_char();
                out.push(match e {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    '"' => '"',
                    '\'' => '\'',
                    '0' => '\0',
                    _ => e,
                });
            } else {
                out.push(c);
                self.next_char();
            }
        }
        Ok(out)
    }

    fn scan_block_scalar(&mut self, literal: bool) -> Result<String, ScanError> {
        // we skip any indentation spec like [+-][1-9] if present
        // minimal approach
        // then read until indentation stops or we run out
        // here we do a trivial approach: read until next blank line
        let mut out = String::new();
        // skip any remainder of line after '|' or '>'
        while let Some(cc) = self.buffer.front() {
            if *cc == '\n' {
                self.next_char();
                break;
            }
            self.next_char();
        }
        // read lines
        let mut is_first_line = true;
        loop {
            self.fill_buffer(1);
            if self.buffer.is_empty() {
                break;
            }
            // we check indentation or next docstart
            // minimal approach: read line by line until empty
            let c = self.buffer.front().copied().unwrap();
            if c == '-' || c == '.' {
                // might be doc start or doc end
                // let's see if next is doc start
                self.fill_buffer(3);
                if self.buffer.len() >= 3 {
                    let triple: Vec<char> = self.buffer.iter().take(3).copied().collect();
                    if triple == vec!['-', '-', '-'] || triple == vec!['.', '.', '.'] {
                        break;
                    }
                }
            }
            // read one line
            if !is_first_line {
                out.push('\n');
            }
            is_first_line = false;
            while let Some(cc) = self.buffer.front() {
                let c2 = *cc;
                if c2 == '\n' {
                    self.next_char();
                    break;
                }
                out.push(c2);
                self.next_char();
            }
        }
        if !literal {
            // fold newlines except consecutive ones
            // minimal approach
            let folded = fold_newlines(&out);
            return Ok(folded);
        }
        Ok(out)
    }

    fn scan_plain_scalar(&mut self) -> Result<String, ScanError> {
        let mut out = String::new();
        loop {
            self.fill_buffer(1);
            let c = match self.buffer.front() {
                Some(cc) => *cc,
                None => {
                    break;
                }
            };
            if c.is_whitespace() || "#,[]{}:\r\n".contains(c) {
                break;
            }
            // if it's a docstart
            if c == '-' || c == '.' {
                self.fill_buffer(3);
                if self.buffer.len() >= 3 {
                    let triple: Vec<char> = self.buffer.iter().take(3).copied().collect();
                    if triple == vec!['-', '-', '-'] || triple == vec!['.', '.', '.'] {
                        break;
                    }
                }
            }
            out.push(c);
            self.next_char();
        }
        Ok(out.trim_end().to_owned())
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            self.fill_buffer(1);
            let c = match self.buffer.front() {
                Some(&cc) => cc,
                None => {
                    self.done = true;
                    break;
                }
            };
            if c == '#' {
                // skip to line end
                while let Some(&c2) = self.buffer.front() {
                    if c2 == '\n' {
                        break;
                    }
                    self.next_char();
                }
            } else if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                self.next_char();
            } else {
                break;
            }
        }
    }
    
    fn fill_buffer(&mut self, n: usize) {
        while self.buffer.len() < n && !self.done {
            if let Some(ch) = self.rdr.next() {
                self.buffer.push_back(ch);
            } else {
                self.done = true;
            }
        }
    }
    
    fn next_char(&mut self) {
        if let Some(c) = self.buffer.pop_front() {
            self.mark.index += 1;
            if c == '\n' {
                self.mark.line += 1;
                self.mark.col = 0;
            } else {
                self.mark.col += 1;
            }
        }
    }
}

/// A simple function to "fold" newlines in ">` block style".
fn fold_newlines(input: &str) -> String {
    // naive approach:
    // - consecutive blank lines become single newline
    // - single newlines become space
    let mut out = String::new();
    let mut lines = input.split('\n').peekable();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            // blank line
            out.push('\n');
            // skip more blank lines?
            while matches!(lines.peek(), Some(&l) if l.is_empty()) {
                let _ = lines.next();
            }
        } else {
            // normal line
            if !out.is_empty() && !out.ends_with('\n') {
                out.push(' ');
            }
            out.push_str(line);
        }
    }
    out
} 