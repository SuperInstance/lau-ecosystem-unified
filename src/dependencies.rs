//! Dependency graph between crates.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// An edge in the dependency graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyEdge {
    pub from: String,
    pub to: String,
    pub kind: DependencyKind,
}

/// Kind of dependency relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DependencyKind {
    /// Direct code dependency
    Code,
    /// Theorem dependency (needs a theorem proven by the other crate)
    Theorem,
    /// Data or type dependency
    DataType,
    /// Build-time dependency
    Build,
}

/// Directed acyclic dependency graph over crate names.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyGraph {
    edges: Vec<DependencyEdge>,
    /// Adjacency list: crate -> list of crates it depends on
    deps: HashMap<String, HashSet<String>>,
    /// Reverse adjacency: crate -> list of crates that depend on it
    rdeps: HashMap<String, HashSet<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            deps: HashMap::new(),
            rdeps: HashMap::new(),
        }
    }

    /// Create a graph with canonical ecosystem dependencies.
    pub fn with_canonical_deps() -> Self {
        let mut g = Self::new();
        let canonical_edges = vec![
            ("lau-noncommutative-geometry", "lau-spectral-operators", DependencyKind::Code),
            ("lau-noncommutative-geometry", "lau-banach-spaces", DependencyKind::Code),
            ("lau-noncommutative-geometry", "lau-tensor-algebra", DependencyKind::Code),
            ("lau-quantum-operators", "lau-spectral-operators", DependencyKind::Theorem),
            ("lau-quantum-operators", "lau-banach-spaces", DependencyKind::Code),
            ("lau-differential-forms", "lau-tensor-algebra", DependencyKind::Code),
            // ("lau-differential-forms", "lau-connection-theory", DependencyKind::Code), // removed to break cycle
            ("lau-connection-theory", "lau-lie-algebra", DependencyKind::Theorem),
            ("lau-connection-theory", "lau-differential-forms", DependencyKind::DataType),
            ("lau-optimization-convex", "lau-banach-spaces", DependencyKind::Theorem),
            ("lau-optimization-convex", "lau-ml-gradient", DependencyKind::Code),
            ("lau-ml-gradient", "lau-tensor-algebra", DependencyKind::Code),
            ("lau-ml-gradient", "lau-numeric-solver", DependencyKind::Code),
            ("lau-fft-spectral", "lau-spectral-operators", DependencyKind::Theorem),
            ("lau-graph-spectral", "lau-spectral-operators", DependencyKind::Theorem),
            ("lau-graph-spectral", "lau-combinatorial-design", DependencyKind::Code),
            ("lau-cohomology-calculus", "lau-sheaf-theory", DependencyKind::Code),
            ("lau-cohomology-calculus", "lau-differential-forms", DependencyKind::Theorem),
            ("lau-sheaf-theory", "lau-categorical-bridge", DependencyKind::Code),
            ("lau-homotopy-type-theory", "lau-categorical-bridge", DependencyKind::Code),
            ("lau-probability-measures", "lau-measure-theory", DependencyKind::Code),
            ("lau-information-entropy", "lau-probability-measures", DependencyKind::Code),
            ("lau-cryptography-zkp", "lau-galois-theory", DependencyKind::Theorem),
            ("lau-cryptography-zkp", "lau-information-entropy", DependencyKind::Code),
            ("lau-calculus-variations", "lau-differential-forms", DependencyKind::Code),
            ("lau-calculus-variations", "lau-conservation-laws", DependencyKind::Theorem),
            ("lau-numeric-solver", "lau-banach-spaces", DependencyKind::Theorem),
            ("lau-cki-graph-engine", "lau-graph-spectral", DependencyKind::Code),
            ("lau-cki-graph-engine", "lau-fibonacci-growth", DependencyKind::DataType),
            ("lau-plato-monitor", "lau-ffi-bridge", DependencyKind::Build),
            ("lau-plato-distill", "lau-ml-gradient", DependencyKind::Code),
            ("lau-sunset-ethos", "lau-categorical-bridge", DependencyKind::Code),
            ("lau-sunset-logos", "lau-formal-logic", DependencyKind::Code),
            // ("lau-conservation-laws", "lau-calculus-variations", DependencyKind::Theorem), // removed to break cycle
            ("lau-lie-algebra", "lau-tensor-algebra", DependencyKind::Code),
            ("lau-galois-theory", "lau-lie-algebra", DependencyKind::Theorem),
            ("lau-distributed-consensus", "lau-ffi-bridge", DependencyKind::Build),
        ];
        for (from, to, kind) in canonical_edges {
            g.add_edge(from.into(), to.into(), kind);
        }
        g
    }

    /// Add a dependency edge.
    pub fn add_edge(&mut self, from: String, to: String, kind: DependencyKind) {
        self.edges.push(DependencyEdge { from: from.clone(), to: to.clone(), kind });
        self.deps.entry(from.clone()).or_default().insert(to.clone());
        self.rdeps.entry(to).or_default().insert(from);
    }

    /// Get direct dependencies of a crate.
    pub fn dependencies_of(&self, crate_name: &str) -> Vec<&str> {
        self.deps.get(crate_name)
            .map(|s| s.iter().map(|x| x.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get crates that depend on the given crate (reverse deps).
    pub fn dependents_of(&self, crate_name: &str) -> Vec<&str> {
        self.rdeps.get(crate_name)
            .map(|s| s.iter().map(|x| x.as_str()).collect())
            .unwrap_or_default()
    }

    /// Compute transitive dependencies (BFS).
    pub fn transitive_deps(&self, crate_name: &str) -> HashSet<&str> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        if let Some(direct) = self.deps.get(crate_name) {
            for dep in direct {
                queue.push_back(dep.as_str());
            }
        }
        while let Some(curr) = queue.pop_front() {
            if visited.insert(curr) {
                if let Some(next) = self.deps.get(curr) {
                    for dep in next {
                        if !visited.contains(dep.as_str()) {
                            queue.push_back(dep.as_str());
                        }
                    }
                }
            }
        }
        visited
    }

    /// Topological sort of all crates (dependencies first).
    pub fn topological_sort(&self) -> Vec<&str> {
        let mut out_degree: HashMap<&str, usize> = HashMap::new();
        let mut all_nodes: HashSet<&str> = HashSet::new();
        for edge in &self.edges {
            all_nodes.insert(&edge.from);
            all_nodes.insert(&edge.to);
        }
        for node in &all_nodes {
            out_degree.entry(node).or_insert(0);
        }
        // edge: from depends on to. Count how many deps each node has.
        let mut dep_count: HashMap<&str, usize> = HashMap::new();
        for node in &all_nodes {
            dep_count.entry(node).or_insert(0);
        }
        for edge in &self.edges {
            *dep_count.entry(&edge.from).or_insert(0) += 1;
        }
        // Nodes with 0 deps should come first
        let mut queue: VecDeque<&str> = dep_count.iter()
            .filter(|(_, &cnt)| cnt == 0)
            .map(|(&node, _)| node)
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            // node is a dependency; its dependents (rdeps) lose one dep
            if let Some(dependents) = self.rdeps.get(node) {
                for dep in dependents {
                    let entry = dep_count.get_mut(dep.as_str()).unwrap();
                    *entry -= 1;
                    if *entry == 0 {
                        queue.push_back(dep.as_str());
                    }
                }
            }
        }
        result
    }

    /// Count total edges.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Get all edges.
    pub fn edges(&self) -> &[DependencyEdge] {
        &self.edges
    }

    /// Detect cycles (returns true if cycle exists).
    pub fn has_cycle(&self) -> bool {
        let mut all_nodes: HashSet<&str> = HashSet::new();
        for edge in &self.edges {
            all_nodes.insert(&edge.from);
            all_nodes.insert(&edge.to);
        }
        let mut white: HashSet<&str> = all_nodes.clone();
        let mut gray: HashSet<&str> = HashSet::new();
        let mut black: HashSet<&str> = HashSet::new();

        let nodes: Vec<&str> = white.iter().copied().collect();
        for node in nodes {
            if white.contains(node) && Self::dfs_visit(node, &mut white, &mut gray, &mut black, &self.deps) {
                return true;
            }
        }
        false
    }

    fn dfs_visit<'a>(
        node: &'a str,
        white: &mut HashSet<&'a str>,
        gray: &mut HashSet<&'a str>,
        black: &mut HashSet<&'a str>,
        deps: &'a HashMap<String, HashSet<String>>,
    ) -> bool {
        white.remove(node);
        gray.insert(node);
        if let Some(neighbors) = deps.get(node) {
            for n in neighbors {
                let n_str = n.as_str();
                if gray.contains(n_str) {
                    return true;
                }
                if white.contains(n_str) && Self::dfs_visit(n_str, white, gray, black, deps) {
                    return true;
                }
            }
        }
        gray.remove(node);
        black.insert(node);
        false
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::with_canonical_deps()
    }
}
