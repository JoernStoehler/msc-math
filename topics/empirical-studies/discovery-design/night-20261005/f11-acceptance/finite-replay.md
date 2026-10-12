# F≤11 finite replay and predicate audit

Owner: `/root/f11_acceptance_lead/finite_replay`; receiving owner: `/root/f11_acceptance_lead`.

Status: **finite replay accepted**, all seven stages passed. One isolated full run started 2026-10-04 22:52:27 UTC and completed naturally after **387.039 seconds**. No further replay or producer was run.

The frozen packet at `/tmp/msc-math/night-20261005/f11/packet` was copied to exclusive scratch `/tmp/msc-math/night-20261005/f11/replay/packet`. No manuscript, original certificate or retained input was edited. The selected invocation is one full `run_checks.py --negative-controls` run, with outputs under fresh scratch `results/`. The audit followed the invoked imports and relevant proof texts before execution.

## Execution safety and dependencies

The invoked import chain is `run_checks.py`; inherited `exact_core.py`, `check_new.py`, `duplicate_workspace.py`; core `verify.py`, `exact_tools.py`, `exact_core.py`, `duplicate_workspace.py`; collar `verify.py`, `geometry.py`, `polynomial.py`, `integer_bernstein.py`. Those paths use only Python standard-library modules. The NumPy/SciPy numerical LP import inside `duplicate_workspace.probe` is dormant and uncalled. No optimizer, discovery producer, network or paid API is invoked.

Writes on the executed paths are supplied output JSON, per-stage logs, summary and incidental child-interpreter bytecode within the scratch copy. Parent `python3 -B` does not propagate `-B` to subprocess interpreters. The master rejects an existing output directory; inherited endpoints opens its JSON with `w`, but this invocation gives it a fresh path. Negative controls mutate in-memory parsed witnesses and do not modify data files.

Environment: `/usr/bin/python3`, Python 3.12.3, GCC 13.3.0, Linux 6.8.0-146-generic x86_64, glibc 2.39; no `PYTHONOPTIMIZE`, ordinary assertions enabled. All 37 entries in the supplied `evidence/verified_code_data_sha256.json` match actual bytes. A full 140-file source snapshot manifest is retained.

## Coverage and intentionally omitted predicates

Core roots are rebuilt from the polar of the difference pentagon and the two interval-axis directions. The root permutation is a full permutation of 60 tetrahedra. Tree traversal requires unique visits, valid midpoint children, all nodes reached and all terminal nodes certified exactly once. Every listed touching row is checked, every constructed relation cancels all forty old coordinates, and every leaf vertex is compared exactly. Family completeness is unnecessary: the individual rows used are checked directly.

Collar arrangement geometry enumerates all nonsingular triples of the defining planes inside the base prism, assigns every choice of zero depth signs, retains full-dimensional cells and triangulates their facets exactly. Singular plane triples are skipped because they do not define isolated vertices; rank-deficient cells follow by continuity. Facet branches with fewer than three noncollinear vertices do not define two-dimensional facets. Every resulting root is traversed and all leaves have exact nonnegative degree-nine homogeneous coefficients. There is no numerical failure or depth-limit acceptance branch.

Two static premises are less explicitly guarded by the general verifier: `Checker(cone)` indexes the policy list by cone, and geometry selects the triangle from `root_permutation[3*cone]`. Checked on the actual frozen packet: policy cone IDs equal their indices 0–19; permutation blocks are complete consecutive prism triples; the prism IDs are `[7,8,9,0,1,2,3,4,5,6,17,18,19,10,11,12,13,14,15,16]`, exactly all twenty. The selected first-three-vertex triangles, after subtracting `eta*a0`, are twenty distinct exact triangles equal to the full natural tilt cover. Each corner has exact maximum zero and minimum minus one on the ten facet vertices. These are accepted static premises for this packet, not a demonstrated coverage hole.

The only ordinary `assert` in the invoked proof code checks that the polar polygon has ten vertices; it is enabled in this replay. All other acceptance checks call `require`, which raises and cannot be disabled by optimization. Core literal differentiated-section crosschecks are subsampled; primary feasibility, stationarity, algebraic derivatives and relation checks are performed for all rows. Collar literal midpoint derivative crosschecks are also supplementary and subsampled.

Collar calls `calc_exact(check=False)` at each collapsed start. This deliberately avoids asserting strict positivity at a zero-new-weight limit. Strict-positive midpoint and endpoint checks provide affine constraints and stationarity at two interpolation nodes; the objective is checked at all three nodes and is quadratic. Therefore the path identities extend to the entire affine path. The conversion from its boundary response to a genuine positive-section witness still uses the analytic closure/anchor argument.

## Finite conclusions and analytic boundary

The core predicates certify exact positive touching sections, their constraints/objective/stationarity/derivatives, full forty-coordinate cancellation and a gap at every vertex of the whole core cover. Convexity of the positive-part volume integral then extends this to every normalized direction with `1/10 <= mu <= 9/10`.

