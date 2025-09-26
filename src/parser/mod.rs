//! YAML parser with state machine architecture
//!
//! This module provides YAML 1.2 parsing with a single state machine implementation.

pub mod ast;
pub mod grammar;
pub mod indentation;
pub mod loader;
pub mod state_machine;

pub use ast::*;
pub use grammar::ParametricContext;
pub use loader::YamlLoader;
pub use state_machine::{State, StateMachine};