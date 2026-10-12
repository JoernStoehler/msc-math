use symplectic::algorithms::capacity_4d::*;
use equal_budget_product_search::{chart::{iid_base_candidate_attempt,ProductChart},cem::*};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde_json::{json,Value};
use std::{io::{self,Read},time::Instant};
fn main(){
 let mut input=String::new();io::stdin().read_to_string(&mut input).unwrap();
 let req:Value=serde_json::from_str(&input).unwrap();
 if req["mode"]=="update" {
  let population:Vec<CemScoredCandidate>=req["population"].as_array().unwrap().iter().map(|r|CemScoredCandidate{
   candidate_id:r["id"].as_str().unwrap().into(),coordinates:serde_json::from_value(r["coordinates"].clone()).unwrap(),sys:r["sys"].as_f64().unwrap()}).collect();
  assert_eq!(population.len(),64);
  let previous=if req["previous"].is_null(){generation_zero_distribution(&population).unwrap()}else{serde_json::from_value(req["previous"].clone()).unwrap()};
  let elites=ranked_elites(&population,16).unwrap();
  let next=update_distribution(&previous,&elites).unwrap();
  println!("{}",json!({"previous":previous,"next":next,"elite_ids":elites.iter().map(|v|&v.candidate_id).collect::<Vec<_>>()}));return;
 }
 let t=Instant::now();let seed=req["seed"].as_u64().unwrap();let generation=req["generation"].as_u64().unwrap() as usize;
 let arm=req["arm"].as_str().unwrap();
 let distribution:Option<CemDistribution>=serde_json::from_value(req["distribution"].clone()).unwrap();
 let material=format!("diagonal-cem-pilot-v1|seed={seed}|generation={generation}");
 let mut rng=ChaCha8Rng::from_seed(*blake3::hash(material.as_bytes()).as_bytes());
 let mut rows=vec![];let mut attempts=vec![];let mut retry=0;
 for attempt in 0..640 {
  if rows.len()==64 {break;}
  let index=rows.len();let id=format!("seed={seed}|arm={arm}|gen={generation}|index={index:03}|attempt={attempt}");
  let proposed=distribution.as_ref().map(|d|d.sample_coordinates(&mut rng));
  let candidate=if let Some(coords)=proposed{ProductChart::from_continuous_coordinates(coords,false).reconstruct_candidate()}
  else{iid_base_candidate_attempt(seed,0,generation*64+index,retry)};
  match candidate {
   Err(error)=>{attempts.push(json!({"id":id,"index":index,"retry":retry,"proposed_coordinates":proposed,"status":"rejected","error":error}));retry+=1;},
   Ok(candidate)=>{
    let dual_policy=&candidate.polytope.dual_vertices_f64;
    let admission=(|| -> Result<(),String> {
     check_facet_count(dual_policy.len()).map_err(|e|format!("{e:?}"))?;
     check_finite_dual_vertices(dual_policy).map_err(|e|format!("{e:?}"))?;
     check_dual_vertex_norm_bounds(dual_policy).map_err(|e|format!("{e:?}"))?;
     let geometry=exact_binary64_polytope_geometry(dual_policy).map_err(|e|format!("{e:?}"))?;
     check_primal_vertex_norm_bounds(&geometry).map_err(|e|format!("{e:?}"))?;
     Ok(())
    })();
    if let Err(error)=admission {
     attempts.push(json!({"id":id,"index":index,"retry":retry,"proposed_coordinates":proposed,"status":"rejected","stage":"evaluator_admission","error":error}));retry+=1;continue;
    }
    let chart=if let Some(c)=proposed{ProductChart::from_continuous_coordinates(c,false)}else{ProductChart::from_factors(&candidate.factors.q_normals,&candidate.factors.q_heights,&candidate.factors.p_normals,&candidate.factors.p_heights).unwrap()};
    let canonical=ProductChart::from_polytope(&candidate.polytope).unwrap();
    let dual:Vec<[f64;4]>=candidate.polytope.dual_vertices_f64.iter().map(|v|[v[0],v[1],v[2],v[3]]).collect();
    rows.push(json!({"id":id,"seed":seed,"arm":arm,"generation":generation,"index":index,"construction_attempt":attempt,"retry":retry,
     "coordinates":chart.continuous_coordinates(),"near_tie":canonical.near_tie,"dual_vertices":dual}));
    attempts.push(json!({"id":id,"index":index,"retry":retry,"proposed_coordinates":proposed,"status":"accepted"}));retry=0;
   }
  }
 }
 println!("{}",json!({"schema":"diagonal-cem-generation-v1","complete":rows.len()==64,"rows":rows,"construction":attempts,"generation_ms":t.elapsed().as_secs_f64()*1000.}));
}
