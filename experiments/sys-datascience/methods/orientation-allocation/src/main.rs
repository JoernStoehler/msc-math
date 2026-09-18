use nalgebra::{Matrix4,Vector4};
use num_traits::ToPrimitive;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde_json::{json,Value};
use std::time::Instant;
use symplectic::algorithms::capacity_4d::*;
use symplectic::geom::polygon::random_polygon_2d;

fn inspect(v:&[Vector4<f64>])->Result<(f64,Vec<Vec<bool>>),String>{
    check_facet_count(v.len()).map_err(|e|format!("{e:?}"))?;
    check_dual_vertex_norm_bounds(v).map_err(|e|format!("{e:?}"))?;
    let g=exact_binary64_polytope_geometry(v).map_err(|e|format!("{e:?}"))?;
    check_primal_vertex_norm_bounds(&g).map_err(|e|format!("{e:?}"))?;
    let p:Vec<_>=g.primal_vertices_exact.iter().map(|x|Vector4::new(x[0].to_f64().unwrap(),x[1].to_f64().unwrap(),x[2].to_f64().unwrap(),x[3].to_f64().unwrap())).collect();
    let volume=euclidean_polytopes::volume_from_incidence_f64(&p,&g.vertex_facet_incidence).map_err(|e|format!("{e:?}"))?;
    let mut incidence:Vec<Vec<bool>>=(0..g.vertex_facet_incidence.nrows()).map(|i|(0..g.vertex_facet_incidence.ncols()).map(|j|g.vertex_facet_incidence[(i,j)]).collect()).collect();
    incidence.sort(); Ok((volume,incidence))
}
fn orientation(theta:f64,phi:f64)->Matrix4<f64>{
    let k1=Matrix4::new(0.,-1.,0.,0.,1.,0.,0.,0.,0.,0.,0.,1.,0.,0.,-1.,0.);
    let k2=Matrix4::new(0.,0.,0.,-1.,0.,0.,1.,0.,0.,-1.,0.,0.,1.,0.,0.,0.);
    theta.cos()*Matrix4::identity()+theta.sin()*(phi.cos()*k1+phi.sin()*k2)
}
fn payload(v:&[Vector4<f64>])->Vec<[f64;4]>{v.iter().map(|v|[v[0],v[1],v[2],v[3]]).collect()}
fn main(){
    let mut output=Vec::<Value>::new();
    for (bi,k) in [3usize,4].into_iter().enumerate(){for block in 0..2{
        let block_id=format!("{k}x{k}-b{block}");
        let mut iid=Vec::new(); let mut base=None;
        for draw in 0..16{
            let timer=Instant::now();let mut rejects=Vec::new();let mut accepted=None;
            for attempt in 0..128{
                let preimage=format!("ds-orientation-allocation-v1|seed=202609140201|bucket={k}x{k}|block={block}|draw={draw}|attempt={attempt}");
                let mut rng=ChaCha8Rng::from_seed(*blake3::hash(preimage.as_bytes()).as_bytes());
                let (qn,qh)=random_polygon_2d(k,0.8,1.2,&mut rng);
                let (pn,ph)=random_polygon_2d(k,0.8,1.2,&mut rng);
                let v:Vec<_>=qn.iter().zip(qh.iter()).map(|(n,h)|Vector4::new(n[0]/h,n[1]/h,0.,0.)).chain(pn.iter().zip(ph.iter()).map(|(n,h)|Vector4::new(0.,0.,n[0]/h,n[1]/h))).collect();
                match inspect(&v){Ok((vol,inc))=>{accepted=Some((v,vol,inc,attempt,preimage));break},Err(e)=>rejects.push(json!({"attempt":attempt,"error":e}))}
            }
            let (v,vol,inc,attempt,preimage)=accepted.expect("construction exhausted; no frozen panel emitted");
            let memberships=if draw==0{json!([{"arm":"iid","step":0},{"arm":"orientation","step":0}])}else{json!([{"arm":"iid","step":draw}])};
            iid.push(json!({"id":format!("{block_id}-iid-{draw}"),"block":block_id,"bucket":format!("{k}x{k}"),"dual_vertices":payload(&v),"memberships":memberships,"proposal_ms":timer.elapsed().as_secs_f64()*1000.,"source_attempt":attempt,"source_seed_preimage":preimage,"construction_rejections":rejects,"target_free_volume":vol,"incidence_signature":inc}));
            if draw==0{base=Some((v,vol,inc));}
        }
        let (base_v,base_vol,base_inc)=base.unwrap();let mut rotated=Vec::new();
        let jform=Matrix4::new(0.,0.,1.,0.,0.,0.,0.,1.,-1.,0.,0.,0.,0.,-1.,0.,0.);
        for step in 1..16{
            let timer=Instant::now();
            let (q,theta,phi)=if step==1{(Matrix4::new(0.6,0.,-0.8,0.,0.,0.6,0.,-0.8,0.8,0.,0.6,0.,0.,0.8,0.,0.6),None,None)}else{
                let index=step-2;let theta=if index%2==0{std::f64::consts::PI/8.}else{std::f64::consts::PI/4.};let phi=2.*std::f64::consts::PI*((index/2)as f64)/7.;(orientation(theta,phi),Some(theta),Some(phi))};
            let orth=(q.transpose()*q-Matrix4::identity()).norm();let det=(q.determinant()-1.).abs();let symp=(q.transpose()*jform*q-jform).norm();
            assert!(orth<=1e-12&&det<=1e-12);if step==1{assert!(symp<=1e-12);}
            let v:Vec<_>=base_v.iter().map(|x|q*x).collect();let (vol,inc)=inspect(&v).expect("transformed geometry invalid; panel not emitted");
            assert_eq!(inc,base_inc,"incidence changed");assert!((vol/base_vol-1.).abs()<=1e-8,"volume changed");
            let matrix:Vec<Vec<f64>>=(0..4).map(|i|(0..4).map(|j|q[(i,j)]).collect()).collect();
            rotated.push(json!({"id":format!("{block_id}-orientation-{step}"),"block":block_id,"bucket":format!("{k}x{k}"),"dual_vertices":payload(&v),"memberships":[{"arm":"orientation","step":step}],"proposal_ms":timer.elapsed().as_secs_f64()*1000.,"matrix":matrix,"theta":theta,"phi":phi,"orthogonality_residual":orth,"determinant_residual":det,"symplectic_residual":symp,"target_free_volume":vol,"incidence_signature":inc,"base_id":format!("{block_id}-iid-0")}));
        }
        output.push(iid[0].clone());
        for step in 1..16{if (bi*2+block)%2==0{output.push(rotated[step-1].clone());output.push(iid[step].clone());}else{output.push(iid[step].clone());output.push(rotated[step-1].clone());}}
    }}
    assert_eq!(output.len(),124);for row in output{println!("{row}");}
}
