#[allow(dead_code)]
#[path = "../../../../polytope-invariant-table/features_face_symplectic.rs"]
mod features_face_symplectic;
#[allow(dead_code)]
#[path = "../../../../polytope-invariant-table/features_helpers.rs"]
mod features_helpers;

use euclidean_polytopes::two_faces_from_vertex_facet_incidence;
use exp_sys_landscape::{
    poly_id_from_dual_vertices, reference::exact_volume_as_f64, SysLandscapePolytopeCache,
};
use nalgebra::Vector4;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
struct TableRow {
    poly_id: String,
    facet_count: usize,
    sys: f64,
}

#[derive(Deserialize)]
struct ProvenanceRow {
    poly_id: String,
    source_name: String,
}

#[derive(Deserialize)]
struct ProducerRow {
    name: String,
    dual_vertices: Vec<[f64; 4]>,
    capacity: f64,
    volume: f64,
}

#[derive(Serialize)]
struct BranchInputRow<'a> {
    poly_id: &'a str,
    facet_count: usize,
    capacity: f64,
    volume: f64,
    sys: f64,
    dual_vertices_f64: &'a [[f64; 4]],
}

#[derive(Deserialize)]
struct PanelRow {
    panel_role: String,
    arm: String,
    bucket: String,
    poly_id_a: String,
    poly_id_b: String,
}

#[derive(Serialize)]
struct FaceRow {
    poly_id: String,
    face_index: usize,
    facet_a: usize,
    facet_b: usize,
    vertex_count: usize,
    area_over_volume_sqrt: f64,
    adjacency_degree: usize,
}

#[derive(Serialize)]
struct GeometryRow {
    poly_id: String,
    panel_roles: Vec<String>,
    arms: Vec<String>,
    buckets: Vec<String>,
    input_facet_count: usize,
    input_sys: f64,
    ordered_face_count: usize,
    ordering_failure_count: usize,
    adjacency_edge_count: usize,
    adjacent_abs_difference_mean: Option<f64>,
    adjacent_endpoint_pearson: Option<f64>,
    top_quartile_face_count: usize,
    top_quartile_induced_edge_count: usize,
    top_quartile_component_count: usize,
    top_quartile_internal_edge_fraction: Option<f64>,
}

