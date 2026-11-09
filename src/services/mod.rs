//! Core services and 4-role semantic folding engine for `kompress-super-ultra`.

pub mod engine;

pub use engine::{
    compute_savings_bps, evaluate_bonobo_resonance, AdvancedKompressEngine, AppState, FoldOutcome,
};
