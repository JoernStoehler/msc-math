# Simultaneous vertex and edge cuts of HKO

New research, 16 October 2026. The main mathematical result is the multicut
lifting theorem in [PROOF.md](PROOF.md): **arbitrarily many simultaneous cuts
cannot improve HKO locally when their limiting supporting normals remain in
compact vertex/edge exposure charts**. The constants do not depend on the
number of added inequalities. Equality in this restricted class consists of
HKO's usual symmetry images. This extends the previously retained one-cut
vertex/edge argument; it does not re-prove or enlarge the unrestricted F≤11
theorem to all F≥12 polytopes.

The genuinely new observation is a union estimate: all removed caps together
lie in a tube around finitely many primal edges/vertices. Their volume is
O(delta_max^3), rather than a bound proportional to the number of cuts.
One deepest cut already provides an O(delta_max^2) capacity decrease through
a feasible Haim–Kislev word. This also allows facet counts growing without
bound along a sequence, provided the stated angular separation persists.

The proof takes the accepted sharp F10 theorem and the existing verified
vertex-insertion construction as inputs. Its analytic union/coupling argument
is new. The unrestricted ≥12-facet question remains open: normals approaching
polar edges or vertices expose primal two-faces or facets, where the cubic
volume estimate fails. Several cuts concentrating near those strata are not
covered by the accepted one-extra-facet proof.

## Numerical experiment

[probe.py](probe.py) reuses the accepted F11 archive's exact HKO geometry and
65 positive touching HK seed words. It simultaneously adds 25 vertex cuts,
50 edge cuts, or all 75, at four radial depths. It estimates volume by Qhull
and searches single-insertion HK weights from every old seed, slot and added
normal. Its capacity numbers are **nominal
estimates of feasible-HK upper bounds**, never exhaustive capacity evaluations
or rigorous numerical enclosures. Constraint residuals are checked in binary64.
All arithmetic in this experiment is binary64 after fixture reconstruction.

The vertex-only panel has 35 genuine facets; the edge-only panel has 60
(the counts here are numerical incidence checks). The mixed 75-inequality
panel has the same body as the edge-only panel: the extra vertex cuts are
exactly redundant by the averaged-normal identity in PROOF.md, §6. It is
therefore not a distinct 85-facet example.

The retained [probe-results.json](probe-results.json) records all 12 cases,
constraints residuals, witnesses, software versions and archive/script hashes.
Every nominal systolic upper-bound estimate is below the HKO benchmark. This is
finite numerical evidence only; the proof supplies no explicit numerical
Hausdorff radius. No result depends on interpreting floating-point output as
an exact certificate.

[verify_geometry.py](verify_geometry.py) additionally checks all 25 exact
redundancy identities, the 25 vertex/50 edge exposures, and 1,750 positive
support gaps, using the accepted archive arithmetic helper. Its
[exact-geometry-results.json](exact-geometry-results.json) records PASS.
This reuses the helper; it is not an independent audit of its arithmetic.

Reproduce from the repository root, writing to a fresh location:

```bash
uv run topics/variation-and-local-maxima/hko-local-maximum/multicut-research-20261016/probe.py \
  --repo "$PWD" --out /tmp/msc-math/hko-20261016/multicut/fresh-results.json
```

The numerical script has inline NumPy/SciPy dependencies. The exact check
uses only the Python standard library:

```bash
python3 topics/variation-and-local-maxima/hko-local-maximum/multicut-research-20261016/verify_geometry.py \
  --repo "$PWD" --out /tmp/msc-math/hko-20261016/multicut/fresh-exact.json
```

The numerical script extracts two immutable archive members; the exact check
extracts one. Both use automatically removed temporary directories and write
only their requested output. The original run was first performed
under `/tmp/msc-math/hko-20261016/multicut/` and deliberately retained here.
The new continuum proof received independent native-agent approval, conditional
on its accepted inputs; [REVIEW.md](REVIEW.md) preserves the source/scope and
replay receipt. This is research-note review, without manuscript integration
or external refereeing. No thesis files were edited.
