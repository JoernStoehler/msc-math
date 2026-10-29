# Exact twenty-facet HKO improvement at t=3/10

Producer-frozen 29 October 2026, 00:41 UTC; integrated by the paper owner after scoped independent mathematical and fresh-copy replay PASS. The reports are `../../reviews/hko-t03-mathematics.md` and `../../reviews/hko-t03-independent-replay.md`. The assembled r4 article received a separate mathematical review; later article revisions require their own delta review. This packet contains no binary executables, NumPy arrays or floating-point discovery predicates. The original producer manifest had SHA-256 `238ec061eb3750f12cf630a89fe936a42ab16ab5279a6bf690b9207654a8bc0e`; only this README's audit-status text and its line in `MANIFEST.sha256` changed for the companion.

Claim supported by the finite checks and COVERAGE-PROOF.md:

- K=H∩L_(3/10)⁻¹H has exactly20genuine facets,75simple vertices.
- Qmax=(205/1019)tau, so c(K)=1019/(410tau)=(1019/1025)c(H).
- V(K)=300748771/52178240−99664289tau²/260891200.
- rho(K)=1.0519141388294875…, exceeding HKO by0.0047005433295293… .

Exact normals are reconstructed at the top of exact_certify.py, overQ(tau), tau⁴−10tau²+5=0. An exactAAembedding explicitly asserts0<tau<1 and the definingquartic. Symmetries used by the word quotient are verified exactly there. No undocumented external field module is imported.

## Replay

Prerequisites: SageMath Python (number fields andAA), C++17 compiler. Run **from this directory** with Sage's Python asSAGE_PYTHON:

```
$SAGE_PYTHON exact_attainer.py
$SAGE_PYTHON exact_geometry.py
$SAGE_PYTHON exact_certify.py
$SAGE_PYTHON exact_refined_leaves.py
g++ -O2 -std=c++17 coverage_replay.cpp -o coverage_replay
./coverage_replay > coverage-summary.json
$SAGE_PYTHON verify_result.py
```

On the task hostSAGE_PYTHON is `/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python`. Producer timings: geometry7.2s; all77,927 exactcurvature exclusions360s;2326 exactleaf words69s; integercoverage a fewseconds. Outputs are overwritten only inside the replay directory; replay a freshcopy to preserve frozen outputs.

`exact_certify.py` in this frozenpacket ends after its curvature check. The earlier development script had an additional nonsingular-only diagnostic scan; that unneeded tail was removed without changing any acceptance predicate. `exact_refined_leaves.py` is the authoritative complete leaf check. Its curvature-count metadata describes a separate input obligation; only exact_certify.py and its separate receipt establish that curvature PASS.

`exclusions.txt` contains77,927 canonical words with certified positive tangent curvature. `leaves.txt` contains2326canonical candidate words. `graph-0.3.txt` is an explicit integer adjacency matrix, fully reconstructed/validated by exact_geometry.py. The integer DFS replay proves every simple directedcycle either contains a certified excludedprefix or occurs in thecheckedleafset. Its acceptance traversal makes no numerical curvature decisions.

All singular KKT families are handled.984have objective≤the explicitly attainedtarget;21are impossible byH/(28/25)⊂K and capacitymonotonicity;25are exactlyinfeasible by full basic-feasible-parameter vertexenumeration. No singularstationary family remains unresolved. The checker does not infer infeasibility from the sign of one particular solution.

## Mathematical inputs and limits

COVERAGE-PROOF.md records the necessary geometric simple-minimizer theorem (Haim–Kislev2019,Theorem1.5), transition necessity, exactgraph completeness, cyclic curvatureinheritance, quotient symmetries, allstationary/singular handling and scalarcapacity implication. Lawrence's simple-polytope volumeformula was sourcechecked in his1989NISTreport §2 (https://nvlpubs.nist.gov/nistpubs/Legacy/IR/nistir89-4123.pdf); two exact genericfunctionals give the samevolume. Twenty-facet genuineness is explicitly asserted by the completeactive-labelunion andsimplevertex certificates.

This fixes one moderate parameter. It does not optimize the family or establish a global bound over other bodies. Discovery scripts and failed loose bounds remain outside this acceptance packet.
