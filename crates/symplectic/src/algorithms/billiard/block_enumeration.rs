//! Block enumeration for the billiard algorithm.
//!
//! Generates all sigma sequences matching the pattern ([Q|QQ][P|PP])^k for k in {2, 3}.
//! Fully lazy: no intermediate collections, constant memory per recursion depth.
//!
//! Cyclic symmetry removal: the first q-block position is fixed, reducing
//! q-permutations from k! to (k-1)!. Total reduction factor: k.
//!
//! See [lem:sigma-structure] and [alg:billiard] in the thesis.
//!
//! Mathematical correspondence: [lem:sigma-structure], [alg:billiard]

/// A block in a k-bounce orbit: either a single facet or an ordered adjacent pair.
///
/// Single blocks correspond to edge interiors of a polygon, pair blocks to vertices.
/// [lem:sigma-structure]: each "bounce" consists of one or two consecutive facets
/// of the same type (q or p).
#[derive(Debug, Clone, Copy)]
pub enum Block {
    /// Single facet (edge interior of the polygon).
    Single(usize),
    /// Ordered pair of adjacent facets (vertex of the polygon).
    /// The two facets appear in this order in sigma.
    Pair(usize, usize),
}

impl Block {
    /// Append this block's facet indices to `buf`.
    #[inline]
    fn push_to(&self, buf: &mut Vec<usize>) {
        match *self {
            Block::Single(i) => buf.push(i),
            Block::Pair(i, j) => {
                buf.push(i);
                buf.push(j);
            }
        }
    }

    /// Check if this block uses facet index `idx`.
    #[inline]
    fn contains(&self, idx: usize) -> bool {
        match *self {
            Block::Single(i) => i == idx,
            Block::Pair(i, j) => i == idx || j == idx,
        }
    }

    /// Check if this block overlaps with `other` (shares any facet index).
    #[inline]
    fn overlaps(&self, other: &Block) -> bool {
        match *other {
            Block::Single(i) => self.contains(i),
            Block::Pair(i, j) => self.contains(i) || self.contains(j),
        }
    }
}

/// Enumerate all valid blocks for a set of facets.
///
/// A block is either:
/// - A single facet index (always valid)
/// - An ordered pair (i, j) where i and j are adjacent
///
/// For pair blocks, both orderings (i, j) and (j, i) are generated.
///
/// [lem:sigma-structure]: blocks correspond to the bounce structure of billiard trajectories.
pub fn enumerate_blocks(facet_indices: &[usize], adj: &nalgebra::DMatrix<bool>) -> Vec<Block> {
    let mut blocks = Vec::new();

    // Single-facet blocks.
    for &i in facet_indices {
        blocks.push(Block::Single(i));
    }

    // Pair blocks: both orderings of each adjacent pair.
    for (a, &i) in facet_indices.iter().enumerate() {
        for &j in &facet_indices[a + 1..] {
            if adj[(i, j)] {
                blocks.push(Block::Pair(i, j));
                blocks.push(Block::Pair(j, i));
            }
        }
    }

    blocks
}

/// Enumerate all sigma sequences matching ([Q|QQ][P|PP])^k, lazily.
///
/// For each valid selection of k non-overlapping q-blocks and k non-overlapping
/// p-blocks, generates all interleavings into the alternating pattern
/// Q_1 P_1 Q_2 P_2 ... Q_k P_k.
///
/// **Cyclic symmetry removal**: the first q-block is fixed (always placed at
/// position 0), and only the remaining k-1 q-blocks are permuted. This divides
/// the total count by k without missing any distinct cyclic orbit.
///
/// The callback receives each sigma as a slice of facet indices.
///
/// [alg:billiard]: block-structured sigma enumeration for Lagrangian products.
pub fn enumerate_k_bounce_sigmas(
    k: usize,
    q_blocks: &[Block],
    p_blocks: &[Block],
    mut callback: impl FnMut(&[usize]),
) {
    if k == 0 {
        return;
    }

    let mut q_sel = Vec::with_capacity(k);
    let mut p_sel = Vec::with_capacity(k);
    let mut sigma = Vec::with_capacity(4 * k);
    let mut q_perm_buf = vec![0usize; k.saturating_sub(1)];
    let mut p_perm_buf = vec![0usize; k];

    // Enumerate non-overlapping q-block selections.
    for_each_non_overlapping(q_blocks, k, &mut q_sel, &mut |q_selection| {
        // Enumerate non-overlapping p-block selections.
        for_each_non_overlapping(p_blocks, k, &mut p_sel, &mut |p_selection| {
            if k == 1 {
                // Only one q-block and one p-block -- no permutations needed.
                sigma.clear();
                q_selection[0].push_to(&mut sigma);
                p_selection[0].push_to(&mut sigma);
                callback(&sigma);
            } else {
                // Permute remaining q-blocks (indices 1..k of q_selection).
                for_each_permutation(k - 1, &mut q_perm_buf, &mut |q_rest_perm| {
                    // Permute all p-blocks.
                    for_each_permutation(k, &mut p_perm_buf, &mut |p_perm| {
                        sigma.clear();
                        // Round 0: fixed first q-block, then p-block at p_perm[0].
                        q_selection[0].push_to(&mut sigma);
                        p_selection[p_perm[0]].push_to(&mut sigma);
                        // Rounds 1..k: permuted q-blocks, permuted p-blocks.
                        for round in 1..k {
                            q_selection[1 + q_rest_perm[round - 1]].push_to(&mut sigma);
                            p_selection[p_perm[round]].push_to(&mut sigma);
                        }
                        callback(&sigma);
                    });
                });
            }
        });
    });
}

/// Lazily enumerate k non-overlapping blocks via callback.
/// `selection` is a reusable buffer (passed in to avoid allocation).
fn for_each_non_overlapping(
    blocks: &[Block],
    k: usize,
    selection: &mut Vec<Block>,
    callback: &mut impl FnMut(&[Block]),
) {
    selection.clear();
    non_overlapping_rec(blocks, k, 0, selection, callback);
}

/// Recursive helper: choose k non-overlapping blocks from blocks[start..].
fn non_overlapping_rec(
    blocks: &[Block],
    k: usize,
    start: usize,
    selection: &mut Vec<Block>,
    callback: &mut impl FnMut(&[Block]),
) {
    if selection.len() == k {
        callback(selection);
        return;
    }

    let remaining = k - selection.len();
    if start + remaining > blocks.len() {
        return;
    }

    for i in start..blocks.len() {
        if selection.iter().any(|s| s.overlaps(&blocks[i])) {
            continue;
        }
        selection.push(blocks[i]);
        non_overlapping_rec(blocks, k, i + 1, selection, callback);
        selection.pop();
    }
}

/// Generate all permutations of {0, ..., n-1} by Heap's algorithm.
fn for_each_permutation(n: usize, buf: &mut [usize], callback: &mut impl FnMut(&[usize])) {
    for (i, slot) in buf.iter_mut().enumerate().take(n) {
        *slot = i;
    }
    heap_permute(n, buf, callback);
}

/// Heap's algorithm: generate all k! permutations of buf[..k].
fn heap_permute(k: usize, buf: &mut [usize], callback: &mut impl FnMut(&[usize])) {
    if k == 1 {
        callback(buf);
        return;
    }
    heap_permute(k - 1, buf, callback);
    for i in 0..k - 1 {
        if k.is_multiple_of(2) {
            buf.swap(i, k - 1);
        } else {
            buf.swap(0, k - 1);
        }
        heap_permute(k - 1, buf, callback);
    }
}
