struct Surface {
    path: &'static str,
    text: &'static str,
}

struct Forbidden {
    needle: &'static str,
    reason: &'static str,
}

#[test]
fn public_api_does_not_reintroduce_recent_kiss_violations() {
    assert_absent(
        &[
            Surface {
                path: "src/lib.rs",
                text: include_str!("../src/lib.rs"),
            },
            Surface {
                path: "src/algebraic_element.rs",
                text: include_str!("../src/algebraic_element.rs"),
            },
            Surface {
                path: "src/arithmetic_ops.rs",
                text: include_str!("../src/arithmetic_ops.rs"),
            },
            Surface {
                path: "src/field_specification.rs",
                text: include_str!("../src/field_specification.rs"),
            },
        ],
        &[
            Forbidden {
                needle: "RationalInterval",
                reason:
                    "field specs should return endpoint pairs; interval machinery stays private",
            },
            Forbidden {
                needle: "BadDegree",
                reason: "wrong coefficient length is a constructor bug, not a recoverable API case",
            },
            Forbidden {
                needle: "pub fn alpha",
                reason: "the public constructor for the chosen real root is root()",
            },
            Forbidden {
                needle: "pub fn coeffs",
                reason: "public API should not expose coefficient storage without a current caller",
            },
            Forbidden {
                needle: "pub fn from_rational",
                reason: "From<BigRational> is the public rational conversion",
            },
            Forbidden {
                needle: "pub fn inverse",
                reason: "division is the advertised public operation",
            },
            Forbidden {
                needle: "impl_scalar_ops",
                reason:
                    "use explicit Qsqrt5::from(..) conversion instead of scalar operator mixing",
            },
            Forbidden {
                needle: "pub enum Sign",
                reason: "std::cmp::Ordering is the built-in exact trichotomy type",
            },
            Forbidden {
                needle: "const DEGREE",
                reason: "the polynomial already determines the field degree",
            },
        ],
    );
}

#[test]
fn consumer_material_does_not_teach_implementation_plumbing() {
    assert_absent(
        &[
            Surface {
                path: "README.md",
                text: include_str!("../README.md"),
            },
            Surface {
                path: "examples/q_sqrt5_vector.rs",
                text: include_str!("../examples/q_sqrt5_vector.rs"),
            },
        ],
        &[
            Forbidden {
                needle: ".clone()",
                reason: "consumer examples should not introduce ownership noise",
            },
            Forbidden {
                needle: ".unwrap()",
                reason: "consumer examples should not expose recoverable-error-shaped construction",
            },
            Forbidden {
                needle: "let root =",
                reason: "one-use root bindings hide the expression being demonstrated",
            },
            Forbidden {
                needle: "Current tests live",
                reason: "consumer README should not contain maintainer navigation",
            },
            Forbidden {
                needle: "Current Scope Limits",
                reason: "scope wording must name what is not provided",
            },
            Forbidden {
                needle: "2 * Qsqrt5::root()",
                reason: "scalar arithmetic should stay uniformly inside Qsqrt5 via From",
            },
            Forbidden {
                needle: "3 * Qsqrt5::root()",
                reason: "scalar arithmetic should stay uniformly inside Qsqrt5 via From",
            },
        ],
    );
}

fn assert_absent(surfaces: &[Surface], forbidden: &[Forbidden]) {
    let mut failures = Vec::new();

    for surface in surfaces {
        for rule in forbidden {
            if surface.text.contains(rule.needle) {
                failures.push(format!(
                    "{} contains {:?}: {}",
                    surface.path, rule.needle, rule.reason
                ));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "KISS API style check failed:\n{}",
        failures.join("\n")
    );
}
