//! Validation fix suggestions and impact analysis

use crate::lexer::Position;
use crate::parser::ast::Node;

/// Suggested fix for validation issues
#[derive(Debug, Clone)]
pub struct ValidationFix<'input> {
    pub fix_type: FixType,
    pub description: String,
    pub position: Position,
    pub replacement_text: Option<String>,
    pub replacement_node: Option<Node<'input>>,
    pub impact: FixImpact,
    pub confidence: f32,
}

/// Types of fixes available
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FixType {
    ReplaceValue,
    AddMissingKey,
    RemoveDuplicateKey,
    CorrectIndentation,
    ChangeType,
    ReorderKeys,
    RefactorStructure,
}

/// Impact level of applying a fix
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FixImpact {
    NoDataChange,
    MinorDataChange,
    MajorDataChange,
    StructuralChange,
}

impl std::fmt::Display for FixType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixType::ReplaceValue => write!(f, "Replace Value"),
            FixType::AddMissingKey => write!(f, "Add Missing Key"),
            FixType::RemoveDuplicateKey => write!(f, "Remove Duplicate Key"),
            FixType::CorrectIndentation => write!(f, "Correct Indentation"),
            FixType::ChangeType => write!(f, "Change Type"),
            FixType::ReorderKeys => write!(f, "Reorder Keys"),
            FixType::RefactorStructure => write!(f, "Refactor Structure"),
        }
    }
}

impl std::fmt::Display for FixImpact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixImpact::NoDataChange => write!(f, "No Data Change"),
            FixImpact::MinorDataChange => write!(f, "Minor Data Change"),
            FixImpact::MajorDataChange => write!(f, "Major Data Change"),
            FixImpact::StructuralChange => write!(f, "Structural Change"),
        }
    }
}
