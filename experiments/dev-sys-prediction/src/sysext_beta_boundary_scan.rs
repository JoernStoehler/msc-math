use exp_sys_landscape::SysLandscapePolytopeCache;
use nalgebra::{DVector, Vector4};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use symplectic::algorithms::facet_adjacency::build_transition_matrix_from_facet_intersections_and_omega;
use symplectic::algorithms::hk2017::for_each_sigma_pruned_by_transition;

const EPS_EIGEN_FLOOR: f64 = 1.0e-10;
const EPS_KKT_RESIDUAL: f64 = 1.0e-7;
const EPS_Q_POSITIVE: f64 = 1.0e-12;

#[derive(Debug)]
struct Cli {
    polytope_table: PathBuf,
    out: PathBuf,
    capacity_source: Option<String>,
    facet_counts: Option<BTreeSet<usize>>,
    max_rows: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
struct PolytopeRow {
    poly_id: String,
    facet_count: usize,
    capacity_source: String,
    dual_vertices_f64: Vec<[f64; 4]>,
}

#[derive(Serialize)]
struct ScanRow {
    poly_id: String,
    facet_count: usize,
    capacity_source: String,
    status: String,
    raw_ok_branches: usize,
    raw_failed_branches: usize,
    min_abs_beta_margin: Option<f64>,
    min_positive_beta_margin: Option<f64>,
    closest_invalid_beta_margin: Option<f64>,
    near_abs_1e_6: usize,
    near_abs_1e_5: usize,
    near_abs_1e_4: usize,
    near_abs_1e_3: usize,
    near_abs_1e_2: usize,
}

struct RawKktResult {
    beta: Vec<f64>,
    q_corrected: f64,
    residual_norm: f64,
}

pub fn run_from_args(argv: impl IntoIterator<Item = impl Into<String>>) {
    let cli = parse_args_from(argv);
    let rows: Vec<PolytopeRow> = load_jsonl(&cli.polytope_table);
    let mut out = Vec::new();
    for row in rows.into_iter().filter(|row| include_row(row, &cli)) {
        if let Some(max_rows) = cli.max_rows {
            if out.len() >= max_rows {
                break;
            }
        }
        out.push(scan_row(row));
    }
    write_jsonl(&cli.out, &out).expect("write output");
    eprintln!("wrote {} rows to {}", out.len(), cli.out.display());
}

fn include_row(row: &PolytopeRow, cli: &Cli) -> bool {
    if let Some(source) = &cli.capacity_source {
        if row.capacity_source != *source {
            return false;
        }
    }
    if let Some(facet_counts) = &cli.facet_counts {
        if !facet_counts.contains(&row.facet_count) {
            return false;
        }
    }
    true
}

fn scan_row(row: PolytopeRow) -> ScanRow {
    let duals = row
        .dual_vertices_f64
        .iter()
        .map(|v| Vector4::new(v[0], v[1], v[2], v[3]))
        .collect::<Vec<_>>();
    let Some(polytope) = SysLandscapePolytopeCache::from_f64_dual_vertices(duals) else {
        return failed_scan_row(row, "polytope_construction_failed");
    };
    let transition = build_transition_matrix_from_facet_intersections_and_omega(
        &polytope.facet_intersection_is_nonempty,
        &polytope.omega_signs,
    );

    let mut raw_ok_branches = 0usize;
    let mut raw_failed_branches = 0usize;
    let mut margins = Vec::new();
    for_each_sigma_pruned_by_transition(&transition, |sigma| {
        match solve_raw_sysext_kkt_for_dual_vertices(&polytope.dual_vertices_f64, sigma) {
            Ok(raw) => {
                if raw.q_corrected.is_finite() && raw.residual_norm.is_finite() {
                    raw_ok_branches += 1;
                    margins.push(raw.beta.iter().copied().fold(f64::INFINITY, f64::min));
                } else {
                    raw_failed_branches += 1;
                }
            }
            Err(_) => {
                raw_failed_branches += 1;
            }
        }
    });

    let min_abs_beta_margin = margins
        .iter()
        .copied()
        .filter(|m| m.is_finite())
        .min_by(|a, b| a.abs().total_cmp(&b.abs()));
    let min_positive_beta_margin = margins
        .iter()
        .copied()
        .filter(|m| m.is_finite() && *m > 0.0)
        .min_by(|a, b| a.total_cmp(b));
    let closest_invalid_beta_margin = margins
        .iter()
        .copied()
        .filter(|m| m.is_finite() && *m <= 0.0)
        .max_by(|a, b| a.total_cmp(b));

    ScanRow {
        poly_id: row.poly_id,
        facet_count: row.facet_count,
        capacity_source: row.capacity_source,
        status: "ok".to_string(),
        raw_ok_branches,
        raw_failed_branches,
        min_abs_beta_margin,
        min_positive_beta_margin,
        closest_invalid_beta_margin,
        near_abs_1e_6: count_abs_near(&margins, 1.0e-6),
        near_abs_1e_5: count_abs_near(&margins, 1.0e-5),
        near_abs_1e_4: count_abs_near(&margins, 1.0e-4),
        near_abs_1e_3: count_abs_near(&margins, 1.0e-3),
        near_abs_1e_2: count_abs_near(&margins, 1.0e-2),
    }
}

fn count_abs_near(margins: &[f64], threshold: f64) -> usize {
    margins
        .iter()
        .filter(|margin| margin.is_finite() && margin.abs() <= threshold)
        .count()
}

fn failed_scan_row(row: PolytopeRow, status: &str) -> ScanRow {
    ScanRow {
        poly_id: row.poly_id,
        facet_count: row.facet_count,
        capacity_source: row.capacity_source,
        status: status.to_string(),
        raw_ok_branches: 0,
        raw_failed_branches: 0,
        min_abs_beta_margin: None,
        min_positive_beta_margin: None,
        closest_invalid_beta_margin: None,
        near_abs_1e_6: 0,
        near_abs_1e_5: 0,
        near_abs_1e_4: 0,
        near_abs_1e_3: 0,
        near_abs_1e_2: 0,
    }
}

fn solve_raw_sysext_kkt_for_dual_vertices(
    dual_vertices: &[Vector4<f64>],
    sigma: &[usize],
) -> Result<RawKktResult, String> {
    let (kkt, rhs) = symplectic::kkt::qp_assembly::build_augmented_system_from_dual_vertices(
        dual_vertices,
        sigma,
    );
    let m = rhs.len() - 5;
    let size = rhs.len();
    let eig = kkt.clone().symmetric_eigen();
    let max_abs_ev = eig
        .eigenvalues
        .iter()
        .map(|e| e.abs())
        .fold(0.0f64, f64::max);
    if max_abs_ev < EPS_EIGEN_FLOOR {
        return Err("singular_matrix".to_string());
    }

    let mut x0 = DVector::zeros(size);
    for i in 0..size {
        if eig.eigenvalues[i].abs() > EPS_EIGEN_FLOOR {
            let coeff = eig.eigenvectors.column(i).dot(&rhs) / eig.eigenvalues[i];
            for j in 0..size {
                x0[j] += coeff * eig.eigenvectors[(j, i)];
            }
        }
    }
    let residual = &kkt * &x0 - rhs;
    let residual_norm = residual.norm();
    if residual_norm > EPS_KKT_RESIDUAL {
        return Err("residual_too_large".to_string());
    }
    let r2_dot_mu: f64 = (m..m + 4).map(|i| residual[i] * x0[i]).sum();
    let q_correction = r2_dot_mu + residual[m + 4] * x0[m + 4];
    let beta: Vec<f64> = (0..m).map(|i| x0[i]).collect();
    let mut q_raw = 0.0;
    for i in 0..m {
        for j in 0..m {
            q_raw += beta[i] * kkt[(i, j)] * beta[j];
        }
    }
    q_raw *= 0.5;
    let q_corrected = q_raw + q_correction;
    if q_corrected <= EPS_Q_POSITIVE {
        return Err("nonpositive_q".to_string());
    }
    Ok(RawKktResult {
        beta,
        q_corrected,
        residual_norm,
    })
}

fn parse_args_from(argv: impl IntoIterator<Item = impl Into<String>>) -> Cli {
    let mut cli = Cli {
        polytope_table: PathBuf::from("experiments/sys-datascience/prepare/polytope-table.jsonl"),
        out: std::env::temp_dir().join("dataset-panel-sysext-beta-boundary-scan.jsonl"),
        capacity_source: None,
        facet_counts: None,
        max_rows: None,
    };
    let mut args = argv.into_iter().map(Into::into).skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--polytope-table" => {
                cli.polytope_table =
                    PathBuf::from(args.next().expect("--polytope-table requires a path"));
            }
            "--out" => {
                cli.out = PathBuf::from(args.next().expect("--out requires a path"));
            }
            "--capacity-source" => {
                cli.capacity_source =
                    Some(args.next().expect("--capacity-source requires a value"));
            }
            "--facet-counts" => {
                cli.facet_counts = Some(
                    args.next()
                        .expect("--facet-counts requires csv")
                        .split(',')
                        .filter(|value| !value.is_empty())
                        .map(|value| value.parse().expect("facet count must be usize"))
                        .collect(),
                );
            }
            "--max-rows" => {
                cli.max_rows = Some(
                    args.next()
                        .expect("--max-rows requires usize")
                        .parse()
                        .expect("--max-rows must be usize"),
                );
            }
            "--help" | "-h" => {
                eprintln!(
                    "Usage: sysext_beta_boundary_scan module args: [--polytope-table PATH] [--out PATH] [--capacity-source SOURCE] [--facet-counts CSV] [--max-rows N]"
                );
                std::process::exit(0);
            }
            other => panic!("unsupported argument: {other}"),
        }
    }
    cli
}

fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file = File::open(path).unwrap_or_else(|err| panic!("open {}: {err}", path.display()));
    BufReader::new(file)
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line =
                line.unwrap_or_else(|err| panic!("read {}:{}: {err}", path.display(), idx + 1));
            (!line.trim().is_empty()).then(|| {
                serde_json::from_str(&line)
                    .unwrap_or_else(|err| panic!("parse {}:{}: {err}", path.display(), idx + 1))
            })
        })
        .collect()
}

fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}
