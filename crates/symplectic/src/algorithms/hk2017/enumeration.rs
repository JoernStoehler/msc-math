//! Sigma traversal for HK2017.

use nalgebra::DMatrix;
use tracing::{info, Level};

use super::combinatorics::combinations;
use super::permutations::for_each_cyclic_permutation;

/// Visit every active-word HK2017 candidate for a flat facet count, without
/// transition pruning.
pub fn for_each_sigma_unpruned_facet_count(facet_count: usize, mut visit: impl FnMut(&[usize])) {
    for_each_sigma_impl(facet_count, &mut visit)
}

/// Visit every active-word HK2017 candidate that survives a flat directed
/// transition matrix.
pub fn for_each_sigma_pruned_by_transition(
    transition_is_allowed: &DMatrix<bool>,
    mut visit: impl FnMut(&[usize]),
) {
    for_each_simple_directed_cycle_canonical(transition_is_allowed, &mut visit)
}

fn for_each_sigma_impl(facet_count: usize, visit: &mut dyn FnMut(&[usize])) {
    if !tracing::enabled!(Level::INFO) {
        for m in 2..=facet_count {
            for subset in combinations(facet_count, m) {
                for_each_cyclic_permutation(&subset, &mut |perm| visit(perm));
            }
        }
        return;
    }

    let mut subset_count = 0u64;
    let mut cyclic_permutation_count = 0u64;
    let mut emitted_sigmas = 0u64;
    let mut emitted_by_len = vec![0u64; facet_count + 1];
    for m in 2..=facet_count {
        for subset in combinations(facet_count, m) {
            subset_count += 1;
            for_each_cyclic_permutation(&subset, &mut |perm| {
                cyclic_permutation_count += 1;
                emitted_sigmas += 1;
                emitted_by_len[perm.len()] += 1;
                visit(perm);
            });
        }
    }
    info!(
        facet_count,
        subset_count,
        cyclic_permutation_count,
        emitted_sigmas,
        emitted_by_len = ?emitted_by_len,
        "hk2017_unpruned_enumeration_summary"
    );
}

/// Visit each simple directed cycle exactly once, using active traversal order.
///
/// Cyclic rotations represent the same active word. The canonical
/// representative starts at the smallest facet index in the cycle. During DFS,
/// every later vertex must therefore be larger than the start vertex.
///
/// Cycles have length at least two. Diagonal entries in `transition_is_allowed`
/// are ignored.
pub fn for_each_simple_directed_cycle_canonical(
    transition_is_allowed: &DMatrix<bool>,
    mut visit: impl FnMut(&[usize]),
) {
    let mut cycles = SimpleDirectedCyclesCanonical::new(transition_is_allowed);
    for sigma in cycles.by_ref() {
        visit(&sigma);
    }
    if tracing::enabled!(Level::INFO) {
        cycles.emit_trace_summary();
    }
}

/// Iterator over simple directed cycles in canonical active-word form.
///
/// Each yielded cycle is a `Vec<usize>` whose first entry is the smallest facet
/// index in that cycle. Cycles have length at least two.
pub struct SimpleDirectedCyclesCanonical<'a> {
    transition_is_allowed: &'a DMatrix<bool>,
    start: usize,
    path: Vec<usize>,
    used: Vec<bool>,
    stack: Vec<SimpleCycleFrame>,
    stats: CycleEnumerationStats,
}

#[derive(Clone, Copy, Debug)]
struct SimpleCycleFrame {
    next_candidate: usize,
}

impl<'a> SimpleDirectedCyclesCanonical<'a> {
    pub fn new(transition_is_allowed: &'a DMatrix<bool>) -> Self {
        assert_eq!(
            transition_is_allowed.nrows(),
            transition_is_allowed.ncols(),
            "transition_is_allowed must be square"
        );
        let facet_count = transition_is_allowed.nrows();
        let mut iterator = Self {
            transition_is_allowed,
            start: 0,
            path: Vec::with_capacity(facet_count),
            used: vec![false; facet_count],
            stack: Vec::with_capacity(facet_count),
            stats: CycleEnumerationStats::new(facet_count),
        };
        iterator.enter_start();
        iterator
    }

