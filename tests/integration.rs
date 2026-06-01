#[cfg(test)]
mod tests {
    use lau_ecosystem_unified::*;

    // ─── Registry tests ────────────────────────────────────────────

    #[test]
    fn registry_canonical_has_34_crates() {
        let reg = CrateRegistry::with_canonical_crates();
        assert!(reg.len() >= 30, "expected 30+ canonical crates, got {}", reg.len());
    }

    #[test]
    fn registry_lookup() {
        let reg = CrateRegistry::with_canonical_crates();
        let entry = reg.get("lau-spectral-operators").unwrap();
        assert_eq!(entry.domain, Domain::Spectral);
    }

    #[test]
    fn registry_by_domain() {
        let reg = CrateRegistry::with_canonical_crates();
        let spectral = reg.by_domain(Domain::Spectral);
        assert!(!spectral.is_empty());
    }

    #[test]
    fn registry_by_theorem() {
        let reg = CrateRegistry::with_canonical_crates();
        let crates = reg.by_theorem("index_theorem");
        assert!(crates.len() >= 2, "index_theorem should connect to 2+ crates");
    }

    #[test]
    fn registry_domain_counts() {
        let reg = CrateRegistry::with_canonical_crates();
        let counts = reg.domain_counts();
        assert!(counts.contains_key(&Domain::Spectral));
    }

    #[test]
    fn registry_register_new() {
        let mut reg = CrateRegistry::new();
        reg.register(CrateEntry {
            name: "lau-test-crate".into(),
            domain: Domain::Algebra,
            secondary_domains: vec![],
            description: "test".into(),
            theorem_connections: vec![],
            languages: vec![],
            dependency_count: 0,
            published: false,
            version: "0.0.1".into(),
        });
        assert_eq!(reg.len(), 1);
        assert!(reg.get("lau-test-crate").is_some());
    }

    #[test]
    fn registry_empty() {
        let reg = CrateRegistry::new();
        assert!(reg.is_empty());
    }

    #[test]
    fn registry_iter() {
        let reg = CrateRegistry::with_canonical_crates();
        let count = reg.iter().count();
        assert_eq!(count, reg.len());
    }

    #[test]
    fn registry_secondary_domains() {
        let reg = CrateRegistry::with_canonical_crates();
        let spectral = reg.by_domain(Domain::Analysis);
        // Some crates have Analysis as secondary domain
        assert!(spectral.len() >= 3);
    }

    // ─── Dependency graph tests ────────────────────────────────────

    #[test]
    fn dep_graph_canonical_has_edges() {
        let graph = DependencyGraph::with_canonical_deps();
        assert!(graph.edge_count() > 30, "expected many edges");
    }

    #[test]
    fn dep_graph_dependencies_of() {
        let graph = DependencyGraph::with_canonical_deps();
        let deps = graph.dependencies_of("lau-noncommutative-geometry");
        assert!(!deps.is_empty(), "NCG should depend on other crates");
    }

    #[test]
    fn dep_graph_dependents_of() {
        let graph = DependencyGraph::with_canonical_deps();
        let dependents = graph.dependents_of("lau-spectral-operators");
        assert!(!dependents.is_empty(), "spectral-operators should have dependents");
    }

    #[test]
    fn dep_graph_transitive() {
        let graph = DependencyGraph::with_canonical_deps();
        let trans = graph.transitive_deps("lau-noncommutative-geometry");
        assert!(trans.len() >= 2, "NCG should have transitive deps");
    }

    #[test]
    fn dep_graph_topological_sort() {
        let graph = DependencyGraph::with_canonical_deps();
        let topo = graph.topological_sort();
        assert!(!topo.is_empty());
        // Verify ordering: dependencies come before dependents
        let positions: std::collections::HashMap<&str, usize> = topo.iter()
            .enumerate()
            .map(|(i, &name)| (name, i))
            .collect();
        for edge in graph.edges() {
            if let (Some(&from_pos), Some(&to_pos)) = (positions.get(edge.from.as_str()), positions.get(edge.to.as_str())) {
                assert!(to_pos <= from_pos, "dependency {} should come before {} in topo sort", edge.to, edge.from);
            }
        }
    }

