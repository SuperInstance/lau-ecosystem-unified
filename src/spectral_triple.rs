//! The unified (A, H, D) spectral triple — the heart of the Grand Unification.

use serde::{Deserialize, Serialize};

/// The *-algebra component of the spectral triple.
/// Represents the noncommutative space of observables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Algebra {
    /// Name of the algebra (e.g., "C∞(M)", "B(H)", "M_n(ℂ)")
    pub name: String,
    /// Whether the algebra is commutative
    pub commutative: bool,
    /// Dimension (None for infinite-dimensional)
    pub dimension: Option<usize>,
    /// Involution type
    pub involution: InvolutionType,
    /// Algebra kind
    pub kind: AlgebraKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvolutionType {
    /// Complex conjugate transpose
    Star,
    /// Real transpose
    Transpose,
    /// Clifford involution
    Clifford,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlgebraKind {
    /// C*-algebra
    CStar,
    /// von Neumann algebra
    VonNeumann,
    /// Group algebra
    Group,
    /// Matrix algebra
    Matrix,
    /// Function algebra
    Function,
    /// Clifford algebra
    Clifford,
    /// Tensor algebra
    Tensor,
    /// Universal enveloping algebra
    UniversalEnveloping,
}

/// The Hilbert space component of the spectral triple.
/// Carries the inner product structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HilbertSpace {
    /// Name of the Hilbert space (e.g., "L²(M, S)", "ℓ²(ℕ)")
    pub name: String,
    /// Whether separable
    pub separable: bool,
    /// Inner product type
    pub inner_product: InnerProductKind,
    /// Dimension (None for infinite-dimensional)
    pub dimension: Option<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InnerProductKind {
    /// Standard positive-definite
    PositiveDefinite,
    /// Indefinite (Krein space)
    Indefinite,
    /// Hermitian
    Hermitian,
    /// L² inner product from measure
    L2,
}

/// The Dirac operator component of the spectral triple.
/// Encodes the geometry and metric.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiracOperator {
    /// Name (e.g., "D = d + d*", "atlas Dirac", "finite Dirac")
    pub name: String,
    /// Operator type
    pub kind: DiracKind,
    /// Whether self-adjoint
    pub self_adjoint: bool,
    /// Whether compact resolvent
    pub compact_resolvent: bool,
    /// Spectral dimension (zeta-regularized)
    pub spectral_dimension: Option<f64>,
    /// Eigenvalue growth rate (Weyl asymptotics)
    pub weyl_exponent: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiracKind {
    /// Classical Dirac operator on spin manifold
    Spin,
    /// De Rham operator d + d*
    DeRham,
    /// Signature operator
    Signature,
    /// Dolbeault operator
    Dolbeault,
    /// Finite spectral triple (matrix)
    Finite,
    /// Product (commutative × finite)
    Product,
    /// Twisted Dirac operator
    Twisted,
    /// Fluctuated Dirac operator
    Fluctuated,
}

/// The complete spectral triple (A, H, D).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectralTriple {
    pub algebra: Algebra,
    pub hilbert_space: HilbertSpace,
    pub dirac_operator: DiracOperator,
    /// Additional metadata
    pub metadata: SpectralTripleMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectralTripleMetadata {
    /// Which theorems this triple connects to
    pub theorem_connections: Vec<String>,
    /// Source crate
    pub source_crate: String,
    /// Physical interpretation (if any)
    pub physical_interpretation: Option<String>,
    /// Whether this triple satisfies Connes' axioms
    pub satisfies_axioms: bool,
}

impl SpectralTriple {
    /// Create the canonical spectral triple of a compact spin manifold.
    pub fn canonical_spin_manifold() -> Self {
        Self {
            algebra: Algebra {
                name: "C^∞(M)".into(),
                commutative: true,
                dimension: None,
                involution: InvolutionType::Star,
                kind: AlgebraKind::Function,
            },
            hilbert_space: HilbertSpace {
                name: "L^2(M, S)".into(),
                separable: true,
                inner_product: InnerProductKind::L2,
                dimension: None,
            },
            dirac_operator: DiracOperator {
                name: "Dirac operator on spin manifold".into(),
                kind: DiracKind::Spin,
                self_adjoint: true,
                compact_resolvent: true,
                spectral_dimension: Some(4.0),
                weyl_exponent: Some(4.0),
            },
            metadata: SpectralTripleMetadata {
                theorem_connections: vec![
                    "index_theorem".into(),
                    "spectral_theorem".into(),
                    "connes_reconstruction".into(),
                ],
                source_crate: "lau-noncommutative-geometry".into(),
                physical_interpretation: Some("Standard Model + Gravity via NCG".into()),
                satisfies_axioms: true,
            },
        }
    }

    /// Create a finite spectral triple (matrix model).
    pub fn finite_triple(n: usize) -> Self {
        Self {
            algebra: Algebra {
                name: format!("M_{}(C)", n),
                commutative: n == 1,
                dimension: Some(n * n),
                involution: InvolutionType::Star,
                kind: AlgebraKind::Matrix,
            },
            hilbert_space: HilbertSpace {
                name: format!("C^{}", n),
                separable: true,
                inner_product: InnerProductKind::PositiveDefinite,
                dimension: Some(n),
            },
            dirac_operator: DiracOperator {
                name: format!("Finite Dirac {}x{}", n, n),
                kind: DiracKind::Finite,
                self_adjoint: true,
                compact_resolvent: true,
                spectral_dimension: Some(0.0),
                weyl_exponent: None,
            },
            metadata: SpectralTripleMetadata {
                theorem_connections: vec!["spectral_theorem".into()],
                source_crate: "lau-noncommutative-geometry".into(),
                physical_interpretation: Some("Internal degrees of freedom (gauge sector)".into()),
                satisfies_axioms: true,
            },
        }
    }

    /// Create the product triple (manifold × finite).
    pub fn product_triple() -> Self {
        let _manifold = Self::canonical_spin_manifold();
        let _finite = Self::finite_triple(96); // ~SM particle content
        Self {
            algebra: Algebra {
                name: "C^∞(M) ⊗ M_96(C)".into(),
                commutative: false,
                dimension: None,
                involution: InvolutionType::Star,
                kind: AlgebraKind::Tensor,
            },
            hilbert_space: HilbertSpace {
                name: "L^2(M, S) ⊗ C^96".into(),
                separable: true,
                inner_product: InnerProductKind::L2,
                dimension: None,
            },
            dirac_operator: DiracOperator {
                name: "Product Dirac: D_M ⊗ 1 + γ_M ⊗ D_F".into(),
                kind: DiracKind::Product,
                self_adjoint: true,
                compact_resolvent: true,
                spectral_dimension: Some(4.0),
                weyl_exponent: Some(4.0),
            },
            metadata: SpectralTripleMetadata {
                theorem_connections: vec![
                    "index_theorem".into(),
                    "spectral_theorem".into(),
                    "connes_reconstruction".into(),
                    "conservation_law".into(),
                ],
                source_crate: "lau-noncommutative-geometry".into(),
                physical_interpretation: Some(
                    "Full Standard Model + GR via NCG product geometry".into()
                ),
                satisfies_axioms: true,
            },
        }
    }

    /// Check if this is a commutative triple (classical geometry).
    pub fn is_commutative(&self) -> bool {
        self.algebra.commutative
    }

    /// Spectral distance estimate from Dirac operator.
    pub fn spectral_distance_hint(&self) -> Option<f64> {
        self.dirac_operator.spectral_dimension
            .map(|d| 1.0 / (d + 1.0))
    }
}
