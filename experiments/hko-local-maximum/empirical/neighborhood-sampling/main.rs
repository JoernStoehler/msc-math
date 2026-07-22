//! Neighborhood-sampling dispatcher for empirical HKO local-maximum checks.
//!
//! This binary groups the nearby-polytope random samplers that share the same
//! workflow: generate a nearby HKO candidate, compute volume/capacity/sys, and
//! write JSONL artifacts for empirical support checks.

#[path = "../../src/flat_polytope.rs"]
mod flat_polytope;

mod samplers;

fn print_usage() {
    eprintln!(
        r#"Usage: hko-neighborhood-sampling <sampler> [sampler options]

Samplers:
  m10                         General fixed-F=10 dual-vertex perturbations.
  m11                         F=10 -> F=11 facet-splitting cuts.
  m10-lagrangian-product      Fixed-F=10 Lagrangian-product box sweep.
  m10-lagrangian-product-probe
                              Fixed-F=10 Lagrangian-product radial boundary probe.

Use `hko-neighborhood-sampling <sampler> --help` for sampler-specific flags."#
    );
}

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args[0] == "--help" || args[0] == "-h" {
        print_usage();
        return;
    }

    let sampler = args.remove(0);
    match sampler.as_str() {
        "m10" => samplers::m10::run(&args),
        "m11" => samplers::m11::run(&args),
        "m10-lagrangian-product" => samplers::m10_lagrangian_product::run(&args),
        "m10-lagrangian-product-probe" => samplers::m10_lagrangian_product_probe::run(&args),
        other => {
            eprintln!("error: unknown sampler: {other}\n");
            print_usage();
            std::process::exit(2);
        }
    }
}
