//! Type-safe registry of all crate capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Broad domain classification for lau-* crates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Domain {
    /// Core algebra: groups, rings, fields, modules
    Algebra,
    /// Analysis: real, complex, functional
    Analysis,
    /// Geometry: differential, algebraic, topology
    Geometry,
    /// Topology: homotopy, homology, cohomology
    Topology,
    /// Number theory
    NumberTheory,
    /// Combinatorics and graph theory
    Combinatorics,
    /// Category theory
    CategoryTheory,
    /// Lie theory and representation theory
    LieTheory,
    /// Mathematical physics
    Physics,
    /// Machine learning and statistics
    ML,
    /// Computer science: algorithms, complexity, cryptography
    CS,
    /// Systems: concurrency, distributed, networking
    Systems,
    /// Numerical methods and solvers
    Numerical,
    /// Signal processing and Fourier analysis
    SignalProcessing,
    /// Quantum computing and information
    Quantum,
    /// Optimization
    Optimization,
    /// Probability and stochastic processes
    Probability,
    /// Information theory
    InformationTheory,
    /// Grand Pattern / cellular graph intelligence
    GrandPattern,
    /// Sunset ecosystem (ethos, pathos, logos)
    Sunset,
    /// PLATO system
    Plato,
    /// Spectral methods
    Spectral,
    /// Interfaces and FFI bridges
    Bridge,
}

/// A single crate entry in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateEntry {
    /// Fully qualified crate name (e.g., "lau-spectral-operators")
    pub name: String,
    /// Primary domain
    pub domain: Domain,
    /// Secondary domains this crate touches
    pub secondary_domains: Vec<Domain>,
    /// Human-readable description
    pub description: String,
    /// Which of the 14 theorems this crate proves or contributes to
    pub theorem_connections: Vec<String>,
    /// Supported language implementations (beyond Rust)
    pub languages: Vec<String>,
    /// Rough dependency count
    pub dependency_count: u32,
    /// Whether the crate is published to crates.io
    pub published: bool,
    /// Semantic version
    pub version: String,
}

/// Type-safe registry of all lau-* crate capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateRegistry {
    crates: HashMap<String, CrateEntry>,
}

