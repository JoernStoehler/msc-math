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
//!
//! **Functions:**
//! - [`load(path)`] → `HashMap<DualVerticesKey, PolytopeRecord>` (empty if file missing)
//! - [`save(path, &HashMap)`] — atomic write (tmp file + rename)
//!
//! **Record methods:**
//! - [`PolytopeRecord::from_polytope(&Polytope4D)`] — create from constructed polytope
//! - [`.to_polytope()`] → `Polytope4D` — reconstruct via `from_rational_parts` (skips vertex enumeration)
//! - [`.with_computed_fields(volume, volume_err, capacity, capacity_err)`] — add capacity/volume
//! - [`.with_sigmas(sigmas, gap_cutoff)`] — add sigma list
//! - [`.key()`] → `DualVerticesKey` — extract the HashMap key
//!
//! # Usage pattern
//!
//! ```rust,ignore
//! let mut db = database::load(&db_path)?;
//!
//! for polytope in &my_polytopes {
//!     let key = polytope.dual_vertices().to_vec();
//!     let record = db.entry(key).or_insert_with(|| {
//!         PolytopeRecord::from_polytope(polytope)
//!     });
//!     if record.capacity.is_none() {
//!         let cap = ehz_capacity(polytope);
//!         let vol = volume(polytope);
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
use std::io::{self, BufRead, Write};
use std::path::Path;
use crate::{ConstructionError, Polytope4D};

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
            .map(|row| {
                std::array::from_fn(|i| format!("{}/{}", row[i].numer(), row[i].denom()))
            })
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
                    let (n, d) = s
                        .split_once('/')
                        .ok_or_else(|| serde::de::Error::custom(
                            format!("expected 'numer/denom', got {:?}", s),
                        ))?;
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
}

/// A near-optimal cyclic permutation and its action value.
#[derive(Serialize, Deserialize, Clone)]
pub struct SigmaAction {
    /// Cyclic permutation of facet indices into `dual_vertices_rational`.
    /// Normalized: perm[0] = min(perm). Represents the facet visit order
    /// of a candidate Reeb orbit.
    pub perm: Vec<usize>,
    /// Action of this orbit: 0.5 / Q(beta), where Q is the KKT dual objective.
    /// Smaller action = tighter capacity bound.
    pub action: f64,
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

impl PolytopeRecord {
    /// Build from a constructed Polytope4D. Stores rational dual vertices
    /// and rational vertices (cached to avoid re-running vertex enumeration).
    pub fn from_polytope(p: &Polytope4D) -> PolytopeRecord {
        PolytopeRecord {
            dual_vertices_rational: p.dual_vertices().to_vec(),
            vertices_rational: p.vertices().to_vec(),
            source: None,
            volume: None,
            volume_err: None,
            capacity: None,
            capacity_err: None,
            sigma_gap_cutoff: None,
            sigmas: None,
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

    /// Reconstruct Polytope4D from cached rational data.
    /// Recomputes vertex_descriptors via O(V·F) rational dot products,
    /// then runs assemble() for incidence/omega/adjacency/f64 copies.
    /// Does NOT re-run vertex enumeration (the expensive O(C(F,4)) step).
    pub fn to_polytope(&self) -> Result<Polytope4D, ConstructionError> {
        Polytope4D::from_rational_parts(
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

/// Save HashMap → JSONL file (atomic: write tmp file, then rename).
pub fn save(path: &Path, db: &HashMap<DualVerticesKey, PolytopeRecord>) -> io::Result<()> {
    let parent = path.parent().unwrap_or(Path::new("."));
    std::fs::create_dir_all(parent)?;

    let tmp_path = path.with_extension("jsonl.tmp");
    {
        let file = std::fs::File::create(&tmp_path)?;
        let mut writer = io::BufWriter::new(file);
        for record in db.values() {
            serde_json::to_writer(&mut writer, record).map_err(|e| {
                io::Error::other(format!("serialize: {}", e))
            })?;
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
    use nalgebra::Vector4;
    use num_bigint::BigInt;

    /// Helper: build a simple polytope for testing.
    fn test_polytope() -> Polytope4D {
        let halfspaces = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
            -Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
        ];
        Polytope4D::from_f64(halfspaces).unwrap()
    }

    /// Round-trip: Polytope4D → PolytopeRecord → JSON → PolytopeRecord → Polytope4D
    #[test]
    fn round_trip_polytope_json_polytope() {
        let p = test_polytope();
        let record = PolytopeRecord::from_polytope(&p);
        let json = serde_json::to_string(&record).unwrap();
        let record2: PolytopeRecord = serde_json::from_str(&json).unwrap();
        let p2 = record2.to_polytope().unwrap();

        assert_eq!(p.facet_count(), p2.facet_count());
        assert_eq!(p.incidence(), p2.incidence());
        assert_eq!(p.omega_signs(), p2.omega_signs());
        assert_eq!(p.vertex_adjacency(), p2.vertex_adjacency());
    }

    /// Verify JSON output uses "numer/denom" strings, not u32 limb arrays.
    #[test]
    fn json_format_human_readable() {
        let p = test_polytope();
        let record = PolytopeRecord::from_polytope(&p);
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
        let p = test_polytope();
        let record = PolytopeRecord::from_polytope(&p)
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
        let p = test_polytope();

        // Stage 1: just rational data
        let record = PolytopeRecord::from_polytope(&p);
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
}
