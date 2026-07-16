//! Combinatorial word enumeration for the flow-graph search.

use nalgebra::DMatrix;
use std::collections::HashSet;

/// Number of flow segments in a tube word.
///
/// A word `[s0, s1, ..., s{k+1}]` has `+k`. Words of length 0 or 1 are not
/// tube words and return `None`.
pub fn plus_depth(word: &[usize]) -> Option<usize> {
    word.len().checked_sub(2)
}

/// Half-cache depth for a polytope with `facet_count` facets.
pub fn half_cache_depth(facet_count: usize) -> usize {
    facet_count.div_ceil(2)
}

/// True when every consecutive transition in `word` is allowed.
pub fn word_has_allowed_transitions(word: &[usize], transition_is_allowed: &DMatrix<bool>) -> bool {
    assert_eq!(
        transition_is_allowed.nrows(),
        transition_is_allowed.ncols(),
        "transition_is_allowed must be square"
    );
    word.windows(2)
        .all(|edge| transition_is_allowed[(edge[0], edge[1])])
}

/// Prefix words that can still be completed to a simple closed facet word.
///
/// This encodes Jörn's current search rule:
/// - all-distinct words are allowed;
/// - `[a,b,...,a]` is allowed, and has only one closure route left;
/// - `[a,b,...,a,b]` is allowed and is closed;
/// - `[a,b,...,a,c]` and longer variants are not allowed.
pub fn is_simple_closable_word(word: &[usize]) -> bool {
    if word.len() < 3 || word[0] == word[1] {
        return false;
    }

    if all_distinct(word) {
        return true;
    }

    let first = word[0];
    let second = word[1];
    if word.last() == Some(&first) {
        return all_distinct(&word[..word.len() - 1]) && !word[1..word.len() - 1].contains(&first);
    }

    if word.len() >= 4 && word[word.len() - 2] == first && word[word.len() - 1] == second {
        return all_distinct(&word[..word.len() - 2])
            && !word[2..word.len() - 2].contains(&first)
            && !word[2..word.len() - 2].contains(&second);
    }

    false
}

pub(crate) fn all_distinct(values: &[usize]) -> bool {
    let mut seen = HashSet::with_capacity(values.len());
    values.iter().all(|value| seen.insert(*value))
}

/// A cached combinatorial tube word before geometric tube data is attached.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CachedWord {
    pub facets: Vec<usize>,
}

impl CachedWord {
    pub fn plus_depth(&self) -> usize {
        plus_depth(&self.facets).expect("cached words have length at least 3")
    }
}

/// Enumerate transition-pruned words through `max_plus_depth`.
///
/// This is the combinatorial half-cache schedule. Geometry later decides which
/// of these words have nonempty tubes under the active action cutoff.
pub fn enumerate_transition_pruned_words(
    transition_is_allowed: &DMatrix<bool>,
    max_plus_depth: usize,
) -> Vec<CachedWord> {
    assert_eq!(
        transition_is_allowed.nrows(),
        transition_is_allowed.ncols(),
        "transition_is_allowed must be square"
    );
    let facet_count = transition_is_allowed.nrows();
    let mut words = Vec::new();

    for start in 0..facet_count {
        for second in 0..facet_count {
            if start == second || !transition_is_allowed[(start, second)] {
                continue;
            }
            let mut word = vec![start, second];
            extend_transition_pruned_words(
                transition_is_allowed,
                max_plus_depth,
                &mut word,
                &mut words,
            );
        }
    }

    words
}

fn extend_transition_pruned_words(
    transition_is_allowed: &DMatrix<bool>,
    max_plus_depth: usize,
    word: &mut Vec<usize>,
    words: &mut Vec<CachedWord>,
) {
    let Some(current_plus) = plus_depth(word) else {
        return;
    };
    if current_plus >= max_plus_depth {
        return;
    }

    let facet_count = transition_is_allowed.nrows();
    let last = *word.last().expect("word has start pair");
    for next in 0..facet_count {
        if !transition_is_allowed[(last, next)] {
            continue;
        }
        word.push(next);
        if is_simple_closable_word(word) {
            words.push(CachedWord {
                facets: word.clone(),
            });
            extend_transition_pruned_words(transition_is_allowed, max_plus_depth, word, words);
        }
        word.pop();
    }
}

/// Counts by plus depth, indexed so `counts[k]` is the number of `+k` words.
pub fn counts_by_plus_depth(words: &[CachedWord], max_plus_depth: usize) -> Vec<usize> {
    let mut counts = vec![0usize; max_plus_depth + 1];
    for word in words {
        let depth = word.plus_depth();
        if depth <= max_plus_depth {
            counts[depth] += 1;
        }
    }
    counts
}

/// True when `word` is represented in the half-cache set.
pub fn cached_words_contain(cache: &[CachedWord], word: &[usize]) -> bool {
    cache.iter().any(|cached| cached.facets == word)
}

