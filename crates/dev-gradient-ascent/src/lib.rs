//! Gradient ascent method development.
//!
//! Instruments for developing the gradient ascent search method: step size
//! calibration, strategy comparison (overshoot vs wiggle vs noise), convergence
//! diagnostics. Findings feed back from exp-sys-landscape/ application runs.
//!
//! Related crates:
//! - `dev-gradient/` — gradient correctness validation
//! - `exp-sys-landscape/boundary-crossing-search/` — applies gradient ascent at scale