impl CrateRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self { crates: HashMap::new() }
    }

    /// Create a registry pre-populated with canonical ecosystem crates.
    pub fn with_canonical_crates() -> Self {
        let mut reg = Self::new();
        reg.populate_canonical();
        reg
    }

    /// Register a new crate.
    pub fn register(&mut self, entry: CrateEntry) {
        self.crates.insert(entry.name.clone(), entry);
    }

    /// Look up a crate by name.
    pub fn get(&self, name: &str) -> Option<&CrateEntry> {
        self.crates.get(name)
    }

    /// List all crates in a given domain.
    pub fn by_domain(&self, domain: Domain) -> Vec<&CrateEntry> {
        self.crates.values()
            .filter(|c| c.domain == domain || c.secondary_domains.contains(&domain))
            .collect()
    }

    /// List crates connected to a specific theorem.
    pub fn by_theorem(&self, theorem: &str) -> Vec<&CrateEntry> {
        self.crates.values()
            .filter(|c| c.theorem_connections.iter().any(|t| t == theorem))
            .collect()
    }

    /// Total number of registered crates.
    pub fn len(&self) -> usize {
        self.crates.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.crates.is_empty()
    }

    /// Iterate over all crate entries.
    pub fn iter(&self) -> impl Iterator<Item = &CrateEntry> {
        self.crates.values()
    }

    /// Count of crates per domain.
    pub fn domain_counts(&self) -> HashMap<Domain, usize> {
        let mut counts = HashMap::new();
        for entry in self.crates.values() {
            *counts.entry(entry.domain).or_insert(0) += 1;
        }
        counts
    }

    /// Populate with a representative set of canonical crates.
    fn populate_canonical(&mut self) {
        let canonical: Vec<CrateEntry> = vec![
            CrateEntry {
                name: "lau-spectral-operators".into(),
                domain: Domain::Spectral,
                secondary_domains: vec![Domain::Analysis, Domain::Physics],
                description: "Spectral operators and noncommutative spectral theory".into(),
                theorem_connections: vec!["spectral_theorem".into(), "index_theorem".into()],
                languages: vec!["C".into(), "CUDA".into()],
                dependency_count: 8,
                published: true,
                version: "0.3.2".into(),
            },
            CrateEntry {
                name: "lau-noncommutative-geometry".into(),
                domain: Domain::Geometry,
                secondary_domains: vec![Domain::Algebra, Domain::Spectral],
                description: "Noncommutative geometry: (A,H,D) spectral triples".into(),
                theorem_connections: vec!["index_theorem".into(), "spectral_theorem".into(), "connes_reconstruction".into()],
                languages: vec!["C".into(), "WASM".into()],
                dependency_count: 12,
                published: true,
                version: "0.4.1".into(),
            },
            CrateEntry {
                name: "lau-homotopy-type-theory".into(),
                domain: Domain::Topology,
                secondary_domains: vec![Domain::CategoryTheory],
                description: "Homotopy type theory and univalent foundations".into(),
                theorem_connections: vec!["univalence".into(), "whitehead".into()],
                languages: vec![],
                dependency_count: 6,
                published: true,
                version: "0.2.0".into(),
            },
            CrateEntry {
                name: "lau-quantum-operators".into(),
                domain: Domain::Quantum,
                secondary_domains: vec![Domain::Spectral, Domain::Physics],
                description: "Quantum operators, C*-algebras, and von Neumann algebras".into(),
                theorem_connections: vec!["spectral_theorem".into(), "gelfand_naimark".into()],
                languages: vec!["CUDA".into(), "OpenCL".into()],
                dependency_count: 10,
                published: true,
                version: "0.3.0".into(),
            },
            CrateEntry {
                name: "lau-fibonacci-growth".into(),
                domain: Domain::NumberTheory,
                secondary_domains: vec![Domain::GrandPattern],
                description: "Fibonacci growth patterns in cellular graph systems".into(),
                theorem_connections: vec!["conservation_law".into()],
                languages: vec!["Chapel".into()],
                dependency_count: 4,
                published: true,
                version: "0.1.5".into(),
            },
            CrateEntry {
                name: "lau-tensor-algebra".into(),
                domain: Domain::Algebra,
                secondary_domains: vec![Domain::Physics, Domain::ML],
                description: "Tensor algebra: symmetric, exterior, and tensor products".into(),
                theorem_connections: vec!["universal_property".into()],
                languages: vec!["C".into(), "CUDA".into()],
                dependency_count: 7,
                published: true,
                version: "0.5.0".into(),
            },
            CrateEntry {
                name: "lau-optimization-convex".into(),
                domain: Domain::Optimization,
                secondary_domains: vec![Domain::Analysis, Domain::ML],
                description: "Convex optimization: gradient descent, interior point methods".into(),
                theorem_connections: vec!["duality_theorem".into()],
                languages: vec!["CUDA".into(), "WASM".into()],
                dependency_count: 9,
                published: true,
                version: "0.4.0".into(),
            },
            CrateEntry {
                name: "lau-differential-forms".into(),
                domain: Domain::Geometry,
                secondary_domains: vec![Domain::Physics, Domain::Topology],
                description: "Exterior calculus and differential forms".into(),
                theorem_connections: vec!["stokes_theorem".into(), "de_rham".into()],
                languages: vec!["C".into()],
                dependency_count: 5,
                published: true,
                version: "0.3.1".into(),
            },
            CrateEntry {
                name: "lau-cki-graph-engine".into(),
                domain: Domain::GrandPattern,
                secondary_domains: vec![Domain::Combinatorics, Domain::Systems],
                description: "Cellular Knowledge Intelligence graph engine".into(),
                theorem_connections: vec!["conservation_law".into()],
                languages: vec!["Go".into(), "WASM".into()],
                dependency_count: 14,
                published: true,
                version: "0.6.2".into(),
            },
            CrateEntry {
                name: "lau-sunset-ethos".into(),
                domain: Domain::Sunset,
                secondary_domains: vec![Domain::CategoryTheory],
                description: "Sunset ethos: the moral/evaluative layer of the trinity".into(),
                theorem_connections: vec![],
                languages: vec![],
                dependency_count: 3,
                published: true,
                version: "0.1.0".into(),
            },
            CrateEntry {
                name: "lau-sunset-pathos".into(),
                domain: Domain::Sunset,
                secondary_domains: vec![Domain::ML],
                description: "Sunset pathos: the emotional/experiential layer".into(),
                theorem_connections: vec![],
                languages: vec![],
                dependency_count: 3,
                published: true,
                version: "0.1.0".into(),
            },
            CrateEntry {
                name: "lau-sunset-logos".into(),
                domain: Domain::Sunset,
                secondary_domains: vec![Domain::CategoryTheory],
                description: "Sunset logos: the rational/structural layer".into(),
                theorem_connections: vec![],
                languages: vec![],
                dependency_count: 3,
                published: true,
                version: "0.1.0".into(),
            },
            CrateEntry {
                name: "lau-plato-monitor".into(),
                domain: Domain::Plato,
                secondary_domains: vec![Domain::Systems],
                description: "PLATO monitoring: ecosystem health and observability".into(),
                theorem_connections: vec![],
                languages: vec!["Go".into()],
                dependency_count: 6,
                published: false,
                version: "0.2.0-beta".into(),
            },
            CrateEntry {
                name: "lau-plato-distill".into(),
                domain: Domain::Plato,
                secondary_domains: vec![Domain::ML],
                description: "PLATO distillation: knowledge compression and transfer".into(),
                theorem_connections: vec![],
                languages: vec!["CUDA".into()],
                dependency_count: 8,
                published: false,
                version: "0.1.0-alpha".into(),
            },
            CrateEntry {
                name: "lau-conservation-laws".into(),
                domain: Domain::Physics,
                secondary_domains: vec![Domain::Spectral],
                description: "Conservation laws: Noether's theorem and spectral invariants".into(),
                theorem_connections: vec!["conservation_law".into(), "noether".into()],
                languages: vec!["C".into()],
                dependency_count: 5,
                published: true,
                version: "0.2.3".into(),
            },
            CrateEntry {
                name: "lau-fft-spectral".into(),
                domain: Domain::SignalProcessing,
                secondary_domains: vec![Domain::Spectral, Domain::Numerical],
                description: "FFT and spectral analysis: fast transforms and spectral methods".into(),
                theorem_connections: vec!["spectral_theorem".into()],
                languages: vec!["CUDA".into(), "OpenCL".into(), "WASM".into()],
                dependency_count: 6,
                published: true,
                version: "0.5.1".into(),
            },
            CrateEntry {
                name: "lau-categorical-bridge".into(),
                domain: Domain::CategoryTheory,
                secondary_domains: vec![Domain::Bridge],
                description: "Categorical bridges: functors between lau-* mathematical domains".into(),
                theorem_connections: vec!["univalence".into(), "yoneda".into()],
                languages: vec![],
                dependency_count: 4,
                published: true,
                version: "0.2.0".into(),
            },
            CrateEntry {
                name: "lau-probability-measures".into(),
                domain: Domain::Probability,
                secondary_domains: vec![Domain::Analysis, Domain::ML],
                description: "Probability measures, Radon-Nikodym, and stochastic calculus".into(),
                theorem_connections: vec!["radon_nikodym".into()],
                languages: vec!["CUDA".into()],
                dependency_count: 7,
                published: true,
                version: "0.3.0".into(),
            },
            CrateEntry {
                name: "lau-information-entropy".into(),
                domain: Domain::InformationTheory,
                secondary_domains: vec![Domain::Probability, Domain::ML],
                description: "Information theory: entropy, mutual information, KL divergence".into(),
                theorem_connections: vec!["conservation_law".into()],
                languages: vec!["CUDA".into(), "WASM".into()],
                dependency_count: 5,
                published: true,
                version: "0.2.2".into(),
            },
            CrateEntry {
                name: "lau-ffi-bridge".into(),
                domain: Domain::Bridge,
                secondary_domains: vec![Domain::Systems],
                description: "FFI bridge: unified bindings for C, CUDA, Chapel, Go, OpenCL, WASM".into(),
                theorem_connections: vec![],
                languages: vec!["C".into(), "CUDA".into(), "Chapel".into(), "Go".into(), "OpenCL".into(), "WASM".into()],
                dependency_count: 20,
                published: true,
                version: "0.7.0".into(),
            },
            CrateEntry {
                name: "lau-lie-algebra".into(),
                domain: Domain::LieTheory,
                secondary_domains: vec![Domain::Algebra, Domain::Physics],
                description: "Lie algebras: representations, root systems, Dynkin diagrams".into(),
                theorem_connections: vec!["weyl_character".into()],
                languages: vec!["C".into()],
                dependency_count: 8,
                published: true,
                version: "0.3.0".into(),
            },
            CrateEntry {
                name: "lau-distributed-consensus".into(),
                domain: Domain::Systems,
                secondary_domains: vec![Domain::Combinatorics],
                description: "Distributed consensus: Byzantine fault tolerance, Raft".into(),
                theorem_connections: vec!["flp_impossibility".into()],
                languages: vec!["Go".into()],
                dependency_count: 6,
                published: true,
                version: "0.2.0".into(),
            },
            CrateEntry {
                name: "lau-numeric-solver".into(),
                domain: Domain::Numerical,
                secondary_domains: vec![Domain::Analysis, Domain::Optimization],
                description: "Numerical solvers: ODE, PDE, linear systems".into(),
                theorem_connections: vec!["stokes_theorem".into()],
                languages: vec!["CUDA".into(), "OpenCL".into()],
                dependency_count: 11,
                published: true,
                version: "0.4.2".into(),
            },
            CrateEntry {
                name: "lau-combinatorial-design".into(),
                domain: Domain::Combinatorics,
                secondary_domains: vec![Domain::NumberTheory, Domain::GrandPattern],
                description: "Combinatorial designs: block designs, codes, finite geometries".into(),
                theorem_connections: vec!["bruck_ryser_chowla".into()],
                languages: vec!["Chapel".into()],
                dependency_count: 5,
                published: true,
                version: "0.2.1".into(),
            },
            CrateEntry {
                name: "lau-ml-gradient".into(),
                domain: Domain::ML,
                secondary_domains: vec![Domain::Optimization, Domain::Numerical],
                description: "ML gradient computation: autodiff, backprop, optimizers".into(),
                theorem_connections: vec!["chain_rule".into()],
                languages: vec!["CUDA".into(), "WASM".into()],
                dependency_count: 12,
                published: true,
                version: "0.5.0".into(),
            },
            CrateEntry {
                name: "lau-cryptography-zkp".into(),
                domain: Domain::CS,
                secondary_domains: vec![Domain::NumberTheory, Domain::InformationTheory],
                description: "Zero-knowledge proofs and post-quantum cryptography".into(),
                theorem_connections: vec![],
                languages: vec!["WASM".into()],
                dependency_count: 9,
                published: true,
                version: "0.3.0".into(),
            },
            CrateEntry {
                name: "lau-sheaf-theory".into(),
                domain: Domain::Topology,
                secondary_domains: vec![Domain::CategoryTheory, Domain::Geometry],
                description: "Sheaf theory: presheaves, sheaf cohomology, derived categories".into(),
                theorem_connections: vec!["de_rham".into(), "grothendieck".into()],
                languages: vec![],
                dependency_count: 6,
                published: true,
                version: "0.2.0".into(),
            },
            CrateEntry {
                name: "lau-cohomology-calculus".into(),
                domain: Domain::Topology,
                secondary_domains: vec![Domain::Algebra, Domain::Geometry],
                description: "Cohomology theories: de Rham, Čech, singular, persistent".into(),
                theorem_connections: vec!["de_rham".into(), "kunneth".into()],
                languages: vec!["C".into()],
                dependency_count: 7,
                published: true,
                version: "0.3.0".into(),
            },
            CrateEntry {
                name: "lau-connection-theory".into(),
                domain: Domain::Geometry,
                secondary_domains: vec![Domain::Physics],
                description: "Connections: Levi-Civita, gauge theory, Yang-Mills".into(),
                theorem_connections: vec!["yang_mills".into(), "stokes_theorem".into()],
                languages: vec!["CUDA".into()],
                dependency_count: 9,
                published: true,
                version: "0.3.1".into(),
            },
            CrateEntry {
                name: "lau-calculus-variations".into(),
                domain: Domain::Analysis,
                secondary_domains: vec![Domain::Physics, Domain::Optimization],
                description: "Calculus of variations: Euler-Lagrange, Hamilton's principle".into(),
                theorem_connections: vec!["noether".into(), "conservation_law".into()],
                languages: vec!["C".into()],
                dependency_count: 6,
                published: true,
                version: "0.2.0".into(),
            },
            CrateEntry {
                name: "lau-measure-theory".into(),
                domain: Domain::Analysis,
                secondary_domains: vec![Domain::Probability],
                description: "Measure theory: Lebesgue, Haar, Borel measures".into(),
                theorem_connections: vec!["radon_nikodym".into(), "riesz_representation".into()],
                languages: vec![],
                dependency_count: 5,
                published: true,
                version: "0.3.0".into(),
            },
            CrateEntry {
                name: "lau-galois-theory".into(),
                domain: Domain::Algebra,
                secondary_domains: vec![Domain::NumberTheory],
                description: "Galois theory: field extensions, Galois groups, solvability".into(),
                theorem_connections: vec!["fundamental_galois".into()],
                languages: vec![],
                dependency_count: 4,
                published: true,
                version: "0.2.0".into(),
            },
            CrateEntry {
                name: "lau-banach-spaces".into(),
                domain: Domain::Analysis,
                secondary_domains: vec![Domain::Spectral],
                description: "Functional analysis: Banach and Hilbert spaces, operators".into(),
                theorem_connections: vec!["spectral_theorem".into(), "hahn_banach".into()],
                languages: vec!["C".into()],
                dependency_count: 6,
                published: true,
                version: "0.3.0".into(),
            },
            CrateEntry {
                name: "lau-graph-spectral".into(),
                domain: Domain::Combinatorics,
                secondary_domains: vec![Domain::Spectral, Domain::GrandPattern],
                description: "Spectral graph theory: eigenvalues, Laplacians, expanders".into(),
                theorem_connections: vec!["spectral_theorem".into(), "conservation_law".into()],
                languages: vec!["CUDA".into(), "Chapel".into()],
                dependency_count: 8,
                published: true,
                version: "0.4.0".into(),
            },
        ];
        for entry in canonical {
            self.register(entry);
        }
    }
}

impl Default for CrateRegistry {
    fn default() -> Self {
        Self::with_canonical_crates()
    }
}
