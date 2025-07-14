//! Anchor and alias resolution with comprehensive cycle detection
//!
//! This module provides robust anchor/alias resolution for YAML documents with
//! complete cycle detection, memory-efficient storage, and blazing-fast lookup.

use super::SemanticError;
use crate::lexer::Position;
use crate::parser::ast::Node;
use std::borrow::Cow;
use std::collections::HashMap;

/// High-performance anchor resolver with cycle detection
#[derive(Debug)]
pub struct AnchorResolver<'input> {
    anchor_registry: AnchorRegistry<'input>,
    resolution_cache: HashMap<String, CachedResolution<'input>>,
    alias_resolution_count: usize,
    cycle_detection_stack: Vec<String>,
}

/// Registry for anchor definitions with efficient lookup
#[derive(Debug, Clone)]
pub struct AnchorRegistry<'input> {
    anchors: HashMap<String, AnchorDefinition<'input>>,
    resolution_order: Vec<String>,
}

/// Anchor definition with complete metadata
#[derive(Debug, Clone)]
pub struct AnchorDefinition<'input> {
    pub name: Cow<'input, str>,
    pub node: Node<'input>,
    pub position: Position,
    pub definition_path: Vec<String>,
    pub first_seen: std::time::Instant,
    pub resolution_count: usize,
}

/// Cached resolution result for performance optimization
#[derive(Debug, Clone)]
struct CachedResolution<'input> {
    resolved_node: Node<'input>,
    cached_at: std::time::Instant,
    access_count: usize,
}

/// Alias resolution context for cycle detection
#[derive(Debug, Clone)]
pub struct ResolutionContext {
    pub current_depth: usize,
    pub max_depth: usize,
    pub resolution_path: Vec<String>,
    pub visited_anchors: Vec<String>,
}

impl<'input> AnchorResolver<'input> {
    /// Create new anchor resolver with optimized configuration
    pub fn new() -> Self {
        Self {
            anchor_registry: AnchorRegistry::new(),
            resolution_cache: HashMap::new(),
            alias_resolution_count: 0,
            cycle_detection_stack: Vec::new(),
        }
    }

    /// Register anchor definition with conflict detection
    pub fn register_anchor(
        &mut self,
        name: Cow<'input, str>,
        node: &Node<'input>,
        position: Position,
        definition_path: Vec<String>,
    ) -> Result<(), SemanticError> {
        let anchor_name = name.to_string();

        // Check for conflicting anchor definitions
        if let Some(existing) = self.anchor_registry.anchors.get(&anchor_name) {
            return Err(SemanticError::ConflictingAnchor {
                anchor_name,
                first_position: existing.position,
                second_position: position,
            });
        }

        // Deep clone the node to avoid borrowing issues
        let cloned_node = self.deep_clone_node(node);

        let definition = AnchorDefinition {
            name: name.clone(),
            node: cloned_node,
            position,
            definition_path,
            first_seen: std::time::Instant::now(),
            resolution_count: 0,
        };

        self.anchor_registry
            .anchors
            .insert(anchor_name.clone(), definition);
        self.anchor_registry.resolution_order.push(anchor_name);

        Ok(())
    }

    /// Resolve alias with comprehensive cycle detection
    pub fn resolve_alias(
        &mut self,
        alias_name: &Cow<'input, str>,
        position: Position,
    ) -> Result<Node<'input>, SemanticError> {
        let alias_key = alias_name.to_string();

        // Check for immediate cycle
        if self.cycle_detection_stack.contains(&alias_key) {
            let cycle_start = self
                .cycle_detection_stack
                .iter()
                .position(|name| name == &alias_key)
                .unwrap_or(0);
            let cycle_path = self.cycle_detection_stack[cycle_start..].to_vec();

            return Err(SemanticError::CircularReference {
                anchor_name: alias_key,
                cycle_path,
                position,
            });
        }

        // Check resolution cache first
        if let Some(cached) = self.resolution_cache.get_mut(&alias_key) {
            cached.access_count += 1;
            self.alias_resolution_count += 1;
            // Clone the resolved node before ending the borrow
            let resolved_node = cached.resolved_node.clone();
            return Ok(resolved_node);
        }

        // Find anchor definition and extract the node to avoid borrowing conflicts
        let anchor_node = {
            let anchor_def = self
                .anchor_registry
                .anchors
                .get_mut(&alias_key)
                .ok_or_else(|| SemanticError::UnresolvedAlias {
                    alias_name: alias_key.clone(),
                    position,
                })?;

            // Update resolution count while we have the mutable reference
            anchor_def.resolution_count += 1;

            // Clone the node to avoid borrowing conflicts
            anchor_def.node.clone()
        };

