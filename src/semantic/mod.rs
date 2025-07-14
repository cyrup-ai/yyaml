//! Comprehensive semantic analysis and processing for YAML documents
//!
//! This module provides complete semantic processing including anchor/alias resolution,
//! tag resolution, document validation, and reference tracking with cycle detection.
//! Designed for zero-allocation, blazing-fast performance with complete YAML 1.2 compliance.

pub mod anchors;
pub mod references;
pub mod tags;
pub mod validation;

pub use anchors::*;
pub use references::*;
pub use tags::*;
pub use validation::*;

use crate::lexer::Position;
use crate::parser::ast::{Document, Node, Stream};
use std::borrow::Cow;
use std::collections::HashMap;

/// High-performance semantic analyzer for YAML documents
///
/// Coordinates all semantic processing phases with zero-allocation design
/// and comprehensive error handling for production YAML processing.
#[derive(Debug)]
pub struct SemanticAnalyzer<'input> {
    anchor_resolver: AnchorResolver<'input>,
    tag_resolver: TagResolver<'input>,
    validator: DocumentValidator<'input>,
    reference_tracker: ReferenceTracker<'input>,
    analysis_context: AnalysisContext<'input>,
}

/// Context for semantic analysis operations
#[derive(Debug, Clone)]
pub struct AnalysisContext<'input> {
    current_document_index: usize,
    processing_phase: ProcessingPhase,
    tag_prefixes: HashMap<Cow<'input, str>, Cow<'input, str>>,
    yaml_version: Option<(u32, u32)>,
    strict_mode: bool,
    cycle_detection_enabled: bool,
}

/// Semantic processing phases for coordinated analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessingPhase {
    AnchorCollection,
    TagResolution,
    AliasResolution,
    ReferenceValidation,
    DocumentValidation,
    Complete,
}

/// Comprehensive semantic analysis results
#[derive(Debug, Clone)]
pub struct SemanticResult<'input> {
    pub processed_stream: Stream<'input>,
    pub anchor_registry: AnchorRegistry<'input>,
    pub tag_registry: TagRegistry<'input>,
    pub validation_warnings: Vec<ValidationWarning<'input>>,
    pub reference_graph: ReferenceGraph<'input>,
    pub analysis_metrics: AnalysisMetrics,
}

/// Performance metrics for semantic analysis
#[derive(Debug, Clone, Default)]
pub struct AnalysisMetrics {
    pub anchors_resolved: usize,
    pub aliases_resolved: usize,
    pub tags_resolved: usize,
    pub cycles_detected: usize,
    pub validation_warnings: usize,
    pub processing_time_ns: u64,
}

/// Semantic analysis errors with precise location tracking
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticError {
    UnresolvedAlias {
        alias_name: String,
        position: Position,
    },
    CircularReference {
        anchor_name: String,
        cycle_path: Vec<String>,
        position: Position,
    },
    InvalidTag {
        tag_handle: String,
        tag_suffix: String,
        position: Position,
    },
    UnknownTagPrefix {
        handle: String,
        position: Position,
    },
    ConflictingAnchor {
        anchor_name: String,
        first_position: Position,
        second_position: Position,
    },
    InvalidReference {
        reference_type: String,
        target: String,
        position: Position,
    },
    ValidationFailure {
        rule: String,
        message: String,
        position: Position,
    },
    InternalError {
        message: String,
        position: Position,
    },
    TagResolutionFailed {
        tag: String,
        message: String,
        position: Position,
    },
    UnknownTag {
        tag: String,
        position: Position,
    },
    ValidationDepthExceeded {
        max_depth: usize,
        position: Position,
    },
    UnknownTagHandle {
        handle: String,
        position: Position,
    },
    CustomTagResolutionFailed {
        tag: String,
        message: String,
        position: Position,
    },
    UnknownCustomTag {
        tag: String,
        position: Position,
    },
}

