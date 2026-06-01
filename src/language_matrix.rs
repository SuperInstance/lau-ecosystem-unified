//! Language implementation matrix across the ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Supported languages in the ecosystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Rust,
    C,
    CUDA,
    Chapel,
    Go,
    OpenCL,
    WASM,
}

impl Language {
    pub fn all() -> &'static [Language] {
        &[
            Language::Rust, Language::C, Language::CUDA, Language::Chapel,
            Language::Go, Language::OpenCL, Language::WASM,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::C => "C",
            Language::CUDA => "CUDA",
            Language::Chapel => "Chapel",
            Language::Go => "Go",
            Language::OpenCL => "OpenCL",
            Language::WASM => "WASM",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Matrix of crate × language implementations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageMatrix {
    /// crate_name -> set of languages it has implementations for
    matrix: HashMap<String, HashSet<Language>>,
}

impl LanguageMatrix {
    pub fn new() -> Self {
        Self { matrix: HashMap::new() }
    }

    pub fn with_canonical() -> Self {
        let mut m = Self::new();
        let entries: Vec<(&str, Vec<Language>)> = vec![
            ("lau-spectral-operators", vec![Language::Rust, Language::C, Language::CUDA]),
            ("lau-noncommutative-geometry", vec![Language::Rust, Language::C, Language::WASM]),
            ("lau-homotopy-type-theory", vec![Language::Rust]),
            ("lau-quantum-operators", vec![Language::Rust, Language::CUDA, Language::OpenCL]),
            ("lau-fibonacci-growth", vec![Language::Rust, Language::Chapel]),
            ("lau-tensor-algebra", vec![Language::Rust, Language::C, Language::CUDA]),
            ("lau-optimization-convex", vec![Language::Rust, Language::CUDA, Language::WASM]),
            ("lau-differential-forms", vec![Language::Rust, Language::C]),
            ("lau-cki-graph-engine", vec![Language::Rust, Language::Go, Language::WASM]),
            ("lau-sunset-ethos", vec![Language::Rust]),
            ("lau-sunset-pathos", vec![Language::Rust]),
            ("lau-sunset-logos", vec![Language::Rust]),
            ("lau-plato-monitor", vec![Language::Rust, Language::Go]),
            ("lau-plato-distill", vec![Language::Rust, Language::CUDA]),
            ("lau-conservation-laws", vec![Language::Rust, Language::C]),
            ("lau-fft-spectral", vec![Language::Rust, Language::CUDA, Language::OpenCL, Language::WASM]),
            ("lau-categorical-bridge", vec![Language::Rust]),
            ("lau-probability-measures", vec![Language::Rust, Language::CUDA]),
            ("lau-information-entropy", vec![Language::Rust, Language::CUDA, Language::WASM]),
            ("lau-ffi-bridge", vec![Language::Rust, Language::C, Language::CUDA, Language::Chapel, Language::Go, Language::OpenCL, Language::WASM]),
            ("lau-lie-algebra", vec![Language::Rust, Language::C]),
            ("lau-distributed-consensus", vec![Language::Rust, Language::Go]),
            ("lau-numeric-solver", vec![Language::Rust, Language::CUDA, Language::OpenCL]),
            ("lau-combinatorial-design", vec![Language::Rust, Language::Chapel]),
            ("lau-ml-gradient", vec![Language::Rust, Language::CUDA, Language::WASM]),
            ("lau-cryptography-zkp", vec![Language::Rust, Language::WASM]),
            ("lau-sheaf-theory", vec![Language::Rust]),
            ("lau-cohomology-calculus", vec![Language::Rust, Language::C]),
            ("lau-connection-theory", vec![Language::Rust, Language::CUDA]),
            ("lau-calculus-variations", vec![Language::Rust, Language::C]),
            ("lau-measure-theory", vec![Language::Rust]),
            ("lau-galois-theory", vec![Language::Rust]),
            ("lau-banach-spaces", vec![Language::Rust, Language::C]),
            ("lau-graph-spectral", vec![Language::Rust, Language::CUDA, Language::Chapel]),
        ];
        for (crate_name, langs) in entries {
            m.register(crate_name, langs);
        }
        m
    }

    /// Register language implementations for a crate.
    pub fn register(&mut self, crate_name: &str, languages: Vec<Language>) {
        self.matrix.insert(
            crate_name.into(),
            languages.into_iter().collect(),
        );
    }

    /// Get languages supported by a crate.
    pub fn languages_for(&self, crate_name: &str) -> Vec<Language> {
        self.matrix.get(crate_name)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Get crates that have an implementation in a given language.
    pub fn crates_with_language(&self, lang: Language) -> Vec<&str> {
        self.matrix.iter()
            .filter(|(_, langs)| langs.contains(&lang))
            .map(|(name, _)| name.as_str())
            .collect()
    }

    /// Count of crates per language.
    pub fn counts_by_language(&self) -> HashMap<Language, usize> {
        let mut counts = HashMap::new();
        for lang in Language::all() {
            counts.insert(*lang, self.crates_with_language(*lang).len());
        }
        counts
    }

    /// Total unique crates in the matrix.
    pub fn len(&self) -> usize {
        self.matrix.len()
    }

    pub fn is_empty(&self) -> bool {
        self.matrix.is_empty()
    }

    /// How many languages a crate supports.
    pub fn language_count(&self, crate_name: &str) -> usize {
        self.matrix.get(crate_name).map(|s| s.len()).unwrap_or(0)
    }

    /// Crates that are polyglot (3+ languages).
    pub fn polyglot_crates(&self) -> Vec<(&str, usize)> {
        self.matrix.iter()
            .filter(|(_, langs)| langs.len() >= 3)
            .map(|(name, langs)| (name.as_str(), langs.len()))
            .collect()
    }
}

impl Default for LanguageMatrix {
    fn default() -> Self {
        Self::with_canonical()
    }
}
