//! Anchor and alias resolution facade module
//!
//! This facade module provides unified access to anchor/alias resolution functionality
//! through a clean, decomposed module structure. All implementation details are
//! organized into focused submodules with logical separation of concerns.
//!
//! ## Architecture Overview
//!
//! The anchor resolution system is decomposed into specialized modules:
//!
//! - **`resolver`**: Core anchor resolution logic with comprehensive cycle detection
//! - **`registry`**: Anchor definition storage, management, and validation
//! - **`cache`**: High-performance caching with intelligent eviction strategies
//! - **`context`**: Resolution context management and cycle detection state
//! - **`statistics`**: Performance monitoring and usage analytics
//! - **`optimization`**: Performance optimization utilities and recommendations
//! - **`types`**: Core types, traits, and shared definitions
//!
//! ## Key Features
//!
//! - **Zero-allocation design**: Optimized for blazing-fast performance
//! - **Comprehensive cycle detection**: Prevents infinite recursion with detailed error reporting
//! - **Intelligent caching**: LRU/LFU eviction with configurable policies
//! - **Performance monitoring**: Detailed statistics and optimization suggestions
//! - **Memory efficiency**: Precise memory usage estimation and optimization
//! - **YAML 1.2 compliance**: Complete specification adherence
//!
//! ## Usage
//!
//! ```rust
//! use yyaml::semantic::anchors::{AnchorResolver, AnchorRegistry};
//!
//! let mut resolver = AnchorResolver::new();
//! // Use resolver for anchor resolution...
//! ```

pub mod cache;
pub mod context;
pub mod optimization;
pub mod registry;
pub mod resolver;
pub mod statistics;
pub mod types;

// Re-export all public types for unified access
pub use resolver::AnchorResolver;
pub use registry::{AnchorRegistry, AnchorDefinition, RegistryStatistics, RegistryValidationError};
pub use cache::{CachedResolution, CacheStatistics, CacheManager, CacheConfig};
pub use optimization::{
    AnchorOptimizations, MemoryUsageEstimate, OptimizationSuggestion, 
    OptimizationReport, ComplexityAnalysis, CacheTuningRecommendations,
    MemoryBreakdown, EfficiencyMetrics, Priority, ComplexAnchor,
    EvictionStrategy, PrefetchSuggestion, MonitoringRecommendation
};
pub use statistics::{AnchorStatistics, AnchorValidationWarning};
pub use context::{ResolutionContext};
pub use types::*;