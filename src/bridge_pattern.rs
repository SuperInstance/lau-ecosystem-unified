//! Bridge between Grand Pattern system and lau-* math crates.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A node in the Grand Pattern cellular graph system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrandPatternNode {
    pub id: String,
    pub node_type: PatternNodeType,
    pub math_crate_link: Option<String>,
    pub conservation_invariant: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternNodeType {
    /// Fibonacci growth node
    Fibonacci,
    /// Spectral agent node
    SpectralAgent,
    /// Conservation law enforcer
    ConservationEnforcer,
    /// Cellular automaton cell
    Cellular,
    /// Graph pattern recognizer
    PatternRecognizer,
    /// Bridge node connecting to lau-* math
    MathBridge,
    /// Knowledge distillation node
    Distillation,
    /// Monitoring node
    Monitor,
}

/// A bridge connecting Grand Pattern concepts to lau-* math crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternBridge {
    pub pattern_concept: String,
    pub math_crate: String,
    pub bridge_kind: BridgeKind,
    pub description: String,
    pub bidirectional: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BridgeKind {
    /// Pattern → Math: using math to prove pattern properties
    PatternToMath,
    /// Math → Pattern: using patterns to discover math structures
    MathToPattern,
    /// Bidirectional equivalence
    Equivalence,
    /// Pattern implements math concept
    Implementation,
    /// Conservation law bridge
    Conservation,
}

/// Bridge manager between Grand Pattern and math crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgePattern {
    nodes: Vec<GrandPatternNode>,
    bridges: Vec<PatternBridge>,
    node_index: HashMap<String, usize>,
}

