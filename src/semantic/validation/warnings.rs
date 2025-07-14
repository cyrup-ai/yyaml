//! Validation warning types and severity management

use super::fixes::ValidationFix;
use crate::lexer::Position;
use crate::semantic::tags::YamlType;

/// Validation warning with detailed information
#[derive(Debug, Clone)]
pub struct ValidationWarning<'input> {
    pub warning_type: WarningType,
    pub message: String,
    pub position: Position,
    pub severity: WarningSeverity,
    pub rule_name: String,
    pub suggestion: Option<String>,
    pub context: ValidationWarningContext<'input>,
}

/// Types of validation warnings
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarningType {
    StructuralIssue,
    SemanticInconsistency,
    PerformanceImpact,
    CompatibilityIssue,
    StyleViolation,
    SecurityConcern,
    DataIntegrity,
}

/// Warning severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WarningSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Context information for validation warnings
#[derive(Debug, Clone)]
pub struct ValidationWarningContext<'input> {
    pub node_path: Vec<String>,
    pub node_type: Option<YamlType>,
    pub related_nodes: Vec<Position>,
    pub constraint_violated: Option<String>,
    pub suggested_fix: Option<ValidationFix<'input>>,
}

impl std::fmt::Display for WarningSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WarningSeverity::Info => write!(f, "INFO"),
            WarningSeverity::Low => write!(f, "LOW"),
            WarningSeverity::Medium => write!(f, "MEDIUM"),
            WarningSeverity::High => write!(f, "HIGH"),
            WarningSeverity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl std::fmt::Display for WarningType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WarningType::StructuralIssue => write!(f, "Structural Issue"),
            WarningType::SemanticInconsistency => write!(f, "Semantic Inconsistency"),
            WarningType::PerformanceImpact => write!(f, "Performance Impact"),
            WarningType::CompatibilityIssue => write!(f, "Compatibility Issue"),
            WarningType::StyleViolation => write!(f, "Style Violation"),
            WarningType::SecurityConcern => write!(f, "Security Concern"),
            WarningType::DataIntegrity => write!(f, "Data Integrity"),
        }
    }
}