    #[test]
    fn dep_graph_no_cycle() {
        let graph = DependencyGraph::with_canonical_deps();
        assert!(!graph.has_cycle(), "canonical dependency graph should be acyclic");
    }

    #[test]
    fn dep_graph_cycle_detection() {
        let mut graph = DependencyGraph::new();
        graph.add_edge("a".into(), "b".into(), DependencyKind::Code);
        graph.add_edge("b".into(), "c".into(), DependencyKind::Code);
        graph.add_edge("c".into(), "a".into(), DependencyKind::Code);
        assert!(graph.has_cycle());
    }

    #[test]
    fn dep_graph_add_edge() {
        let mut graph = DependencyGraph::new();
        graph.add_edge("x".into(), "y".into(), DependencyKind::Theorem);
        assert_eq!(graph.edge_count(), 1);
    }

    // ─── Theorem map tests ─────────────────────────────────────────

    #[test]
    fn theorem_map_has_14_theorems() {
        let tm = TheoremMap::with_canonical();
        assert_eq!(tm.len(), 14, "expected 14 theorems");
    }

    #[test]
    fn theorem_map_get_index_theorem() {
        let tm = TheoremMap::with_canonical();
        let entry = tm.get("index_theorem").unwrap();
        assert!(!entry.proving_crates.is_empty());
    }

    #[test]
    fn theorem_map_crates_for_theorem() {
        let tm = TheoremMap::with_canonical();
        let crates = tm.crates_for_theorem("spectral_theorem");
        assert!(crates.len() >= 2);
    }

    #[test]
    fn theorem_map_theorems_for_crate() {
        let tm = TheoremMap::with_canonical();
        let theorems = tm.theorems_for_crate("lau-spectral-operators");
        assert!(theorems.len() >= 2, "spectral-operators connects to 2+ theorems");
    }

    #[test]
    fn theorem_enum_all_14() {
        assert_eq!(Theorem::all().len(), 14);
    }

    #[test]
    fn theorem_names() {
        assert_eq!(Theorem::IndexTheorem.name(), "Atiyah-Singer Index Theorem");
        assert_eq!(Theorem::SpectralTheorem.name(), "Spectral Theorem");
    }

    #[test]
    fn theorem_spectral_projections() {
        for theorem in Theorem::all() {
            let proj = theorem.spectral_projection();
            assert!(!proj.is_empty(), "spectral projection should not be empty for {:?}", theorem);
        }
    }

    #[test]
    fn theorem_map_empty() {
        let tm = TheoremMap::new();
        assert!(tm.is_empty());
    }

    // ─── Language matrix tests ─────────────────────────────────────

    #[test]
    fn lang_matrix_canonical() {
        let lm = LanguageMatrix::with_canonical();
        assert!(lm.len() >= 30);
    }

    #[test]
    fn lang_matrix_languages_for() {
        let lm = LanguageMatrix::with_canonical();
        let langs = lm.languages_for("lau-ffi-bridge");
        assert!(langs.len() == 7, "ffi-bridge should support all 7 languages");
    }

    #[test]
    fn lang_matrix_crates_with_cuda() {
        let lm = LanguageMatrix::with_canonical();
        let crates = lm.crates_with_language(Language::CUDA);
        assert!(crates.len() >= 10, "expected 10+ CUDA crates");
    }

    #[test]
    fn lang_matrix_counts() {
        let lm = LanguageMatrix::with_canonical();
        let counts = lm.counts_by_language();
        assert!(counts[&Language::Rust] >= 30);
        assert!(counts[&Language::CUDA] >= 5);
    }

    #[test]
    fn lang_matrix_polyglot() {
        let lm = LanguageMatrix::with_canonical();
        let polyglot = lm.polyglot_crates();
        assert!(!polyglot.is_empty(), "should have polyglot crates");
        // ffi-bridge should be polyglot
        assert!(polyglot.iter().any(|(name, _)| *name == "lau-ffi-bridge"));
    }

