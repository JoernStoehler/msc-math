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
