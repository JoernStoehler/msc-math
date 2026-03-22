//! Cyclic permutation generation for the HK2017 enumeration.
//!
//! A cyclic permutation of m elements is an equivalence class of permutations under
//! cyclic rotation: [a, b, c] ~ [b, c, a] ~ [c, a, b]. There are (m-1)! distinct
//! cyclic permutations. We fix the first element and permute the rest via Heap's
//! algorithm, yielding exactly one representative per equivalence class.
//!
//! Two interfaces:
//! - `cyclic_permutations`: returns all permutations as `Vec<Vec<usize>>` (allocating).
//! - `for_each_cyclic_permutation`: callback-based, single-buffer, zero heap allocation
//!   per permutation (preferred for the capacity algorithm's inner loop).
//!
//! Mathematical correspondence: [alg:ehz] step "enumerate cyclic permutations of S"

/// Generate all cyclic permutations of the given elements.
///
/// Fixes `elements[0]` at position 0 and permutes the rest, yielding (m-1)!
/// distinct cyclic equivalence-class representatives.
///
/// Returns `vec![elements.to_vec()]` for 0- or 1-element input (consistent with 0! = 1).
///
/// [alg:ehz]: cyclic permutation enumeration.
pub fn cyclic_permutations(elements: &[usize]) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    for_each_cyclic_permutation(elements, &mut |p| result.push(p.to_vec()));
    result
}

/// Call `callback` once for each cyclic permutation of `elements`.
///
/// Uses a single internal buffer (no heap allocation per permutation).
/// The callback receives a slice that is valid only during the call.
///
/// [alg:ehz]: cyclic permutation enumeration (zero-alloc variant).
pub fn for_each_cyclic_permutation(
    elements: &[usize],
    callback: &mut impl FnMut(&[usize]),
) {
    if elements.len() <= 1 {
        callback(elements);
        return;
    }

    let mut buf = elements.to_vec();
    let k = buf.len() - 1;
    // Fix buf[0], permute buf[1..] via Heap's algorithm.
    heap_perms_buf(&mut buf, 1, k, callback);
}

/// Heap's algorithm on `buf[offset..offset+k]`, calling `callback` with the full buffer.
///
/// Generates all k! permutations of the sub-slice while leaving `buf[..offset]` fixed.
fn heap_perms_buf(
    buf: &mut [usize],
    offset: usize,
    k: usize,
    callback: &mut impl FnMut(&[usize]),
) {
    if k == 1 {
        callback(buf);
        return;
    }
    heap_perms_buf(buf, offset, k - 1, callback);
    for i in 0..k - 1 {
        if k.is_multiple_of(2) {
            buf.swap(offset + i, offset + k - 1);
        } else {
            buf.swap(offset, offset + k - 1);
        }
        heap_perms_buf(buf, offset, k - 1, callback);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    // Tests for permutations: cyclic permutation enumeration correctness.
    //
    // Proposition: cyclic_permutations(S) produces exactly (|S|-1)! unique cyclic
    // equivalence-class representatives, all starting with S[0].
    //
    // Strategy: exhaustive for |S| <= 5, count + uniqueness + structural checks.

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
}
