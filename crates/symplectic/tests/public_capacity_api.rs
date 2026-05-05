use symplectic::{
    capacity_hk2017_unpruned_f64, known_polytopes, CapacityError, ConstructionError,
    PredicateVerdict,
};

fn dual_vertices(polytope: &symplectic::Polytope4D) -> Vec<[f64; 4]> {
    polytope
        .dual_vertices_f64()
        .iter()
        .map(|a| [a[0], a[1], a[2], a[3]])
        .collect()
}

#[test]
fn unpruned_f64_capacity_accepts_dual_vertices_without_polytope_anchor() {
    let kp = known_polytopes::simplex();
    let duals = dual_vertices(&kp.polytope);

    let result = capacity_hk2017_unpruned_f64(&duals, 0.0).expect("simplex capacity");

    let capacity = result.capacity().expect("simplex should have a capacity");
    assert!(
        (capacity - kp.capacity).abs() < 1e-6,
        "simplex capacity: got {capacity}, expected {}",
        kp.capacity
    );
    assert!(result.min_action.lower <= result.min_action.upper);
    assert_eq!(result.min_action.upper, capacity);
    assert!(
        (result.min_action.lower - kp.capacity).abs() < 1e-6,
        "simplex lower action bound: got {}, expected {}",
        result.min_action.lower,
        kp.capacity
    );

    let best = result
        .best_orbit()
        .expect("simplex should have a best orbit");
    assert_eq!(best.admissible, PredicateVerdict::True);
    assert_eq!(best.action.upper, capacity);
    assert!(!best.sigma.is_empty());
    assert_eq!(best.sigma.len(), best.beta.len());
    assert!(best.q > 0.0);
    assert!(best.q_error_bound >= 0.0);
}

#[test]
fn unpruned_f64_capacity_rejects_negative_action_gap() {
    let kp = known_polytopes::simplex();
    let duals = dual_vertices(&kp.polytope);

    let err = capacity_hk2017_unpruned_f64(&duals, -1.0)
        .expect_err("negative action gap should be rejected");

    assert_eq!(
        err,
        CapacityError::OrbitSearch(symplectic::OrbitSearchError::InvalidGap)
    );
}

#[test]
fn unpruned_f64_capacity_rejects_nan_action_gap() {
    let kp = known_polytopes::simplex();
    let duals = dual_vertices(&kp.polytope);

    let err = capacity_hk2017_unpruned_f64(&duals, f64::NAN)
        .expect_err("NaN action gap should be rejected");

    assert_eq!(
        err,
        CapacityError::OrbitSearch(symplectic::OrbitSearchError::InvalidGap)
    );
}

#[test]
fn unpruned_f64_capacity_reports_geometry_errors() {
    let too_few_facets = [[1.0, 0.0, 0.0, 0.0]; 4];

    let err = capacity_hk2017_unpruned_f64(&too_few_facets, 0.0)
        .expect_err("invalid geometry should be rejected");

    assert_eq!(
        err,
        CapacityError::Geometry(ConstructionError::TooFewFacets(4))
    );
}