#[derive(Serialize)]
struct Summary {
    method: String,
    evidence_class: String,
    panel_path: String,
    table_path: String,
    provenance_path: String,
    producer_paths: Vec<String>,
    panel_sha256: String,
    table_sha256: String,
    provenance_sha256: String,
    producer_sha256: Vec<String>,
    panel_record_count: usize,
    distinct_polytope_count: usize,
    successful_rows: usize,
    failures: BTreeMap<String, String>,
    invariance_contract: Vec<String>,
    target_boundary: String,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut panel = None;
    let mut table = None;
    let mut provenance = None;
    let mut producer_paths = Vec::new();
    let mut out = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--panel" => panel = args.next().map(PathBuf::from),
            "--table" => table = args.next().map(PathBuf::from),
            "--provenance" => provenance = args.next().map(PathBuf::from),
            "--producer" => producer_paths.push(PathBuf::from(
                args.next().expect("--producer requires a path"),
            )),
            "--out-dir" => out = args.next().map(PathBuf::from),
            other => panic!("unsupported argument {other}"),
        }
    }
    let panel = panel.expect("--panel required");
    let table = table.expect("--table required");
    let provenance = provenance.expect("--provenance required");
    assert!(
        !producer_paths.is_empty(),
        "at least one --producer required"
    );
    let out = out.expect("--out-dir required");
    fs::create_dir_all(&out).expect("create out dir");

    let panel_rows: Vec<PanelRow> = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(&panel)
        .expect("open panel")
        .deserialize()
        .map(|row| row.expect("parse panel"))
        .collect();
    let mut membership: BTreeMap<String, (BTreeSet<String>, BTreeSet<String>, BTreeSet<String>)> =
        BTreeMap::new();
    for row in &panel_rows {
        for id in [&row.poly_id_a, &row.poly_id_b] {
            if id.is_empty() {
                continue;
            }
            let entry = membership.entry(id.clone()).or_default();
            entry.0.insert(row.panel_role.clone());
            entry.1.insert(row.arm.clone());
            entry.2.insert(row.bucket.clone());
        }
    }
    let wanted: BTreeSet<String> = membership.keys().cloned().collect();
    let input = File::open(&table).expect("open table");
    let mut selected = BTreeMap::new();
    for line in BufReader::new(input).lines() {
        let line = line.expect("read table");
        if line.trim().is_empty() {
            continue;
        }
        let row: TableRow = serde_json::from_str(&line).expect("parse table");
        if wanted.contains(&row.poly_id) {
            assert!(
                selected.insert(row.poly_id.clone(), row).is_none(),
                "duplicate poly_id"
            );
        }
    }

    let mut source_by_id = BTreeMap::new();
    let mut selected_provenance_values = Vec::new();
    for line in BufReader::new(File::open(&provenance).expect("open provenance")).lines() {
        let line = line.expect("read provenance");
        if line.trim().is_empty() {
            continue;
        }
        let row: ProvenanceRow = serde_json::from_str(&line).expect("parse provenance");
        if wanted.contains(&row.poly_id) {
            selected_provenance_values.push(
                serde_json::from_str::<serde_json::Value>(&line).expect("parse provenance value"),
            );
            assert!(
                source_by_id.insert(row.poly_id, row.source_name).is_none(),
                "panel row has multiple provenance owners"
            );
        }
    }
    let mut producer_by_name = BTreeMap::new();
    for path in &producer_paths {
        for line in BufReader::new(File::open(path).expect("open producer")).lines() {
            let line = line.expect("read producer");
            if line.trim().is_empty() {
                continue;
            }
            let row: ProducerRow = serde_json::from_str(&line).expect("parse producer");
            assert!(
                producer_by_name.insert(row.name.clone(), row).is_none(),
                "duplicate producer name"
            );
        }
    }

    let mut geometry = Vec::new();
    let mut faces_out = Vec::new();
    let mut branch_inputs = Vec::new();
    let mut failures = BTreeMap::new();
    for id in &wanted {
        let Some(row) = selected.get(id) else {
            failures.insert(id.clone(), "missing_from_table".to_string());
            continue;
        };
        let Some(source_name) = source_by_id.get(id) else {
            failures.insert(id.clone(), "missing_provenance".to_string());
            continue;
        };
        let Some(producer) = producer_by_name.get(source_name) else {
            failures.insert(id.clone(), format!("missing_producer_row:{source_name}"));
            continue;
        };
        let duals = producer
            .dual_vertices
            .iter()
            .map(|v| Vector4::new(v[0], v[1], v[2], v[3]))
            .collect::<Vec<_>>();
        if poly_id_from_dual_vertices(&duals) != *id {
            failures.insert(id.clone(), "producer_poly_id_mismatch".to_string());
            continue;
        }
        let Some(polytope) = SysLandscapePolytopeCache::from_f64_dual_vertices(duals) else {
            failures.insert(id.clone(), "reconstruction_failed".to_string());
            continue;
        };
        branch_inputs.push(BranchInputRow {
            poly_id: id,
            facet_count: row.facet_count,
            capacity: producer.capacity,
            volume: producer.volume,
            sys: row.sys,
            dual_vertices_f64: &producer.dual_vertices,
        });
        let two_faces = two_faces_from_vertex_facet_incidence(&polytope.vertex_facet_incidence);
        let (ordered, ordering_failure_count) =
            features_face_symplectic::ordered_two_face_symplectic_areas(
                &two_faces,
                &polytope.vertices_f64,
                &polytope.vertex_facet_incidence,
            );
        let volume = exact_volume_as_f64(
            &polytope.vertices,
            &polytope.vertex_facet_incidence,
        );
        let scale = volume.sqrt();
        let areas: Vec<f64> = ordered.iter().map(|f| f.area / scale).collect();
        let mut adjacency = Vec::new();
        let mut degrees = vec![0usize; ordered.len()];
        for i in 0..ordered.len() {
            for j in i + 1..ordered.len() {
                let common = ordered[i]
                    .vertices
                    .iter()
                    .filter(|v| ordered[j].vertices.contains(v))
                    .count();
                if common >= 2 {
                    adjacency.push((i, j));
                    degrees[i] += 1;
                    degrees[j] += 1;
                }
            }
        }
        let adjacent_abs_difference_mean = mean(
            adjacency
                .iter()
                .map(|&(i, j)| (areas[i] - areas[j]).abs())
                .collect(),
        );
        let adjacent_endpoint_pearson = pearson_endpoints(&adjacency, &areas);
        let top_count = ((ordered.len() + 3) / 4).max(1).min(ordered.len());
        let mut ranked: Vec<usize> = (0..ordered.len()).collect();
        ranked.sort_by(|&i, &j| areas[j].total_cmp(&areas[i]).then(i.cmp(&j)));
        let top: BTreeSet<usize> = ranked.into_iter().take(top_count).collect();
        let top_edges: Vec<(usize, usize)> = adjacency
            .iter()
            .copied()
            .filter(|(i, j)| top.contains(i) && top.contains(j))
            .collect();
        let components = component_count(&top, &top_edges);
        let internal_fraction = if adjacency.is_empty() {
            None
        } else {
            Some(top_edges.len() as f64 / adjacency.len() as f64)
        };
        let member = membership.get(id).unwrap();
        geometry.push(GeometryRow {
            poly_id: id.clone(),
            panel_roles: member.0.iter().cloned().collect(),
            arms: member.1.iter().cloned().collect(),
            buckets: member.2.iter().cloned().collect(),
            input_facet_count: row.facet_count,
            input_sys: row.sys,
            ordered_face_count: ordered.len(),
            ordering_failure_count,
            adjacency_edge_count: adjacency.len(),
            adjacent_abs_difference_mean,
            adjacent_endpoint_pearson,
            top_quartile_face_count: top_count,
            top_quartile_induced_edge_count: top_edges.len(),
            top_quartile_component_count: components,
            top_quartile_internal_edge_fraction: internal_fraction,
        });
        for (idx, face) in ordered.iter().enumerate() {
            faces_out.push(FaceRow {
                poly_id: id.clone(),
                face_index: idx,
                facet_a: face.facets[0],
                facet_b: face.facets[1],
                vertex_count: face.vertices.len(),
                area_over_volume_sqrt: areas[idx],
                adjacency_degree: degrees[idx],
            });
        }
    }
    geometry.sort_by(|a, b| a.poly_id.cmp(&b.poly_id));
    write_jsonl(out.join("geometry-summary.jsonl"), &geometry);
    write_jsonl(out.join("face-areas.jsonl"), &faces_out);
    write_jsonl(out.join("branch-input-table.jsonl"), &branch_inputs);
    write_jsonl(
        out.join("branch-input-provenance.jsonl"),
        &selected_provenance_values,
    );
    let summary = Summary {
        method: "residual-exemplar-incidence-inspection-v1".to_string(), evidence_class: "post-target_G_hypothesis_seed".to_string(),
        panel_path: panel.display().to_string(), table_path: table.display().to_string(),
        provenance_path: provenance.display().to_string(), producer_paths: producer_paths.iter().map(|p| p.display().to_string()).collect(),
        panel_sha256: sha256(&panel), table_sha256: sha256(&table), provenance_sha256: sha256(&provenance),
        producer_sha256: producer_paths.iter().map(|p| sha256(p)).collect(),
        panel_record_count: panel_rows.len(), distinct_polytope_count: wanted.len(), successful_rows: geometry.len(), failures,
        invariance_contract: vec![
            "node identity is an unordered pair of containing facets; all summaries are invariant to facet and vertex relabeling".to_string(),
            "node weight is unsigned symplectic two-face area divided by sqrt(volume), invariant under Sp(4), translation, and positive scaling up to f64 reconstruction".to_string(),
            "two nodes are adjacent exactly when their reconstructed two-faces share at least two vertices (a polytope edge)".to_string(),
            "top quartile uses ceil(face_count/4), ordered by normalized area with face index only as an exact-tie breaker".to_string(),
        ],
        target_boundary: "panel selection used sys/residuals; geometry summaries do not consume sys but remain post-selection G evidence and are not proposer validation or mechanism".to_string(),
    };
    let f = File::create(out.join("summary.json")).expect("create summary");
    serde_json::to_writer_pretty(BufWriter::new(f), &summary).expect("write summary");
}

