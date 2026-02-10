use super::*;
use nalgebra::Vector4;

fn dummy_polytope() -> Polytope4D {
    let normals = vec![
        Vector4::x(),
        -Vector4::x(),
        Vector4::y(),
        -Vector4::y(),
        Vector4::z(),
        -Vector4::z(),
        Vector4::w(),
        -Vector4::w(),
    ];
    Polytope4D::new(normals, vec![1.0; 8]).unwrap()
}

#[test]
fn volume_returns_one() {
    let (vol, _) = volume_stub(&dummy_polytope());
    assert!((vol - 1.0).abs() < 1e-12);
}

#[test]
fn capacity_returns_one() {
    let (cap, _) = capacity_stub(&dummy_polytope());
    assert!((cap - 1.0).abs() < 1e-12);
}

#[test]
fn stubs_take_approximately_1ms() {
    let p = dummy_polytope();
    let (_, vol_time) = volume_stub(&p);
    let (_, cap_time) = capacity_stub(&p);
    // Allow wide tolerance: 0.5ms to 10ms (CI environments can be slow)
    let min = Duration::from_micros(500);
    let max = Duration::from_millis(10);
    assert!(
        vol_time >= min && vol_time <= max,
        "volume_stub took {vol_time:?}, expected {min:?}..{max:?}"
    );
    assert!(
        cap_time >= min && cap_time <= max,
        "capacity_stub took {cap_time:?}, expected {min:?}..{max:?}"
    );
}
