use euclidean_polytopes::{edges_from_vertex_facet_incidence, two_faces_from_vertex_facet_incidence, vertex_facets_from_vertex_facet_incidence};
use exp_sys_landscape::{capacity_billiard, poly_id, reference::exact_volume_as_f64, SysLandscapePolytopeCache};
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::Instant;
use symplectic::algorithms::billiard::bounce_count_from_sigma_for_facets;
use symplectic::{classify_facets_from_dual_vertices, systolic_ratio};

const ARTIFACTS: &str = "artifacts";

#[derive(Deserialize)] struct H { normal: [f64;2], height: f64 }
#[derive(Deserialize)] struct Candidate {
    candidate_id:String, bucket:String, path_label:String, relative_rotation_rad:f64,
    branch_h_predicted_sys:f64, q_hrep:Vec<H>, p_hrep:Vec<H>, edge_formula_r:f64,
}
#[derive(Serialize, Deserialize)] struct Preflight {
    candidate_id:String, facet_count:usize, vertex_count:usize, edge_count:usize,
    ridge_count:usize, volume:f64, edge_formula_r:f64, min_support_height:f64,
    max_support_height:f64, passed:bool, candidates_sha256:String, evaluator_source_sha256:String,
}
#[derive(Serialize, Deserialize)] struct Target {
    schema:&'static str, candidate_id:String, bucket:String, path_label:String,
    relative_rotation_rad:f64, poly_id:String, volume:f64, capacity:f64, sys:f64,
    branch_h_predicted_sys:f64, branch_h_absolute_error:f64, bounces:Option<usize>,
    best_sigma:Vec<usize>, iterations:u64, min_action_lower:f64, min_action_upper:f64,
    best_orbit_admissibility:String, best_orbit_beta_margin:f64, time_capacity_ms:f64,
    candidates_sha256:String, preflight_sha256:String, evaluator_source_sha256:String,
    capacity_implementation_manifest_sha256:String,
}

