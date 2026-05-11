//! Polytope JSONL storage for experiment-owned caches and datasets.
//!
//! # Why
//!
//! Experiments store rational polytope records in one or more local JSONL files.
//! The helpers in this module read and write those files, but the caller owns
//! the path policy: there is no canonical mutable shared cache path here.
//!
//! # Public API
//!
//! **Types:**
//! - [`PolytopeRecord`] — one database row: rational geometry + optional capacity/volume/sigmas
//! - [`DualVerticesKey`] — `Vec<[BigRational; 4]>`, the HashMap key (rational dual vertices)
//! - [`Source`] — provenance enum: `Random { master_seed, attempt, ... }`,
//!   `LagrangianProduct { n1, n2, ... }`, `Known { name }`
//! - [`SigmaAction`] — one near-optimal orbit: `perm: Vec<usize>` + `action: f64`
//! - [`OrbitScalars`] — bounded search / best-orbit scalar payload
//!
//! **Functions:**
//! - [`load(path)`] → `HashMap<DualVerticesKey, PolytopeRecord>` (empty if file missing)
//! - [`save(path, &HashMap)`] — atomic write (tmp file + rename)
//!
//! **Record methods:**
//! - [`PolytopeRecord::from_dual_vertices_and_vertices`] — create from explicit rational geometry
//! - [`.dual_vertices_and_vertices()`] — clone explicit rational geometry for consumers
//! - [`.with_computed_fields(volume, volume_err, capacity, capacity_err)`] — add capacity/volume
//! - [`.with_sigmas(sigmas, gap_cutoff)`] — add sigma list
//! - [`.key()`] → `DualVerticesKey` — extract the HashMap key
//!
//! # Usage pattern
//!
//! ```rust,ignore
//! let mut db = database::load(&db_path)?;
//!
//! for (dual_vertices, vertices) in &my_flat_polytopes {
//!     let key = dual_vertices.clone();
//!     let record = db.entry(key).or_insert_with(|| {
//!         PolytopeRecord::from_dual_vertices_and_vertices(dual_vertices.clone(), vertices.clone())
//!     });
//!     if record.capacity.is_none() {
//!         let cap = my_experiment_capacity(dual_vertices);
//!         let vol = my_experiment_volume_f64(dual_vertices, vertices);
//!         *record = record.clone().with_computed_fields(vol, 0.0, cap, 0.0);
//!     }
//!     // Use record.capacity, record.sigmas, etc.
//! }
//!
//! database::save(&db_path, &db)?;
//! ```
//!
//! # Storage format
//!
//! JSONL (one JSON object per line). BigRational values are serialized as
//! `"numerator/denominator"` strings (e.g. `"3/7"`, `"-1/2"`) for human
//! readability and `jq`/Python compatibility.

use num_rational::BigRational;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

/// The key type: rational dual vertices.
/// BigRational implements Hash + Eq, so this works as a HashMap key.
pub type DualVerticesKey = Vec<[BigRational; 4]>;

/// Serde module for `Vec<[BigRational; 4]>` as `Vec<["numer/denom"; 4]>`.
///
/// Each BigRational is serialized as a JSON string `"numerator/denominator"`.
/// Example: `BigRational(3, 7)` → `"3/7"`, `BigRational(-1, 2)` → `"-1/2"`.
mod rational_vec4_serde {
    use num_bigint::BigInt;
    use num_rational::BigRational;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::str::FromStr;

    type Vec4 = Vec<[BigRational; 4]>;

