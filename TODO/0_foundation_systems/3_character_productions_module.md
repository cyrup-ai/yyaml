# Task: Create Character Productions Module

## Description
Create new `src/parser/character_productions.rs` module implementing YAML 1.2 character productions [1]-[40] with Unicode validation, BOM detection, encoding support, and escape sequence parsing.

## Target Files
- **Primary**: `src/parser/character_productions.rs` (new file)
- **Secondary**: `src/parser/mod.rs` (add module import)
- **Integration**: `src/scanner.rs` (integrate character validation)

## Success Criteria
- [ ] Character productions [1]-[40] fully implemented
- [ ] c-printable, nb-json, c-byte-order-mark productions working
- [ ] Unicode character class validation methods (ns-dec-digit, ns-hex-digit, etc.)
- [ ] BOM detection and UTF-8/UTF-16/UTF-32 encoding support
- [ ] Line break normalization (b-line-feed, b-carriage-return, b-break)
- [ ] Escape sequence parsing for double-quoted scalars [41]-[62]
- [ ] Whitespace productions (s-space, s-tab, s-white, ns-char)
- [ ] Integration with existing scanner.rs tokenization

## Implementation Notes
- **Architecture**: New module that integrates with existing grammar system
- **Unicode Support**: Full Unicode character set validation with exclusions
- **Encoding Detection**: BOM detection with null pattern encoding deduction  
- **Escape Sequences**: Complete escape sequence parsing for double-quoted scalars
- **Scanner Integration**: Extend existing tokenization with character validation

## Dependencies
- **Blocks**: Task 0_grammar_parametric_productions.md (needs parametric Production variants)
- **Blocks**: Task 1_grammar_context_system.md (needs context system for character handling)

## Complexity Estimate  
**High** - Complex Unicode handling, encoding detection, and escape sequence parsing

## Constraints
- DO NOT MOCK, FABRICATE, FAKE or SIMULATE ANY OPERATION or DATA
- Make ONLY MINIMAL, SURGICAL CHANGES required
- Never use unwrap() or expect() in src/*
- Preserve zero-allocation optimizations using Cow<str>