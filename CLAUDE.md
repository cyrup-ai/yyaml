# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**yyaml** is a Rust YAML parser library providing serde support. It's a YAML 1.2 compliant parser with state machine architecture, focusing on zero-allocation optimizations and comprehensive spec compliance.

## Essential Commands

```bash
# Build the project
cargo build

# Run library tests only (all pass)
cargo test --lib

# Run specific test to avoid infinite loop bug
cargo test test_name

# WARNING: One test has an infinite loop bug - avoid running all tests
# cargo test  # DO NOT RUN - will hang on test_minimal_tagged_sequence

# Check for warnings
cargo clippy

# Format code
cargo fmt

# Run with debug logging
RUST_LOG=debug cargo test test_name -- --nocapture
```

## Architecture Overview

### Core Components

**Data Model (`src/yaml.rs`)**
- `Yaml` enum represents all YAML values with proper Hash implementation
- `LinkedHashMap` preserves insertion order for mappings
- Supports all YAML types: scalars, sequences, mappings, aliases

**Parser Pipeline**
1. **Scanner (`src/scanner/mod.rs`)** - Tokenizes input into `TokenType` events
2. **State Machine (`src/parser/state_machine.rs`)** - Single state machine parser
3. **Loader (`src/parser/loader.rs`)** - Public API via `YamlLoader::load_from_str()`

**Parser State Machine**
- `State` enum defines parsing states (StreamStart, BlockNode, FlowSequenceEntry, etc.)
- `StateMachine` builds AST directly during parsing
- Uses `YamlBuilder` enum for constructing AST nodes

### Current Issues

**Critical Bug**: `test_minimal_tagged_sequence` causes infinite loop - parser gets stuck in recursion when handling certain tagged sequences.

**Parser Modules Status**:
- `parser/state_machine.rs` - Main state machine implementation
- `parser/grammar.rs` - Production rules for YAML grammar
- `parser/indentation.rs` - Indentation tracking
- `parser/flow.rs` - Flow collection parsing
- `parser/document.rs` - Document-level parsing

## Key Constraints

Per TODO.md, strict compliance guidelines:
- **Zero allocation** where possible using `Cow<str>`
- **No unsafe code**
- **No unwrap/expect** in source code
- **Comprehensive error handling** with position tracking
- **Full YAML 1.2 spec compliance** as target

## Test Structure

**Main Test Files**:
- `tests/test_de.rs` - Deserialization tests
- `tests/test_yaml.rs` - YAML parsing tests
- `tests/test_yaml_1_2_compliance.rs` - Spec compliance tests
- `tests/debug_*.rs` - Debug utilities for specific issues

**Test Coverage**:
- Library tests: 81 passing
- Integration tests: Most passing except infinite loop case
- RFC compliance tests planned in `tests/rfc_compliance/` (TODO)

## Development Workflow

1. Make changes to parser/scanner modules
2. Run `cargo test --lib` first to verify library tests
3. Test specific integration tests individually
4. Use debug builds with `RUST_LOG=debug` for tracing
5. Avoid running full test suite until infinite loop is fixed

## Next Priority Tasks

Per TODO.md Phase plan:
1. Fix infinite loop in `test_minimal_tagged_sequence`
2. Implement parametric productions for YAML 1.2 spec
3. Add character productions module
4. Complete RFC compliance test suite
5. Achieve 100% spec compliance