impl BridgePattern {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            bridges: Vec::new(),
            node_index: HashMap::new(),
        }
    }

    pub fn with_canonical() -> Self {
        let mut bp = Self::new();

        let nodes = vec![
            GrandPatternNode {
                id: "fib-growth-core".into(),
                node_type: PatternNodeType::Fibonacci,
                math_crate_link: Some("lau-fibonacci-growth".into()),
                conservation_invariant: Some("Fibonacci ratio φ conservation".into()),
                description: "Core Fibonacci growth pattern with φ-ratio conservation".into(),
            },
            GrandPatternNode {
                id: "spectral-agent-1".into(),
                node_type: PatternNodeType::SpectralAgent,
                math_crate_link: Some("lau-spectral-operators".into()),
                conservation_invariant: Some("Spectral flow invariant".into()),
                description: "Spectral agent operating on graph eigenstructure".into(),
            },
            GrandPatternNode {
                id: "conservation-enforcer".into(),
                node_type: PatternNodeType::ConservationEnforcer,
                math_crate_link: Some("lau-conservation-laws".into()),
                conservation_invariant: Some("Total spectral energy conservation".into()),
                description: "Enforces conservation laws across pattern transformations".into(),
            },
            GrandPatternNode {
                id: "cellular-graph".into(),
                node_type: PatternNodeType::Cellular,
                math_crate_link: Some("lau-cki-graph-engine".into()),
                conservation_invariant: None,
                description: "Cellular automaton on graph structures".into(),
            },
            GrandPatternNode {
                id: "graph-spectral-pattern".into(),
                node_type: PatternNodeType::PatternRecognizer,
                math_crate_link: Some("lau-graph-spectral".into()),
                conservation_invariant: Some("Eigenvalue sum conservation".into()),
                description: "Spectral pattern recognition on cellular graphs".into(),
            },
            GrandPatternNode {
                id: "ncg-bridge".into(),
                node_type: PatternNodeType::MathBridge,
                math_crate_link: Some("lau-noncommutative-geometry".into()),
                conservation_invariant: None,
                description: "Bridge to noncommutative geometry for pattern analysis".into(),
            },
            GrandPatternNode {
                id: "plato-distill-node".into(),
                node_type: PatternNodeType::Distillation,
                math_crate_link: Some("lau-plato-distill".into()),
                conservation_invariant: None,
                description: "PLATO distillation node for knowledge compression".into(),
            },
            GrandPatternNode {
                id: "plato-monitor-node".into(),
                node_type: PatternNodeType::Monitor,
                math_crate_link: Some("lau-plato-monitor".into()),
                conservation_invariant: None,
                description: "PLATO monitoring for pattern health".into(),
            },
            GrandPatternNode {
                id: "ethos-evaluator".into(),
                node_type: PatternNodeType::MathBridge,
                math_crate_link: Some("lau-sunset-ethos".into()),
                conservation_invariant: None,
                description: "Ethos evaluation layer for pattern alignment".into(),
            },
            GrandPatternNode {
                id: "pathos-experience".into(),
                node_type: PatternNodeType::MathBridge,
                math_crate_link: Some("lau-sunset-pathos".into()),
                conservation_invariant: None,
                description: "Pathos experiential layer for pattern aesthetics".into(),
            },
            GrandPatternNode {
                id: "logos-structure".into(),
                node_type: PatternNodeType::MathBridge,
                math_crate_link: Some("lau-sunset-logos".into()),
                conservation_invariant: None,
                description: "Logos structural layer for pattern reasoning".into(),
            },
            GrandPatternNode {
                id: "lie-pattern".into(),
                node_type: PatternNodeType::SpectralAgent,
                math_crate_link: Some("lau-lie-algebra".into()),
                conservation_invariant: Some("Casimir invariant preservation".into()),
                description: "Lie group symmetry patterns in cellular graphs".into(),
            },
        ];

        let bridges = vec![
            PatternBridge {
                pattern_concept: "Fibonacci growth".into(),
                math_crate: "lau-fibonacci-growth".into(),
                bridge_kind: BridgeKind::Implementation,
                description: "Fibonacci growth patterns implemented via number-theoretic crate".into(),
                bidirectional: true,
            },
            PatternBridge {
                pattern_concept: "Spectral decomposition".into(),
                math_crate: "lau-spectral-operators".into(),
                bridge_kind: BridgeKind::Equivalence,
                description: "Pattern spectral decomposition = operator spectral theory".into(),
                bidirectional: true,
            },
            PatternBridge {
                pattern_concept: "Conservation enforcement".into(),
                math_crate: "lau-conservation-laws".into(),
                bridge_kind: BridgeKind::Conservation,
                description: "Conservation laws enforce pattern invariants".into(),
                bidirectional: false,
            },
            PatternBridge {
                pattern_concept: "Graph topology".into(),
                math_crate: "lau-graph-spectral".into(),
                bridge_kind: BridgeKind::PatternToMath,
                description: "Graph pattern analysis via spectral graph theory".into(),
                bidirectional: true,
            },
            PatternBridge {
                pattern_concept: "Noncommutative patterns".into(),
                math_crate: "lau-noncommutative-geometry".into(),
                bridge_kind: BridgeKind::MathToPattern,
                description: "NCG provides framework for noncommutative pattern spaces".into(),
                bidirectional: false,
            },
            PatternBridge {
                pattern_concept: "Cellular automata".into(),
                math_crate: "lau-cki-graph-engine".into(),
                bridge_kind: BridgeKind::Implementation,
                description: "Graph-based cellular automata via CKI engine".into(),
                bidirectional: true,
            },
            PatternBridge {
                pattern_concept: "Symmetry detection".into(),
                math_crate: "lau-lie-algebra".into(),
                bridge_kind: BridgeKind::PatternToMath,
                description: "Pattern symmetry groups classified via Lie theory".into(),
                bidirectional: false,
            },
            PatternBridge {
                pattern_concept: "Knowledge distillation".into(),
                math_crate: "lau-plato-distill".into(),
                bridge_kind: BridgeKind::Implementation,
                description: "Pattern knowledge compressed via PLATO distillation".into(),
                bidirectional: false,
            },
            PatternBridge {
                pattern_concept: "Trinity alignment".into(),
                math_crate: "lau-sunset-ethos".into(),
                bridge_kind: BridgeKind::Conservation,
                description: "Ethos-pathos-logos trinity evaluates pattern alignment".into(),
                bidirectional: false,
            },
            PatternBridge {
                pattern_concept: "Categorical patterns".into(),
                math_crate: "lau-categorical-bridge".into(),
                bridge_kind: BridgeKind::Equivalence,
                description: "Patterns as objects in a category, bridges as functors".into(),
                bidirectional: true,
            },
            PatternBridge {
                pattern_concept: "Optimization landscape".into(),
                math_crate: "lau-optimization-convex".into(),
                bridge_kind: BridgeKind::PatternToMath,
                description: "Pattern optimization via convex methods".into(),
                bidirectional: false,
            },
            PatternBridge {
                pattern_concept: "Information flow".into(),
                math_crate: "lau-information-entropy".into(),
                bridge_kind: BridgeKind::Conservation,
                description: "Information conservation in pattern transformations".into(),
                bidirectional: true,
            },
        ];

        for node in nodes {
            bp.add_node(node);
        }
        for bridge in bridges {
            bp.add_bridge(bridge);
        }
        bp
    }

    pub fn add_node(&mut self, node: GrandPatternNode) {
        let idx = self.nodes.len();
        self.node_index.insert(node.id.clone(), idx);
        self.nodes.push(node);
    }

    pub fn add_bridge(&mut self, bridge: PatternBridge) {
        self.bridges.push(bridge);
    }

    pub fn get_node(&self, id: &str) -> Option<&GrandPatternNode> {
        self.node_index.get(id).map(|&i| &self.nodes[i])
    }

    /// Get all bridges linking to a specific math crate.
    pub fn bridges_for_crate(&self, crate_name: &str) -> Vec<&PatternBridge> {
        self.bridges.iter().filter(|b| b.math_crate == crate_name).collect()
    }

    /// Get all nodes linked to a specific math crate.
    pub fn nodes_for_crate(&self, crate_name: &str) -> Vec<&GrandPatternNode> {
        self.nodes.iter().filter(|n| n.math_crate_link.as_deref() == Some(crate_name)).collect()
    }

    /// Count conservation invariants across all nodes.
    pub fn conservation_invariant_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.conservation_invariant.is_some()).count()
    }

    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn bridge_count(&self) -> usize { self.bridges.len() }

    /// All bridges.
    pub fn all_bridges(&self) -> &[PatternBridge] {
        &self.bridges
    }

    /// All nodes.
    pub fn all_nodes(&self) -> &[GrandPatternNode] {
        &self.nodes
    }

    /// Bridges by kind.
    pub fn bridges_by_kind(&self, kind: BridgeKind) -> Vec<&PatternBridge> {
        self.bridges.iter().filter(|b| b.bridge_kind == kind).collect()
    }
}

impl Default for BridgePattern {
    fn default() -> Self {
        Self::with_canonical()
    }
}
