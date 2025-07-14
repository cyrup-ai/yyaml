# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**yyaml** is a Rust YAML parser library providing serde support for yaml-rust2. It aims to be a modern replacement for the unmaintained `yyaml` crate with zero-allocation optimizations and comprehensive YAML 1.2 support.

## Essential Commands

```bash
# Build the project
cargo build

# Run tests (11/12 currently pass)
cargo test

# Run a specific test
cargo test test_name

# Check for warnings
cargo clippy

# Format code
cargo fmt
```

## Architecture Overview

### Core Components

**Data Model (`src/yaml.rs`)**
- `Yaml` enum represents all YAML values with proper Hash implementation
- `LinkedHashMap` preserves insertion order for mappings
- Supports all YAML types: scalars, sequences, mappings, aliases

**Parser Pipeline**
1. **Scanner (`src/scanner.rs`)** - Tokenizes input into `TokenType` variants
2. **Parser (`src/parser/mod.rs`)** - Event-driven state machine with `Event` enum
3. **Loader (`src/parser/loader.rs`)** - Builds AST via `YamlReceiver`

**Parser Modules**
- `parser/block.rs` - Block mappings and sequences (currently has bugs)
- `parser/flow.rs` - Flow syntax `[]` and `{}`  
- `parser/document.rs` - Document-level parsing
- `parser/loader.rs` - Public API `YamlLoader::load_from_str()`

### Current Status

**Working**: Basic values, single-line mappings, flow sequences, tokenization
**Broken**: Multi-line block mappings fail with "did not find expected node content" error

The main bug is in `src/parser/block.rs` around line handling for subsequent mapping entries.

## Key Constraints

Based on TODO.md, this project follows strict guidelines:
- **Zero allocation** where possible using `Cow<str>`
- **No unsafe code** 
- **No unwrap/expect** in source code
- **Comprehensive error handling** with position tracking
- **YAML 1.2 compliance** target

## Test Structure

Tests in `tests/test_de.rs` cover:
- All YAML value types and conversions
- Flow and block syntax
- Anchors/aliases, nested structures
- Edge cases and YAML 1.2 features

`tests/test_se.rs` is empty - serialization not implemented yet.

## Next Priority Tasks

1. Fix multi-line block mapping parser bug
2. Implement serde `Deserializer` trait
3. Complete flow parsing in `parser/flow.rs`
4. Add YAML emission in `src/emitter.rs`
5. Implement comprehensive error messages with context