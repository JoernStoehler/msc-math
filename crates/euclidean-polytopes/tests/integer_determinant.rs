use euclidean_polytopes::determinant_4d_integer;
use num_bigint::BigInt;
use proptest::prelude::*;

// Independent definition using all 24 permutations, rather than cofactors.
fn determinant_by_permutations(rows: &[[BigInt; 4]; 4]) -> BigInt {
    let mut result = BigInt::from(0);
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                for l in 0..4 {
                    let columns = [i, j, k, l];
                    if (0..4).any(|a| (a + 1..4).any(|b| columns[a] == columns[b])) {
                        continue;
                    }
                    let inversions = (0..4)
                        .map(|a| (a + 1..4).filter(|&b| columns[a] > columns[b]).count())
                        .sum::<usize>();
                    let term = &rows[0][i] * &rows[1][j] * &rows[2][k] * &rows[3][l];
                    if inversions % 2 == 0 {
                        result += term;
                    } else {
                        result -= term;
                    }
                }
            }
        }
    }
    result
}

#[test]
fn determinant_preserves_orientation_and_handles_singular_and_large_inputs() {
    let mut rows = [
        [2, -3, 5, 7],
        [11, 13, -17, 19],
        [-23, 29, 31, -37],
        [41, -43, 47, 53],
    ]
    .map(|row| row.map(BigInt::from));
    let original = determinant_by_permutations(&rows);
    assert_eq!(determinant_4d_integer(&rows), original);
    rows.swap(0, 1);
    assert_eq!(determinant_4d_integer(&rows), -original);
    rows[2] = rows[1].clone();
    assert_eq!(determinant_4d_integer(&rows), BigInt::from(0));

    let large = BigInt::from(1) << 100_usize;
    let diagonal = std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            if i == j {
                large.clone()
            } else {
                BigInt::from(0)
            }
        })
    });
    assert_eq!(determinant_4d_integer(&diagonal), large.pow(4));
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]
    #[test]
    fn determinant_agrees_with_permutation_definition(entries in prop::array::uniform16(-100_i64..=100)) {
        let rows = std::array::from_fn(|i| std::array::from_fn(|j| BigInt::from(entries[4 * i + j])));
        prop_assert_eq!(determinant_4d_integer(&rows), determinant_by_permutations(&rows));
    }
}
