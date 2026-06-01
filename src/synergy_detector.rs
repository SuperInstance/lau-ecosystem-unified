//! Detects pairs of crates that compose to produce emergent results.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Kind of emergent synergy between two crates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SynergyKind {
    /// Combining creates new mathematical structures
    MathematicalEmergence,
    /// Performance speedup when used together
    PerformanceSynergy,
    /// One enables the other's theorems
    TheoremAmplification,
    /// Cross-domain bridge (e.g., physics + ML)
    CrossDomainBridge,
    /// Spectral enhancement: combined spectral methods exceed parts
    SpectralEnhancement,
    /// Grand Pattern integration with math core
    PatternIntegration,
}

/// A detected synergy between two crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Synergy {
    pub crate_a: String,
    pub crate_b: String,
    pub kind: SynergyKind,
    pub description: String,
    pub emergent_property: String,
}

/// Detects synergies across the ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynergyDetector {
    synergies: Vec<Synergy>,
    /// Index: crate -> synergies involving it
    by_crate: HashMap<String, Vec<usize>>,
}

impl SynergyDetector {
    pub fn new() -> Self {
        Self {
            synergies: Vec::new(),
            by_crate: HashMap::new(),
        }
    }

    pub fn with_canonical() -> Self {
        let mut sd = Self::new();
        let synergies = vec![
            Synergy {
                crate_a: "lau-noncommutative-geometry".into(),
                crate_b: "lau-quantum-operators".into(),
                kind: SynergyKind::MathematicalEmergence,
                description: "NCG + quantum operators → noncommutative quantum field theory".into(),
                emergent_property: "Full quantum gravity spectral model via (A,H,D)".into(),
            },
            Synergy {
                crate_a: "lau-fft-spectral".into(),
                crate_b: "lau-graph-spectral".into(),
                kind: SynergyKind::SpectralEnhancement,
                description: "FFT + graph spectral → ultra-fast spectral clustering".into(),
                emergent_property: "O(n log n) spectral graph partitioning".into(),
            },
            Synergy {
                crate_a: "lau-cki-graph-engine".into(),
                crate_b: "lau-graph-spectral".into(),
                kind: SynergyKind::PatternIntegration,
                description: "Cellular graph intelligence + spectral graph theory".into(),
                emergent_property: "Self-organizing graph structures with spectral guarantees".into(),
            },
            Synergy {
                crate_a: "lau-optimization-convex".into(),
                crate_b: "lau-ml-gradient".into(),
                kind: SynergyKind::PerformanceSynergy,
                description: "Convex optimization + autodiff → provably optimal ML training".into(),
                emergent_property: "Convergence guarantees for gradient-based learning".into(),
            },
            Synergy {
                crate_a: "lau-conservation-laws".into(),
                crate_b: "lau-fibonacci-growth".into(),
                kind: SynergyKind::TheoremAmplification,
                description: "Conservation laws constrain Fibonacci growth patterns".into(),
                emergent_property: "Bounded Fibonacci growth with conservation invariants".into(),
            },
            Synergy {
                crate_a: "lau-homotopy-type-theory".into(),
                crate_b: "lau-sheaf-theory".into(),
                kind: SynergyKind::MathematicalEmergence,
                description: "HoTT + sheaves → higher topos theory".into(),
                emergent_property: "Computational higher category theory".into(),
            },
            Synergy {
                crate_a: "lau-cryptography-zkp".into(),
                crate_b: "lau-information-entropy".into(),
                kind: SynergyKind::CrossDomainBridge,
                description: "ZK proofs + information theory → tighter security bounds".into(),
                emergent_property: "Minimum-entropy ZK proof systems".into(),
            },
            Synergy {
                crate_a: "lau-differential-forms".into(),
                crate_b: "lau-connection-theory".into(),
                kind: SynergyKind::MathematicalEmergence,
                description: "Differential forms + connections → gauge theory".into(),
                emergent_property: "Complete gauge-theoretic framework".into(),
            },
            Synergy {
                crate_a: "lau-lie-algebra".into(),
                crate_b: "lau-quantum-operators".into(),
                kind: SynergyKind::CrossDomainBridge,
                description: "Lie algebras + quantum operators → representation theory of quantum groups".into(),
                emergent_property: "Quantum group representations with spectral decomposition".into(),
            },
            Synergy {
                crate_a: "lau-plato-distill".into(),
                crate_b: "lau-ml-gradient".into(),
                kind: SynergyKind::PerformanceSynergy,
                description: "Distillation + gradient computation → compressed model training".into(),
                emergent_property: "Knowledge transfer with gradient-based distillation".into(),
            },
            Synergy {
                crate_a: "lau-probability-measures".into(),
                crate_b: "lau-information-entropy".into(),
                kind: SynergyKind::TheoremAmplification,
                description: "Probability measures + information theory → information geometry".into(),
                emergent_property: "Statistical manifolds with Fisher information metric".into(),
            },
            Synergy {
                crate_a: "lau-cohomology-calculus".into(),
                crate_b: "lau-spectral-operators".into(),
                kind: SynergyKind::SpectralEnhancement,
                description: "Cohomology + spectral theory → Hodge theory".into(),
                emergent_property: "Hodge decomposition on spectral manifolds".into(),
            },
            Synergy {
                crate_a: "lau-galois-theory".into(),
                crate_b: "lau-cryptography-zkp".into(),
                kind: SynergyKind::CrossDomainBridge,
                description: "Galois theory + ZK cryptography → pairing-based proofs".into(),
                emergent_property: "Galois-theoretic zero-knowledge protocols".into(),
            },
            Synergy {
                crate_a: "lau-calculus-variations".into(),
                crate_b: "lau-conservation-laws".into(),
                kind: SynergyKind::TheoremAmplification,
                description: "Calculus of variations + conservation → Lagrangian mechanics".into(),
                emergent_property: "Complete Lagrangian field theory with conservation laws".into(),
            },
            Synergy {
                crate_a: "lau-distributed-consensus".into(),
                crate_b: "lau-cki-graph-engine".into(),
                kind: SynergyKind::PatternIntegration,
                description: "Consensus + cellular graphs → distributed pattern recognition".into(),
                emergent_property: "Byzantine-tolerant cellular graph consensus".into(),
            },
            Synergy {
                crate_a: "lau-sunset-ethos".into(),
                crate_b: "lau-categorical-bridge".into(),
                kind: SynergyKind::CrossDomainBridge,
                description: "Ethos (evaluative) + categorical bridges → value-aware functors".into(),
                emergent_property: "Category-theoretic value alignment framework".into(),
            },
            Synergy {
                crate_a: "lau-banach-spaces".into(),
                crate_b: "lau-spectral-operators".into(),
                kind: SynergyKind::SpectralEnhancement,
                description: "Banach spaces + spectral operators → operator algebras on Banach spaces".into(),
                emergent_property: "Banach C*-algebra spectral theory".into(),
            },
            Synergy {
                crate_a: "lau-tensor-algebra".into(),
                crate_b: "lau-lie-algebra".into(),
                kind: SynergyKind::MathematicalEmergence,
                description: "Tensor algebra + Lie algebra → universal enveloping algebras".into(),
                emergent_property: "PBW theorem and representation theory via tensor products".into(),
            },
            Synergy {
                crate_a: "lau-ffi-bridge".into(),
                crate_b: "lau-cki-graph-engine".into(),
                kind: SynergyKind::PerformanceSynergy,
                description: "FFI bridge + graph engine → multi-language graph computation".into(),
                emergent_property: "Polyglot cellular graph processing pipeline".into(),
            },
            Synergy {
                crate_a: "lau-numeric-solver".into(),
                crate_b: "lau-connection-theory".into(),
                kind: SynergyKind::CrossDomainBridge,
                description: "Numeric solver + connections → numerical gauge theory".into(),
                emergent_property: "Lattice gauge theory simulations".into(),
            },
        ];
        for synergy in synergies {
            sd.add_synergy(synergy);
        }
        sd
    }

