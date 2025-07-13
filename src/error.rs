use std::error::Error;
use std::fmt;

/// The scanning position for error reporting.
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub struct Marker {
    pub index: usize,
    pub line: usize,
    pub col: usize,
}

/// The parse error used by the scanner/parser if something goes wrong.
#[derive(Clone, Debug)]
pub struct ScanError {
    pub mark: Marker,
    pub info: String,
}

impl ScanError {
    pub fn new(mark: Marker, info: &str) -> Self {
        ScanError {
            mark,
            info: info.to_owned(),
        }
    }
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {} col {}",
            self.info, self.mark.line, self.mark.col + 1
        )
    }
}

impl Error for ScanError {} 