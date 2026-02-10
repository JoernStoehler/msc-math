use super::*;

#[test]
fn cyclic_perms_of_3_elements() {
    // {0, 1, 2}: fix 0, permute {1, 2} → 2! = 2 permutations
    let perms = cyclic_permutations(&[0, 1, 2]);
    assert_eq!(perms.len(), 2);
    // Should get [0, 1, 2] and [0, 2, 1]
    assert!(perms.contains(&vec![0, 1, 2]));
    assert!(perms.contains(&vec![0, 2, 1]));
}

#[test]
fn cyclic_perms_of_4_elements() {
    // {0, 1, 2, 3}: fix 0, permute {1, 2, 3} → 3! = 6 permutations
    let perms = cyclic_permutations(&[0, 1, 2, 3]);
    assert_eq!(perms.len(), 6);
    // All should start with 0
    for p in &perms {
        assert_eq!(p[0], 0);
    }
}

#[test]
fn cyclic_perms_of_2_elements() {
    let perms = cyclic_permutations(&[3, 7]);
    assert_eq!(perms.len(), 1);
    assert_eq!(perms[0], vec![3, 7]);
}

#[test]
fn cyclic_perms_of_1_element() {
    let perms = cyclic_permutations(&[5]);
    assert_eq!(perms.len(), 1);
    assert_eq!(perms[0], vec![5]);
}
