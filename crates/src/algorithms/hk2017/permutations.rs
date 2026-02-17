/// Generate all cyclic permutations of a set of elements.
///
/// A cyclic permutation is an equivalence class of permutations under cyclic shifts.
/// For m elements there are (m-1)! cyclic permutations.
/// Each returned Vec is a full ordering of the input elements, with `elements[0]` fixed at position 0.
///
/// We fix the first element and permute the rest, which gives exactly one
/// representative per equivalence class.
///
/// Returns a single empty permutation for empty input (consistent with 0! = 1).
/// Returns `vec![vec![x]]` for single-element input.
pub fn cyclic_permutations(elements: &[usize]) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    for_each_cyclic_permutation(elements, &mut |p| result.push(p.to_vec()));
    result
}

/// Call `callback` once for each cyclic permutation of `elements`.
///
/// Uses a single buffer (no heap allocation per permutation).
/// The callback receives a slice that is only valid during the call.
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
    // Fix buf[0], permute buf[1..] via Heap's algorithm
    heap_perms_buf(&mut buf, 1, k, callback);
}

/// Heap's algorithm on buf[offset..offset+k], calling callback with the full buf.
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
#[path = "permutations_test.rs"]
mod permutations_test;
