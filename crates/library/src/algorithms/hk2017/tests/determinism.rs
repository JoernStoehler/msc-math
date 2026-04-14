mod tests_determinism {
    use std::path::PathBuf;
    use std::sync::LazyLock;

    use crate::algorithms::hk2017::generate_capacity_fixtures::{
        load_dataset_entries, polytope_catalog, DatasetEntry, FIXTURE_PATH,
    };

    static DATASET: LazyLock<Vec<DatasetEntry>> = LazyLock::new(|| {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(FIXTURE_PATH);
        load_dataset_entries(&path)
    });

    /// Verify polytope_catalog() is deterministic (same seed -> same polytopes).
    ///
    /// Calls polytope_catalog() twice and verifies identical output. Critical invariant
    /// for fixture generation: non-determinism would silently invalidate the fixture.
    #[test]
    #[ignore] // ~17s debug, ~1s release: constructs all 33 polytopes twice. Run during fixture regeneration.
    fn catalog_determinism() {
        let c1 = polytope_catalog();
        let c2 = polytope_catalog();
        assert_eq!(c1.len(), c2.len());
        for (a, b) in c1.iter().zip(c2.iter()) {
            assert_eq!(a.name, b.name);
            assert_eq!(
                a.polytope.dual_vertices_f64(),
                b.polytope.dual_vertices_f64(),
                "'{}': dual vertices non-deterministic",
                a.name
            );
        }
    }

    /// Detect fixture staleness: compares polytope names in fixture vs current catalog.
    ///
    /// If this test warns, regenerate the fixture:
    /// `cargo test --release regenerate_test_dataset -- --ignored --nocapture`
    #[test]
    #[ignore] // ~17s debug, ~1s release: constructs all 33 polytopes. Run during fixture regeneration.
    fn fixture_staleness_check() {
        let catalog = polytope_catalog();
        let dataset = &*DATASET;

        let catalog_names: std::collections::HashSet<&str> =
            catalog.iter().map(|c| c.name.as_str()).collect();
        let fixture_names: std::collections::HashSet<&str> =
            dataset.iter().map(|tp| tp.name.as_str()).collect();

        let missing: Vec<_> = catalog_names.difference(&fixture_names).collect();
        let orphaned: Vec<_> = fixture_names.difference(&catalog_names).collect();

        for name in &missing {
            eprintln!("WARNING: catalog polytope '{}' not in fixture", name);
        }
        for name in &orphaned {
            eprintln!(
                "WARNING: fixture polytope '{}' not in current catalog",
                name
            );
        }

        if !missing.is_empty() || !orphaned.is_empty() {
            eprintln!(
                "WARNING: fixture staleness detected ({} missing, {} orphaned). \
                 Regenerate with: cargo test --release regenerate_test_dataset -- --ignored --nocapture",
                missing.len(),
                orphaned.len()
            );
        } else {
            eprintln!("Fixture covers all {} catalog polytopes", catalog.len());
        }
    }
}