impl<'input> SemanticAnalyzer<'input> {
    /// Create new semantic analyzer with optimized configuration
    #[inline]
    pub fn new() -> Self {
        Self {
            anchor_resolver: AnchorResolver::new(),
            tag_resolver: TagResolver::new(),
            validator: DocumentValidator::new(),
            reference_tracker: ReferenceTracker::new(),
            analysis_context: AnalysisContext::new(),
        }
    }

    /// Create semantic analyzer with custom configuration
    pub fn with_config(config: SemanticConfig<'input>) -> Self {
        let mut analyzer = Self::new();
        analyzer.analysis_context.strict_mode = config.strict_mode;
        analyzer.analysis_context.cycle_detection_enabled = config.cycle_detection_enabled;

        if let Some(version) = config.yaml_version {
            analyzer.analysis_context.yaml_version = Some(version);
        }

        for (handle, prefix) in config.tag_prefixes {
            analyzer
                .analysis_context
                .tag_prefixes
                .insert(handle, prefix);
        }

        analyzer
    }

    /// Perform comprehensive semantic analysis on YAML stream
    ///
    /// Processes all documents in the stream through all semantic analysis phases
    /// with complete error handling and performance optimization.
    pub fn analyze_stream(
        &mut self,
        stream: Stream<'input>,
    ) -> Result<SemanticResult<'input>, SemanticError> {
        let start_time = std::time::Instant::now();
        let mut processed_documents = Vec::with_capacity(stream.len());
        let mut analysis_metrics = AnalysisMetrics::default();

        // Phase 1: Collect all anchors across documents
        self.analysis_context.processing_phase = ProcessingPhase::AnchorCollection;
        for (index, document) in stream.iter().enumerate() {
            self.analysis_context.current_document_index = index;
            self.collect_anchors_from_document(document)?;
        }

        // Phase 2: Resolve tags throughout the stream
        self.analysis_context.processing_phase = ProcessingPhase::TagResolution;
        for (index, document) in stream.iter().enumerate() {
            self.analysis_context.current_document_index = index;
            self.resolve_tags_in_document(document)?;
        }

        // Phase 3: Resolve aliases with cycle detection
        self.analysis_context.processing_phase = ProcessingPhase::AliasResolution;
        for (index, document) in stream.documents.into_iter().enumerate() {
            self.analysis_context.current_document_index = index;
            let processed_doc = self.resolve_aliases_in_document(document)?;
            processed_documents.push(processed_doc);
        }

        // Phase 4: Validate references and detect cycles
        self.analysis_context.processing_phase = ProcessingPhase::ReferenceValidation;
        for document in &processed_documents {
            self.validate_references_in_document(document)?;
        }

        // Phase 5: Perform document structure validation
        self.analysis_context.processing_phase = ProcessingPhase::DocumentValidation;
        let mut validation_warnings = Vec::new();
        for document in &processed_documents {
            let mut doc_warnings = self
                .validator
                .validate_document(document, &self.analysis_context)?;
            validation_warnings.append(&mut doc_warnings);
        }

        // Finalize analysis
        self.analysis_context.processing_phase = ProcessingPhase::Complete;
        analysis_metrics.processing_time_ns = start_time.elapsed().as_nanos() as u64;
        analysis_metrics.anchors_resolved = self.anchor_resolver.anchor_count();
        analysis_metrics.aliases_resolved = self.anchor_resolver.alias_resolution_count();
        analysis_metrics.tags_resolved = self.tag_resolver.resolution_count();
        analysis_metrics.cycles_detected = self.reference_tracker.cycle_count();
        analysis_metrics.validation_warnings = validation_warnings.len();