        // Add to cycle detection stack
        self.cycle_detection_stack.push(alias_key.clone());

        // Perform recursive resolution
        let resolved_node = self.resolve_node_recursive(&anchor_node)?;

        // Remove from cycle detection stack
        self.cycle_detection_stack.pop();

        // Cache the resolution
        self.cache_resolution(alias_key.clone(), &resolved_node);

        self.alias_resolution_count += 1;
        Ok(resolved_node)
    }

    /// Recursively resolve nodes with alias expansion
    fn resolve_node_recursive(
        &mut self,
        node: &Node<'input>,
    ) -> Result<Node<'input>, SemanticError> {
        match node {
            Node::Alias(alias_node) => {
                // Recursive alias resolution
                self.resolve_alias(&alias_node.name, alias_node.position)
            }
            Node::Sequence(seq_node) => {
                let mut resolved_items = Vec::with_capacity(seq_node.items.len());
                for item in &seq_node.items {
                    let resolved_item = self.resolve_node_recursive(item)?;
                    resolved_items.push(resolved_item);
                }

                Ok(Node::Sequence(crate::parser::ast::SequenceNode::new(
                    resolved_items,
                    seq_node.style,
                    seq_node.position,
                )))
            }
            Node::Mapping(map_node) => {
                let mut resolved_pairs = Vec::with_capacity(map_node.pairs.len());
                for pair in &map_node.pairs {
                    let resolved_key = self.resolve_node_recursive(&pair.key)?;
                    let resolved_value = self.resolve_node_recursive(&pair.value)?;
                    resolved_pairs.push(crate::parser::ast::MappingPair::new(
                        resolved_key,
                        resolved_value,
                    ));
                }

                Ok(Node::Mapping(crate::parser::ast::MappingNode::new(
                    resolved_pairs,
                    map_node.style,
                    map_node.position,
                )))
            }
            Node::Anchor(anchor_node) => {
                // Resolve the anchored content
                let resolved_inner = self.resolve_node_recursive(&anchor_node.node)?;
                Ok(Node::Anchor(crate::parser::ast::AnchorNode::new(
                    anchor_node.name.clone(),
                    Box::new(resolved_inner),
                    anchor_node.position,
                )))
            }
            Node::Tagged(tagged_node) => {
                // Resolve tagged content
                let resolved_inner = self.resolve_node_recursive(&tagged_node.node)?;
                Ok(Node::Tagged(crate::parser::ast::TaggedNode::new(
                    tagged_node.handle.clone(),
                    tagged_node.suffix.clone(),
                    Box::new(resolved_inner),
                    tagged_node.position,
                )))
            }
            // For scalar and null nodes, return as-is
            other => Ok(self.deep_clone_node(other)),
        }
    }

    /// Deep clone node to avoid borrowing conflicts
    fn deep_clone_node(&self, node: &Node<'input>) -> Node<'input> {
        match node {
            Node::Scalar(scalar) => Node::Scalar(scalar.clone()),
            Node::Sequence(seq) => {
                let cloned_items = seq
                    .items
                    .iter()
                    .map(|item| self.deep_clone_node(item))
                    .collect();
                Node::Sequence(crate::parser::ast::SequenceNode::new(
                    cloned_items,
                    seq.style,
                    seq.position,
                ))
            }
            Node::Mapping(map) => {
                let cloned_pairs = map
                    .pairs
                    .iter()
                    .map(|pair| {
                        crate::parser::ast::MappingPair::new(
                            self.deep_clone_node(&pair.key),
                            self.deep_clone_node(&pair.value),
                        )
                    })
                    .collect();
                Node::Mapping(crate::parser::ast::MappingNode::new(
                    cloned_pairs,
                    map.style,
                    map.position,
                ))
            }
            Node::Anchor(anchor) => Node::Anchor(crate::parser::ast::AnchorNode::new(
                anchor.name.clone(),
                Box::new(self.deep_clone_node(&anchor.node)),
                anchor.position,
            )),
            Node::Alias(alias) => Node::Alias(crate::parser::ast::AliasNode::new(
                alias.name.clone(),
                alias.position,
            )),
            Node::Tagged(tagged) => Node::Tagged(crate::parser::ast::TaggedNode::new(
                tagged.handle.clone(),
                tagged.suffix.clone(),
                Box::new(self.deep_clone_node(&tagged.node)),
                tagged.position,
            )),
            Node::Null(null_node) => Node::Null(null_node.clone()),
        }
    }

    /// Cache resolution result for performance
    fn cache_resolution(&mut self, alias_name: String, resolved_node: &Node<'input>) {
        let cached = CachedResolution {
            resolved_node: self.deep_clone_node(resolved_node),
            cached_at: std::time::Instant::now(),
            access_count: 1,
        };
        self.resolution_cache.insert(alias_name, cached);
    }

    /// Get comprehensive anchor statistics
    pub fn get_anchor_statistics(&self) -> AnchorStatistics {
        let total_anchors = self.anchor_registry.anchors.len();
        let total_resolutions = self
            .anchor_registry
            .anchors
            .values()
            .map(|def| def.resolution_count)
            .sum();
        let cached_resolutions = self.resolution_cache.len();
        let avg_resolution_count = if total_anchors > 0 {
            total_resolutions as f64 / total_anchors as f64
        } else {
            0.0
        };

        AnchorStatistics {
            total_anchors,
            total_resolutions,
            cached_resolutions,
            alias_resolution_count: self.alias_resolution_count,
            avg_resolution_count,
            cache_hit_ratio: if self.alias_resolution_count > 0 {
                cached_resolutions as f64 / self.alias_resolution_count as f64
            } else {
                0.0
            },
        }
    }

    /// Validate all anchor definitions for potential issues
    pub fn validate_anchors(&self) -> Vec<AnchorValidationWarning> {
        let mut warnings = Vec::new();

        for (name, definition) in &self.anchor_registry.anchors {
            // Check for unused anchors
            if definition.resolution_count == 0 {
                warnings.push(AnchorValidationWarning::UnusedAnchor {
                    anchor_name: name.clone(),
                    position: definition.position,
                });
            }

            // Check for deeply nested anchor definitions
            if definition.definition_path.len() > 10 {
                warnings.push(AnchorValidationWarning::DeeplyNestedAnchor {
                    anchor_name: name.clone(),
                    depth: definition.definition_path.len(),
                    position: definition.position,
                });
            }

            // Check for potential circular structures within the anchor
            if self.contains_potential_cycles(&definition.node, name) {
                warnings.push(AnchorValidationWarning::PotentialCircularStructure {
                    anchor_name: name.clone(),
                    position: definition.position,
                });
            }
        }

        warnings
    }

    /// Check if node contains potential circular references
    fn contains_potential_cycles(&self, node: &Node<'input>, anchor_name: &str) -> bool {
        match node {
            Node::Alias(alias) => alias.name.as_ref() == anchor_name,
            Node::Sequence(seq) => seq
                .items
                .iter()
                .any(|item| self.contains_potential_cycles(item, anchor_name)),
            Node::Mapping(map) => map.pairs.iter().any(|pair| {
                self.contains_potential_cycles(&pair.key, anchor_name)
                    || self.contains_potential_cycles(&pair.value, anchor_name)
            }),
            Node::Anchor(anchor) => {
                anchor.name.as_ref() == anchor_name
                    || self.contains_potential_cycles(&anchor.node, anchor_name)
            }
            Node::Tagged(tagged) => self.contains_potential_cycles(&tagged.node, anchor_name),
            _ => false,
        }
    }

    /// Get anchor registry for external access
    #[inline]
    pub fn get_registry(&self) -> AnchorRegistry<'input> {
        self.anchor_registry.clone()
    }

    /// Get number of registered anchors
    #[inline]
    pub fn anchor_count(&self) -> usize {
        self.anchor_registry.anchors.len()
    }

    /// Get alias resolution count
    #[inline]
    pub fn alias_resolution_count(&self) -> usize {
        self.alias_resolution_count
    }

    /// Reset resolver for new analysis
    pub fn reset(&mut self) {
        self.anchor_registry.anchors.clear();
        self.anchor_registry.resolution_order.clear();
        self.resolution_cache.clear();
        self.alias_resolution_count = 0;
        self.cycle_detection_stack.clear();
    }

    /// Optimize resolution cache by removing least recently used entries
    pub fn optimize_cache(&mut self, max_cache_size: usize) {
        if self.resolution_cache.len() <= max_cache_size {
            return;
        }

        // Collect cache entries with access statistics
        let mut cache_entries: Vec<_> = self
            .resolution_cache
            .iter()
            .map(|(name, cached)| (name.clone(), cached.access_count, cached.cached_at))
            .collect();

        // Sort by access count (ascending) and then by age (oldest first)
        cache_entries.sort_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2)));

        // Remove least accessed/oldest entries
        let to_remove = cache_entries.len() - max_cache_size;
        for i in 0..to_remove {
            self.resolution_cache.remove(&cache_entries[i].0);
        }
    }

    /// Resolve all aliases in dependency order
    pub fn resolve_all_aliases(&mut self) -> Result<Vec<(String, Node<'input>)>, SemanticError> {
        let mut resolved_aliases = Vec::new();

        // Clone the resolution order to avoid borrowing conflicts
        let resolution_order = self.anchor_registry.resolution_order.clone();

        for anchor_name in resolution_order {
            if let Some(definition) = self.anchor_registry.anchors.get(&anchor_name) {
                let position = definition.position;
                let resolved = self.resolve_alias(&Cow::Owned(anchor_name.clone()), position)?;
                resolved_aliases.push((anchor_name, resolved));
            }
        }

        Ok(resolved_aliases)
    }
}

