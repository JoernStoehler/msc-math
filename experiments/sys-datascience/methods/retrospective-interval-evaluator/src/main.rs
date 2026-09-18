//! One request on stdin; the Python supervisor supplies per-request isolation.
use nalgebra::Vector4;
use num_traits::ToPrimitive;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{io::{self, Read}, time::Instant};
use symplectic::algorithms::capacity_4d::*;

#[derive(Deserialize)]
struct Input { dual_vertices: Vec<[f64; 4]> }
fn failure(stage: &str, error: impl std::fmt::Debug) -> Value {
    json!({"status":"error", "error_stage":stage, "error_type":format!("{error:?}")})
}
fn evaluate(input: Input) -> Value {
    let dual: Vec<_> = input.dual_vertices.iter().map(|v| Vector4::from_row_slice(v)).collect();
    if let Err(e) = check_facet_count(dual.len()) { return failure("input_bounds",e); }
    if let Err(e) = check_finite_dual_vertices(&dual) { return failure("geometry",e); }
    if let Err(e) = check_dual_vertex_norm_bounds(&dual) { return failure("input_bounds",e); }
    let t = Instant::now();
    let geometry = match exact_binary64_polytope_geometry(&dual) { Ok(g)=>g, Err(e)=>return failure("geometry",e) };
    let geometry_ms = t.elapsed().as_secs_f64()*1000.;
    if let Err(e) = check_primal_vertex_norm_bounds(&geometry) { return failure("input_bounds",e); }
    let primal: Option<Vec<Vector4<f64>>> = geometry.primal_vertices_exact.iter().map(|v| {
        Some(Vector4::new(v[0].to_f64()?,v[1].to_f64()?,v[2].to_f64()?,v[3].to_f64()?))
    }).collect();
    let Some(primal) = primal else { return failure("volume", "PrimalConversion"); };
    let t = Instant::now();
    let volume = match euclidean_polytopes::volume_from_incidence_f64(&primal,&geometry.vertex_facet_incidence) {
        Ok(v) if v.is_finite() && v > 0. => v,
        Ok(_) => return failure("volume","NonpositiveOrNonfiniteVolume"),
        Err(e) => return failure("volume",e),
    };
    let volume_ms = t.elapsed().as_secs_f64()*1000.;
    let t = Instant::now();
    let result = match capacity(&geometry) { Ok(v)=>v, Err(e)=>return failure("capacity",e) };
    let capacity_ms = t.elapsed().as_secs_f64()*1000.;
    let scalar_result = capacity_value(&result,1e-10);
    let (value, scalar_error) = match scalar_result { Ok(v)=>(Some(v),None), Err(e)=>(None,Some(format!("{e:?}"))) };
    let exact = match &result { Capacity4d::Product(p)=>Some(p.capacity_exact().to_string()), Capacity4d::General(_)=>None };
    json!({"status":"capacity_interval","capacity_scalar":value,"scalar_tolerance":1e-10,
        "scalar_error":scalar_error,"capacity_lower":result.bounds().lower(),
        "capacity_upper":result.bounds().upper(),"capacity_exact":exact,"capacity_route":result.route_name(),
        "volume":volume,"numerical_sys_scalar":value.map(|v|v*v/(2.*volume)),
        "geometry_ms":geometry_ms,"volume_ms":volume_ms,"capacity_ms":capacity_ms,
        "primal_vertex_count":primal.len(),"fresh_geometry_and_input_policy_passed":true})
}
fn main() {
    let t=Instant::now();
    let mut text=String::new();
    let mut result=match io::stdin().read_to_string(&mut text) {
        Err(e)=>failure("input_io",e),
        Ok(_)=>match serde_json::from_str::<Input>(&text) { Ok(v)=>evaluate(v),Err(e)=>failure("input_schema",e) }
    };
    result["worker_wall_ms"]=json!(t.elapsed().as_secs_f64()*1000.);
    println!("{result}");
}