    pub fn serialize<S: Serializer>(data: &Vec4, serializer: S) -> Result<S::Ok, S::Error> {
        let string_data: Vec<[String; 4]> = data
            .iter()
            .map(|row| std::array::from_fn(|i| format!("{}/{}", row[i].numer(), row[i].denom())))
            .collect();
        string_data.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec4, D::Error> {
        let string_data: Vec<[String; 4]> = Deserialize::deserialize(deserializer)?;
        string_data
            .into_iter()
            .map(|row| {
                let mut arr = [(); 4].map(|_| BigRational::default());
                for (i, s) in row.iter().enumerate() {
                    let (n, d) = s.split_once('/').ok_or_else(|| {
                        serde::de::Error::custom(format!("expected 'numer/denom', got {:?}", s))
                    })?;
                    let numer = BigInt::from_str(n)
                        .map_err(|e| serde::de::Error::custom(format!("bad numerator: {e}")))?;
                    let denom = BigInt::from_str(d)
                        .map_err(|e| serde::de::Error::custom(format!("bad denominator: {e}")))?;
                    arr[i] = BigRational::new(numer, denom);
                }
                Ok(arr)
            })
            .collect::<Result<Vec4, _>>()
    }
}

/// A database record. Fields beyond dual_vertices_rational are progressively filled.
#[derive(Serialize, Deserialize, Clone)]
pub struct PolytopeRecord {
    // Always present — the defining data
    #[serde(with = "rational_vec4_serde")]
    pub dual_vertices_rational: Vec<[BigRational; 4]>,
    #[serde(with = "rational_vec4_serde")]
    pub vertices_rational: Vec<[BigRational; 4]>,

    // Source / provenance
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    // Computed results — filled progressively
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_err: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity_err: Option<f64>,

    // Sigma list — expensive, filled separately
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigma_gap_cutoff: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sigmas: Option<Vec<SigmaAction>>,

    // Orbit/KKT scalar payload — filled when a richer orbit result is available
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orbit_scalars: Option<OrbitScalars>,
}

/// A near-optimal cyclic permutation and its action value.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SigmaAction {
    /// Cyclic permutation of facet indices into `dual_vertices_rational`.
    /// Normalized: perm[0] = min(perm). Represents the facet visit order
    /// of a candidate Reeb orbit.
    pub perm: Vec<usize>,
    /// Action of this orbit: 0.5 / Q(beta), where Q is the KKT dual objective.
    /// Smaller action = tighter capacity bound.
    pub action: f64,
}

/// Bounded search-level and best-orbit scalar payload.
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct OrbitScalars {
    /// Number of sigma candidates examined by the frontend.
    pub iterations: u64,
    /// Number of orbits retained in the shared result payload.
    pub returned_orbit_count: usize,
    /// Convenience scalar `min(beta)` of the best orbit.
    pub best_beta_margin: f64,
    /// Absolute error bound for the best orbit's `q`.
    pub best_q_error_bound: f64,
    /// Whether the best orbit payload carries closure multipliers.
    pub best_has_mu: bool,
    /// Whether the best orbit payload carries the normalization multiplier.
    pub best_has_xi: bool,
    /// Whether the best orbit required exact admissibility certification.
    pub best_is_admissible_exact: bool,
    /// Whether the best orbit remained indeterminate in the f64 path.
    pub best_is_indeterminate_f64: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "family")]
pub enum Source {
    #[serde(rename = "random")]
    Random {
        master_seed: u64,
        attempt: u64,
        facet_count_target: usize,
        h_min: f64,
        h_max: f64,
    },
    #[serde(rename = "lagrangian_product")]
    LagrangianProduct {
        n1: usize,
        n2: usize,
        circumradius_q: f64,
        circumradius_p: f64,
        rotation_p_rad: f64,
    },
    #[serde(rename = "known")]
    Known { name: String },
}

#[derive(Debug)]
pub enum MergeError {
    Io {
        path: PathBuf,
        source: io::Error,
    },
    Conflict {
        first_path: PathBuf,
        second_path: PathBuf,
        field: &'static str,
    },
}

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MergeError::Io { path, source } => {
                write!(f, "{}: {}", path.display(), source)
            }
            MergeError::Conflict {
                first_path,
                second_path,
                field,
            } => write!(
                f,
                "conflicting field '{}' while merging {} and {}",
                field,
                first_path.display(),
                second_path.display()
            ),
        }
    }
}

impl std::error::Error for MergeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MergeError::Io { source, .. } => Some(source),
            MergeError::Conflict { .. } => None,
        }
    }
}

