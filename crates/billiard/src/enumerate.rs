//! Block enumeration for the billiard algorithm.
//!
//! Generates all (S, σ) matching the pattern ([Q|QQ][P|PP])^k for k ∈ {2, 3}.
//!
//! See chapter-billiard.tex, Lemma 6.5 (lem:sigma-structure) and the
//! algorithm pseudocode in Section 6.4.

/// A block in a k-bounce orbit: either a single facet or an ordered adjacent pair.
#[derive(Debug, Clone)]
pub enum Block {
    /// Single facet (edge interior).
    Single(usize),
    /// Ordered pair of adjacent facets (vertex of polygon).
    /// The two facets appear in this order in σ.
    Pair(usize, usize),
}

impl Block {
    /// Return the facet indices in this block, in order.
    pub fn indices(&self) -> Vec<usize> {
        match *self {
            Block::Single(i) => vec![i],
            Block::Pair(i, j) => vec![i, j],
        }
    }

    /// Return the set of facet indices used (for overlap checking).
    pub fn facet_set(&self) -> Vec<usize> {
        self.indices()
    }
}

/// Enumerate all valid blocks for a set of facets.
///
/// A block is either:
/// - A single facet index (always valid)
/// - An ordered pair (i, j) where i and j are adjacent
///
/// For pair blocks, both orderings (i, j) and (j, i) are generated.
pub fn enumerate_blocks(facet_indices: &[usize], adj: &[Vec<bool>]) -> Vec<Block> {
    let mut blocks = Vec::new();

    // Single-facet blocks
    for &i in facet_indices {
        blocks.push(Block::Single(i));
    }

    // Pair blocks: both orderings of each adjacent pair
    for (a, &i) in facet_indices.iter().enumerate() {
        for &j in &facet_indices[a + 1..] {
            if adj[i][j] {
                blocks.push(Block::Pair(i, j));
                blocks.push(Block::Pair(j, i));
            }
        }
    }

    blocks
}

/// Check if two blocks share any facet index.
fn blocks_overlap(a: &Block, b: &Block) -> bool {
    let a_set = a.facet_set();
    let b_set = b.facet_set();
    a_set.iter().any(|i| b_set.contains(i))
}

/// Enumerate all σ sequences matching ([Q|QQ][P|PP])^k.
///
/// For each valid selection of k non-overlapping q-blocks and k non-overlapping
/// p-blocks, generates all interleavings into the alternating pattern.
///
/// The callback receives each σ as a slice of facet indices.
pub fn enumerate_k_bounce_sigmas(
    k: usize,
    q_blocks: &[Block],
    p_blocks: &[Block],
    mut callback: impl FnMut(&[usize]),
) {
    if k == 0 {
        return;
    }

    // Choose k non-overlapping q-blocks
    let q_selections = select_non_overlapping(q_blocks, k);
    let p_selections = select_non_overlapping(p_blocks, k);

    // For each combination of q-selection and p-selection,
    // generate all orderings and flatten into σ.
    //
    // Pattern: Q-block_1, P-block_1, Q-block_2, P-block_2, ..., Q-block_k, P-block_k
    //
    // We enumerate all permutations of q-blocks and p-blocks independently.
    let mut q_perm_buf = vec![0usize; k];
    let mut p_perm_buf = vec![0usize; k];

    for q_sel in &q_selections {
        for p_sel in &p_selections {
            // Generate all permutations of q-blocks
            for_each_permutation(k, &mut q_perm_buf, &mut |q_perm| {
                // Generate all permutations of p-blocks
                for_each_permutation(k, &mut p_perm_buf, &mut |p_perm| {
                    // Flatten: Q[q_perm[0]], P[p_perm[0]], Q[q_perm[1]], P[p_perm[1]], ...
                    let mut sigma = Vec::with_capacity(4 * k); // worst case: all pairs
                    for round in 0..k {
                        sigma.extend(q_sel[q_perm[round]].indices());
                        sigma.extend(p_sel[p_perm[round]].indices());
                    }
                    callback(&sigma);
                });
            });
        }
    }
}

/// Select `k` non-overlapping blocks from `blocks`.
///
/// Returns all combinations of k blocks where no two share a facet index.
fn select_non_overlapping(blocks: &[Block], k: usize) -> Vec<Vec<Block>> {
    let mut result = Vec::new();
    let mut selection = Vec::with_capacity(k);
    select_non_overlapping_rec(blocks, k, 0, &mut selection, &mut result);
    result
}

fn select_non_overlapping_rec(
    blocks: &[Block],
    k: usize,
    start: usize,
    selection: &mut Vec<Block>,
    result: &mut Vec<Vec<Block>>,
) {
    if selection.len() == k {
        result.push(selection.clone());
        return;
    }

    let remaining = k - selection.len();
    if start + remaining > blocks.len() {
        return;
    }

    for i in start..blocks.len() {
        // Check no overlap with already-selected blocks
        if selection.iter().any(|s| blocks_overlap(s, &blocks[i])) {
            continue;
        }
        selection.push(blocks[i].clone());
        select_non_overlapping_rec(blocks, k, i + 1, selection, result);
        selection.pop();
    }
}

/// Generate all permutations of {0, ..., n-1} by Heap's algorithm.
fn for_each_permutation(n: usize, buf: &mut [usize], callback: &mut impl FnMut(&[usize])) {
    // Initialize identity permutation
    for (i, slot) in buf.iter_mut().enumerate().take(n) {
        *slot = i;
    }
    heap_permute(n, buf, callback);
}

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
