//! Certified fast path for the exact facet-rank validation in `geometry`.

use algebraic_numbers::rank;
use nalgebra::{DMatrix, Vector4};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use std::cmp::Ordering;

const COORDINATE_TRIPLES: [[usize; 3]; 4] = [[1, 2, 3], [0, 2, 3], [0, 1, 3], [0, 1, 2]];

pub(super) fn affine_rank_at_least_three_exact(points: &[Vector4<BigRational>]) -> bool {
    if points.len() < 4 {
        return false;
    }
    if certified_affine_rank_at_least_three(points) {
        return true;
    }

    let origin = &points[0];
    let differences = DMatrix::from_fn(points.len() - 1, 4, |row, coordinate| {
        points[row + 1][coordinate].clone() - origin[coordinate].clone()
    });
    rank(&differences) >= 3
}

/// One-sided f64 filter for affine rank at least three.
///
/// Every coordinate interval encloses the exact rational vertex coordinate.
/// Therefore a 3x3 determinant interval excluding zero certifies a nonzero
/// exact minor. Failure to certify is not a rank verdict; the caller falls
/// back to exact row reduction.
fn certified_affine_rank_at_least_three(points: &[Vector4<BigRational>]) -> bool {
    let Some(interval_points) = points
        .iter()
        .map(|point| {
            point
                .iter()
                .map(rational_interval)
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>()
    else {
        return false;
    };
    let origin = &interval_points[0];
    let differences = interval_points[1..]
        .iter()
        .map(|point| {
            (0..4)
                .map(|coordinate| point[coordinate].sub(origin[coordinate]))
                .collect::<Option<Vec<_>>>()
        })
        .collect::<Option<Vec<_>>>();
    let Some(differences) = differences else {
        return false;
    };

    for i in 0..differences.len() - 2 {
        for j in i + 1..differences.len() - 1 {
            for k in j + 1..differences.len() {
                for coordinates in COORDINATE_TRIPLES {
                    let rows = [
                        differences[i].as_slice(),
                        differences[j].as_slice(),
                        differences[k].as_slice(),
                    ];
                    if determinant3_interval(rows, coordinates)
                        .is_some_and(RankInterval::excludes_zero)
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[derive(Clone, Copy)]
struct RankInterval {
    lo: f64,
    hi: f64,
}

impl RankInterval {
    fn add(self, rhs: Self) -> Option<Self> {
        outward_interval(checked_add(self.lo, rhs.lo)?, checked_add(self.hi, rhs.hi)?)
    }

    fn neg(self) -> Self {
        Self {
            lo: -self.hi,
            hi: -self.lo,
        }
    }

    fn sub(self, rhs: Self) -> Option<Self> {
        self.add(rhs.neg())
    }

    fn mul(self, rhs: Self) -> Option<Self> {
        let products = [
            checked_mul(self.lo, rhs.lo)?,
            checked_mul(self.lo, rhs.hi)?,
            checked_mul(self.hi, rhs.lo)?,
            checked_mul(self.hi, rhs.hi)?,
        ];
        outward_interval(
            products.iter().copied().fold(f64::INFINITY, f64::min),
            products.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        )
    }

    fn excludes_zero(self) -> bool {
        self.lo > 0.0 || self.hi < 0.0
    }
}

fn rational_interval(value: &BigRational) -> Option<RankInterval> {
    let rounded = value.to_f64()?;
    if !normal_or_zero(rounded) {
        return None;
    }
    let rounded_exact = BigRational::from_float(rounded)?;
    Some(match rounded_exact.cmp(value) {
        Ordering::Less => RankInterval {
            lo: rounded,
            hi: rounded.next_up(),
        },
        Ordering::Equal => RankInterval {
            lo: rounded,
            hi: rounded,
        },
        Ordering::Greater => RankInterval {
            lo: rounded.next_down(),
            hi: rounded,
        },
    })
}

fn determinant3_interval(
    rows: [&[RankInterval]; 3],
    coordinates: [usize; 3],
) -> Option<RankInterval> {
    let first = rows[1][coordinates[1]]
        .mul(rows[2][coordinates[2]])?
        .sub(rows[1][coordinates[2]].mul(rows[2][coordinates[1]])?)?;
    let second = rows[1][coordinates[0]]
        .mul(rows[2][coordinates[2]])?
        .sub(rows[1][coordinates[2]].mul(rows[2][coordinates[0]])?)?;
    let third = rows[1][coordinates[0]]
        .mul(rows[2][coordinates[1]])?
        .sub(rows[1][coordinates[1]].mul(rows[2][coordinates[0]])?)?;
    rows[0][coordinates[0]]
        .mul(first)?
        .sub(rows[0][coordinates[1]].mul(second)?)?
        .add(rows[0][coordinates[2]].mul(third)?)
}

fn outward_interval(lo: f64, hi: f64) -> Option<RankInterval> {
    if !lo.is_finite() || !hi.is_finite() || lo > hi {
        return None;
    }
    Some(RankInterval {
        lo: lo.next_down(),
        hi: hi.next_up(),
    })
}

fn checked_add(left: f64, right: f64) -> Option<f64> {
    if !normal_or_zero(left) || !normal_or_zero(right) {
        return None;
    }
    let result = left + right;
    if !normal_or_zero(result) || (result == 0.0 && left != -right) {
        return None;
    }
    Some(result)
}

fn checked_mul(left: f64, right: f64) -> Option<f64> {
    if !normal_or_zero(left) || !normal_or_zero(right) {
        return None;
    }
    let result = left * right;
    if !normal_or_zero(result) || (result == 0.0 && left != 0.0 && right != 0.0) {
        return None;
    }
    Some(result)
}

fn normal_or_zero(value: f64) -> bool {
    value == 0.0 || (value.is_finite() && value.is_normal())
}

#[cfg(test)]
mod tests {
    use super::certified_affine_rank_at_least_three;
    use algebraic_numbers::rank;
    use nalgebra::{DMatrix, Vector4};
    use num_rational::BigRational;
    use proptest::prelude::*;

    fn q(numerator: i64, denominator: i64) -> BigRational {
        BigRational::new(numerator.into(), denominator.into())
    }

    fn moment_curve_point(t: BigRational) -> Vector4<BigRational> {
        let t2 = t.clone() * t.clone();
        let t3 = t2.clone() * t.clone();
        let t4 = t3.clone() * t.clone();
        Vector4::new(t, t2, t3, t4)
    }

    #[test]
    fn filter_certifies_a_well_conditioned_nondyadic_minor() {
        let points = vec![
            moment_curve_point(q(1, 3)),
            moment_curve_point(q(2, 3)),
            moment_curve_point(q(4, 3)),
            moment_curve_point(q(8, 3)),
        ];
        assert!(certified_affine_rank_at_least_three(&points));
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn certified_filter_never_claims_a_zero_exact_minor(
            coordinates in proptest::collection::vec((-20_i64..=20, 1_i64..=9), 16..=32),
        ) {
            let points = coordinates
                .chunks_exact(4)
                .map(|coordinate| {
                    Vector4::new(
                        BigRational::new(coordinate[0].0.into(), coordinate[0].1.into()),
                        BigRational::new(coordinate[1].0.into(), coordinate[1].1.into()),
                        BigRational::new(coordinate[2].0.into(), coordinate[2].1.into()),
                        BigRational::new(coordinate[3].0.into(), coordinate[3].1.into()),
                    )
                })
                .collect::<Vec<_>>();

            if certified_affine_rank_at_least_three(&points) {
                let differences = DMatrix::from_fn(points.len() - 1, 4, |row, coordinate| {
                    points[row + 1][coordinate].clone() - points[0][coordinate].clone()
                });
                prop_assert!(rank(&differences) >= 3);
            }
        }
    }
}
