//! Purpose: sign classification for ordered scalar types.
//! Context: field arithmetic and small linear algebra use this enum for exact
//! branching instead of floating-point epsilon logic.

/// Exact sign of one ordered scalar value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}
