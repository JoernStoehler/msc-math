use super::*;

#[test]
fn sweep_produces_expected_row_count() {
    // 6 facet counts × 3 height ranges = 18 rows
    let rows = run_sweep(10, 42);
    assert_eq!(rows.len(), 18);
}

#[test]
fn counts_add_up() {
    let rows = run_sweep(50, 0);
    for row in &rows {
        assert_eq!(
            row.n_total, 50,
            "n_total should match n_attempts for F={}",
            row.facet_count
        );
        assert!(
            row.n_accepted <= row.n_total,
            "n_accepted ({}) > n_total ({}) for F={}",
            row.n_accepted,
            row.n_total,
            row.facet_count
        );
        let expected_ratio = row.n_accepted as f64 / row.n_total as f64;
        assert!(
            (row.acceptance_ratio - expected_ratio).abs() < 1e-12,
            "acceptance_ratio mismatch for F={}",
            row.facet_count
        );
    }
}
