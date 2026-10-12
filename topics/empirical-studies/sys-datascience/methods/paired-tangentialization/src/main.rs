use nalgebra::{Vector2,Vector4};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use serde_json::{json,Value};
use std::{fs::OpenOptions,io::Write,path::PathBuf};
use symplectic::geom::polygon::{random_polygon_2d,polygon_area};
use symplectic::algorithms::capacity_4d::exact_binary64_polytope_geometry;
#[derive(Clone,Serialize)]
struct Factor { normals:Vec<Vector2<f64>>, heights:Vec<f64> }
fn active(f: &Factor) -> bool {
    if f.normals.len() != f.heights.len() || f.normals.len() < 3 {
        return false;
    }
    for i in 0..f.normals.len() {
        let j = (i + 1) % f.normals.len();
        let a = f.normals[i];
        let b = f.normals[j];
        let det = a[0] * b[1] - a[1] * b[0];
        if det.abs() < 1e-12 {
            return false;
        }
        let x = (f.heights[i] * b[1] - f.heights[j] * a[1]) / det;
        let y = (a[0] * f.heights[j] - b[0] * f.heights[i]) / det;
        if f.normals
            .iter()
            .zip(&f.heights)
            .any(|(n, h)| n[0] * x + n[1] * y > *h + 1e-9)
        {
            return false;
        }
    }
    true
}

fn normalize(mut f: Factor) -> Option<Factor> {
    if !active(&f) {
        return None;
    }
    let area = polygon_area(&f.normals, &f.heights)?;
    if !(area.is_finite() && area > 0.0) {
        return None;
    }
    let s = area.sqrt().recip();
    for h in &mut f.heights {
        *h *= s;
    }
    Some(f)
}


fn dual(q:&Factor,p:&Factor)->Vec<[f64;4]> {
 q.normals.iter().zip(&q.heights).map(|(n,h)|[n[0]/h,n[1]/h,0.,0.])
 .chain(p.normals.iter().zip(&p.heights).map(|(n,h)|[0.,0.,n[0]/h,n[1]/h])).collect()
}
fn construct(k:usize,m:usize,row:usize,attempt:usize)->Result<Vec<Value>,String> {
 let seed_text=format!("ds-tangentialization-v1|seed=202609140101|bucket={k}x{m}|row={row}|attempt={attempt}");
 let mut rng=ChaCha8Rng::from_seed(*blake3::hash(seed_text.as_bytes()).as_bytes());
 let (qn,qh)=random_polygon_2d(k,0.8,1.2,&mut rng);
 let (pn,ph)=random_polygon_2d(m,0.8,1.2,&mut rng);
 let pre=vec![Factor{normals:qn.clone(),heights:qh},Factor{normals:pn.clone(),heights:ph},
 Factor{normals:qn,heights:vec![1.;k]},Factor{normals:pn,heights:vec![1.;m]}];
 let post:Vec<Factor>=pre.iter().enumerate().map(|(i,f)|normalize(f.clone()).ok_or(format!("factor {i}: inactive/nonpositive area"))).collect::<Result<_,_>>()?;
 let areas_pre:Vec<_>=pre.iter().map(|f|polygon_area(&f.normals,&f.heights).unwrap()).collect();
 let areas_post:Vec<_>=post.iter().map(|f|polygon_area(&f.normals,&f.heights).unwrap()).collect();
 if areas_post.iter().any(|a|(a-1.).abs()>1e-10){return Err("normalization residual".into());}
 let mut result=vec![];
 for (arm,qi,pi) in [("00",0,1),("10",2,1),("01",0,3),("11",2,3)] {
  let vertices=dual(&post[qi],&post[pi]);
  let v:Vec<_>=vertices.iter().map(|v|Vector4::from_row_slice(v)).collect();
  let geometry=exact_binary64_polytope_geometry(&v).map_err(|e|format!("arm {arm}: {e:?}"))?;
  result.push(json!({"id":format!("{seed_text}|arm={arm}"),"latent_id":seed_text,"bucket":format!("{k}x{m}"),
  "row":row,"attempt":attempt,"arm":arm,"dual_vertices":vertices,"factor_order":["Bq","Bp","Tq","Tp"],
  "factors_pre":pre,"factors_post":post,"areas_pre":areas_pre,"areas_post":areas_post,
  "primal_vertex_count":geometry.primal_vertices_exact.len()}));
 }
 Ok(result)
}
fn main(){
 let out=PathBuf::from(std::env::args().nth(1).expect("output directory"));
 let mut log=OpenOptions::new().write(true).create_new(true).open(out.join("construction.jsonl")).unwrap();
 let mut grid=vec![];
 for row in 0..8 {for (k,m) in [(4,4),(4,6)] {
  let mut accepted=false;
  for attempt in 0..128 {
   match construct(k,m,row,attempt) {
    Ok(mut arms)=>{writeln!(log,"{}",json!({"bucket":format!("{k}x{m}"),"row":row,"attempt":attempt,"status":"accepted"})).unwrap();grid.append(&mut arms);accepted=true;break;},
    Err(reason)=>{writeln!(log,"{}",json!({"bucket":format!("{k}x{m}"),"row":row,"attempt":attempt,"status":"rejected","reason":reason})).unwrap();}
   }
  }
  assert!(accepted,"exhausted attempt cap; no targets/input grid written");
 }}
 assert_eq!(grid.len(),64);
 let mut output=OpenOptions::new().write(true).create_new(true).open(out.join("inputs.jsonl")).unwrap();
 for v in grid {writeln!(output,"{v}").unwrap();}
}