impl<'input> AnchorRegistry<'input> {
    /// Create new anchor registry
    pub fn new() -> Self {
        Self {
            anchors: HashMap::new(),
            resolution_order: Vec::new(),
        }
    }

    /// Get anchor definition by name
    #[inline]
    pub fn get_anchor(&self, name: &str) -> Option<&AnchorDefinition<'input>> {
        self.anchors.get(name)
    }

    /// Get all anchor names
    pub fn anchor_names(&self) -> Vec<&str> {
        self.anchors.keys().map(|s| s.as_str()).collect()
    }

    /// Get anchors in resolution order
    pub fn anchors_in_order(&self) -> Vec<&AnchorDefinition<'input>> {
        self.resolution_order
            .iter()
            .filter_map(|name| self.anchors.get(name))
            .collect()
    }

    /// Check if anchor exists
    #[inline]
    pub fn contains_anchor(&self, name: &str) -> bool {
        self.anchors.contains_key(name)
    }

    /// Get anchor count
    #[inline]
    pub fn len(&self) -> usize {
        self.anchors.len()
    }

    /// Check if registry is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.anchors.is_empty()
    }
}

impl<'input> AnchorDefinition<'input> {
    /// Create new anchor definition
    pub fn new(
        name: Cow<'input, str>,
        node: Node<'input>,
        position: Position,
        definition_path: Vec<String>,
    ) -> Self {
        Self {
            name,
            node,
            position,
            definition_path,
            first_seen: std::time::Instant::now(),
            resolution_count: 0,
        }
    }