impl PolytopeRecord {
    /// Build from explicit rational dual vertices and rational primal vertices.
    pub fn from_dual_vertices_and_vertices(
        dual_vertices_rational: Vec<[BigRational; 4]>,
        vertices_rational: Vec<[BigRational; 4]>,
    ) -> PolytopeRecord {
        PolytopeRecord {
            dual_vertices_rational,
            vertices_rational,
            source: None,
            volume: None,
            volume_err: None,
            capacity: None,
            capacity_err: None,
            sigma_gap_cutoff: None,
            sigmas: None,
            orbit_scalars: None,
        }
    }

    /// Add volume + capacity results to an existing record.
    pub fn with_computed_fields(
        mut self,
        volume: f64,
        volume_err: f64,
        capacity: f64,
        capacity_err: f64,
    ) -> PolytopeRecord {
        self.volume = Some(volume);
        self.volume_err = Some(volume_err);
        self.capacity = Some(capacity);
        self.capacity_err = Some(capacity_err);
        self
    }

    /// Add sigma list to an existing record.
    /// sigma_gap_cutoff: action gap above best_action within which sigmas were collected.
    pub fn with_sigmas(mut self, sigmas: Vec<SigmaAction>, gap_cutoff: f64) -> PolytopeRecord {
        self.sigmas = Some(sigmas);
        self.sigma_gap_cutoff = Some(gap_cutoff);
        self
    }

    /// Add bounded orbit/KKT scalar payload to an existing record.
    pub fn with_orbit_scalars(mut self, scalars: OrbitScalars) -> PolytopeRecord {
        self.orbit_scalars = Some(scalars);
        self
    }

    /// Clone the explicit rational geometry stored in this record.
    pub fn dual_vertices_and_vertices(&self) -> (Vec<[BigRational; 4]>, Vec<[BigRational; 4]>) {
        (
            self.dual_vertices_rational.clone(),
            self.vertices_rational.clone(),
        )
    }

    /// Extract the HashMap key from this record.
    pub fn key(&self) -> DualVerticesKey {
        self.dual_vertices_rational.clone()
    }
}

/// Load database from JSONL file → HashMap.
/// Each line is a JSON-serialized PolytopeRecord.
/// The key is extracted from each record's dual_vertices_rational.
/// Returns empty HashMap if file doesn't exist or is empty.
pub fn load(path: &Path) -> io::Result<HashMap<DualVerticesKey, PolytopeRecord>> {
    let file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(HashMap::new()),
        Err(e) => return Err(e),
    };

    let reader = io::BufReader::new(file);
    let mut db = HashMap::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let record: PolytopeRecord = serde_json::from_str(&line).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("line {}: {}", line_num + 1, e),
            )
        })?;
        db.insert(record.key(), record);
    }

    Ok(db)
}

fn merge_option_field<T: Clone + PartialEq>(
    dest: &mut Option<T>,
    src: &Option<T>,
    first_path: &Path,
    second_path: &Path,
    field: &'static str,
) -> Result<(), MergeError> {
    match (dest.as_ref(), src.as_ref()) {
        (None, Some(value)) => *dest = Some(value.clone()),
        (Some(left), Some(right)) if left != right => {
            return Err(MergeError::Conflict {
                first_path: first_path.to_path_buf(),
                second_path: second_path.to_path_buf(),
                field,
            });
        }
        _ => {}
    }
    Ok(())
}

fn merge_record(
    dest: &mut PolytopeRecord,
    src: &PolytopeRecord,
    first_path: &Path,
    second_path: &Path,
) -> Result<(), MergeError> {
    if dest.dual_vertices_rational != src.dual_vertices_rational {
        return Err(MergeError::Conflict {
            first_path: first_path.to_path_buf(),
            second_path: second_path.to_path_buf(),
            field: "dual_vertices_rational",
        });
    }
    if dest.vertices_rational != src.vertices_rational {
        return Err(MergeError::Conflict {
            first_path: first_path.to_path_buf(),
            second_path: second_path.to_path_buf(),
            field: "vertices_rational",
        });
    }

    merge_option_field(
        &mut dest.source,
        &src.source,
        first_path,
        second_path,
        "source",
    )?;
    merge_option_field(
        &mut dest.volume,
        &src.volume,
        first_path,
        second_path,
        "volume",
    )?;
    merge_option_field(
        &mut dest.volume_err,
        &src.volume_err,
        first_path,
        second_path,
        "volume_err",
    )?;
    merge_option_field(
        &mut dest.capacity,
        &src.capacity,
        first_path,
        second_path,
        "capacity",
    )?;
    merge_option_field(
        &mut dest.capacity_err,
        &src.capacity_err,
        first_path,
        second_path,
        "capacity_err",
    )?;
    merge_option_field(
        &mut dest.sigma_gap_cutoff,
        &src.sigma_gap_cutoff,
        first_path,
        second_path,
        "sigma_gap_cutoff",
    )?;
    merge_option_field(
        &mut dest.sigmas,
        &src.sigmas,
        first_path,
        second_path,
        "sigmas",
    )?;
    merge_option_field(
        &mut dest.orbit_scalars,
        &src.orbit_scalars,
        first_path,
        second_path,
        "orbit_scalars",
    )?;
    Ok(())
}