fn sha256(path: impl AsRef<Path>) -> String { format!("{:x}", Sha256::digest(std::fs::read(path).unwrap())) }
fn rows() -> Vec<Candidate> {
    BufReader::new(File::open(format!("{ARTIFACTS}/candidates.jsonl")).unwrap()).lines()
        .map(|line| serde_json::from_str(&line.unwrap()).unwrap()).collect()
}
fn poly(row:&Candidate) -> SysLandscapePolytopeCache {
    let qn=row.q_hrep.iter().map(|x|Vector2::from(x.normal)).collect::<Vec<_>>();
    let qh=row.q_hrep.iter().map(|x|x.height).collect::<Vec<_>>();
    let pn=row.p_hrep.iter().map(|x|Vector2::from(x.normal)).collect::<Vec<_>>();
    let ph=row.p_hrep.iter().map(|x|x.height).collect::<Vec<_>>();
    SysLandscapePolytopeCache::from_lagrangian_product(&qn,&qh,&pn,&ph).unwrap()
}
fn preflight() {
    let candidates_sha256=sha256(format!("{ARTIFACTS}/candidates.jsonl"));
    let evaluator_source_sha256=sha256("src/main.rs");
    let mut output=BufWriter::new(File::create(format!("{ARTIFACTS}/api-verification.jsonl")).unwrap());
    for row in rows() {
        let poly=poly(&row); let inc=&poly.vertex_facet_incidence;
        let vf=vertex_facets_from_vertex_facet_incidence(inc);
        let edges=edges_from_vertex_facet_incidence(inc);
        let faces=two_faces_from_vertex_facet_incidence(inc);
        let volume=exact_volume_as_f64(&poly.vertices,inc);
        let heights=row.q_hrep.iter().chain(&row.p_hrep).map(|x|x.height).collect::<Vec<_>>();
        let min_h=heights.iter().copied().fold(f64::INFINITY,f64::min);
        let max_h=heights.iter().copied().fold(f64::NEG_INFINITY,f64::max);
        let passed=poly.dual_vertices.len()==9 && poly.vertices.len()==18 && edges.len()==36
            && faces.len()==27 && vf.iter().all(|x|x.len()==4) && (volume-18.0).abs()<1e-10
            && min_h>=0.8 && max_h<=1.2 && row.edge_formula_r.is_finite();
        let check=Preflight{candidate_id:row.candidate_id,facet_count:poly.dual_vertices.len(),
            vertex_count:poly.vertices.len(),edge_count:edges.len(),ridge_count:faces.len(),volume,
            edge_formula_r:row.edge_formula_r,min_support_height:min_h,max_support_height:max_h,passed,
            candidates_sha256:candidates_sha256.clone(),evaluator_source_sha256:evaluator_source_sha256.clone()};
        assert!(passed); writeln!(output,"{}",serde_json::to_string(&check).unwrap()).unwrap();
    }
}
fn evaluate(candidate_id:&str) {
    let candidates_sha256=sha256(format!("{ARTIFACTS}/candidates.jsonl"));
    let preflight_path=format!("{ARTIFACTS}/api-verification.jsonl");
    let preflight_sha256=sha256(&preflight_path);
    let checks:Vec<Preflight>=BufReader::new(File::open(&preflight_path).unwrap()).lines()
        .map(|line|serde_json::from_str(&line.unwrap()).unwrap()).collect();
    assert!(checks.len()==2 && checks.iter().all(|x|x.passed));
    if checks.iter().any(|x|x.candidates_sha256!=candidates_sha256) {
        eprintln!("warning: preflight records different candidate bytes; continuing with semantic checks. Reassess retained interpretation before treating this run as equivalent.");
    }
    if candidate_id.ends_with("delta2") {
        let previous:serde_json::Value=serde_json::from_reader(File::open(format!("{ARTIFACTS}/target-delta1.json")).expect("delta1 must precede delta2")).unwrap();
        assert!(previous["sys"].as_f64().unwrap()<=1.0,"delta1 crossed one: stop before delta2");
    }
    let row=rows().into_iter().find(|x|x.candidate_id==candidate_id).expect("frozen candidate id");
    let poly=poly(&row); let volume=exact_volume_as_f64(&poly.vertices,&poly.vertex_facet_incidence);
    let start=Instant::now();
    let result=capacity_billiard(&poly.dual_vertices_f64,&poly.dual_vertices,&poly.facet_intersection_is_nonempty,&poly.omega_signs).unwrap();
    let elapsed=start.elapsed().as_secs_f64()*1000.0;
    let capacity=result.min_action; let sys=systolic_ratio(capacity,volume); let best=result.best_orbit();
    let classification=classify_facets_from_dual_vertices(&poly.dual_vertices_f64).unwrap();
    let bounces=bounce_count_from_sigma_for_facets(&classification.q_indices,&classification.p_indices,&best.sigma);
    let manifest="../ridge-endpoint-path/artifacts/capacity-implementation-manifest.json";
    let path_label=row.path_label.clone();
    let target=Target{schema:"ridge-symmetry-completion.target.v1",candidate_id:row.candidate_id,
        bucket:row.bucket,path_label:row.path_label,relative_rotation_rad:row.relative_rotation_rad,
        poly_id:poly_id(&poly),volume,capacity,sys,branch_h_predicted_sys:row.branch_h_predicted_sys,
        branch_h_absolute_error:(sys-row.branch_h_predicted_sys).abs(),bounces,best_sigma:best.sigma.clone(),
        iterations:result.iterations,min_action_lower:result.min_action_lower,min_action_upper:result.min_action_upper,
        best_orbit_admissibility:format!("{:?}",best.admissibility),best_orbit_beta_margin:best.beta_margin,
        time_capacity_ms:elapsed,candidates_sha256,preflight_sha256,evaluator_source_sha256:sha256("src/main.rs"),
        capacity_implementation_manifest_sha256:sha256(manifest)};
    let path=format!("{ARTIFACTS}/target-{path_label}.json");
    serde_json::to_writer_pretty(File::create(path).unwrap(),&target).unwrap();
}
fn main(){
    let args=std::env::args().collect::<Vec<_>>();
    match args.as_slice(){
        [_,mode] if mode=="--preflight"=>preflight(),
        [_,mode,id] if mode=="--evaluate"=>evaluate(id),
        _=>panic!("use --preflight or --evaluate CANDIDATE_ID"),
    }
}