The collar predicates certify complete affine weight paths, nonnegative polynomial policy masses, all mass-plus-forty-gradient coefficient identities, uniformly nonnegative anchor corrections on `[0,1]^10`, and complete exact polynomial positivity on the lower collar. The planar vertex-cone formula and its cleared polynomial are crosschecked against clipped-pentagon Fubini integration at one exact interior point of every sign cell. These point comparisons supplement the algebraic construction and supplied derivation; they do not alone establish a rational identity on a cell.

Inherited scripts establish contact, cancellation, quadrature, symmetry and finite-bank obstruction identities. Their success does not independently establish uniform endpoint estimates or other-face neighborhoods.

The remaining mathematical arrows are the polar-loop capacity upper bound and convention; smooth feasible continuation; symmetry annihilation and the closure correction; convex interpolation and integral formula; nonnegative homogeneous-coefficient implication; boundary continuity; duplicate-label interchange for the upper collar; inherited uniform cap/remainder/endpoint arguments; polar Hausdorff compactness and the final contradiction. The supplied sharp transverse ten-facet theorem and local symmetry chart are accepted inputs, not re-proved. `PROOF.md` explicitly claims an existential neighborhood, no numerical radius or equality classification, no unrestricted-facet theorem and no proof-assistant formalization. Historical `inherited/math/REMAINING_AND_HAUSDORFF.md` reports a missing interior argument at its earlier stage; the current `PROOF.md` explicitly supersedes it with the core/collar certificates.

This replay can accept the finite arithmetic and coverage predicates of the frozen packet; it cannot on its own accept all those analytic dependencies or the full local maximality theorem.

## Actual replay results

| Stage | Result | Return code | Seconds |
| --- | --- | ---: | ---: |
| `inherited_ridges` | PASS | 0 | 6.701 |
| `inherited_endpoints` | PASS | 0 | 11.866 |
| `core` | PASS | 0 | 262.052 |
| `collar` | PASS | 0 | 66.362 |
| `reject_missing_core_leaf` | PASS | 1 | 0.498 |
| `reject_negative_policy_weight` | PASS | 1 | 0.195 |
| `reject_false_polynomial_bound` | PASS | 1 | 39.363 |

The three controls failed at the intended messages: `every leaf certified exactly once`, `nonnegative path weight`, and `nonnegative exact homogeneous coefficients`. Their three `SHOULD_NOT_EXIST` success JSON files are absent. A nonzero exit alone would not have sufficed: the master checked the expected predicate text too.

Core: 3,258 touching rows, 824 relations, 1,968 covering leaves, 704 exact volume points and 13 supplementary literal-gradient comparisons. The exact minimum vertex gap is retained in `core.json`; its informational decimal is approximately `1.039106033025555e-6`, exceeding the exact required `1/1000000`. The informational smallest base weight is approximately `1.7274631464864187e-10`; this supplies no numerical spatial radius.

Collar: 257 paths, twenty policies, three supplementary literal-gradient comparisons, 160 sign cells, 880 reconstructed roots, 887 leaves and 195,140 coefficients, of which 55,928 are exactly zero. All 160 exact Fubini/polynomial comparisons and 500 supplementary field-sign comparisons passed. The informational maximum anchor mix is approximately `2.2966457902118265e-9`.

Every original snapshot file and every corresponding scratch input remained byte-identical across the run: 140/140 in each place. Scratch added only 8 bytecode files in `__pycache__` directories. The four positive stages' mathematical output payloads are identical to the archive's historical clean replay after removing only timing/interpreter/platform/command fields; exact algebraic values, counts, margins and input hashes match.

The retained evidence is beside this report in [`finite-replay/`](finite-replay): [master log](finite-replay/master.log), [stage summary](finite-replay/SUMMARY.json), all seven complete stdout/stderr logs and four positive output JSONs, [environment](finite-replay/environment.json), [full source hashes](finite-replay/source-sha256.json), [claimed manifest comparison](finite-replay/claimed-manifest-check.json), [static domain verification](finite-replay/static-domain.json), [post-run integrity](finite-replay/integrity.json), and [retained output hashes](finite-replay/output-sha256.json). The latter covers every retained file except itself and this report.

Key executed source SHA-256 values:

- `run_checks.py`: `73aab664d4a307c5e6a8687531bb9ea5659a20e481af3f07686502bcdc94b0a0`
- `core/verify.py`: `1ac68d95a82b83b85253b02f188220c4cf640f50405daa948f9fa050df069e46`
- `collar/verify.py`: `5e998d02cdc230bc7de33335d4ecef548eef10f67746d0d9a63c36a1684d9423`

The three negative controls provide evidence of sensitivity to those three defects. They are not exhaustive adversarial testing of the checker. No failed predicate or unexplained replay difference remains. The finite review stops here; theorem integration and the accepted analytic dependencies remain with the receiving owner.