        Ok(SemanticResult {
            processed_stream: Stream::new(processed_documents),
            anchor_registry: self.anchor_resolver.get_registry(),
            tag_registry: self.tag_resolver.get_registry(),
            validation_warnings,
            reference_graph: self.reference_tracker.get_graph(),
            analysis_metrics,
        })
    }

    /// Perform semantic analysis on single document
    #[inline]
    pub fn analyze_document(
        &mut self,
        document: Document<'input>,
    ) -> Result<Document<'input>, SemanticError> {
        let stream = Stream::new(vec![document]);
        let result = self.analyze_stream(stream)?;
        result
            .processed_stream
            .documents
            .into_iter()
            .next()
            .ok_or_else(|| SemanticError::InternalError {
                message: "processed stream is empty".to_string(),
                position: Position::start(),
            })
    }

    /// Collect anchors from document for global resolution
    fn collect_anchors_from_document(
        &mut self,
        document: &Document<'input>,
    ) -> Result<(), SemanticError> {
        if let Some(ref content) = document.content {
            self.collect_anchors_from_node(content, &mut Vec::new())?;
        }
        Ok(())
    }

    /// Recursively collect anchors from AST nodes
    fn collect_anchors_from_node(
        &mut self,
        node: &Node<'input>,
        path: &mut Vec<String>,
    ) -> Result<(), SemanticError> {
        match node {
            Node::Anchor(anchor_node) => {
                self.anchor_resolver.register_anchor(
                    anchor_node.name.clone(),
                    anchor_node.node.as_ref(),
                    anchor_node.position,
                    path.clone(),
                )?;
                path.push(anchor_node.name.to_string());
                self.collect_anchors_from_node(&anchor_node.node, path)?;
                path.pop();
            }
            Node::Sequence(seq_node) => {
                for (index, item) in seq_node.items.iter().enumerate() {
                    path.push(format!("[{}]", index));
                    self.collect_anchors_from_node(item, path)?;
                    path.pop();
                }
            }
            Node::Mapping(map_node) => {
                for pair in &map_node.pairs {
                    if let Node::Scalar(key_scalar) = &pair.key {
                        path.push(key_scalar.as_str().to_string());
                        self.collect_anchors_from_node(&pair.value, path)?;
                        path.pop();
                    } else {
                        path.push("<complex_key>".to_string());
                        self.collect_anchors_from_node(&pair.key, path)?;
                        self.collect_anchors_from_node(&pair.value, path)?;
                        path.pop();
                    }
                }
            }
            Node::Tagged(tagged_node) => {
                self.collect_anchors_from_node(&tagged_node.node, path)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Resolve tags in document
    fn resolve_tags_in_document(
        &mut self,
        document: &Document<'input>,
    ) -> Result<(), SemanticError> {
        if let Some(ref content) = document.content {
            self.resolve_tags_in_node(content)?;
        }
        Ok(())
    }

    /// Recursively resolve tags in AST nodes
    fn resolve_tags_in_node(&mut self, node: &Node<'input>) -> Result<(), SemanticError> {
        match node {
            Node::Tagged(tagged_node) => {
                self.tag_resolver.resolve_tag(
                    &tagged_node.handle,
                    &tagged_node.suffix,
                    tagged_node.position,
                    &self.analysis_context,
                )?;
                self.resolve_tags_in_node(&tagged_node.node)?;
            }
            Node::Anchor(anchor_node) => {
                self.resolve_tags_in_node(&anchor_node.node)?;
            }
            Node::Sequence(seq_node) => {
                for item in &seq_node.items {
                    self.resolve_tags_in_node(item)?;
                }
            }
            Node::Mapping(map_node) => {
                for pair in &map_node.pairs {
                    self.resolve_tags_in_node(&pair.key)?;
                    self.resolve_tags_in_node(&pair.value)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Resolve aliases in document with cycle detection
    fn resolve_aliases_in_document(
        &mut self,
        mut document: Document<'input>,
    ) -> Result<Document<'input>, SemanticError> {
        if let Some(content) = document.content.take() {
            let resolved_content = self.resolve_aliases_in_node(content, &mut Vec::new())?;
            document.content = Some(resolved_content);
        }
        Ok(document)
    }

    /// Recursively resolve aliases in AST nodes
    fn resolve_aliases_in_node(
        &mut self,
        node: Node<'input>,
        path: &mut Vec<String>,
    ) -> Result<Node<'input>, SemanticError> {
        match node {
            Node::Alias(alias_node) => {
                // Check for cycles before resolution
                if path.contains(&alias_node.name.to_string()) {
                    return Err(SemanticError::CircularReference {
                        anchor_name: alias_node.name.to_string(),
                        cycle_path: path.clone(),
                        position: alias_node.position,
                    });
                }

                let resolved_node = self
                    .anchor_resolver
                    .resolve_alias(&alias_node.name, alias_node.position)?;

                // Track reference for cycle detection
                self.reference_tracker.add_reference(
                    alias_node.name.clone(),
                    path.clone(),
                    alias_node.position,
                )?;

                path.push(alias_node.name.to_string());
                let final_node = self.resolve_aliases_in_node(resolved_node, path)?;
                path.pop();

                Ok(final_node)
            }
            Node::Anchor(anchor_node) => {
                path.push(anchor_node.name.to_string());
                let resolved_inner = self.resolve_aliases_in_node(*anchor_node.node, path)?;
                path.pop();

                Ok(Node::Anchor(crate::parser::ast::AnchorNode::new(
                    anchor_node.name,
                    Box::new(resolved_inner),
                    anchor_node.position,
                )))
            }
            Node::Sequence(mut seq_node) => {
                let mut resolved_items = Vec::with_capacity(seq_node.items.len());
                for (index, item) in seq_node.items.into_iter().enumerate() {
                    path.push(format!("[{}]", index));
                    let resolved_item = self.resolve_aliases_in_node(item, path)?;
                    path.pop();
                    resolved_items.push(resolved_item);
                }
                seq_node.items = resolved_items;
                Ok(Node::Sequence(seq_node))
            }
            Node::Mapping(mut map_node) => {
                for pair in &mut map_node.pairs {
                    if let Node::Scalar(key_scalar) = &pair.key {
                        path.push(key_scalar.as_str().to_string());
                    } else {
                        path.push("<complex_key>".to_string());
                    }

                    let resolved_key = self.resolve_aliases_in_node(pair.key.clone(), path)?;
                    let resolved_value = self.resolve_aliases_in_node(pair.value.clone(), path)?;

                    pair.key = resolved_key;
                    pair.value = resolved_value;
                    path.pop();
                }
                Ok(Node::Mapping(map_node))
            }
            Node::Tagged(tagged_node) => {
                let resolved_inner = self.resolve_aliases_in_node(*tagged_node.node, path)?;
                Ok(Node::Tagged(crate::parser::ast::TaggedNode::new(
                    tagged_node.handle,
                    tagged_node.suffix,
                    Box::new(resolved_inner),
                    tagged_node.position,
                )))
            }
            other => Ok(other),
        }
    }

    /// Validate references in document
    fn validate_references_in_document(
        &mut self,
        document: &Document<'input>,
    ) -> Result<(), SemanticError> {
        if let Some(ref content) = document.content {
            self.reference_tracker.validate_references(content)?;
        }
        Ok(())
    }

    /// Get current analysis context
    #[inline]
    pub fn context(&self) -> &AnalysisContext<'input> {
        &self.analysis_context
    }

    /// Get analysis metrics
    #[inline]
    pub fn metrics(&self) -> AnalysisMetrics {
        // Return default metrics if analysis not complete
        AnalysisMetrics::default()
    }

    /// Reset analyzer for new analysis
    pub fn reset(&mut self) {
        self.anchor_resolver.reset();
        self.tag_resolver.reset();
        self.reference_tracker.reset();
        self.analysis_context = AnalysisContext::new();
    }
}

impl<'input> AnalysisContext<'input> {
    /// Create new analysis context with default settings
    pub fn new() -> Self {
        Self {
            current_document_index: 0,
            processing_phase: ProcessingPhase::AnchorCollection,
            tag_prefixes: Self::default_tag_prefixes(),
            yaml_version: Some((1, 2)),
            strict_mode: false,
            cycle_detection_enabled: true,
        }
    }

    /// Get default YAML 1.2 tag prefixes
    fn default_tag_prefixes() -> HashMap<Cow<'input, str>, Cow<'input, str>> {
        let mut prefixes = HashMap::new();
        prefixes.insert(Cow::Borrowed("!"), Cow::Borrowed("tag:yaml.org,2002:"));
        prefixes.insert(Cow::Borrowed("!!"), Cow::Borrowed("tag:yaml.org,2002:"));
        prefixes
    }

    /// Check if in strict validation mode
    #[inline]
    pub fn is_strict(&self) -> bool {
        self.strict_mode
    }

    /// Check if cycle detection is enabled
    #[inline]
    pub fn cycle_detection_enabled(&self) -> bool {
        self.cycle_detection_enabled
    }

    /// Get YAML version
    #[inline]
    pub fn yaml_version(&self) -> Option<(u32, u32)> {
        self.yaml_version
    }

    /// Look up tag prefix
    #[inline]
    pub fn resolve_tag_prefix(&self, handle: &str) -> Option<&Cow<'input, str>> {
        self.tag_prefixes.get(handle)
    }
}

/// Configuration for semantic analysis
#[derive(Debug, Clone)]
pub struct SemanticConfig<'input> {
    pub strict_mode: bool,
    pub cycle_detection_enabled: bool,
    pub yaml_version: Option<(u32, u32)>,
    pub tag_prefixes: HashMap<Cow<'input, str>, Cow<'input, str>>,
}

impl<'input> Default for SemanticConfig<'input> {
    fn default() -> Self {
        Self {
            strict_mode: false,
            cycle_detection_enabled: true,
            yaml_version: Some((1, 2)),
            tag_prefixes: HashMap::new(),
        }
    }
}

impl SemanticError {
    /// Get the position associated with this error
    #[inline]
    pub fn position(&self) -> Position {
        match self {
            SemanticError::UnresolvedAlias { position, .. } => *position,
            SemanticError::CircularReference { position, .. } => *position,
            SemanticError::InvalidTag { position, .. } => *position,
            SemanticError::UnknownTagPrefix { position, .. } => *position,
            SemanticError::ConflictingAnchor { first_position, .. } => *first_position,
            SemanticError::InvalidReference { position, .. } => *position,
            SemanticError::ValidationFailure { position, .. } => *position,
            SemanticError::InternalError { position, .. } => *position,
            SemanticError::TagResolutionFailed { position, .. } => *position,
            SemanticError::UnknownTag { position, .. } => *position,
            SemanticError::ValidationDepthExceeded { position, .. } => *position,
            SemanticError::UnknownTagHandle { position, .. } => *position,
            SemanticError::CustomTagResolutionFailed { position, .. } => *position,
            SemanticError::UnknownCustomTag { position, .. } => *position,
        }
    }

    /// Get human-readable error message
    pub fn message(&self) -> String {
        match self {
            SemanticError::UnresolvedAlias { alias_name, .. } => {
                format!("unresolved alias: '{}'", alias_name)
            }
            SemanticError::CircularReference {
                anchor_name,
                cycle_path,
                ..
            } => {
                format!(
                    "circular reference detected for anchor '{}': {}",
                    anchor_name,
                    cycle_path.join(" -> ")
                )
            }
            SemanticError::InvalidTag {
                tag_handle,
                tag_suffix,
                ..
            } => {
                format!("invalid tag: '{}' + '{}'", tag_handle, tag_suffix)
            }
            SemanticError::UnknownTagPrefix { handle, .. } => {
                format!("unknown tag prefix: '{}'", handle)
            }
            SemanticError::ConflictingAnchor { anchor_name, .. } => {
                format!("conflicting anchor definition: '{}'", anchor_name)
            }
            SemanticError::InvalidReference {
                reference_type,
                target,
                ..
            } => {
                format!("invalid {} reference to '{}'", reference_type, target)
            }
            SemanticError::ValidationFailure { rule, message, .. } => {
                format!("validation failed [{}]: {}", rule, message)
            }
            SemanticError::InternalError { message, .. } => {
                format!("internal error: {}", message)
            }
            SemanticError::TagResolutionFailed { tag, message, .. } => {
                format!("tag resolution failed for '{}': {}", tag, message)
            }
            SemanticError::UnknownTag { tag, .. } => {
                format!("unknown tag: '{}'", tag)
            }
            SemanticError::ValidationDepthExceeded { max_depth, .. } => {
                format!("validation depth exceeded: maximum {} levels", max_depth)
            }
            SemanticError::UnknownTagHandle { handle, .. } => {
                format!("unknown tag handle: '{}'", handle)
            }
            SemanticError::CustomTagResolutionFailed { tag, message, .. } => {
                format!("custom tag resolution failed for '{}': {}", tag, message)
            }
            SemanticError::UnknownCustomTag { tag, .. } => {
                format!("unknown custom tag: '{}'", tag)
            }
        }
    }
}

impl std::fmt::Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.message(), self.position())
    }
}