/// Return one valid split of a simple closed raw word into two half-cache words.
///
/// The input is the raw closed tube word `[s0,s1,...,s{m-1},s0,s1]`.
/// The returned pair concatenates along its shared two-face to recover `word`.
pub fn split_closed_word_into_half_words(
    word: &[usize],
    half_depth: usize,
) -> Option<(Vec<usize>, Vec<usize>)> {
    let total_plus = plus_depth(word)?;
    if total_plus == 0 || word.len() < 4 {
        return None;
    }
    if word[word.len() - 2] != word[0] || word[word.len() - 1] != word[1] {
        return None;
    }

    for left_plus in 1..total_plus {
        let right_plus = total_plus - left_plus;
        if left_plus > half_depth || right_plus > half_depth {
            continue;
        }
        let left_len = left_plus + 2;
        let right_start = left_len - 2;
        let left = word[..left_len].to_vec();
        let right = word[right_start..].to_vec();
        if is_simple_closable_word(&left) && is_simple_closable_word(&right) {
            return Some((left, right));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::hk2017::for_each_sigma_pruned_by_transition;
    use nalgebra::DMatrix;

    fn complete_transition_matrix(facet_count: usize) -> DMatrix<bool> {
        DMatrix::from_fn(facet_count, facet_count, |i, j| i != j)
    }

    fn closed_raw_word(sigma: &[usize]) -> Vec<usize> {
        let mut word = sigma.to_vec();
        word.push(sigma[0]);
        word.push(sigma[1]);
        word
    }

    #[test]
    fn simple_closable_word_accepts_exactly_the_prefix_shapes_we_use() {
        assert!(is_simple_closable_word(&[0, 1, 2]));
        assert!(is_simple_closable_word(&[0, 1, 2, 3]));
        assert!(is_simple_closable_word(&[0, 1, 2, 0]));
        assert!(is_simple_closable_word(&[0, 1, 2, 0, 1]));

        assert!(!is_simple_closable_word(&[0]));
        assert!(!is_simple_closable_word(&[0, 1]));
        assert!(!is_simple_closable_word(&[0, 0, 1]));
        assert!(!is_simple_closable_word(&[0, 1, 2, 0, 3]));
        assert!(!is_simple_closable_word(&[0, 1, 2, 0, 3, 4]));
        assert!(!is_simple_closable_word(&[0, 1, 2, 1]));
    }

    #[test]
    fn complete_graph_counts_include_closure_special_prefixes() {
        let transition = complete_transition_matrix(5);
        let words = enumerate_transition_pruned_words(&transition, 2);
        let counts = counts_by_plus_depth(&words, 2);

        assert_eq!(counts[1], 5 * 4 * 4);
        assert_eq!(counts[2], 5 * 4 * (3 * 3 + 1));
    }

    #[test]
    fn half_cache_splits_every_transition_pruned_closed_word_on_complete_graphs() {
        for facet_count in 5..=8 {
            let transition = complete_transition_matrix(facet_count);
            let half_depth = half_cache_depth(facet_count);
            let cache = enumerate_transition_pruned_words(&transition, half_depth);

            let mut missing = Vec::new();
            for_each_sigma_pruned_by_transition(&transition, |sigma| {
                let closed = closed_raw_word(sigma);
                let Some((left, right)) = split_closed_word_into_half_words(&closed, half_depth)
                else {
                    missing.push(closed);
                    return;
                };
                if !cached_words_contain(&cache, &left) || !cached_words_contain(&cache, &right) {
                    missing.push(closed);
                }
            });

            assert!(
                missing.is_empty(),
                "missing half-cache split for F={facet_count}: {missing:?}"
            );
        }
    }

    #[test]
    fn half_cache_splits_every_transition_pruned_closed_word_on_sparse_graph() {
        let transition = DMatrix::from_row_slice(
            6,
            6,
            &[
                false, true, false, false, true, false, //
                false, false, true, false, false, true, //
                true, false, false, true, false, false, //
                false, true, false, false, true, false, //
                false, false, true, false, false, true, //
                true, false, false, true, false, false, //
            ],
        );
        let half_depth = half_cache_depth(6);
        let cache = enumerate_transition_pruned_words(&transition, half_depth);

        let mut checked = 0usize;
        for_each_sigma_pruned_by_transition(&transition, |sigma| {
            checked += 1;
            let closed = closed_raw_word(sigma);
            assert!(word_has_allowed_transitions(&closed, &transition));
            let (left, right) = split_closed_word_into_half_words(&closed, half_depth)
                .expect("transition-pruned closed word should split");
            assert!(cached_words_contain(&cache, &left), "left={left:?}");
            assert!(cached_words_contain(&cache, &right), "right={right:?}");
        });
        assert!(checked > 0);
    }

    #[test]
    fn transition_pruned_words_never_use_forbidden_edges() {
        let transition = DMatrix::from_row_slice(
            4,
            4,
            &[
                false, true, false, false, //
                false, false, true, false, //
                true, false, false, true, //
                false, false, false, false, //
            ],
        );
        let words = enumerate_transition_pruned_words(&transition, half_cache_depth(4));
        assert!(!words.is_empty());
        for word in words {
            assert!(word_has_allowed_transitions(&word.facets, &transition));
        }
    }
}
