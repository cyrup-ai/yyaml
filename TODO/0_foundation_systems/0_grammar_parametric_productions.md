# Task: Extend Grammar with Parametric Productions

## Description
Extend the existing `Production` enum in `src/parser/grammar.rs` with all 236+ parametric productions from YAML 1.2 spec, adding support for context, indentation, and chomping parameters.

## Target Files
- **Primary**: `src/parser/grammar.rs` (lines 35-75 - extend Production enum)
- **Secondary**: `src/parser/grammar.rs` (lines 150-250 - add parameter structures)

## Success Criteria
- [ ] All 236+ YAML 1.2 productions [1]-[236+] added to Production enum
- [ ] ParametricContext enum added (BlockIn, BlockOut, BlockKey, FlowIn, FlowOut, FlowKey)  
- [ ] ChompingBehavior enum added (Strip, Clip, Keep)
- [ ] Parametric production variants support context, indentation (n,m), chomping (t) parameters
- [ ] Zero breaking changes to existing Production enum variants
- [ ] Code compiles without warnings

## Implementation Notes
- **Architecture**: Surgical enhancement to existing enum, NOT replacement
- **Backward Compatibility**: Preserve all existing Production variants
- **Parameter System**: Add parametric variants as new enum arms with parameter fields
- **Character Productions**: Focus on [1]-[40] foundation character classes
- **Line Numbers**: Extend Production enum around line 35-75 in grammar.rs

## Dependencies
- None (foundational task)

## Complexity Estimate
**Medium-High** - Requires understanding YAML 1.2 spec parametric system and careful enum extension

## Constraints
- DO NOT MOCK, FABRICATE, FAKE or SIMULATE ANY OPERATION or DATA
- Make ONLY MINIMAL, SURGICAL CHANGES required
- Never use unwrap() or expect() in src/*
- Preserve zero-allocation optimizations using Cow<str>