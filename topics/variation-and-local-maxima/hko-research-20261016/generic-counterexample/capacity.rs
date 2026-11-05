//! Bounded production-capacity control for the retained dyadic HKO perturbation.
use nalgebra::Vector4;
use num_rational::BigRational;
use serde_json::json;
use std::{env, fs};
use symplectic::capacity_4d::{capacity_from_dual_vertices, exact_binary64_polytope_geometry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        return Err("usage: hko-generic-counterexample FIXTURE.json OUTPUT.json".into());
    }
    let fixture: serde_json::Value = serde_json::from_str(&fs::read_to_string(&args[1])?)?;
    let rows: Vec<[f64; 4]> = serde_json::from_value(fixture["dual_vertices"].clone())?;
    let duals = rows
        .iter()
        .map(|r| Vector4::from_row_slice(r))
        .collect::<Vec<_>>();
    let geometry = exact_binary64_polytope_geometry(&duals)?;
    let volume = euclidean_polytopes::volume_from_incidence_exact(
        &geometry.primal_vertices_exact,
        &geometry.vertex_facet_incidence,
    );
    let capacity = capacity_from_dual_vertices(&duals)?;
    let bounds = capacity.bounds();
    let denominator = BigRational::from_integer(2.into()) * &volume;
    let lower = BigRational::from_float(bounds.lower()).ok_or("invalid lower bound")?;
    let upper = BigRational::from_float(bounds.upper()).ok_or("invalid upper bound")?;
    let scale = BigRational::from_integer(1_000_000_000_000_i64.into());
    let ratio_lower = ((&lower * &lower / &denominator) * &scale).to_integer();
    let ratio_upper = ((&upper * &upper / &denominator) * &scale).to_integer() + 1;
    assert!(ratio_lower > 1_000_000_000_000_i64.into());
    let output = json!({
        "route": capacity.route_name(),
        "capacity_lower": bounds.lower(),
        "capacity_upper": bounds.upper(),
        "volume_exact": volume.to_string(),
        "vertices": geometry.primal_vertices_exact.len(),
        "facets": duals.len(),
        "systolic_ratio_decimal_enclosure": [format!("{ratio_lower}/1000000000000"), format!("{ratio_upper}/1000000000000")],
        "ratio_enclosure_method": "Exact rational arithmetic on outward binary64 capacity endpoints and exact rational volume, then outward decimal rounding at 1e-12.",
        "meaning": "Production capacity enclosure for exact dyadic input, with exact incidence volume; independent of the analytic inclusion certificate."
    });
    fs::write(&args[2], serde_json::to_string_pretty(&output)? + "\n")?;
    println!("{}", output);
    Ok(())
}
