# Exact local-cover certificate rerun, 22 October 2026

Private report for the bounded local-cover mathematical review. Inspected checkout `/workspaces/msc-math-local-cover-review-20261022` at HEAD `5193274f835bc1e48108fe43a0377fc3edc00904`; run completed about 2026-10-22 13:42 UTC. No source, manuscript, registry, or build edit was made. The checkout had an existing/concurrent modification to `meta-topics/project-management/current.json`; this work did not touch it.

## Execution identity and result

Directory: `topics/products/pentagon-arbitrary-partner/local-cover`

Command (with `PYTHONDONTWRITEBYTECODE=1`, shell CPU limit 30 seconds):

```sh
PYTHONDONTWRITEBYTECODE=1 timeout 120s /usr/bin/time -v -o /tmp/msc-math/local-cover-review/20261022/certificate-sol/time.txt uv run --no-project --offline --with sympy==1.14.0 python verify_local_cover.py --fixture-code pentagon_fixture.py --out /tmp/msc-math/local-cover-review/20261022/certificate-sol/result.json
```

`uv` 0.12.11, environment Python 3.13.3 and SymPy 1.14.0. The existing offline cache sufficed; no install campaign. Exit 0; stdout: `PASS: exact 50-marginal certificate, rank 18, unique base placements.` The checker reported internal 1.10846647 seconds; `/usr/bin/time` recorded 2.07 seconds wall, 1.51 seconds user plus 0.13 seconds system CPU, and 57,168 KB peak resident memory. `time.txt` and `result.json` are the fresh evidence.

SHA-256:

- `verify_local_cover.py`: `65f3f6265210970e4394cb199213b171a51c190c56c661d3a1902ce9b08d4f5e`
- `pentagon_fixture.py`: `3e4c9b5efd8ae424d50c0b3b27a433da6e49a80b868a8e6b97239db59eef9a88`
- fresh `result.json`: `2be12a35d39d64840fcf485fa9e7f315ccafd52a6e77def3f55053ed3fa6de0e`
- retained `verification/20260924.json`: `2660b62143df418cf64cd9a4e248f53dda86d96d7d1d98cb52ff173b4165f961`

The fresh result agrees with the retained output exactly after removing the nondeterministic `elapsed_seconds` key.

## Predicates and mathematical interface

The imported executable fixture constructs ten oriented triangle templates from five positive normal-support triples, finds exact feasible translations in the candidate pentagon, verifies the placed vertices' hull equals the candidate pentagon, and asserts the candidate area ratio. The verifier converts coordinates into ordered Q(sqrt(5)) after dividing y by tau, checks the converted hull equality, builds the ten vertices of H-H, and for every one of the ten placed triangles finds two H-vertex endpoints whose directed difference is a vertex of H-H. The fresh output lists the same ten endpoint-index pairs as the retained result. These are precisely the finite predicates used by the preliminary continuum review's exposed-difference argument for uniqueness of a template placement inside H. The exposed-face and compactness implications themselves are mathematical arguments, not checker assertions.

For the first-order certificate, the verifier finds five incident triangle labels at each of H's five vertices; the ten incoming/outgoing slots each have five label choices. It builds the 28-by-50 system: 18 anchored translation-gradient cancellation rows and ten per-slot normalization rows. It asserts exact rank 28, solves for 50 marginals, checks exact constraints and positivity, and asserts rank 18 of within-slot gradient differences. The smallest marginal is `33/218 - 39 sqrt(5)/1090`, approximately 0.07137004484175065. The full 5^10 gradient bank is counted implicitly, not enumerated. Given the note's product-distribution and convex-geometric argument, these finite facts support zero lying in the interior of the convex hull of its stated first-order rows. The checker does not prove that the rows form a valid first-order lower model for hull area; that relies on the local signed-polygon estimate under independent continuum review.

The JSON `interpretation` field asserts local rigidity conditional on that estimate. Treat `PASS` as confirmation of these exact finite computations only. It neither certifies the winding/deletion estimate, the passage to arbitrary nearby convex bodies, template theorem/capacity assumptions, nor an unrestricted global optimum. The preliminary continuum note explicitly had not rerun the arithmetic; this run fills that execution gap, subject to review of the source-to-lemma interface.

## Write effects

The script refuses an existing output file and writes JSON only to the supplied `--out` path. Importing the fixture can otherwise produce Python bytecode; this run disabled bytecode writes. The command did not create repository files. The `uv` offline invocation may use its existing cache and transient environment outside the checkout. The separate version query and JSON comparison were read-only apart from ordinary cache access.
