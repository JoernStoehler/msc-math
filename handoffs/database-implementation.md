# Handoff: Polytope Database Implementation

**Created**: 2026-04-02
**Status**: Ready to implement
**Branch**: Create a new worktree for this work

## Task

Implement the shared polytope database crate at `crates/database/`. This enables
cross-experiment data reuse: polytope rational data, volume, capacity, and sigma
lists are computed once and cached in a JSONL file.

## What to Build

Three pieces of work, in order:

### 1. Library change: `Polytope4D::from_rational_parts` constructor

**File**: `crates/library/src/geom/polytope.rs`

Add a constructor that takes pre-computed rational dual vertices and rational vertices
(skipping vertex enumeration). It must:

- Recompute `vertex_descriptors: Vec<BTreeSet<usize>>` by checking, for each vertex v
  and facet f, whether `a_f · v = 1` (exact rational dot product). This is O(V·F).
- Convert rationals to f64 via `rational_to_f64` (in `geom/rational_arithmetic.rs`).
- Call the existing private `assemble()` method (line ~284) which builds incidence,
  omega_signs, vertex_adjacency, and stores all fields.

Test: construct a polytope via `from_f64()`, then reconstruct via
`from_rational_parts(p.dual_vertices().to_vec(), p.vertices().to_vec())`.
Verify identical incidence, omega_signs, adjacency matrices.

### 2. Database crate: `crates/database/`

**Existing stub**: `crates/database/src/lib.rs` has a skeleton docstring. Rewrite it.

The full API spec with types, functions, and doc comments is in the plan below.
Key points:

- `PolytopeRecord` struct with `BigRational` fields and serde derives
- `load(path) → HashMap`, `save(path, &HashMap)` functions
- `from_polytope()`, `to_polytope()`, `with_computed_fields()`, `with_sigmas()` methods
- `save()` uses atomic write: write to tmp file, then `std::fs::rename`
- `load()` handles empty/missing file gracefully (return empty HashMap)

**Dependency changes needed**:
- `Cargo.toml` (workspace root): change `num-rational = "0.4"` to
  `num-rational = { version = "0.4", features = ["serde"] }`. Same for `num-bigint`.
  Add `blake3 = "1"`.
- `crates/database/Cargo.toml`: add `num-rational`, `num-bigint`, `blake3` as
  workspace deps.

### 3. Library change: `generate_polytope` PRNG function

**File**: `crates/library/src/random.rs`

Add a function that generates a single polytope attempt with an independent seed
derived from `(master_seed, attempt)` via `blake3::derive_key`. See plan for exact
code. This does NOT replace the existing `sample_random_polytope` — it wraps it
with deterministic independent seeding.

## Important: Things NOT to Do

- **Do NOT canonicalize facet ordering.** Different orderings of dual vertices produce
  different HashMap keys. This is intentional.
- **Do NOT store `dual_vertices_f64`** in PolytopeRecord. `f64_to_rational` is lossless
  (IEEE-754 exact conversion); compute f64 on demand via `rational_to_f64`.
- **Do NOT store `sys`** (systolic ratio). It's trivially `capacity² / (2 * volume)`.
- **Do NOT add custom serde for BigRational.** Use the default derive (enabled by the
  `serde` feature on `num-rational`).
- **Do NOT create an index file or custom lookup structure.** The HashMap built from
  full-file load is sufficient.
- **Do NOT modify existing experiments.** The database is new infrastructure. Existing
  experiments continue to work unchanged.

## Verification Checklist

Run these before presenting work:

1. `cd crates/library/ && cargo test --release --lib` — existing tests pass
2. `cd crates/library/ && cargo clippy --lib -- -D warnings` — clean
3. Round-trip test: `Polytope4D → PolytopeRecord → JSON string → PolytopeRecord → Polytope4D` produces matching capacity/volume
4. `from_rational_parts` produces identical incidence/omega_signs/adjacency as `from_f64`
5. `generate_polytope(seed, 0)` ≠ `generate_polytope(seed, 1)`, and both are reproducible
6. `save()` then `load()` round-trips the HashMap exactly
7. Progressive fill: record with only rational data deserializes, then adding computed fields and re-serializing also works

## Full Plan

The complete type definitions, function signatures, doc comments, and architectural
decisions are below.

---

## Architecture

```
On disk:    data/polytopes.jsonl  (flat list of PolytopeRecord objects)

On load:    parse every line → HashMap<DualVerticesKey, PolytopeRecord>
            (key = rational dual vertices, extracted from each record)
            (HashMap hashes the key internally)

On sync:    serialize entire HashMap → write tmp file → atomic rename

Per process: load() at start, sync() at end. One read, one write.
```

**SLURM concurrency**: Each SLURM job writes to its own output file. A merge step
combines per-job files into the main database after all jobs complete. No concurrent
writes to the same file.

## Layer Stack

```
Polytope4D, CapacityResult, ...     (library types — unchanged)
        ↕
PolytopeRecord struct (serde Serialize/Deserialize, Option fields for progressive fill)
        ↕
serde + serde_json                  (already in deps)
        ↕
JSONL file                          (one JSON object per line)
        ↕
disk
```

