use nalgebra::DMatrix;

pub(crate) fn for_each_transition_pruned_sigma(
    transition_is_allowed: &DMatrix<bool>,
    mut visit: impl FnMut(&[usize]),
) {
    let cycles = SimpleDirectedCyclesCanonical::new(transition_is_allowed);
    for sigma in cycles {
        visit(&sigma);
    }
}

pub(crate) fn for_each_product_billiard_sigma(
    q_facet_indices: &[usize],
    p_facet_indices: &[usize],
    facet_intersection_is_nonempty: &DMatrix<bool>,
    transition_is_allowed: &DMatrix<bool>,
    mut visit: impl FnMut(&[usize]),
) {
    assert_eq!(
        facet_intersection_is_nonempty.shape(),
        transition_is_allowed.shape(),
        "facet_intersection_is_nonempty and transition_is_allowed must have the same shape"
    );
    let facet_count = facet_intersection_is_nonempty.nrows();
    assert_eq!(
        facet_intersection_is_nonempty.ncols(),
        facet_count,
        "facet_intersection_is_nonempty must be square"
    );
    assert!(
        q_facet_indices
            .iter()
            .chain(p_facet_indices.iter())
            .all(|&facet| facet < facet_count),
        "q_facet_indices and p_facet_indices must index the facet matrices"
    );

    let q_blocks = enumerate_blocks(q_facet_indices, facet_intersection_is_nonempty);
    let p_blocks = enumerate_blocks(p_facet_indices, facet_intersection_is_nonempty);

    for k in 2..=3 {
        enumerate_k_bounce_sigmas(k, &q_blocks, &p_blocks, |sigma| {
            if is_feasible_cycle(sigma, transition_is_allowed) {
                visit(sigma);
            }
        });
    }
}

fn is_feasible_cycle(sigma: &[usize], transition_is_allowed: &DMatrix<bool>) -> bool {
    let m = sigma.len();
    if m == 0 {
        return true;
    }
    (0..m).all(|k| transition_is_allowed[(sigma[k], sigma[(k + 1) % m])])
}

struct SimpleDirectedCyclesCanonical<'a> {
    transition_is_allowed: &'a DMatrix<bool>,
    start: usize,
    path: Vec<usize>,
    used: Vec<bool>,
    stack: Vec<SimpleCycleFrame>,
}

#[derive(Clone, Copy, Debug)]
struct SimpleCycleFrame {
    next_candidate: usize,
}

impl<'a> SimpleDirectedCyclesCanonical<'a> {
    fn new(transition_is_allowed: &'a DMatrix<bool>) -> Self {
        assert_eq!(
            transition_is_allowed.nrows(),
            transition_is_allowed.ncols(),
            "transition_is_allowed must be square"
        );
        let facet_count = transition_is_allowed.nrows();
        let mut cycles = Self {
            transition_is_allowed,
            start: 0,
            path: Vec::with_capacity(facet_count),
            used: vec![false; facet_count],
            stack: Vec::with_capacity(facet_count),
        };
        cycles.enter_start();
        cycles
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
            if self.used[next] || !self.transition_is_allowed[(current, next)] {
                continue;
            }

            self.path.push(next);
            self.used[next] = true;
            self.stack.push(SimpleCycleFrame {
                next_candidate: self.start + 1,
            });
            if self.path.len() >= 2 && self.transition_is_allowed[(next, self.start)] {
                return Some(self.path.clone());
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Block {
    Single(usize),
    Pair(usize, usize),
}

impl Block {
    fn push_to(&self, buf: &mut Vec<usize>) {
        match *self {
            Block::Single(i) => buf.push(i),
            Block::Pair(i, j) => {
                buf.push(i);
                buf.push(j);
            }
        }
    }

    fn contains(&self, idx: usize) -> bool {
        match *self {
            Block::Single(i) => i == idx,
            Block::Pair(i, j) => i == idx || j == idx,
        }
    }

    fn overlaps(&self, other: &Block) -> bool {
        match *other {
            Block::Single(i) => self.contains(i),
            Block::Pair(i, j) => self.contains(i) || self.contains(j),
        }
    }
}

fn enumerate_blocks(
    facet_indices: &[usize],
    facet_intersection_is_nonempty: &DMatrix<bool>,
) -> Vec<Block> {
    let mut blocks = Vec::new();
    for &i in facet_indices {
        blocks.push(Block::Single(i));
    }
    for (a, &i) in facet_indices.iter().enumerate() {
        for &j in &facet_indices[a + 1..] {
            if facet_intersection_is_nonempty[(i, j)] {
                blocks.push(Block::Pair(i, j));
                blocks.push(Block::Pair(j, i));
            }
        }
    }
    blocks
}

fn enumerate_k_bounce_sigmas(
    k: usize,
    q_blocks: &[Block],
    p_blocks: &[Block],
    mut callback: impl FnMut(&[usize]),
) {
    if k == 0 {
        return;
    }

    let mut q_selection = Vec::with_capacity(k);
    let mut p_selection = Vec::with_capacity(k);
    let mut sigma = Vec::with_capacity(4 * k);
    let mut q_perm_buf = vec![0usize; k.saturating_sub(1)];
    let mut p_perm_buf = vec![0usize; k];

    for_each_non_overlapping(q_blocks, k, &mut q_selection, &mut |selected_q| {
        for_each_non_overlapping(p_blocks, k, &mut p_selection, &mut |selected_p| {
            if k == 1 {
                sigma.clear();
                selected_q[0].push_to(&mut sigma);
                selected_p[0].push_to(&mut sigma);
                callback(&sigma);
            } else {
                for_each_permutation(k - 1, &mut q_perm_buf, &mut |q_rest_perm| {
                    for_each_permutation(k, &mut p_perm_buf, &mut |p_perm| {
                        sigma.clear();
                        selected_q[0].push_to(&mut sigma);
                        selected_p[p_perm[0]].push_to(&mut sigma);
                        for round in 1..k {
                            selected_q[1 + q_rest_perm[round - 1]].push_to(&mut sigma);
                            selected_p[p_perm[round]].push_to(&mut sigma);
                        }
                        callback(&sigma);
                    });
                });
            }
        });
    });
}

fn for_each_non_overlapping(
    blocks: &[Block],
    k: usize,
    selection: &mut Vec<Block>,
    callback: &mut impl FnMut(&[Block]),
) {
    selection.clear();
    non_overlapping_rec(blocks, k, 0, selection, callback);
}

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

fn for_each_permutation(n: usize, buf: &mut [usize], callback: &mut impl FnMut(&[usize])) {
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
