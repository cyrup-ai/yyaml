//! RFC Compliance Test Suite for YAML 1.2.2
//! 
//! This module contains comprehensive test coverage for every MUST/MUST NOT requirement
//! in the YAML 1.2.2 specification (RFC). Tests are organized to mirror the exact 
//! structure of the specification document for complete traceability.
//! 
//! ## Test Structure
//! 
//! The test structure mirrors the RFC documentation structure:
//! 
//! ```
//! tests/rfc_compliance/
//! ├── ch01_introduction/          # Chapter 1: Introduction
//! ├── ch02_language_overview/     # Chapter 2: Language Overview  
//! ├── ch03_processes_models/      # Chapter 3: Processes & Models
//! ├── ch04_syntax_conventions/    # Chapter 4: Syntax Conventions
//! ├── ch05_character_productions/ # Chapter 5: Character Productions ✅ 
//! ├── ch06_structural_productions/# Chapter 6: Structural Productions
//! ├── ch07_flow_style_productions/# Chapter 7: Flow Style Productions
//! ├── ch08_block_style_productions/# Chapter 8: Block Style Productions
//! ├── ch09_document_stream_productions/# Chapter 9: Document Stream Productions
//! └── ch10_recommended_schemas/   # Chapter 10: Recommended Schemas
//! ```
//! 
//! ## Completed Chapters
//! 
//! ### ✅ Chapter 5: Character Productions
//! 
//! Complete test coverage for all character-level requirements:
//! - Character set compliance (printable characters)
//! - Character encoding support (UTF-8/16/32, BOM handling)
//! - Indicator character recognition  
//! - Line break normalization
//! - Whitespace handling
//! - Escape sequence processing
//! 
//! ## Usage
//! 
//! Run all RFC compliance tests:
//! ```bash
//! cargo test --test rfc_compliance
//! ```
//! 
//! Run specific chapter tests:
//! ```bash
//! cargo test rfc_compliance::ch05_character_productions
//! ```
//! 
//! ## Documentation Links
//! 
//! Each test module is hyperlinked from the corresponding RFC documentation
//! in `docs/` for complete traceability between requirements and tests.

pub mod ch05_character_productions;