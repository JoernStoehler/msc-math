use crate::panel_io::load_jsonl;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct PolytopeRow {
    pub(crate) poly_id: String,
    pub(crate) facet_count: usize,
    pub(crate) capacity_source: String,
    pub(crate) capacity: f64,
    pub(crate) volume: f64,
    pub(crate) sys: f64,
    pub(crate) dual_vertices_f64: Vec<[f64; 4]>,
}

#[derive(Serialize)]
pub(crate) struct ProvenanceRow {
    poly_id: String,
    dataset: String,
    role: String,
    source_name: String,
    seed_index: usize,
    best_strategy: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct BasepointSelectionFacetSummary {
    pub(crate) available_rows: usize,
    pub(crate) selected_rows: usize,
    pub(crate) selected_poly_ids: Vec<String>,
    pub(crate) selected_sys: Vec<f64>,
}

pub(crate) struct BasepointSelection {
    pub(crate) rows: Vec<PolytopeRow>,
    pub(crate) summary_by_facet: BTreeMap<usize, BasepointSelectionFacetSummary>,
}

pub(crate) fn select_basepoints(
    polytope_table: &Path,
    basepoint_counts: &BTreeMap<usize, usize>,
    source: &str,
    selection_seed: &str,
) -> BasepointSelection {
    // Selection is conditional on facet count. Do not interpret the resulting
    // panel as first drawing F independently and then drawing a0 from one
    // common basepoint law.
    let wanted = basepoint_counts.keys().copied().collect::<BTreeSet<_>>();
    let mut by_facet = basepoint_counts
        .keys()
        .map(|facet_count| (*facet_count, Vec::new()))
        .collect::<BTreeMap<_, Vec<PolytopeRow>>>();
    for row in load_jsonl::<PolytopeRow>(polytope_table) {
        if wanted.contains(&row.facet_count) && row.capacity_source == source {
            by_facet.entry(row.facet_count).or_default().push(row);
        }
    }

    let mut selected = Vec::new();
    let mut summary_by_facet = BTreeMap::new();
    for (facet_count, basepoints_per_facet) in basepoint_counts {
        let candidates = by_facet
            .get_mut(facet_count)
            .unwrap_or_else(|| panic!("missing facet bucket F={facet_count}"));
        sort_candidates(candidates, selection_seed);
        assert!(
            candidates.len() >= *basepoints_per_facet,
            "requested {basepoints_per_facet} basepoints for F={facet_count}, found {}",
            candidates.len()
        );
        let chosen = candidates
            .iter()
            .take(*basepoints_per_facet)
            .cloned()
            .collect::<Vec<_>>();
        summary_by_facet.insert(
            *facet_count,
            BasepointSelectionFacetSummary {
                available_rows: candidates.len(),
                selected_rows: chosen.len(),
                selected_poly_ids: chosen.iter().map(|row| row.poly_id.clone()).collect(),
                selected_sys: chosen.iter().map(|row| row.sys).collect(),
            },
        );
        selected.extend(chosen);
    }
    BasepointSelection {
        rows: selected,
        summary_by_facet,
    }
}

pub(crate) fn provenance_rows(panel_rows: &[PolytopeRow], source: &str) -> Vec<ProvenanceRow> {
    panel_rows
        .iter()
        .enumerate()
        .map(|(index, row)| ProvenanceRow {
            poly_id: row.poly_id.clone(),
            dataset: source.to_string(),
            role: "prediction_basepoint".to_string(),
            source_name: format!("{source}_prediction_panel_{index}"),
            seed_index: index,
            best_strategy: None,
        })
        .collect()
}

fn sort_candidates(candidates: &mut [PolytopeRow], selection_seed: &str) {
    candidates.sort_by(|a, b| {
        stable_hash_key(selection_seed, &a.poly_id)
            .cmp(&stable_hash_key(selection_seed, &b.poly_id))
            .then(a.poly_id.cmp(&b.poly_id))
    });
}

fn stable_hash_key(seed: &str, poly_id: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in seed.bytes().chain([0xff]).chain(poly_id.bytes()) {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