    fn enter_start(&mut self) {
        self.path.clear();
        self.stack.clear();
        if self.start < self.transition_is_allowed.nrows() {
            self.path.push(self.start);
            self.used[self.start] = true;
            self.stack.push(SimpleCycleFrame {
                next_candidate: self.start + 1,
            });
            self.stats.record_prefix();
        }
    }

    fn leave_start(&mut self) {
        if let Some(&start) = self.path.first() {
            self.used[start] = false;
        }
        self.path.clear();
        self.stack.clear();
        self.start += 1;
        self.enter_start();
    }

    pub fn dfs_prefix_count(&self) -> u64 {
        self.stats.dfs_prefix_count
    }

    pub fn edge_rejections(&self) -> u64 {
        self.stats.edge_rejections
    }

    pub fn emitted_cycles(&self) -> u64 {
        self.stats.emitted_sigmas
    }

    pub fn emitted_by_len(&self) -> &[u64] {
        &self.stats.emitted_by_len
    }

    pub fn emit_trace_summary(&self) {
        if tracing::enabled!(Level::INFO) {
            info!(
                facet_count = self.transition_is_allowed.nrows(),
                dfs_prefix_count = self.stats.dfs_prefix_count,
                edge_rejections = self.stats.edge_rejections,
                emitted_sigmas = self.stats.emitted_sigmas,
                emitted_by_len = ?self.stats.emitted_by_len,
                "hk2017_directed_cycle_summary"
            );
        }
    }
}

impl Iterator for SimpleDirectedCyclesCanonical<'_> {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.stack.is_empty() {
                if self.start >= self.transition_is_allowed.nrows() {
                    return None;
                }
                self.leave_start();
                continue;
            }

            let current = *self
                .path
                .last()
                .expect("path is nonempty while stack is nonempty");
            let frame = self.stack.last_mut().expect("stack is nonempty");
            if frame.next_candidate >= self.transition_is_allowed.nrows() {
                self.stack.pop();
                let removed = self.path.pop().expect("path has a frame");
                self.used[removed] = false;
                continue;
            }

            let next = frame.next_candidate;
            frame.next_candidate += 1;
            if self.used[next] {
                continue;
            }
            if !self.transition_is_allowed[(current, next)] {
                self.stats.record_edge_rejection();
                continue;
            }

            self.path.push(next);
            self.used[next] = true;
            // Restart at `start + 1` because `start` is the canonical minimum
            // vertex; `used` prevents repeats.
            self.stack.push(SimpleCycleFrame {
                next_candidate: self.start + 1,
            });
            self.stats.record_prefix();
            if self.path.len() >= 2 && self.transition_is_allowed[(next, self.start)] {
                self.stats.record_emitted_cycle(self.path.len());
                return Some(self.path.clone());
            }
        }
    }
}

struct CycleEnumerationStats {
    dfs_prefix_count: u64,
    edge_rejections: u64,
    emitted_sigmas: u64,
    emitted_by_len: Vec<u64>,
}

impl CycleEnumerationStats {
    fn new(facet_count: usize) -> Self {
        Self {
            dfs_prefix_count: 0,
            edge_rejections: 0,
            emitted_sigmas: 0,
            emitted_by_len: vec![0; facet_count + 1],
        }
    }

    fn record_prefix(&mut self) {
        self.dfs_prefix_count += 1;
    }

    fn record_edge_rejection(&mut self) {
        self.edge_rejections += 1;
    }

    fn record_emitted_cycle(&mut self, cycle_len: usize) {
        self.emitted_sigmas += 1;
        self.emitted_by_len[cycle_len] += 1;
    }
}
