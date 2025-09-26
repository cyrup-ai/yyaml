# Task: Add Parametric Context System to Grammar

## Description
Extend `ParseContext` struct in `src/parser/grammar.rs` with context and indentation stack support for parametric production handling.

## Target Files
- **Primary**: `src/parser/grammar.rs` (lines 100-150 - extend ParseContext struct)
- **Secondary**: `src/parser/grammar.rs` (add parametric production methods)

## Success Criteria
- [ ] `context_stack: Vec<ParametricContext>` field added to ParseContext struct
- [ ] `indent_stack: Vec<usize>` field added for indentation parameter tracking  
- [ ] Parametric production methods added with context and indentation parameter support
- [ ] Context transition methods for pushing/popping BLOCK/FLOW contexts
- [ ] Indentation parameter calculation methods for s-indent(n), s-indent(n+1+m)
- [ ] Zero breaking changes to existing ParseContext functionality

## Implementation Notes  
- **Architecture**: Extend existing ParseContext struct with new fields
- **Context Stack**: Track BLOCK-IN/OUT/KEY and FLOW-IN/OUT/KEY contexts
- **Indentation Stack**: Support parametric indentation calculations
- **Method Integration**: Add parametric methods that work with existing grammar methods
- **Line Numbers**: Extend ParseContext around lines 100-150 in grammar.rs

## Dependencies
- **Blocks**: Task 0_grammar_parametric_productions.md (needs ParametricContext enum)

## Complexity Estimate
**Medium** - Straightforward struct extension with stack-based parameter tracking

## Constraints
- DO NOT MOCK, FABRICATE, FAKE or SIMULATE ANY OPERATION or DATA
- Make ONLY MINIMAL, SURGICAL CHANGES required  
- Never use unwrap() or expect() in src/*
- Preserve zero-allocation optimizations using Cow<str>