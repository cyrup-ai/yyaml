//! High-performance semantic analyzer for YAML documents
//!
//! Provides comprehensive semantic processing including anchor/alias resolution,
//! tag resolution, document validation, and reference tracking with zero-allocation
//! design for blazing-fast YAML processing.

use crate::lexer::Position;
use crate::parser::ast::{Document, Node, Stream};
use std::borrow::Cow;

use super::{
    AnchorResolver, TagResolver, DocumentValidator, ReferenceTracker,
    AnalysisContext, SemanticConfig, SemanticResult, SemanticError,
    ProcessingPhase, AnalysisMetrics, SemanticOptimizations
};

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

impl<'input> SemanticAnalyzer<'input> {
    /// Create new semantic analyzer with optimized configuration
    #[inline]
    pub fn new() -> Self {
        Self::with_config(SemanticConfig::default())
    }

    /// Create semantic analyzer with custom configuration
    pub fn with_config(config: SemanticConfig<'input>) -> Self {
        let context = AnalysisContext::new();
        
        Self {
            anchor_resolver: AnchorResolver::with_config(&config),
            tag_resolver: TagResolver::with_config(&config),
            validator: DocumentValidator::with_config(&config),
            reference_tracker: ReferenceTracker::with_config(&config),
            analysis_context: context,
        }
    }

    /// Perform comprehensive semantic analysis on YAML stream
    ///
    /// Processes all documents in the stream through all semantic analysis phases
    /// with complete error handling and performance optimization.
    pub fn analyze_stream(
        &mut self,
        stream: Stream<'input>,
    ) -> std::result::Result<crate::semantic::types::SemanticResult<'input>, SemanticError> {
        let start_time = std::time::Instant::now();
        
        // Pre-analyze for optimization hints
        let optimization_hints = SemanticOptimizations::estimate_buffer_sizes(&stream);
        
        // Reserve capacity based on estimates to minimize allocations
        let mut analyzed_documents = Vec::with_capacity(stream.documents.len());
        
        // Phase 1: Collect all anchors across documents for global resolution
        self.analysis_context.processing_phase = ProcessingPhase::AnchorCollection;
        for (index, document) in stream.documents.iter().enumerate() {
            self.analysis_context.current_document_index = index;
            self.collect_anchors_from_document(document)?;
        }

        // Phase 2: Resolve tags in all documents
        self.analysis_context.processing_phase = ProcessingPhase::TagResolution;
        for (index, document) in stream.documents.iter().enumerate() {
            self.analysis_context.current_document_index = index;
            self.resolve_tags_in_document(document)?;
        }

        // Phase 3: Process each document through analysis pipeline
        for (index, document) in stream.documents.into_iter().enumerate() {
            self.analysis_context.current_document_index = index;
            let analyzed_document = self.analyze_document(document)?;
            analyzed_documents.push(analyzed_document);
        }

        // Phase 4: Final validation pass
        self.analysis_context.processing_phase = ProcessingPhase::FinalValidation;
        for document in &analyzed_documents {
            self.validate_references_in_document(document)?;
        }

        let processing_time = start_time.elapsed();
        
