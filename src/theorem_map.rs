//! Maps each of the 14 theorems to their proving crates.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The 14 grand unification theorems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Theorem {
    /// Atiyah-Singer Index Theorem
    IndexTheorem,
    /// Spectral Theorem (bounded/unbounded operators)
    SpectralTheorem,
    /// Connes' Reconstruction Theorem
    ConnesReconstruction,
    /// Stokes' Theorem (generalized)
    StokesTheorem,
    /// De Rham Theorem
    DeRhamTheorem,
    /// Noether's Theorem
    NoetherTheorem,
    /// Conservation Law (unified)
    ConservationLaw,
    /// Gelfand-Naimark Theorem
    GelfandNaimark,
    /// Weyl Character Formula
    WeylCharacter,
    /// Univalence Axiom
    Univalence,
    /// Yoneda Lemma
    Yoneda,
    /// Radon-Nikodym Theorem
    RadonNikodym,
    /// Duality Theorem (optimization)
    DualityTheorem,
    /// Whitehead's Theorem
    Whitehead,
}

impl Theorem {
    /// All 14 theorems.
    pub fn all() -> &'static [Theorem] {
        &[
            Theorem::IndexTheorem,
            Theorem::SpectralTheorem,
            Theorem::ConnesReconstruction,
            Theorem::StokesTheorem,
            Theorem::DeRhamTheorem,
            Theorem::NoetherTheorem,
            Theorem::ConservationLaw,
            Theorem::GelfandNaimark,
            Theorem::WeylCharacter,
            Theorem::Univalence,
            Theorem::Yoneda,
            Theorem::RadonNikodym,
            Theorem::DualityTheorem,
            Theorem::Whitehead,
        ]
    }

    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            Theorem::IndexTheorem => "Atiyah-Singer Index Theorem",
            Theorem::SpectralTheorem => "Spectral Theorem",
            Theorem::ConnesReconstruction => "Connes' Reconstruction Theorem",
            Theorem::StokesTheorem => "Stokes' Theorem",
            Theorem::DeRhamTheorem => "De Rham Theorem",
            Theorem::NoetherTheorem => "Noether's Theorem",
            Theorem::ConservationLaw => "Conservation Law (Unified)",
            Theorem::GelfandNaimark => "Gelfand-Naimark Theorem",
            Theorem::WeylCharacter => "Weyl Character Formula",
            Theorem::Univalence => "Univalence Axiom",
            Theorem::Yoneda => "Yoneda Lemma",
            Theorem::RadonNikodym => "Radon-Nikodym Theorem",
            Theorem::DualityTheorem => "Duality Theorem",
            Theorem::Whitehead => "Whitehead's Theorem",
        }
    }

    /// Which spectral component of (A,H,D) this theorem projects onto.
    pub fn spectral_projection(&self) -> &'static str {
        match self {
            Theorem::IndexTheorem => "D → index pairing with K-theory",
            Theorem::SpectralTheorem => "A → spectral resolution of *-algebra",
            Theorem::ConnesReconstruction => "(A,H,D) → full triple reconstruction from spectral data",
            Theorem::StokesTheorem => "D → integration as adjoint of d on Hilbert space",
            Theorem::DeRhamTheorem => "H → cohomology via differential complex in H",
            Theorem::NoetherTheorem => "A → symmetries of *-algebra → conserved currents",
            Theorem::ConservationLaw => "(A,H,D) → spectral flow invariants",
            Theorem::GelfandNaimark => "A → commutative C* = C₀(X)",
            Theorem::WeylCharacter => "A → character ring of Lie algebra representations",
            Theorem::Univalence => "H → homotopy type from identity types",
            Theorem::Yoneda => "A → universal property via representable functors",
            Theorem::RadonNikodym => "H → density operators between measures on H",
            Theorem::DualityTheorem => "(A,H) → dual pairing via inner product structure",
            Theorem::Whitehead => "H → weak equivalence via homotopy groups",
        }
    }
}

/// Mapping from theorems to the crates that prove or implement them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoremMap {
    map: HashMap<String, TheoremEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoremEntry {
    pub theorem: String,
    pub proving_crates: Vec<String>,
    pub contributing_crates: Vec<String>,
    pub description: String,
    pub spectral_projection: String,
}

