// pub mod rule_engine;
// pub mod unused_state_variables;
// pub mod vyper;
// pub mod soroban;

// pub use rule_engine::RuleEngine;
// pub use unused_state_variables::UnusedStateVariablesRule;
// pub use vyper::parser;
// pub use soroban::{SorobanAnalyzer, SorobanContract};


//! gasguard_engine: core static analysis engine skeleton
//! - Provides Engine, Analyzer and Rule traits
//! - Minimal parser to load a contract (raw bytes)
//! - Placeholder analyzer to satisfy acceptance criteria

pub mod analyzer;
pub mod engine;
pub mod parser;
pub mod report;
pub mod rule;

pub use analyzer::{Analyzer, PlaceholderAnalyzer};
pub use engine::Engine;
pub use parser::Contract;
pub use report::{AnalysisReport, Issue, Severity};
pub use rule::Rule;