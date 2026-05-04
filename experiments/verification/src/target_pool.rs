//! Target-pool selection and shared cache loading for verification binaries.

use crate::io::RunMode;
use nalgebra::Vector4;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use symplectic::database::{self, DualVerticesKey, PolytopeRecord, Source};
use symplectic::geom::known_polytopes;
use symplectic::geom::polytope::Polytope4D;

pub const GEOMETRY_TOL: f64 = 1e-6;
pub const ACTION_TOL: f64 = 1e-5;
pub const SCALAR_TOL: f64 = 1e-10;
pub const MINIMUM_ACTION_GAP_TOL: f64 = 1e-12;
pub const EXCLUDED_KNOWN_NAMES: &[&str] = &["crosspolytope"];
pub const SMOKE_TARGET_NAMES: &[&str] = &[
    "simplex",
    "hypercube",
    "lagrangian_triangle_product",
    "random_F5_seeded",
    "transformed_0",
];

#[derive(Clone)]
pub struct Target {
    pub name: String,
    pub family: String,
    pub source_kind: String,
    pub polytope: Polytope4D,
    pub expected_min_orbit_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct CorrectnessRow {
    name: String,
    test_group: String,
    dual_vertices: Vec<[f64; 4]>,
}

pub fn build_target_pool(manifest_dir: &Path, mode: RunMode) -> Vec<Target> {
    let db = load_shared_cache(manifest_dir);
    let mut targets = Vec::new();
    targets.extend(build_known_targets());
    targets.extend(build_random_targets(&db));
    targets.extend(build_lagrangian_product_targets(&db));
    targets.extend(build_correctness_targets(manifest_dir));

    let deduped = dedupe_targets(targets);
    match mode {
        RunMode::Smoke => select_smoke_targets(deduped),
        RunMode::Full => deduped,
    }
}

pub fn target_map(manifest_dir: &Path, mode: RunMode) -> HashMap<String, Target> {
    build_target_pool(manifest_dir, mode)
        .into_iter()
        .map(|target| (target.name.clone(), target))
        .collect()
}

fn load_shared_cache(manifest_dir: &Path) -> HashMap<DualVerticesKey, PolytopeRecord> {
    // These are optional shared-catalog inputs. Missing files load as empty via
    // `database::load`; conflicts or parse errors should still fail loudly.
    let paths = [
        manifest_dir.join("orbit-recovery/polytopes.jsonl"),
        manifest_dir.join("../combinatorial-cells/polytopes.jsonl"),
        manifest_dir.join("../sys-landscape/cache.jsonl"),
    ];
    let refs = paths.iter().map(PathBuf::as_path).collect::<Vec<_>>();
    database::load_many(&refs).unwrap_or_else(|err| {
        let path_list = paths
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>()
            .join(", ");
        panic!("failed to load shared verification cache surface from [{path_list}]: {err}")
    })
}

fn build_known_targets() -> Vec<Target> {
    known_polytopes::all_known()
        .into_iter()
        .filter(|kp| !EXCLUDED_KNOWN_NAMES.contains(&kp.name))
        .map(|kp| Target {
            name: kp.name.to_string(),
            family: "known".to_string(),
            source_kind: "known_polytopes".to_string(),
            polytope: kp.polytope.clone(),
            expected_min_orbit_count: expected_min_orbit_count(kp.name),
        })
        .collect()
}

fn build_random_targets(db: &HashMap<DualVerticesKey, PolytopeRecord>) -> Vec<Target> {
    let mut by_facet_count: BTreeMap<usize, (u64, Target)> = BTreeMap::new();

    for record in db.values() {
        let Some(Source::Random {
            attempt,
            facet_count_target,
            ..
        }) = record.source.as_ref()
        else {
            continue;
        };

        let target = Target {
            name: format!("random_F{facet_count_target}_seeded"),
            family: "random".to_string(),
            source_kind: "shared_cache_random_stratum".to_string(),
            polytope: record
                .to_polytope()
                .expect("shared cache row should reconstruct to a polytope"),
            expected_min_orbit_count: None,
        };
        match by_facet_count.get_mut(facet_count_target) {
            Some((best_attempt, best_target)) => {
                if *attempt < *best_attempt {
                    *best_attempt = *attempt;
                    *best_target = target;
                }
            }
            None => {
                by_facet_count.insert(*facet_count_target, (*attempt, target));
            }
        }
    }

    by_facet_count
        .into_values()
        .map(|(_, target)| target)
        .collect()
}

fn build_lagrangian_product_targets(db: &HashMap<DualVerticesKey, PolytopeRecord>) -> Vec<Target> {
    let mut by_pair: BTreeMap<(usize, usize), (String, Target)> = BTreeMap::new();

    for record in db.values() {
        let Some(Source::LagrangianProduct { n1, n2, .. }) = record.source.as_ref() else {
            continue;
        };

        let signature = serde_json::to_string(&record.dual_vertices_rational)
            .expect("failed to serialize dual-vertex signature");
        let target = Target {
            name: format!("lagrangian_product_{n1}x{n2}"),
            family: "lagrangian_product".to_string(),
            source_kind: "shared_cache_pair_stratum".to_string(),
            polytope: record
                .to_polytope()
                .expect("shared cache row should reconstruct to a polytope"),
            expected_min_orbit_count: None,
        };

        match by_pair.get_mut(&(*n1, *n2)) {
            Some((best_signature, best_target)) => {
                if signature < *best_signature {
                    *best_signature = signature;
                    *best_target = target;
                }
            }
            None => {
                by_pair.insert((*n1, *n2), (signature, target));
            }
        }
    }

    by_pair.into_values().map(|(_, target)| target).collect()
}

fn build_correctness_targets(manifest_dir: &Path) -> Vec<Target> {
    let path = manifest_dir.join("correctness/correctness.jsonl");
    let file = File::open(&path).unwrap_or_else(|err| {
        panic!(
            "failed to open correctness dataset {}: {err}",
            path.display()
        )
    });
    let reader = BufReader::new(file);

    let mut by_group = BTreeMap::<String, CorrectnessRow>::new();
    for (line_index, line) in reader.lines().enumerate() {
        let line_number = line_index + 1;
        let line = line.unwrap_or_else(|err| {
            panic!(
                "failed to read correctness row {}:{}: {err}",
                path.display(),
                line_number
            )
        });
        if line.trim().is_empty() {
            continue;
        }
        let row: CorrectnessRow = serde_json::from_str(&line).unwrap_or_else(|err| {
            panic!(
                "failed to parse correctness row {}:{}: {err}",
                path.display(),
                line_number
            )
        });
        if !matches!(
            row.test_group.as_str(),
            "scaled" | "transformed" | "perturbed"
        ) {
            continue;
        }
        by_group
            .entry(row.test_group.clone())
            .and_modify(|current| {
                if row.name < current.name {
                    *current = CorrectnessRow {
                        name: row.name.clone(),
                        test_group: row.test_group.clone(),
                        dual_vertices: row.dual_vertices.clone(),
                    };
                }
            })
            .or_insert(row);
    }

    by_group
        .into_values()
        .map(|row| {
            let dual_vertices = row
                .dual_vertices
                .iter()
                .map(|coords| Vector4::new(coords[0], coords[1], coords[2], coords[3]))
                .collect();
            let polytope = Polytope4D::from_f64(dual_vertices).unwrap_or_else(|err| {
                panic!(
                    "failed to reconstruct correctness target {} from {}: {err:?}",
                    row.name,
                    path.display()
                )
            });
            Target {
                name: row.name,
                family: format!("correctness_{}", row.test_group),
                source_kind: "correctness_dataset".to_string(),
                polytope,
                expected_min_orbit_count: None,
            }
        })
        .collect()
}

fn dedupe_targets(targets: Vec<Target>) -> Vec<Target> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();

    for target in targets {
        let key = target.polytope.dual_vertices().to_vec();
        if seen.insert(key) {
            deduped.push(target);
        }
    }

    deduped
}

fn select_smoke_targets(targets: Vec<Target>) -> Vec<Target> {
    let by_name = targets
        .into_iter()
        .map(|target| (target.name.clone(), target))
        .collect::<HashMap<_, _>>();

    SMOKE_TARGET_NAMES
        .iter()
        .map(|name| {
            by_name
                .get(*name)
                .unwrap_or_else(|| panic!("smoke target {name} missing from target pool"))
                .clone()
        })
        .collect()
}

fn expected_min_orbit_count(name: &str) -> Option<usize> {
    match name {
        "simplex" => Some(6),
        "hypercube" => Some(2),
        _ => None,
    }
}
