// ── Focused theorem-to-code regression tests ─────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cyclic_embedding_accepts_rotations_and_rejects_reversal() {
        let source = Obstruction {
            labels: vec![0, 2, 1],
            mask: label_mask(&[0, 2, 1]),
        };
        assert!(contains_certified_subword(
            &[3, 0, 4, 2, 5, 1],
            std::slice::from_ref(&source)
        ));
        assert!(contains_certified_subword(
            &[2, 5, 1, 3, 0, 4],
            std::slice::from_ref(&source)
        ));
        assert!(!contains_certified_subword(
            &[3, 0, 4, 1, 5, 2],
            std::slice::from_ref(&source)
        ));
    }

    #[test]
    fn nextafter_helpers_enclose_zero() {
        assert!(next_down(0.0) < 0.0);
        assert!(next_up(0.0) > 0.0);
        let cancellation = Interval::point(0.1).sub(Interval::point(0.1));
        assert!(cancellation.lo <= 0.0 && cancellation.hi >= 0.0);
    }

    #[test]
    fn interval_determinant_has_expected_controls() {
        let identity = (0..3)
            .map(|row| {
                (0..3)
                    .map(|col| Interval::point(usize::from(row == col) as f64))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let determinant = interval_determinant(&identity);
        assert!(determinant.lo <= 1.0 && determinant.hi >= 1.0);

        let singular = vec![
            vec![Interval::point(1.0), Interval::point(2.0)],
            vec![Interval::point(1.0), Interval::point(2.0)],
        ];
        let determinant = interval_determinant(&singular);
        assert!(determinant.lo <= 0.0 && determinant.hi >= 0.0);
    }

    #[test]
    fn batched_rounding_envelope_contains_exact_matrix_products() {
        assert!(
            gradual_underflow_available(),
            "the supported development target must preserve subnormals"
        );
        let left = DMatrix::from_fn(6, 6, |row, col| {
            let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
            sign * ((row + 1) as f64) * 10.0_f64.powi(col as i32 - 3)
        });
        let right = DMatrix::from_fn(6, 3, |row, col| {
            let sign = if (2 * row + col) % 3 == 0 { -1.0 } else { 1.0 };
            sign * ((col + 2) as f64) * 10.0_f64.powi(2 - row as i32)
        });
        assert_product_rounding_enclosed(&left, &right);
        assert_product_then_subtraction_enclosed(&left, &right, |row, col| {
            usize::from(row == col) as f64
        });

        // One-column output exercises nalgebra's non-GEMM matrix-vector path.
        let vector = DMatrix::from_fn(6, 1, |row, _| {
            (-1.0_f64).powi(row as i32) * 10.0_f64.powi(row as i32 - 3)
        });
        assert_product_rounding_enclosed(&left, &vector);
        assert_product_then_subtraction_enclosed(&left, &vector, |row, _| {
            usize::from(row == 5) as f64
        });

        let minimum_subnormal = f64::from_bits(1);
        let underflow_left =
            DMatrix::from_row_slice(2, 2, &[minimum_subnormal, 0.0, 0.0, minimum_subnormal]);
        let underflow_right = DMatrix::from_element(2, 1, 0.5);
        assert_product_rounding_enclosed(&underflow_left, &underflow_right);
    }

    #[test]
    fn capacity_interval_uses_maximum_q_endpoints() {
        let candidate = |q_lower, q_upper| Decision {
            kind: DecisionKind::Accept,
            action: Some(1.0),
            beta_radius: Some(0.0),
            q_radius: Some(0.0),
            q_lower: Some(q_lower),
            q_upper: Some(q_upper),
            exact_fallback: false,
        };
        let decisions = [candidate(2.0, 3.0), candidate(4.0, 5.0)];
        let mut stats = RouteStats::default();
        record_case_capacity_interval(&mut stats, &decisions, GuardKind::BatchedAnalyticEnvelope);
        let (lower, upper) = stats
            .best_action_lower
            .zip(stats.best_action_upper)
            .expect("positive Q intervals produce an action interval");
        assert!(lower <= 0.1 && 0.1 <= upper);
        assert!(lower <= 0.125 && 0.125 <= upper);
        assert!(upper < 0.13, "the lower-Q candidate must not set Q_max");
    }

    #[test]
    fn exact_rational_interval_handles_rounding_and_underflow() {
        for value in [
            BigRational::new(1.into(), 10.into()),
            BigRational::new((-1).into(), 10.into()),
            BigRational::new(1.into(), num_bigint::BigInt::from(1_u8) << 1100_u32),
        ] {
            let (lower, upper) = exact_rational_to_f64_interval(&value);
            assert!(
                f64_to_rational(lower) <= value,
                "lower endpoint exceeds exact rational"
            );
            assert!(
                value <= f64_to_rational(upper),
                "upper endpoint is below exact rational"
            );
        }
    }

    #[test]
    fn fused_kkt_entry_radius_norm_encloses_exact_assembly_error() {
        let duals = hko_pentagon().dual_vertices_f64.clone();
        let word = vec![0, 1, 6, 7, 3, 4, 5, 9];
        let (matrix, _) = build_augmented_system_from_dual_vertices(&duals, &word);
        let bound = exact_kkt_entry_radius_inf_norm(&duals, &word)
            .expect("finite fixture has a finite assembly bound");
        let exact_duals = exact_binary64_dual_vertex_arrays(&duals);

        let exact_norm = (0..word.len())
            .map(|row| {
                (0..word.len())
                    .filter(|&col| col != row)
                    .map(|col| {
                        let exact = if row < col {
                            omega_exact(&exact_duals[word[row]], &exact_duals[word[col]])
                        } else {
                            omega_exact(&exact_duals[word[col]], &exact_duals[word[row]])
                        };
                        (exact - f64_to_rational(matrix[(row, col)])).abs()
                    })
                    .fold(BigRational::zero(), |sum, error| sum + error)
            })
            .max()
            .expect("nonempty word");
        assert!(
            exact_norm <= f64_to_rational(bound),
            "fused norm missed exact KKT assembly error"
        );
    }

    #[test]
    fn analytic_omega_roundoff_radius_encloses_exact_formula_error() {
        let minimum_subnormal = f64::from_bits(1);
        let pairs = [
            (
                Vector4::new(1e3, 1e-3, -1e3, -1e-3),
                Vector4::new(-1e-3, 1e3, 1e-3, -1e3),
            ),
            (
                Vector4::new(0.1, -0.2, 0.3, -0.4),
                Vector4::new(-0.5, 0.6, -0.7, 0.8),
            ),
            (
                Vector4::new(minimum_subnormal, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 0.5, 0.0),
            ),
        ];
        for (left, right) in pairs {
            let left_exact = std::array::from_fn(|index| f64_to_rational(left[index]));
            let right_exact = std::array::from_fn(|index| f64_to_rational(right[index]));
            let exact = omega_exact(&left_exact, &right_exact);
            let computed =
                left[0] * right[2] - left[2] * right[0] + left[1] * right[3] - left[3] * right[1];
            let error = (exact - f64_to_rational(computed)).abs();
            let radius =
                omega_roundoff_radius(&left, &right).expect("finite inputs have finite bound");
            assert!(
                error <= f64_to_rational(radius),
                "analytic omega roundoff bound missed exact error"
            );
        }
    }

    fn assert_product_rounding_enclosed(left: &DMatrix<f64>, right: &DMatrix<f64>) {
        let (gamma, underflow) =
            dot_product_error_parameters(left.ncols()).expect("small test dot product");
        let computed = left * right;
        let magnitude_upper = positive_product_upper(
            &left.map(|value| value.abs()),
            &right.map(|value| value.abs()),
            gamma,
            underflow,
        )
        .expect("finite positive product");

        for row in 0..left.nrows() {
            for col in 0..right.ncols() {
                let exact = (0..left.ncols())
                    .map(|mid| {
                        f64_to_rational(left[(row, mid)]) * f64_to_rational(right[(mid, col)])
                    })
                    .fold(BigRational::zero(), |sum, term| sum + term);
                let error = (exact - f64_to_rational(computed[(row, col)])).abs();
                let error_upper = add_up(mul_up(gamma, magnitude_upper[(row, col)]), underflow);
                assert!(
                    error <= f64_to_rational(error_upper),
                    "entry ({row},{col}) escaped the rounding envelope"
                );
            }
        }
    }

    fn assert_product_then_subtraction_enclosed(
        left: &DMatrix<f64>,
        right: &DMatrix<f64>,
        target: impl Fn(usize, usize) -> f64,
    ) {
        let (gamma, underflow) =
            dot_product_error_parameters(left.ncols()).expect("small test dot product");
        let computed = left * right;
        let magnitude_upper = positive_product_upper(
            &left.map(|value| value.abs()),
            &right.map(|value| value.abs()),
            gamma,
            underflow,
        )
        .expect("finite positive product");

        for row in 0..left.nrows() {
            for col in 0..right.ncols() {
                let target = target(row, col);
                let exact = (0..left.ncols())
                    .map(|mid| {
                        f64_to_rational(left[(row, mid)]) * f64_to_rational(right[(mid, col)])
                    })
                    .fold(BigRational::zero(), |sum, term| sum + term)
                    - f64_to_rational(target);
                let computed_residual = computed[(row, col)] - target;
                let error = (exact - f64_to_rational(computed_residual)).abs();
                let augmented_magnitude = add_up(magnitude_upper[(row, col)], target.abs());
                let error_upper = add_up(mul_up(gamma, augmented_magnitude), underflow);
                assert!(
                    error <= f64_to_rational(error_upper),
                    "augmented entry ({row},{col}) escaped the rounding envelope"
                );
            }
        }
    }
}
