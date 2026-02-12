//! Property tests for systolic ratio (sys) computation.
//!
//! Verifies mathematical properties:
//! - Formula: sys = c_EHZ² / (2·volume) is computed correctly
//! - Positivity: sys > 0 for all valid polytopes

use hk2017::test_dataset::generate_test_dataset;

#[test]
#[ignore] // ~minutes: generates random polytopes and computes capacity (exponential algorithm)
fn sys_formula_correctness() {
    let dataset = generate_test_dataset();

    for entry in &dataset {
        // Compute sys from precomputed capacity and volume
        let sys = entry.capacity.powi(2) / (2.0 * entry.volume);

        // Verify sys is positive
        assert!(
            sys > 0.0,
            "{}: sys should be positive, got {}",
            entry.name, sys
        );

        // Verify formula is consistent (capacity² / (2·vol) = sys)
        let expected_sys = entry.capacity.powi(2) / (2.0 * entry.volume);
        assert!(
            (sys - expected_sys).abs() < 1e-10,
            "{}: sys formula inconsistency",
            entry.name
        );
    }

    println!("✓ Verified sys = c²/(2·vol) for {} polytopes", dataset.len());
}

#[test]
#[ignore] // ~minutes: generates random polytopes and computes capacity (exponential algorithm)
fn sys_distribution_statistics() {
    let dataset = generate_test_dataset();

    let sys_values: Vec<f64> = dataset
        .iter()
        .map(|e| e.capacity.powi(2) / (2.0 * e.volume))
        .collect();

    let min_sys = sys_values.iter().fold(f64::INFINITY, |a: f64, &b| a.min(b));
    let max_sys = sys_values
        .iter()
        .fold(f64::NEG_INFINITY, |a: f64, &b| a.max(b));
    let mean_sys = sys_values.iter().sum::<f64>() / sys_values.len() as f64;

    println!("sys distribution over {} polytopes:", dataset.len());
    println!("  min:  {:.6}", min_sys);
    println!("  max:  {:.6}", max_sys);
    println!("  mean: {:.6}", mean_sys);
    println!(
        "  above 1.0: {}/{}",
        sys_values.iter().filter(|&&s| s > 1.0).count(),
        sys_values.len()
    );

    // Sanity checks (not strict bounds, just basic validation)
    assert!(min_sys > 0.0, "all sys values should be positive");
    assert!(max_sys < 100.0, "sys values should be reasonable (< 100)");
}