fn mean(xs: Vec<f64>) -> Option<f64> {
    (!xs.is_empty()).then(|| xs.iter().sum::<f64>() / xs.len() as f64)
}

fn pearson_endpoints(edges: &[(usize, usize)], values: &[f64]) -> Option<f64> {
    if edges.len() < 2 {
        return None;
    }
    let xs: Vec<f64> = edges
        .iter()
        .flat_map(|&(i, j)| [values[i], values[j]])
        .collect();
    let ys: Vec<f64> = edges
        .iter()
        .flat_map(|&(i, j)| [values[j], values[i]])
        .collect();
    let mx = xs.iter().sum::<f64>() / xs.len() as f64;
    let my = ys.iter().sum::<f64>() / ys.len() as f64;
    let cov = xs
        .iter()
        .zip(&ys)
        .map(|(x, y)| (x - mx) * (y - my))
        .sum::<f64>();
    let vx = xs.iter().map(|x| (x - mx).powi(2)).sum::<f64>();
    let vy = ys.iter().map(|y| (y - my).powi(2)).sum::<f64>();
    (vx > 0.0 && vy > 0.0).then(|| cov / (vx * vy).sqrt())
}

fn component_count(nodes: &BTreeSet<usize>, edges: &[(usize, usize)]) -> usize {
    let mut unseen = nodes.clone();
    let mut count = 0;
    while let Some(&start) = unseen.iter().next() {
        count += 1;
        unseen.remove(&start);
        let mut q = VecDeque::from([start]);
        while let Some(v) = q.pop_front() {
            for &(a, b) in edges {
                let n = if a == v {
                    Some(b)
                } else if b == v {
                    Some(a)
                } else {
                    None
                };
                if let Some(n) = n {
                    if unseen.remove(&n) {
                        q.push_back(n);
                    }
                }
            }
        }
    }
    count
}

fn sha256(path: &Path) -> String {
    let mut f = File::open(path).expect("hash open");
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).expect("hash read");
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn write_jsonl<T: Serialize>(path: PathBuf, rows: &[T]) {
    let mut w = BufWriter::new(File::create(path).expect("create jsonl"));
    for row in rows {
        serde_json::to_writer(&mut w, row).expect("json");
        writeln!(w).expect("newline");
    }
}