    /// Get anchor name as string
    #[inline]
    pub fn name_str(&self) -> &str {
        self.name.as_ref()
    }

    /// Get definition path as string
    pub fn path_string(&self) -> String {
        self.definition_path.join(".")
    }

    /// Check if anchor has been resolved
    #[inline]
    pub fn is_resolved(&self) -> bool {
        self.resolution_count > 0
    }

    /// Get time since first seen
    pub fn age(&self) -> std::time::Duration {
        self.first_seen.elapsed()
    }
}

impl Default for ResolutionContext {
    fn default() -> Self {
        Self {
            current_depth: 0,
            max_depth: 100, // Reasonable recursion limit
            resolution_path: Vec::new(),
            visited_anchors: Vec::new(),
        }
    }
}

impl ResolutionContext {
    /// Create new resolution context with custom max depth
    pub fn with_max_depth(max_depth: usize) -> Self {
        Self {
            max_depth,
            ..Default::default()
        }
    }

    /// Check if at maximum depth
    #[inline]
    pub fn at_max_depth(&self) -> bool {
        self.current_depth >= self.max_depth
    }

    /// Enter new resolution level
    pub fn enter(&mut self, anchor_name: String) -> Result<(), SemanticError> {
        if self.at_max_depth() {
            return Err(SemanticError::CircularReference {
                anchor_name: anchor_name.clone(),
                cycle_path: self.resolution_path.clone(),
                position: Position::start(),
            });
        }

        if self.visited_anchors.contains(&anchor_name) {
            return Err(SemanticError::CircularReference {
                anchor_name,
                cycle_path: self.resolution_path.clone(),
                position: Position::start(),
            });
        }

        self.current_depth += 1;
        self.resolution_path.push(anchor_name.clone());
        self.visited_anchors.push(anchor_name);
        Ok(())
    }

    /// Exit current resolution level
    pub fn exit(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
            self.resolution_path.pop();
            self.visited_anchors.pop();
        }
    }
}

