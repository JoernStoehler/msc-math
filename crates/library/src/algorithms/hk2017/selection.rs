//! Subset-selection utilities for exhaustive HK2017 search.

/// Generate all combinations of `k` elements from `{0, ..., n-1}` in lexicographic order.
///
/// Returns an empty vec if `k == 0` or `k > n`.
///
/// [alg:ehz]: "for each S subseteq {1, ..., F} with |S| >= 2".
pub fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    if k == 0 || k > n {
        return Vec::new();
    }
    let mut result = Vec::new();
    let mut combo = vec![0usize; k];
    combinations_rec(n, k, 0, 0, &mut combo, &mut result);
    result
}

fn combinations_rec(
    n: usize,
    k: usize,
    start: usize,
    depth: usize,
    combo: &mut [usize],
    result: &mut Vec<Vec<usize>>,
) {
    if depth == k {
        result.push(combo.to_vec());
        return;
    }
    for i in start..=(n - k + depth) {
        combo[depth] = i;
        combinations_rec(n, k, i + 1, depth + 1, combo, result);
    }
}