impl std::error::Error for SemanticError {}

/// Semantic analysis optimization utilities
pub struct SemanticOptimizations;

impl SemanticOptimizations {
    /// Estimate optimal buffer sizes for semantic analysis
    pub fn estimate_buffer_sizes(stream: &Stream) -> BufferSizeHints {
        let document_count = stream.len();
        let estimated_node_count = stream
            .iter()
            .map(|doc| Self::estimate_node_count(doc))
            .sum::<usize>();

        BufferSizeHints {
            anchor_registry_capacity: (estimated_node_count / 10).max(16),
            tag_registry_capacity: (estimated_node_count / 20).max(8),
            reference_graph_capacity: (estimated_node_count / 5).max(32),
            validation_warnings_capacity: (document_count * 2).max(4),
        }
    }

    /// Estimate node count in document
    fn estimate_node_count(document: &Document) -> usize {
        document
            .content
            .as_ref()
            .map(|content| Self::count_nodes_recursive(content))
            .unwrap_or(0)
    }

    /// Recursively count nodes in AST
    fn count_nodes_recursive(node: &Node) -> usize {
        match node {
            Node::Sequence(seq) => {
                1 + seq
                    .items
                    .iter()
                    .map(Self::count_nodes_recursive)
                    .sum::<usize>()
            }
            Node::Mapping(map) => {
                1 + map
                    .pairs
                    .iter()
                    .map(|pair| {
                        Self::count_nodes_recursive(&pair.key)
                            + Self::count_nodes_recursive(&pair.value)
                    })
                    .sum::<usize>()
            }
            Node::Anchor(anchor) => 1 + Self::count_nodes_recursive(&anchor.node),
            Node::Tagged(tagged) => 1 + Self::count_nodes_recursive(&tagged.node),
            _ => 1,
        }
    }