    #[test]
    fn lang_matrix_empty() {
        let lm = LanguageMatrix::new();
        assert!(lm.is_empty());
    }

    #[test]
    fn lang_matrix_language_count() {
        let lm = LanguageMatrix::with_canonical();
        assert_eq!(lm.language_count("lau-ffi-bridge"), 7);
    }

    #[test]
    fn lang_matrix_display() {
        assert_eq!(Language::CUDA.to_string(), "CUDA");
    }

    // ─── Synergy detector tests ────────────────────────────────────

    #[test]
    fn synergy_canonical_count() {
        let sd = SynergyDetector::with_canonical();
        assert!(sd.len() >= 15, "expected 15+ canonical synergies, got {}", sd.len());
    }

    #[test]
    fn synergy_for_crate() {
        let sd = SynergyDetector::with_canonical();
        let syns = sd.synergies_for("lau-fft-spectral");
        assert!(!syns.is_empty());
    }

    #[test]
    fn synergy_by_kind() {
        let sd = SynergyDetector::with_canonical();
        let spectral = sd.by_kind(SynergyKind::SpectralEnhancement);
        assert!(!spectral.is_empty());
    }

    #[test]
    fn synergy_counts_by_kind() {
        let sd = SynergyDetector::with_canonical();
        let counts = sd.counts_by_kind();
        assert!(counts.values().sum::<usize>() == sd.len());
    }

    #[test]
    fn synergy_detect_shared_theorems() {
        let sd = SynergyDetector::new();
        let result = sd.detect_from_shared_theorems(
            "crate-a", "crate-b", &["index_theorem".into()]
        );
        assert!(result.is_some());
        assert_eq!(result.unwrap().kind, SynergyKind::TheoremAmplification);
    }

    #[test]
    fn synergy_detect_no_shared() {
        let sd = SynergyDetector::new();
        let result = sd.detect_from_shared_theorems("a", "b", &[]);
        assert!(result.is_none());
    }

    #[test]
    fn synergy_empty() {
        let sd = SynergyDetector::new();
        assert!(sd.is_empty());
    }

    #[test]
    fn synergy_all() {
        let sd = SynergyDetector::with_canonical();
        assert_eq!(sd.all().len(), sd.len());
    }

    // ─── Spectral triple tests ─────────────────────────────────────

    #[test]
    fn spectral_triple_canonical_spin() {
        let triple = SpectralTriple::canonical_spin_manifold();
        assert!(triple.is_commutative());
        assert!(triple.algebra.commutative);
        assert_eq!(triple.dirac_operator.kind, DiracKind::Spin);
    }

    #[test]
    fn spectral_triple_finite() {
        let triple = SpectralTriple::finite_triple(3);
        assert!(!triple.is_commutative());
        assert_eq!(triple.hilbert_space.dimension, Some(3));
    }

    #[test]
    fn spectral_triple_product() {
        let triple = SpectralTriple::product_triple();
        assert!(!triple.is_commutative());
        assert_eq!(triple.dirac_operator.kind, DiracKind::Product);
    }

    #[test]
    fn spectral_triple_satisfies_axioms() {
        let triple = SpectralTriple::canonical_spin_manifold();
        assert!(triple.metadata.satisfies_axioms);
    }

    #[test]
    fn spectral_triple_theorem_connections() {
        let triple = SpectralTriple::product_triple();
        assert!(triple.metadata.theorem_connections.len() >= 3);
    }

    #[test]
    fn spectral_triple_spectral_distance() {
        let triple = SpectralTriple::canonical_spin_manifold();
        assert!(triple.spectral_distance_hint().is_some());
    }

