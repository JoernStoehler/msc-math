//! Named exact fields supported by the algebraic exactness spike.
//!
//! The experiment-owned exact catalog stores one row-level field tag plus
//! canonical basis coefficients for every coordinate in that row.

use serde::{Deserialize, Serialize};

/// Named exact field families supported by the experiment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedFieldTag {
    /// The base field `Q`.
    Rational,
    /// `Q[t]/(t^4 - 10 t^2 + 5)` with distinguished real root `t = tan(pi/5)`.
    PentagonTanPiFifth,
}

impl NamedFieldTag {
    /// Human-readable field description.
    pub fn description(self) -> &'static str {
        match self {
            Self::Rational => "Q",
            Self::PentagonTanPiFifth => "Q[t]/(t^4 - 10 t^2 + 5), t = tan(pi/5)",
        }
    }

    /// Canonical basis labels for coefficient vectors serialized in the exact catalog.
    pub fn basis_labels(self) -> &'static [&'static str] {
        match self {
            Self::Rational => &["1"],
            Self::PentagonTanPiFifth => &["1", "t", "t^2", "t^3"],
        }
    }
}
