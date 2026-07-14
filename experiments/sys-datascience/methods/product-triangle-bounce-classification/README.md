# Exact rational product-triangle bounce classification

This packet addresses two separate finite predicates on products of two
origin-containing full-dimensional triangles.

1. An all-single three-bounce word is transition-feasible exactly when the
   cross-sign bipartite graph has a directed Hamiltonian six-cycle.
2. On a strict cross-sign cell, every feasible all-single three-bounce QP
   action is strictly below the complete mathematical two-bounce value `A2`.

The packet is a falsifier/support screen, not a theorem and not a physical
trajectory-completeness result. It uses the transition-sufficiency/sign
semantics in `formal/search-pruning-correctness.tex`; zero cross pairings are a
separate boundary stratum and do not support strictness.

## Inputs and provenance

The retained input is the first (and only) 3x3 bucket of the retained product
stream: 1,024 rows from
`experiments/sys-datascience/produce/random-product.jsonl`. The exact stored
class-minimum join is the existing 10,240-row artifact
`../product-bounce-distribution/artifacts/class-minima.jsonl`; it was not
regenerated. The three required LFS inputs were hydrated locally before the
run. Source hashes at implementation time are:

| path | SHA-256 |
|---|---|
| `formal/product-two-bounce-class.tex` | `4363cf62a8bd1877388f7d4184819faf2da8f55e08c87cb91e51b9bfaf47bdd9` |
| `formal/search-pruning-correctness.tex` | `c1463effabe2f210098aa55bae4491591dd8ac263ced68430838e3bed6eb2bfa` |
| `crates/symplectic/src/algorithms/facet_adjacency.rs` | `0f80eaf391a052791fdb531ba92ba473ddcf0a43bdc83018c9b04e4e248f4708` |
| `crates/symplectic/src/algorithms/billiard/block_enumeration.rs` | `805c1c666a915255d26b26aed8da8238027490391058081abd8f8912c0f25f31` |
| `../product-bounce-distribution/README.md` | `bb5026ce7b2636e11daf3cecdedd6914040bb88a79ea782481da38892e81dde1` |
| raw retained 3x3 stream | `66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736` |
| retained class minima | `187089804bd17fdac76bdaf51a8d8202e67fb2b14779fe9a418cc8da47c7b4c4` |

## Two-stage contract

`analyze.py` has an explicit geometry freeze followed by target reveal.

* Geometry freeze reads only each row's name and exact rational dual vertices.
  It records the exact 3x3 cross-sign matrix, strict/zero stratum, a
  canonical phenotype under the nine independent cyclic row/column relabelings,
  all q0-fixed transition-feasible six-cycle identities, exact barycentric QP
  weights/objectives/actions, and `A2`.
* Target reveal joins the frozen names to stored class minima and then reads the
  raw target fields. It reports all-single versus stored A3 availability and
  action, strict dominance, producer label, and the exact check
  `sys = capacity^2/(2 volume)`.

For the complete mathematical `A2`, the implementation uses the proved
difference-body/polar identity `A2 = 1/M`. For a triangle, independently
computed positive barycentrics `lambda_i` give
`(P-P)^circ = conv{+/- lambda_i a_i}`; therefore
`M = max_ij |lambda_i mu_j omega(a_i,b_j)|`. The first 16 retained rows are
also checked by an independent exact half-space reconstruction of both
difference-body polars; all 16 agree exactly.

The QP stage uses each word's exact positive triangle barycentrics with q and p
masses 1/2. Its objective is the direct finite sum
`Q = sum_{j<k} beta_j beta_k omega(a_sigma[j],a_sigma[k])`, and action is
`1/(2Q)`. No stored A3 action is used in this calculation.

## Commands

From the repository root:

```bash
python3 -m unittest discover -s experiments/sys-datascience/methods/product-triangle-bounce-classification -p 'test_*.py'
python3 experiments/sys-datascience/methods/product-triangle-bounce-classification/analyze.py \
  --input experiments/sys-datascience/produce/random-product.jsonl \
  --class-minima experiments/sys-datascience/methods/product-bounce-distribution/artifacts/class-minima.jsonl \
  --out experiments/sys-datascience/methods/product-triangle-bounce-classification/artifacts \
  --stress-count 20000 --stress-seed 20260714
```

The stress count and seed were fixed before interpreting output. The generator
constructs three rational dual vertices from an explicit positive integer
barycentric relation, and checks exact full dimension and origin interiority.
It completed 20,000 pairs in 15.3 seconds wall time (99% CPU, one local
process), well below the ten-minute envelope.

## Results

The generated artifacts are `geometry-freeze.json`, `target-reveal.json`,
`stress-summary.json`, and `summary.json`.

* Retained geometry: 1,024 strict-sign rows, 0 zero-pairing rows, 814 feasible
  q0-fixed all-single words. The exact A2 formula agrees with the independent
  half-space construction on 16/16 checks and with the stored exact 2-bounce
  minima on all 1,024 rows.
* Target reveal: 718 rows have an all-single feasible word and stored A3; 306
  rows have neither. Every one of the 718 rows with a feasible all-single word
  has all such actions strictly below exact A2. Stored A3 is a class-minimum
  stream result and may include pair blocks; equal availability here is an
  observation about this retained stream, not a global availability theorem.
  The stored `sys` identity has maximum absolute error `2.78e-17` from f64
  serialization.
* Stress screen: 19,968 strict-sign pairs and 32 zero-pairing pairs, with
  11,101 feasible all-single words. There are **zero strict-sign
  counterexamples**. The 32 boundary pairs produced 27 exact equality
  witnesses (`A3 = A2`), retained in `stress-summary.json`; they are not
  counted as support for strictness. The smallest strict positive margin is
  `A2-A3 = 1700/1750580447 = 9.7110647095e-7`, at stress index 2546, with
  action/A2 ratio about `0.9999902427`. Its exact word, weights, objective,
  signs, and A2 are retained.

## Disposition and limits

The finite retained and generated inputs support promoting statement 1 as an
action-free classification lemma candidate: the implementation's transition
predicate is exactly the directed Hamiltonian-cycle predicate, and the
observed counts are internally consistent. They do not prove it for all
triangles.

Statement 2 survives this falsifier screen but is not proved. The near-boundary
strict witness and the exact boundary equalities argue for keeping zero cells
separate in any proof. The packet does not establish physical trajectory
completeness, global A3 nonexistence, or any claim outside the retained stream
and the declared rational generator.
