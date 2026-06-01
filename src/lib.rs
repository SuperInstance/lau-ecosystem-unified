//! # lau-ecosystem-unified
//!
//! THE SYNERGY CRATE that bridges all 320+ `lau-*` crates in the SuperInstance ecosystem.
//!
//! ## Architecture
//!
//! The SuperInstance ecosystem unifies:
//! - **Grand Unification**: all 14 theorems as spectral projections of (A,H,D)
//! - **320+ math crates** spanning pure math, physics, ML, CS, and systems
//! - **Multi-language platform**: C, CUDA, Chapel, Go, OpenCL, WASM
//! - **Grand Pattern system**: 30+ repos for cellular graph intelligence
//! - **Sunset ecosystem**: trinity architecture (ethos, pathos, logos)
//! - **PLATO system**: distillation, rooms, monitoring
//! - **Conservation laws**, spectral agents, fibonacci growth
//!
//! ## Modules

pub mod registry;
pub mod dependencies;
pub mod theorem_map;
pub mod language_matrix;
pub mod synergy_detector;
pub mod spectral_triple;
pub mod health;
pub mod bridge_pattern;

pub use registry::{CrateRegistry, CrateEntry, Domain};
pub use dependencies::{DependencyGraph, DependencyEdge, DependencyKind};
pub use theorem_map::{TheoremMap, Theorem};
pub use language_matrix::{LanguageMatrix, Language};
pub use synergy_detector::{SynergyDetector, Synergy, SynergyKind};
pub use spectral_triple::{SpectralTriple, Algebra, HilbertSpace, DiracOperator, DiracKind};
pub use health::{EcosystemHealth, CrateHealth, HealthStatus};
pub use bridge_pattern::{BridgePattern, PatternBridge, GrandPatternNode, PatternNodeType, BridgeKind};
