# Task: Add Parametric Support to State Machine

## Description
Extend existing `State` enum in `src/parser/state_machine.rs` with parametric state variants and add parametric state transition methods.

## Target Files
- **Primary**: `src/parser/state_machine.rs` (lines 8-25 - extend State enum)
- **Secondary**: `src/parser/state_machine.rs` (lines 50-150 - add transition methods)

## Success Criteria
- [ ] Parametric state variants added to existing State enum without breaking current states
- [ ] Context parameter fields added to relevant states (Block*/Flow* states)
- [ ] Indentation parameter tracking added to block-related states  
- [ ] State transition methods handle parametric context changes
- [ ] Integration with existing state machine logic preserved
- [ ] Zero breaking changes to current StateMachine functionality

## Implementation Notes
- **Architecture**: Extend existing State enum, NOT replacement
- **Parametric States**: Add context/indentation fields to relevant state variants
- **Transition Methods**: Enhance existing state transition logic with parameter handling
- **Backward Compatibility**: All existing states and transitions must continue working
- **Line Numbers**: Extend State enum around lines 8-25, add methods around 50-150

## Dependencies
- **Blocks**: Task 1_grammar_context_system.md (needs ParametricContext and stack methods)

## Complexity Estimate
**Medium-High** - Complex state machine extension with parameter integration

## Constraints
- DO NOT MOCK, FABRICATE, FAKE or SIMULATE ANY OPERATION or DATA
- Make ONLY MINIMAL, SURGICAL CHANGES required
- Never use unwrap() or expect() in src/*
- Preserve zero-allocation optimizations using Cow<str>