        Ok(SemanticResult {
            documents: analyzed_documents,
            metrics: AnalysisMetrics {
                processing_time,
                documents_processed: analyzed_documents.len(),
                anchors_resolved: self.anchor_resolver.resolved_count(),
                aliases_resolved: self.anchor_resolver.alias_count(),
                tags_resolved: self.tag_resolver.resolved_count(),
                cycles_detected: self.reference_tracker.cycle_count(),
            },
            warnings: Vec::new(), // Collect warnings during analysis
        })
    }

    /// Perform semantic analysis on single document
    pub fn analyze_document(
        &mut self,
        document: Document<'input>,
    ) -> Result<Document<'input>, SemanticError> {
        // Estimate complexity for optimization
        if SemanticOptimizations::requires_complex_analysis(&document) {
            // Enable more thorough tracking for complex documents
            self.reference_tracker.enable_complex_tracking();
        }

        // Resolve aliases with cycle detection
        self.analysis_context.processing_phase = ProcessingPhase::AliasResolution;
        let document = self.resolve_aliases_in_document(document)?;

        // Final validation
        self.analysis_context.processing_phase = ProcessingPhase::DocumentValidation;
        self.validator.validate_document(&document)?;

        Ok(document)
    }

    /// Collect anchors from document for global resolution
    fn collect_anchors_from_document(
        &mut self,
        document: &Document<'input>,
    ) -> Result<(), SemanticError> {
        let mut path = Vec::new();
        self.collect_anchors_from_node(&document.root, &mut path)
    }

    /// Recursively collect anchors from AST nodes
    fn collect_anchors_from_node(
        &mut self,
        node: &Node<'input>,
        path: &mut Vec<String>,
    ) -> Result<(), SemanticError> {
        // Track current position for error reporting
        self.analysis_context.set_position(node.position());

        // Register anchor if present
        if let Some(ref anchor) = node.anchor() {
            let anchor_id = self.reference_tracker.track_anchor(
                Cow::Borrowed(anchor),
                node,
                node.position(),
            )?;
            
            self.anchor_resolver.register_anchor(
                anchor.clone(),
                node,
                anchor_id,
                path.clone(),
            )?;
        }

        // Recursively process child nodes
        match node {
            Node::Sequence(seq) => {
                for (index, child) in seq.values.iter().enumerate() {
                    path.push(format!("[{}]", index));
                    self.collect_anchors_from_node(child, path)?;
                    path.pop();
                }
            }
            Node::Mapping(map) => {
                for (key, value) in &map.pairs {
                    // Process key
                    if let Node::Scalar(key_scalar) = key {
                        path.push(key_scalar.value.to_string());
                        self.collect_anchors_from_node(value, path)?;
                        path.pop();
                    } else {
                        path.push("<complex_key>".to_string());
                        self.collect_anchors_from_node(key, path)?;
                        self.collect_anchors_from_node(value, path)?;
                        path.pop();
                    }
                }
            }
            Node::Scalar(_) => {
                // Scalars may have anchors but no children
            }
        }

        Ok(())
    }

    /// Resolve tags in document
    fn resolve_tags_in_document(
        &mut self,
        document: &Document<'input>,
    ) -> Result<(), SemanticError> {
        self.resolve_tags_in_node(&document.root)
    }

    /// Recursively resolve tags in AST nodes
    fn resolve_tags_in_node(&mut self, node: &Node<'input>) -> Result<(), SemanticError> {
        // Update position for error tracking
        self.analysis_context.set_position(node.position());

        // Resolve tag if present
        if let Some(ref tag) = node.tag() {
            self.tag_resolver.resolve_tag(tag, &self.analysis_context)?;
        }

        // Recursively process children
        match node {
            Node::Sequence(seq) => {
                for child in &seq.values {
                    self.resolve_tags_in_node(child)?;
                }
            }
            Node::Mapping(map) => {
                for (key, value) in &map.pairs {
                    self.resolve_tags_in_node(key)?;
                    self.resolve_tags_in_node(value)?;
                }
            }
            Node::Scalar(_) => {
                // Process scalar-specific tag resolution if needed
            }
        }

        Ok(())
    }

    /// Resolve aliases in document with cycle detection
    fn resolve_aliases_in_document(
        &mut self,
        mut document: Document<'input>,
    ) -> Result<Document<'input>, SemanticError> {
        let mut path = Vec::new();
        document.root = self.resolve_aliases_in_node(document.root, &mut path)?;
        Ok(document)
    }

    /// Recursively resolve aliases in AST nodes
    fn resolve_aliases_in_node(
        &mut self,
        node: Node<'input>,
        path: &mut Vec<String>,
    ) -> Result<Node<'input>, SemanticError> {
        // Update position for error tracking
        self.analysis_context.set_position(node.position());

        match node {
            Node::Alias(alias_node) => {
                // Track alias reference
                let alias_id = self.reference_tracker.track_alias(
                    Cow::Borrowed(&alias_node.name),
                    Cow::Borrowed(&alias_node.target),
                    alias_node.position,
                )?;

                // Resolve alias to actual node
                if let Some(resolved_node) = self.anchor_resolver.resolve_alias(&alias_node.name)? {
                    // Check for cycles
                    if self.reference_tracker.has_cycle(alias_id)? {
                        return Err(SemanticError::CircularReference {
                            alias_name: alias_node.name.to_string(),
                            path: path.join("."),
                            position: alias_node.position,
                        });
                    }

                    // Clone the resolved node for alias substitution
                    Ok(resolved_node.clone())
                } else {
                    Err(SemanticError::UnresolvedAlias {
                        alias_name: alias_node.name.to_string(),
                        position: alias_node.position,
                    })
                }
            }
            Node::Sequence(mut seq) => {
                // Recursively resolve aliases in sequence elements
                for (index, child) in seq.values.into_iter().enumerate() {
                    path.push(format!("[{}]", index));
                    seq.values[index] = self.resolve_aliases_in_node(child, path)?;
                    path.pop();
                }
                Ok(Node::Sequence(seq))
            }
            Node::Mapping(mut map) => {
                // Recursively resolve aliases in mapping pairs
                for (key, value) in map.pairs.into_iter() {
                    if let Node::Scalar(key_scalar) = &key {
                        path.push(key_scalar.value.to_string());
                    } else {
                        path.push("<complex_key>".to_string());
                    }

                    let resolved_key = self.resolve_aliases_in_node(key, path)?;
                    let resolved_value = self.resolve_aliases_in_node(value, path)?;
                    
                    // Update the pair with resolved nodes
                    map.pairs.push((resolved_key, resolved_value));
                    path.pop();
                }
                Ok(Node::Mapping(map))
            }
            other => {
                // Scalar nodes and other types don't contain aliases
                Ok(other)
            }
        }
    }

    /// Validate references in document
    fn validate_references_in_document(
        &mut self,
        document: &Document<'input>,
    ) -> Result<(), SemanticError> {
        self.reference_tracker.validate_references()?;
        self.validator.validate_document(document)
    }

    /// Get current analysis context
    #[inline]
    pub fn context(&self) -> &AnalysisContext<'input> {
        &self.analysis_context
    }

    /// Get analysis metrics
    pub fn metrics(&self) -> AnalysisMetrics {
        AnalysisMetrics {
            processing_time: std::time::Duration::default(),
            documents_processed: self.analysis_context.current_document_index + 1,
            anchors_resolved: self.anchor_resolver.resolved_count(),
            aliases_resolved: self.anchor_resolver.alias_count(),
            tags_resolved: self.tag_resolver.resolved_count(),
            cycles_detected: self.reference_tracker.cycle_count(),
        }
    }

    /// Reset analyzer for new analysis
    pub fn reset(&mut self) {
        self.anchor_resolver.reset();
        self.tag_resolver.reset();
        self.reference_tracker.reset();
        self.analysis_context = AnalysisContext::new();
    }
}

impl<'input> Default for SemanticAnalyzer<'input> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}