impl TheoremMap {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn with_canonical() -> Self {
        let mut tm = Self::new();
        let entries = vec![
            TheoremEntry {
                theorem: "index_theorem".into(),
                proving_crates: vec!["lau-spectral-operators".into(), "lau-noncommutative-geometry".into()],
                contributing_crates: vec!["lau-cohomology-calculus".into(), "lau-banach-spaces".into()],
                description: "Atiyah-Singer Index Theorem: analytical index = topological index".into(),
                spectral_projection: Theorem::IndexTheorem.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "spectral_theorem".into(),
                proving_crates: vec!["lau-spectral-operators".into(), "lau-banach-spaces".into()],
                contributing_crates: vec!["lau-quantum-operators".into(), "lau-fft-spectral".into(), "lau-graph-spectral".into()],
                description: "Spectral Theorem: self-adjoint operators decompose via spectral measures".into(),
                spectral_projection: Theorem::SpectralTheorem.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "connes_reconstruction".into(),
                proving_crates: vec!["lau-noncommutative-geometry".into()],
                contributing_crates: vec!["lau-spectral-operators".into(), "lau-banach-spaces".into()],
                description: "Connes' Reconstruction: spectral data reconstructs the manifold".into(),
                spectral_projection: Theorem::ConnesReconstruction.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "stokes_theorem".into(),
                proving_crates: vec!["lau-differential-forms".into()],
                contributing_crates: vec!["lau-connection-theory".into(), "lau-numeric-solver".into()],
                description: "Stokes' Theorem: ∫_M dω = ∫_∂M ω".into(),
                spectral_projection: Theorem::StokesTheorem.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "de_rham".into(),
                proving_crates: vec!["lau-cohomology-calculus".into()],
                contributing_crates: vec!["lau-differential-forms".into(), "lau-sheaf-theory".into()],
                description: "De Rham Theorem: de Rham cohomology ≅ singular cohomology".into(),
                spectral_projection: Theorem::DeRhamTheorem.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "noether".into(),
                proving_crates: vec!["lau-conservation-laws".into()],
                contributing_crates: vec!["lau-calculus-variations".into(), "lau-lie-algebra".into()],
                description: "Noether's Theorem: symmetries ↔ conserved quantities".into(),
                spectral_projection: Theorem::NoetherTheorem.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "conservation_law".into(),
                proving_crates: vec!["lau-conservation-laws".into()],
                contributing_crates: vec!["lau-fibonacci-growth".into(), "lau-graph-spectral".into(), "lau-information-entropy".into()],
                description: "Unified conservation law: spectral flow invariants in (A,H,D)".into(),
                spectral_projection: Theorem::ConservationLaw.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "gelfand_naimark".into(),
                proving_crates: vec!["lau-quantum-operators".into()],
                contributing_crates: vec!["lau-banach-spaces".into()],
                description: "Gelfand-Naimark: C*-algebras ≅ closed *-subalgebras of B(H)".into(),
                spectral_projection: Theorem::GelfandNaimark.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "weyl_character".into(),
                proving_crates: vec!["lau-lie-algebra".into()],
                contributing_crates: vec!["lau-tensor-algebra".into()],
                description: "Weyl Character Formula: characters of irreducible representations".into(),
                spectral_projection: Theorem::WeylCharacter.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "univalence".into(),
                proving_crates: vec!["lau-homotopy-type-theory".into()],
                contributing_crates: vec!["lau-categorical-bridge".into()],
                description: "Univalence Axiom: (A = B) ≃ (A ≃ B)".into(),
                spectral_projection: Theorem::Univalence.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "yoneda".into(),
                proving_crates: vec!["lau-categorical-bridge".into()],
                contributing_crates: vec!["lau-sheaf-theory".into()],
                description: "Yoneda Lemma: objects determined by their representable functors".into(),
                spectral_projection: Theorem::Yoneda.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "radon_nikodym".into(),
                proving_crates: vec!["lau-measure-theory".into()],
                contributing_crates: vec!["lau-probability-measures".into()],
                description: "Radon-Nikodym: absolute continuity → density".into(),
                spectral_projection: Theorem::RadonNikodym.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "duality_theorem".into(),
                proving_crates: vec!["lau-optimization-convex".into()],
                contributing_crates: vec!["lau-banach-spaces".into(), "lau-ml-gradient".into()],
                description: "Duality Theorem: primal optimum = dual optimum under constraint qualification".into(),
                spectral_projection: Theorem::DualityTheorem.spectral_projection().into(),
            },
            TheoremEntry {
                theorem: "whitehead".into(),
                proving_crates: vec!["lau-homotopy-type-theory".into()],
                contributing_crates: vec!["lau-cohomology-calculus".into()],
                description: "Whitehead's Theorem: weak homotopy equivalence between CW complexes".into(),
                spectral_projection: Theorem::Whitehead.spectral_projection().into(),
            },
        ];
        for entry in entries {
            tm.map.insert(entry.theorem.clone(), entry);
        }
        tm
    }

    /// Look up theorem by name.
    pub fn get(&self, theorem: &str) -> Option<&TheoremEntry> {
        self.map.get(theorem)
    }

    /// All theorem entries.
    pub fn all(&self) -> Vec<&TheoremEntry> {
        self.map.values().collect()
    }

    /// Count of theorems mapped.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Crates involved in a theorem (both proving and contributing).
    pub fn crates_for_theorem(&self, theorem: &str) -> Vec<&str> {
        self.map.get(theorem)
            .map(|e| {
                let mut crates: Vec<&str> = e.proving_crates.iter()
                    .chain(e.contributing_crates.iter())
                    .map(|s| s.as_str())
                    .collect();
                crates.sort();
                crates.dedup();
                crates
            })
            .unwrap_or_default()
    }

    /// Find all theorems a crate is involved in.
    pub fn theorems_for_crate(&self, crate_name: &str) -> Vec<&str> {
        self.map.iter()
            .filter(|(_, e)| {
                e.proving_crates.iter().any(|c| c == crate_name) ||
                e.contributing_crates.iter().any(|c| c == crate_name)
            })
            .map(|(k, _)| k.as_str())
            .collect()
    }
}

impl Default for TheoremMap {
    fn default() -> Self {
        Self::with_canonical()
    }
}