## Database Crate API (`crates/database/`)

### Types

```rust
/// The key type: rational dual vertices.
/// BigRational implements Hash + Eq, so this works as a HashMap key.
pub type DualVerticesKey = Vec<[BigRational; 4]>;

/// A database record. Fields beyond dual_vertices_rational are progressively filled.
#[derive(Serialize, Deserialize, Clone)]
pub struct PolytopeRecord {
    // Always present — the defining data
    pub dual_vertices_rational: Vec<[BigRational; 4]>,
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

#[derive(Serialize, Deserialize, Clone)]
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
```

### Functions

```rust
/// Load database from JSONL file → HashMap.
/// Each line is a JSON-serialized PolytopeRecord.
/// The key is extracted from each record's dual_vertices_rational.
/// Returns empty HashMap if file doesn't exist or is empty.
pub fn load(path: &Path) -> io::Result<HashMap<DualVerticesKey, PolytopeRecord>>

/// Save HashMap → JSONL file (atomic: write tmp file, then rename).
pub fn save(path: &Path, db: &HashMap<DualVerticesKey, PolytopeRecord>) -> io::Result<()>

impl PolytopeRecord {
    /// Build from a constructed Polytope4D. Stores rational dual vertices
    /// and rational vertices (cached to avoid re-running vertex enumeration).
    pub fn from_polytope(p: &Polytope4D) -> PolytopeRecord

    /// Add volume + capacity results to an existing record.
    pub fn with_computed_fields(self, volume: f64, volume_err: f64,
                                capacity: f64, capacity_err: f64) -> PolytopeRecord

    /// Add sigma list to an existing record.
    /// sigma_gap_cutoff: action gap above best_action within which sigmas were collected.
    pub fn with_sigmas(self, sigmas: Vec<SigmaAction>, gap_cutoff: f64) -> PolytopeRecord

    /// Reconstruct Polytope4D from cached rational data.
    /// Recomputes vertex_descriptors via O(V·F) rational dot products,
    /// then runs assemble() for incidence/omega/adjacency/f64 copies.
    /// Does NOT re-run vertex enumeration (the expensive O(C(F,4)) step).
    pub fn to_polytope(&self) -> Result<Polytope4D, ConstructionError>

    /// Extract the HashMap key from this record.
    pub fn key(&self) -> DualVerticesKey
}
```

## Library Changes

### New constructor: `Polytope4D::from_rational_parts`

**File**: `crates/library/src/geom/polytope.rs`

```rust
/// Construct from pre-computed rational dual vertices and vertices.
///
/// Recomputes vertex_descriptors by checking, for each vertex v and facet f,
/// whether a_f · v = 1 (exact rational dot product). This is O(V·F) — much
/// cheaper than vertex enumeration which is O(C(F,4)).
///
/// Then calls assemble() to build incidence, omega_signs, vertex_adjacency,
/// and f64 copies.
pub fn from_rational_parts(
    dual_vertices: Vec<[BigRational; 4]>,
    vertices: Vec<[BigRational; 4]>,
) -> Result<Self, ConstructionError>
```

### PRNG generation: counter + hash

**File**: `crates/library/src/random.rs`

```rust
/// Generate a single polytope attempt with an independent seed.
/// The (master_seed, attempt) pair fully determines the attempt.
///
/// Uses blake3 key derivation to produce a 32-byte seed from
/// (master_seed, attempt), then seeds ChaCha8Rng for the actual
/// random number generation.
pub fn generate_polytope(
    facet_count: usize, h_min: f64, h_max: f64,
    master_seed: u64, attempt: u64,
) -> Result<Polytope4D, ConstructionError> {
    let mut key_material = [0u8; 16];
    key_material[..8].copy_from_slice(&master_seed.to_le_bytes());
    key_material[8..].copy_from_slice(&attempt.to_le_bytes());
    let seed = blake3::derive_key("polytope-gen", &key_material);
    let mut rng = ChaCha8Rng::from_seed(seed);
    sample_random_polytope(facet_count, h_min, h_max, &mut rng)
}
```

## Dependency Changes

- `num-rational`: change to `{ version = "0.4", features = ["serde"] }`
- `num-bigint`: change to `{ version = "0.4", features = ["serde"] }`
- `blake3`: add `"1"` to workspace deps

## Files to Create / Modify

| File | Action | What |
|------|--------|------|
| `crates/database/src/lib.rs` | Rewrite | PolytopeRecord, DualVerticesKey, Source, SigmaAction, load(), save() |
| `crates/database/Cargo.toml` | Edit | Add num-rational, num-bigint (with serde features), blake3 |
| `crates/library/src/geom/polytope.rs` | Edit | Add `from_rational_parts` constructor |
| `crates/library/src/random.rs` | Edit | Add `generate_polytope` with counter+hash PRNG |
| `Cargo.toml` (workspace root) | Edit | Add blake3, enable serde features on num-rational/num-bigint |
| `data/polytopes.jsonl` | Create | Initially empty |