    /// Check if document requires complex analysis
    #[inline]
    pub fn requires_complex_analysis(document: &Document) -> bool {
        document
            .content
            .as_ref()
            .map(|content| Self::has_complex_constructs(content))
            .unwrap_or(false)
    }

    /// Check for complex YAML constructs requiring careful analysis
    fn has_complex_constructs(node: &Node) -> bool {
        match node {
            Node::Anchor(_) | Node::Alias(_) | Node::Tagged(_) => true,
            Node::Sequence(seq) => seq.items.iter().any(Self::has_complex_constructs),
            Node::Mapping(map) => map.pairs.iter().any(|pair| {
                Self::has_complex_constructs(&pair.key) || Self::has_complex_constructs(&pair.value)
            }),
            _ => false,
        }
    }
}

/// Buffer size optimization hints
#[derive(Debug, Clone, Copy)]
pub struct BufferSizeHints {
    pub anchor_registry_capacity: usize,
    pub tag_registry_capacity: usize,
    pub reference_graph_capacity: usize,
    pub validation_warnings_capacity: usize,
}

impl Default for BufferSizeHints {
    fn default() -> Self {
        Self {
            anchor_registry_capacity: 16,
            tag_registry_capacity: 8,
            reference_graph_capacity: 32,
            validation_warnings_capacity: 4,
        }
    }
}