    /// Add a detected synergy.
    pub fn add_synergy(&mut self, synergy: Synergy) {
        let idx = self.synergies.len();
        self.by_crate.entry(synergy.crate_a.clone()).or_default().push(idx);
        self.by_crate.entry(synergy.crate_b.clone()).or_default().push(idx);
        self.synergies.push(synergy);
    }

    /// Get all synergies involving a crate.
    pub fn synergies_for(&self, crate_name: &str) -> Vec<&Synergy> {
        self.by_crate.get(crate_name)
            .map(|indices| indices.iter().map(|&i| &self.synergies[i]).collect())
            .unwrap_or_default()
    }

    /// Get all synergies of a specific kind.
    pub fn by_kind(&self, kind: SynergyKind) -> Vec<&Synergy> {
        self.synergies.iter().filter(|s| s.kind == kind).collect()
    }

    /// Count synergies.
    pub fn len(&self) -> usize {
        self.synergies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.synergies.is_empty()
    }

    /// All synergies.
    pub fn all(&self) -> &[Synergy] {
        &self.synergies
    }

    /// Detect new synergies based on shared theorems between two registries.
    pub fn detect_from_shared_theorems(
        &self,
        crate_a: &str,
        crate_b: &str,
        shared_theorems: &[String],
    ) -> Option<Synergy> {
        if shared_theorems.is_empty() {
            return None;
        }
        Some(Synergy {
            crate_a: crate_a.into(),
            crate_b: crate_b.into(),
            kind: SynergyKind::TheoremAmplification,
            description: format!("Shared theorems: {}", shared_theorems.join(", ")),
            emergent_property: format!("Amplified theorem power via {} shared results", shared_theorems.len()),
        })
    }

    /// Count synergies by kind.
    pub fn counts_by_kind(&self) -> HashMap<SynergyKind, usize> {
        let mut counts = HashMap::new();
        for s in &self.synergies {
            *counts.entry(s.kind).or_insert(0) += 1;
        }
        counts
    }
}

impl Default for SynergyDetector {
    fn default() -> Self {
        Self::with_canonical()
    }
}
