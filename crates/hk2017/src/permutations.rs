/// Generate all cyclic permutations of a set of elements.
///
/// A cyclic permutation is an equivalence class of permutations under cyclic shifts.
/// For m elements there are (m-1)! cyclic permutations.
///
/// We fix the first element and permute the rest, which gives exactly one
/// representative per equivalence class.
pub fn cyclic_permutations(elements: &[usize]) -> Vec<Vec<usize>> {
    if elements.len() <= 1 {
        return vec![elements.to_vec()];
    }

    let first = elements[0];
    let rest: Vec<usize> = elements[1..].to_vec();

    let mut result = Vec::new();
    let mut perm = rest;
    let k = perm.len();
    heap_permutations(&mut perm, k, &mut |p| {
        let mut full = vec![first];
        full.extend_from_slice(p);
        result.push(full);
    });

    result
}

/// Heap's algorithm for generating all permutations.
fn heap_permutations(arr: &mut Vec<usize>, k: usize, callback: &mut impl FnMut(&[usize])) {
    if k == 1 {
        callback(arr);
        return;
    }
    heap_permutations(arr, k - 1, callback);
    for i in 0..k - 1 {
        if k.is_multiple_of(2) {
            arr.swap(i, k - 1);
        } else {
            arr.swap(0, k - 1);
        }
        heap_permutations(arr, k - 1, callback);
    }
}

#[cfg(test)]
#[path = "permutations_test.rs"]
mod permutations_test;