/// Load and fieldwise-merge multiple JSONL files.
///
/// Later files may fill missing fields in earlier records for the same polytope.
/// If two files disagree on a concrete field value for the same polytope, this
/// returns a conflict error instead of silently choosing one.
pub fn load_many(paths: &[&Path]) -> Result<HashMap<DualVerticesKey, PolytopeRecord>, MergeError> {
    let mut merged: HashMap<DualVerticesKey, PolytopeRecord> = HashMap::new();
    let mut origins: HashMap<DualVerticesKey, PathBuf> = HashMap::new();

    for path in paths {
        let db = load(path).map_err(|source| MergeError::Io {
            path: (*path).to_path_buf(),
            source,
        })?;
        for (key, record) in db {
            if let Some(existing) = merged.get_mut(&key) {
                let first_path = origins
                    .get(&key)
                    .expect("origin should exist for merged record")
                    .clone();
                merge_record(existing, &record, &first_path, path)?;
            } else {
                origins.insert(key.clone(), (*path).to_path_buf());
                merged.insert(key, record);
            }
        }
    }

    Ok(merged)
}

/// Save HashMap → JSONL file (atomic: write tmp file, then rename).
pub fn save(path: &Path, db: &HashMap<DualVerticesKey, PolytopeRecord>) -> io::Result<()> {
    let parent = path.parent().unwrap_or(Path::new("."));
    std::fs::create_dir_all(parent)?;

    let tmp_path = path.with_extension("jsonl.tmp");
    {
        let file = std::fs::File::create(&tmp_path)?;
        let mut writer = io::BufWriter::new(file);
        for record in db.values() {
            serde_json::to_writer(&mut writer, record)
                .map_err(|e| io::Error::other(format!("serialize: {}", e)))?;
            writer.write_all(b"\n")?;
        }
        writer.flush()?;
    }

    std::fs::rename(&tmp_path, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::vertex_enumeration::{
        construct_rational_pipeline, facet_intersection_is_nonempty_from_incidence,
        omega_signs_from_rational_dual_vertices, rationalize_f64_dual_vertices,
        vertex_facet_incidence_from_descriptors,
    };
    use nalgebra::DMatrix;
    use nalgebra::Vector4;
    use num_bigint::BigInt;

    #[derive(Clone)]
    struct TestGeometry {
        dual_vertices: Vec<[BigRational; 4]>,
        vertices: Vec<[BigRational; 4]>,
        vertex_facet_incidence: DMatrix<bool>,
        facet_intersection_is_nonempty: DMatrix<bool>,
        omega_signs: DMatrix<i8>,
    }

    /// Helper: build simple flat rational geometry for testing.
    fn test_geometry() -> TestGeometry {
        let halfspaces = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            -Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
        ];
        let dual_vertices = rationalize_f64_dual_vertices(&halfspaces).unwrap();
        let (vertices, vertex_descriptors) = construct_rational_pipeline(&dual_vertices).unwrap();
        let vertex_facet_incidence =
            vertex_facet_incidence_from_descriptors(&vertex_descriptors, dual_vertices.len());
        let facet_intersection_is_nonempty =
            facet_intersection_is_nonempty_from_incidence(&vertex_facet_incidence);
        let omega_signs = omega_signs_from_rational_dual_vertices(&dual_vertices);

        TestGeometry {
            dual_vertices,
            vertices,
            vertex_facet_incidence,
            facet_intersection_is_nonempty,
            omega_signs,
        }
    }

    /// Round-trip: rational geometry -> PolytopeRecord -> JSON -> rational geometry.
    #[test]
    fn round_trip_rational_geometry_json() {
        let geometry = test_geometry();
        let record = PolytopeRecord::from_dual_vertices_and_vertices(
            geometry.dual_vertices.clone(),
            geometry.vertices.clone(),
        );
        let json = serde_json::to_string(&record).unwrap();
        let record2: PolytopeRecord = serde_json::from_str(&json).unwrap();
        let (dual_vertices, vertices) = record2.dual_vertices_and_vertices();
        let (_, vertex_descriptors) = construct_rational_pipeline(&dual_vertices).unwrap();
        let vertex_facet_incidence =
            vertex_facet_incidence_from_descriptors(&vertex_descriptors, dual_vertices.len());

        assert_eq!(geometry.dual_vertices, dual_vertices);
        assert_eq!(geometry.vertices, vertices);
        assert_eq!(geometry.vertex_facet_incidence, vertex_facet_incidence);
        assert_eq!(
            geometry.omega_signs,
            omega_signs_from_rational_dual_vertices(&dual_vertices)
        );
        assert_eq!(
            geometry.facet_intersection_is_nonempty,
            facet_intersection_is_nonempty_from_incidence(&vertex_facet_incidence)
        );
    }

    /// Verify JSON output uses "numer/denom" strings, not u32 limb arrays.
    #[test]
    fn json_format_human_readable() {
        let geometry = test_geometry();
        let record = PolytopeRecord::from_dual_vertices_and_vertices(
            geometry.dual_vertices,
            geometry.vertices,
        );
        let json = serde_json::to_string_pretty(&record).unwrap();

        // The first dual vertex is (1, 0, 0, 0) from f64,
        // so the rational should be "1/1" or similar simple fraction.
        assert!(
            json.contains('/'),
            "JSON should contain 'numer/denom' strings"
        );
        // Should NOT contain the u32 limb format [[sign, [limbs...]],...]
        assert!(
            !json.contains("[[1,["),
            "JSON should not use default BigInt limb format"
        );
    }

    /// save() then load() round-trips the HashMap exactly.
    #[test]
    fn save_load_round_trip() {
        let geometry = test_geometry();
        let record = PolytopeRecord::from_dual_vertices_and_vertices(
            geometry.dual_vertices,
            geometry.vertices,
        )
        .with_computed_fields(1.23, 0.01, 4.56, 0.02);

        let mut db = HashMap::new();
        db.insert(record.key(), record);

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.jsonl");

        save(&path, &db).unwrap();
        let loaded = load(&path).unwrap();

        assert_eq!(loaded.len(), 1);
        let (key, rec) = loaded.iter().next().unwrap();
        assert_eq!(key, &db.keys().next().unwrap().clone());
        assert_eq!(rec.volume, Some(1.23));
        assert_eq!(rec.capacity, Some(4.56));
    }

    /// load() returns empty HashMap for nonexistent file.
    #[test]
    fn load_missing_file() {
        let db = load(Path::new("/nonexistent/path.jsonl")).unwrap();
        assert!(db.is_empty());
    }

    /// Progressive fill: record without computed fields deserializes,
    /// then adding computed fields and re-serializing works.
    #[test]
    fn progressive_fill() {
        let geometry = test_geometry();

        // Stage 1: just rational data
        let record = PolytopeRecord::from_dual_vertices_and_vertices(
            geometry.dual_vertices,
            geometry.vertices,
        );
        assert!(record.volume.is_none());
        assert!(record.sigmas.is_none());

        let json1 = serde_json::to_string(&record).unwrap();
        let record: PolytopeRecord = serde_json::from_str(&json1).unwrap();

        // Stage 2: add computed fields
        let record = record.with_computed_fields(1.0, 0.01, 2.0, 0.02);
        assert_eq!(record.volume, Some(1.0));
        assert!(record.sigmas.is_none());

        let json2 = serde_json::to_string(&record).unwrap();
        let record: PolytopeRecord = serde_json::from_str(&json2).unwrap();

        // Stage 3: add sigmas
        let sigmas = vec![SigmaAction {
            perm: vec![0, 1, 2],
            action: 0.5,
        }];
        let record = record.with_sigmas(sigmas, 0.1);
        assert!(record.sigmas.is_some());

        let json3 = serde_json::to_string(&record).unwrap();
        let record: PolytopeRecord = serde_json::from_str(&json3).unwrap();
        assert_eq!(record.volume, Some(1.0));
        assert_eq!(record.sigmas.as_ref().unwrap().len(), 1);
    }

    /// Source enum serializes with tagged union format.
    #[test]
    fn source_serde() {
        let src = Source::Random {
            master_seed: 42,
            attempt: 7,
            facet_count_target: 6,
            h_min: 0.5,
            h_max: 2.0,
        };
        let json = serde_json::to_string(&src).unwrap();
        assert!(json.contains("\"family\":\"random\""));
        let _: Source = serde_json::from_str(&json).unwrap();
    }

    /// DualVerticesKey works as HashMap key (Hash + Eq on BigRational).
    #[test]
    fn dual_vertices_key_hash_eq() {
        let one = BigRational::new(BigInt::from(1), BigInt::from(1));
        let two = BigRational::new(BigInt::from(2), BigInt::from(1));

        let key1: DualVerticesKey = vec![[one.clone(), one.clone(), one.clone(), one.clone()]];
        let key2: DualVerticesKey = vec![[one.clone(), one.clone(), one.clone(), two.clone()]];

        let mut map: HashMap<DualVerticesKey, &str> = HashMap::new();
        map.insert(key1.clone(), "a");
        map.insert(key2.clone(), "b");

        assert_eq!(map.len(), 2);
        assert_eq!(map[&key1], "a");
        assert_eq!(map[&key2], "b");
    }

    /// load_many() fills missing fields from later files.
    #[test]
    fn load_many_merges_missing_fields() {
        let geometry = test_geometry();
        let key = geometry.dual_vertices.clone();
        let base = PolytopeRecord::from_dual_vertices_and_vertices(
            geometry.dual_vertices,
            geometry.vertices,
        );
        let mut enriched = base.clone();
        enriched.capacity = Some(4.5);

        let dir = tempfile::tempdir().unwrap();
        let path_a = dir.path().join("a.jsonl");
        let path_b = dir.path().join("b.jsonl");

        let mut db_a = HashMap::new();
        db_a.insert(key.clone(), base);
        save(&path_a, &db_a).unwrap();

        let mut db_b = HashMap::new();
        db_b.insert(key.clone(), enriched);
        save(&path_b, &db_b).unwrap();

        let merged = load_many(&[path_a.as_path(), path_b.as_path()]).unwrap();
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[&key].capacity, Some(4.5));
    }

    /// load_many() fails loudly when two files disagree on a concrete field.
    #[test]
    fn load_many_rejects_conflicting_fields() {
        let geometry = test_geometry();
        let key = geometry.dual_vertices.clone();

        let mut left = PolytopeRecord::from_dual_vertices_and_vertices(
            geometry.dual_vertices,
            geometry.vertices,
        );
        left.capacity = Some(4.5);
        let mut right = left.clone();
        right.capacity = Some(5.5);

        let dir = tempfile::tempdir().unwrap();
        let path_a = dir.path().join("a.jsonl");
        let path_b = dir.path().join("b.jsonl");

        let mut db_a = HashMap::new();
        db_a.insert(key.clone(), left);
        save(&path_a, &db_a).unwrap();

        let mut db_b = HashMap::new();
        db_b.insert(key, right);
        save(&path_b, &db_b).unwrap();

        let err = match load_many(&[path_a.as_path(), path_b.as_path()]) {
            Ok(_) => panic!("expected merge conflict"),
            Err(err) => err,
        };
        match err {
            MergeError::Conflict { field, .. } => assert_eq!(field, "capacity"),
            other => panic!("expected conflict, got {other}"),
        }
    }
}