/// Anchor resolution statistics
#[derive(Debug, Clone)]
pub struct AnchorStatistics {
    pub total_anchors: usize,
    pub total_resolutions: usize,
    pub cached_resolutions: usize,
    pub alias_resolution_count: usize,
    pub avg_resolution_count: f64,
    pub cache_hit_ratio: f64,
}

/// Anchor validation warnings
#[derive(Debug, Clone)]
pub enum AnchorValidationWarning {
    UnusedAnchor {
        anchor_name: String,
        position: Position,
    },
    DeeplyNestedAnchor {
        anchor_name: String,
        depth: usize,
        position: Position,
    },
    PotentialCircularStructure {
        anchor_name: String,
        position: Position,
    },
}

impl AnchorValidationWarning {
    /// Get warning message
    pub fn message(&self) -> String {
        match self {
            AnchorValidationWarning::UnusedAnchor { anchor_name, .. } => {
                format!("unused anchor: '{}'", anchor_name)
            }
            AnchorValidationWarning::DeeplyNestedAnchor {
                anchor_name, depth, ..
            } => {
                format!("deeply nested anchor '{}' at depth {}", anchor_name, depth)
            }
            AnchorValidationWarning::PotentialCircularStructure { anchor_name, .. } => {
                format!("potential circular structure in anchor '{}'", anchor_name)
            }
        }
    }

    /// Get warning position
    pub fn position(&self) -> Position {
        match self {
            AnchorValidationWarning::UnusedAnchor { position, .. } => *position,
            AnchorValidationWarning::DeeplyNestedAnchor { position, .. } => *position,
            AnchorValidationWarning::PotentialCircularStructure { position, .. } => *position,
        }
    }
}

/// Anchor resolution optimizations
pub struct AnchorOptimizations;

impl AnchorOptimizations {
    /// Calculate optimal cache size based on document characteristics
    pub fn calculate_optimal_cache_size(
        anchor_count: usize,
        estimated_alias_count: usize,
    ) -> usize {
        // Use heuristic: cache size should be 2x the number of anchors or alias count, whichever is larger
        let base_size = (anchor_count * 2).max(estimated_alias_count);

        // Cap at reasonable maximum to prevent excessive memory usage
        base_size.min(1000).max(16)
    }

    /// Estimate memory usage for anchor registry
    pub fn estimate_memory_usage(registry: &AnchorRegistry) -> MemoryUsageEstimate {
        let anchor_memory = registry.anchors.len() * std::mem::size_of::<AnchorDefinition>();
        let name_memory = registry
            .anchors
            .keys()
            .map(|name| name.len())
            .sum::<usize>();
        let path_memory = registry
            .anchors
            .values()
            .map(|def| def.definition_path.iter().map(|p| p.len()).sum::<usize>())
            .sum::<usize>();

        MemoryUsageEstimate {
            anchor_definitions: anchor_memory,
            anchor_names: name_memory,
            definition_paths: path_memory,
            total: anchor_memory + name_memory + path_memory,
        }
    }

    /// Suggest optimizations based on anchor usage patterns
    pub fn suggest_optimizations(statistics: &AnchorStatistics) -> Vec<OptimizationSuggestion> {
        let mut suggestions = Vec::new();

        if statistics.cache_hit_ratio < 0.5 {
            suggestions.push(OptimizationSuggestion::IncreaseCacheSize);
        }

        if statistics.avg_resolution_count > 10.0 {
            suggestions.push(OptimizationSuggestion::PrecomputeFrequentAliases);
        }

        if statistics.total_anchors > 100 {
            suggestions.push(OptimizationSuggestion::UseIncrementalResolution);
        }

        suggestions
    }
}

/// Memory usage estimation
#[derive(Debug, Clone)]
pub struct MemoryUsageEstimate {
    pub anchor_definitions: usize,
    pub anchor_names: usize,
    pub definition_paths: usize,
    pub total: usize,
}

/// Optimization suggestions for anchor processing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationSuggestion {
    IncreaseCacheSize,
    PrecomputeFrequentAliases,
    UseIncrementalResolution,
}

impl OptimizationSuggestion {
    pub fn description(&self) -> &'static str {
        match self {
            OptimizationSuggestion::IncreaseCacheSize => {
                "increase resolution cache size to improve hit ratio"
            }
            OptimizationSuggestion::PrecomputeFrequentAliases => {
                "precompute frequently referenced aliases for better performance"
            }
            OptimizationSuggestion::UseIncrementalResolution => {
                "use incremental resolution for large anchor sets"
            }
        }
    }
}
