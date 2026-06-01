//! Ecosystem health monitoring: test counts, coverage, publish status.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Health status of a single crate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateHealth {
    pub name: String,
    pub test_count: u32,
    pub coverage_percent: f64,
    pub published: bool,
    pub version: String,
    pub status: HealthStatus,
    pub last_updated: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Hash)]
pub enum HealthStatus {
    /// All tests passing, published, good coverage
    Healthy,
    /// Most tests passing, may need attention
    Warning,
    /// Failing tests or not published
    Critical,
    /// Work in progress
    InDevelopment,
}

/// Overall ecosystem health.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHealth {
    crates: HashMap<String, CrateHealth>,
}

impl EcosystemHealth {
    pub fn new() -> Self {
        Self { crates: HashMap::new() }
    }

    pub fn with_canonical() -> Self {
        let mut h = Self::new();
        let entries = vec![
            ("lau-spectral-operators", 47, 92.3, true, "0.3.2", HealthStatus::Healthy, "2025-12-01"),
            ("lau-noncommutative-geometry", 63, 88.7, true, "0.4.1", HealthStatus::Healthy, "2025-12-10"),
            ("lau-homotopy-type-theory", 31, 85.0, true, "0.2.0", HealthStatus::Healthy, "2025-11-15"),
            ("lau-quantum-operators", 55, 90.1, true, "0.3.0", HealthStatus::Healthy, "2025-11-28"),
            ("lau-fibonacci-growth", 22, 78.5, true, "0.1.5", HealthStatus::Warning, "2025-10-20"),
            ("lau-tensor-algebra", 40, 91.0, true, "0.5.0", HealthStatus::Healthy, "2025-12-05"),
            ("lau-optimization-convex", 38, 89.2, true, "0.4.0", HealthStatus::Healthy, "2025-11-30"),
            ("lau-differential-forms", 29, 86.4, true, "0.3.1", HealthStatus::Healthy, "2025-11-22"),
            ("lau-cki-graph-engine", 51, 82.0, true, "0.6.2", HealthStatus::Healthy, "2025-12-15"),
            ("lau-sunset-ethos", 12, 65.0, true, "0.1.0", HealthStatus::Warning, "2025-09-01"),
            ("lau-sunset-pathos", 10, 60.0, true, "0.1.0", HealthStatus::Warning, "2025-09-01"),
            ("lau-sunset-logos", 15, 70.0, true, "0.1.0", HealthStatus::Warning, "2025-09-01"),
            ("lau-plato-monitor", 18, 55.0, false, "0.2.0-beta", HealthStatus::InDevelopment, "2025-12-20"),
            ("lau-plato-distill", 8, 40.0, false, "0.1.0-alpha", HealthStatus::Critical, "2025-12-18"),
            ("lau-conservation-laws", 33, 87.5, true, "0.2.3", HealthStatus::Healthy, "2025-11-10"),
            ("lau-fft-spectral", 45, 93.0, true, "0.5.1", HealthStatus::Healthy, "2025-12-08"),
            ("lau-categorical-bridge", 24, 82.0, true, "0.2.0", HealthStatus::Healthy, "2025-10-15"),
            ("lau-probability-measures", 35, 88.0, true, "0.3.0", HealthStatus::Healthy, "2025-11-25"),
            ("lau-information-entropy", 28, 84.5, true, "0.2.2", HealthStatus::Healthy, "2025-11-20"),
            ("lau-ffi-bridge", 42, 76.0, true, "0.7.0", HealthStatus::Warning, "2025-12-12"),
            ("lau-lie-algebra", 37, 89.0, true, "0.3.0", HealthStatus::Healthy, "2025-11-18"),
            ("lau-distributed-consensus", 30, 80.0, true, "0.2.0", HealthStatus::Healthy, "2025-10-30"),
            ("lau-numeric-solver", 52, 91.5, true, "0.4.2", HealthStatus::Healthy, "2025-12-03"),
            ("lau-combinatorial-design", 25, 79.0, true, "0.2.1", HealthStatus::Warning, "2025-10-25"),
            ("lau-ml-gradient", 44, 87.0, true, "0.5.0", HealthStatus::Healthy, "2025-12-07"),
            ("lau-cryptography-zkp", 36, 83.0, true, "0.3.0", HealthStatus::Healthy, "2025-11-12"),
            ("lau-sheaf-theory", 20, 75.0, true, "0.2.0", HealthStatus::Warning, "2025-10-08"),
            ("lau-cohomology-calculus", 32, 85.0, true, "0.3.0", HealthStatus::Healthy, "2025-11-05"),
            ("lau-connection-theory", 34, 86.0, true, "0.3.1", HealthStatus::Healthy, "2025-11-14"),
            ("lau-calculus-variations", 27, 83.0, true, "0.2.0", HealthStatus::Healthy, "2025-10-18"),
            ("lau-measure-theory", 30, 88.0, true, "0.3.0", HealthStatus::Healthy, "2025-11-02"),
            ("lau-galois-theory", 23, 80.0, true, "0.2.0", HealthStatus::Healthy, "2025-10-12"),
            ("lau-banach-spaces", 38, 90.0, true, "0.3.0", HealthStatus::Healthy, "2025-11-08"),
            ("lau-graph-spectral", 41, 87.0, true, "0.4.0", HealthStatus::Healthy, "2025-12-02"),
        ];
        for (name, tests, coverage, published, version, status, updated) in entries {
            h.register(CrateHealth {
                name: name.into(),
                test_count: tests,
                coverage_percent: coverage,
                published,
                version: version.into(),
                status,
                last_updated: updated.into(),
            });
        }
        h
    }