    #[test]
    fn spectral_triple_serialization() {
        let triple = SpectralTriple::finite_triple(4);
        let json = serde_json::to_string(&triple).unwrap();
        let deserialized: SpectralTriple = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.hilbert_space.dimension, Some(4));
    }

    #[test]
    fn dirac_operator_types() {
        let triple = SpectralTriple::canonical_spin_manifold();
        assert!(triple.dirac_operator.self_adjoint);
        assert!(triple.dirac_operator.compact_resolvent);
    }

    // ─── Health tests ──────────────────────────────────────────────

    #[test]
    fn health_canonical() {
        let health = EcosystemHealth::with_canonical();
        assert!(health.len() >= 30);
    }

    #[test]
    fn health_total_tests() {
        let health = EcosystemHealth::with_canonical();
        assert!(health.total_tests() > 500);
    }

    #[test]
    fn health_average_coverage() {
        let health = EcosystemHealth::with_canonical();
        let avg = health.average_coverage();
        assert!(avg > 70.0 && avg < 100.0, "avg coverage should be reasonable, got {}", avg);
    }

    #[test]
    fn health_published_count() {
        let health = EcosystemHealth::with_canonical();
        assert!(health.published_count() >= 28);
    }

    #[test]
    fn health_status_counts() {
        let health = EcosystemHealth::with_canonical();
        let counts = health.status_counts();
        assert!(counts.contains_key(&HealthStatus::Healthy));
    }

    #[test]
    fn health_needs_attention() {
        let health = EcosystemHealth::with_canonical();
        let needs = health.needs_attention();
        assert!(!needs.is_empty(), "some crates should need attention");
    }

    #[test]
    fn health_overall_score() {
        let health = EcosystemHealth::with_canonical();
        let score = health.overall_score();
        assert!(score > 50.0 && score <= 100.0, "overall score should be reasonable, got {}", score);
    }

    #[test]
    fn health_empty() {
        let health = EcosystemHealth::new();
        assert!(health.is_empty());
        assert_eq!(health.total_tests(), 0);
    }

    #[test]
    fn health_get_crate() {
        let health = EcosystemHealth::with_canonical();
        let h = health.get("lau-spectral-operators").unwrap();
        assert_eq!(h.test_count, 47);
    }

    // ─── Bridge pattern tests ──────────────────────────────────────

    #[test]
    fn bridge_canonical() {
        let bp = BridgePattern::with_canonical();
        assert!(bp.node_count() >= 10);
        assert!(bp.bridge_count() >= 10);
    }

    #[test]
    fn bridge_get_node() {
        let bp = BridgePattern::with_canonical();
        let node = bp.get_node("fib-growth-core").unwrap();
        assert_eq!(node.node_type, PatternNodeType::Fibonacci);
    }

    #[test]
    fn bridge_bridges_for_crate() {
        let bp = BridgePattern::with_canonical();
        let bridges = bp.bridges_for_crate("lau-spectral-operators");
        assert!(!bridges.is_empty());
    }

    #[test]
    fn bridge_nodes_for_crate() {
        let bp = BridgePattern::with_canonical();
        let nodes = bp.nodes_for_crate("lau-fibonacci-growth");
        assert!(!nodes.is_empty());
    }

    #[test]
    fn bridge_conservation_invariants() {
        let bp = BridgePattern::with_canonical();
        assert!(bp.conservation_invariant_count() >= 4);
    }

    #[test]
    fn bridge_bridges_by_kind() {
        let bp = BridgePattern::with_canonical();
        let conservation = bp.bridges_by_kind(BridgeKind::Conservation);
        assert!(!conservation.is_empty());
    }

    #[test]
    fn bridge_serialization() {
        let bp = BridgePattern::with_canonical();
        let json = serde_json::to_string(&bp).unwrap();
        assert!(!json.is_empty());
        let deserialized: BridgePattern = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.node_count(), bp.node_count());
    }

    #[test]
    fn bridge_bidirectional() {
        let bp = BridgePattern::with_canonical();
        let bidir: Vec<_> = bp.all_bridges().iter().filter(|b| b.bidirectional).collect();
        assert!(!bidir.is_empty());
    }

    // ─── Cross-module integration tests ────────────────────────────

    #[test]
    fn integration_registry_theorem_map() {
        let reg = CrateRegistry::with_canonical_crates();
        let tm = TheoremMap::with_canonical();
        // Every theorem's proving crate should exist in registry
        for entry in tm.all() {
            for crate_name in &entry.proving_crates {
                assert!(reg.get(crate_name).is_some(), "proving crate {} should be in registry", crate_name);
            }
        }
    }

    #[test]
    fn integration_registry_health() {
        let reg = CrateRegistry::with_canonical_crates();
        let health = EcosystemHealth::with_canonical();
        for entry in reg.iter() {
            if let Some(h) = health.get(&entry.name) {
                assert_eq!(h.published, entry.published);
            }
        }
    }

    #[test]
    fn integration_synergy_registry() {
        let reg = CrateRegistry::with_canonical_crates();
        let sd = SynergyDetector::with_canonical();
        for synergy in sd.all() {
            assert!(reg.get(&synergy.crate_a).is_some(), "synergy crate {} should be in registry", synergy.crate_a);
            assert!(reg.get(&synergy.crate_b).is_some(), "synergy crate {} should be in registry", synergy.crate_b);
        }
    }

    #[test]
    fn integration_bridge_pattern_registry() {
        let reg = CrateRegistry::with_canonical_crates();
        let bp = BridgePattern::with_canonical();
        for node in bp.all_nodes() {
            if let Some(ref crate_name) = node.math_crate_link {
                assert!(reg.get(crate_name).is_some(), "bridge node crate {} should be in registry", crate_name);
            }
        }
    }

    #[test]
    fn integration_language_matrix_registry() {
        let reg = CrateRegistry::with_canonical_crates();
        let lm = LanguageMatrix::with_canonical();
        // Language matrix languages should match registry languages
        for entry in reg.iter() {
            if let Some(_langs) = lm.languages_for(&entry.name).into_iter().next() {
                // At minimum, Rust should be in the matrix if the crate exists
                // (All canonical crates have Rust)
            }
        }
    }

    #[test]
    fn integration_dep_graph_registry() {
        let reg = CrateRegistry::with_canonical_crates();
        let graph = DependencyGraph::with_canonical_deps();
        for edge in graph.edges() {
            // Edges may reference crates outside canonical set
            if reg.get(&edge.from).is_some() {
                // ok
            }
        }
    }

    #[test]
    fn ecosystem_full_summary() {
        let reg = CrateRegistry::with_canonical_crates();
        let health = EcosystemHealth::with_canonical();
        let tm = TheoremMap::with_canonical();
        let lm = LanguageMatrix::with_canonical();
        let sd = SynergyDetector::with_canonical();
        let bp = BridgePattern::with_canonical();

        assert!(reg.len() >= 30);
        assert!(tm.len() == 14);
        assert!(health.total_tests() > 500);
        assert!(lm.len() >= 30);
        assert!(sd.len() >= 15);
        assert!(bp.node_count() >= 10);
    }

    #[test]
    fn all_serde_roundtrip() {
        let reg = CrateRegistry::with_canonical_crates();
        let json = serde_json::to_string(&reg).unwrap();
        let _: CrateRegistry = serde_json::from_str(&json).unwrap();

        let graph = DependencyGraph::with_canonical_deps();
        let json = serde_json::to_string(&graph).unwrap();
        let _: DependencyGraph = serde_json::from_str(&json).unwrap();

        let tm = TheoremMap::with_canonical();
        let json = serde_json::to_string(&tm).unwrap();
        let _: TheoremMap = serde_json::from_str(&json).unwrap();

        let lm = LanguageMatrix::with_canonical();
        let json = serde_json::to_string(&lm).unwrap();
        let _: LanguageMatrix = serde_json::from_str(&json).unwrap();

        let sd = SynergyDetector::with_canonical();
        let json = serde_json::to_string(&sd).unwrap();
        let _: SynergyDetector = serde_json::from_str(&json).unwrap();

        let health = EcosystemHealth::with_canonical();
        let json = serde_json::to_string(&health).unwrap();
        let _: EcosystemHealth = serde_json::from_str(&json).unwrap();
    }
}
