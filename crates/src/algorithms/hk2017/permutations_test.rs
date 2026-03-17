//! Tests for permutations: cyclic permutation enumeration correctness.
//!
//! Proposition: cyclic_permutations(S) produces exactly (|S|-1)! unique cyclic
//! equivalence-class representatives, all starting with S[0].
//!
//! Strategy: exhaustive for |S| <= 5, count + uniqueness + structural checks.

use super::permutations::{cyclic_permutations, for_each_cyclic_permutation};
use std::collections::HashSet;

#[test]
fn cyclic_perms_of_3_elements() {
    // {0, 1, 2}: fix 0, permute {1, 2} -> 2! = 2 permutations.
    let perms = cyclic_permutations(&[0, 1, 2]);
    assert_eq!(perms.len(), 2);
    assert!(perms.contains(&vec![0, 1, 2]));
    assert!(perms.contains(&vec![0, 2, 1]));
}

#[test]
fn cyclic_perms_of_4_elements() {
    // {0, 1, 2, 3}: fix 0, permute {1, 2, 3} -> 3! = 6 permutations.
    let perms = cyclic_permutations(&[0, 1, 2, 3]);
    assert_eq!(perms.len(), 6);
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

#[test]
fn cyclic_perms_of_empty() {
    let perms = cyclic_permutations(&[]);
    assert_eq!(perms.len(), 1);
    assert!(perms[0].is_empty());
}

#[test]
fn cyclic_perms_of_5_elements_count() {
    // 4! = 24 cyclic permutations.
    let perms = cyclic_permutations(&[10, 20, 30, 40, 50]);
    assert_eq!(perms.len(), 24);
    for p in &perms {
        assert_eq!(p[0], 10);
        assert_eq!(p.len(), 5);
    }
}

#[test]
fn all_permutations_unique() {
    let perms = cyclic_permutations(&[0, 1, 2, 3, 4]);
    let as_set: HashSet<Vec<usize>> = perms.iter().cloned().collect();
    assert_eq!(
        as_set.len(),
        perms.len(),
        "duplicate permutations detected"
    );
}

#[test]
fn each_permutation_is_valid() {
    // Every permutation must contain exactly the input elements (as a set).
    let elements = vec![2, 5, 8, 11];
    let perms = cyclic_permutations(&elements);
    let expected_set: HashSet<usize> = elements.iter().copied().collect();
    for p in &perms {
        let got_set: HashSet<usize> = p.iter().copied().collect();
        assert_eq!(got_set, expected_set, "permutation {:?} has wrong elements", p);
    }
}

#[test]
fn no_cyclic_duplicates() {
    // No two output permutations should be cyclic rotations of each other.
    let perms = cyclic_permutations(&[0, 1, 2, 3]);
    for (i, a) in perms.iter().enumerate() {
        for (j, b) in perms.iter().enumerate() {
            if i >= j {
                continue;
            }
            let is_rotation = (0..a.len()).any(|shift| {
                (0..a.len()).all(|k| a[(k + shift) % a.len()] == b[k])
            });
            assert!(
                !is_rotation,
                "permutations {:?} and {:?} are cyclic rotations of each other",
                a, b
            );
        }
    }
}

#[test]
fn callback_matches_allocating() {
    // for_each_cyclic_permutation and cyclic_permutations must produce the same output.
    let elements = [1, 3, 5, 7];
    let allocating = cyclic_permutations(&elements);
    let mut callback_results = Vec::new();
    for_each_cyclic_permutation(&elements, &mut |p| callback_results.push(p.to_vec()));
    assert_eq!(allocating, callback_results);
}