    pub fn register(&mut self, health: CrateHealth) {
        self.crates.insert(health.name.clone(), health);
    }

    pub fn get(&self, name: &str) -> Option<&CrateHealth> {
        self.crates.get(name)
    }

    /// Total test count across all crates.
    pub fn total_tests(&self) -> u32 {
        self.crates.values().map(|c| c.test_count).sum()
    }

    /// Average coverage across all crates.
    pub fn average_coverage(&self) -> f64 {
        if self.crates.is_empty() { return 0.0; }
        self.crates.values().map(|c| c.coverage_percent).sum::<f64>() / self.crates.len() as f64
    }

    /// Count of published crates.
    pub fn published_count(&self) -> usize {
        self.crates.values().filter(|c| c.published).count()
    }

    /// Count of crates by health status.
    pub fn status_counts(&self) -> HashMap<HealthStatus, usize> {
        let mut counts = HashMap::new();
        for c in self.crates.values() {
            *counts.entry(c.status).or_insert(0) += 1;
        }
        counts
    }

    /// Crates that need attention (Warning or Critical).
    pub fn needs_attention(&self) -> Vec<&CrateHealth> {
        self.crates.values()
            .filter(|c| c.status == HealthStatus::Warning || c.status == HealthStatus::Critical)
            .collect()
    }

    /// Total number of crates tracked.
    pub fn len(&self) -> usize {
        self.crates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.crates.is_empty()
    }

    /// Overall ecosystem health score (0-100).
    pub fn overall_score(&self) -> f64 {
        if self.crates.is_empty() { return 0.0; }
        let published_ratio = self.published_count() as f64 / self.len() as f64;
        let avg_coverage = self.average_coverage() / 100.0;
        let healthy_ratio = self.status_counts().get(&HealthStatus::Healthy).copied().unwrap_or(0) as f64 / self.len() as f64;
        (published_ratio * 30.0 + avg_coverage * 40.0 + healthy_ratio * 30.0).min(100.0)
    }
}

impl Default for EcosystemHealth {
    fn default() -> Self {
        Self::with_canonical()
    }
}
