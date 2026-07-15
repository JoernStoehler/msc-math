# Generator equivalence regression matrix

This target-free packet prevents later generator work from counting a change
of representation, a component marginal, or a proved orbit/gauge move as new
geometric coverage. Its authoritative compact output is
`artifacts/matrix.json`; `matrix.tsv` is the copy-edit/review view. Every row
states its comparison level, conditioning, transformation, expected outcome in
all declared views (including separate signed and absolute symplectic-feature
views), proof/source status, arithmetic boundary, machine-readable executable
control status, and allocation-time collapse rule.
`zero` and `nonzero` are interpreted according to the generated
`view_definitions`: geometry residual is measured after the named transform,
while metric and symplectic feature views compare the two nominal arms
directly. This is why a polar pair has zero transformed-geometry residual but
generally nonzero Euclidean-feature difference.
The producer rejects optimized Python because its exact witness and schema
guards deliberately use assertions; `python3 -O` is not an authorized replay
mode.

Independent positive factor scalings are not a broad geometry-coverage
collapse: direct Euclidean and signed/absolute symplectic features change.
Collapse those arms only after independently area-normalizing both factors, or
when a consumer explicitly declares a quotient/invariance under independent
positive factor scalings. Capacity and `sys` equivalence remain separately
theorem-gated.

The crucial law boundary is narrow. Sorted IID uniform angles and
Dirichlet-(1,...,1) cyclic gaps agree as an angle proposal after accounting for
a common uniform rotation and cyclic root, including under the same angle-only
condition `max gap < pi`. The current baseline also has IID support marks and a
support-dependent active-facet acceptance boundary. It is therefore not the
same full accepted law as the equal-support Dirichlet arm. The matrix contains
a separate negative row that must fail if this marginal identity is promoted.

The polar rows use an explicit marked origin. For
`T={x:<n_i,x><=1}`, `T^circ=conv{n_i}`. If `T` and `T^circ` are independently
area-normalized, the corrected identity is

```text
(T^circ)_norm = (area(T) area(T^circ))^(-1/2) (T_norm)^circ.
```

Thus the normalized pair is a scaled polar pair, not generally a literal polar
pair. A double-polar control and a missing-origin negative control guard both
parts of this contract.

Four-sided broken opposite-support pairs always translate to their
width-matched symmetric strips because two independent normal directions give
two translation equations in two unknowns. With three pairs, all equations
must still be consistent. The retained active six-sided rational witness is
inconsistent and prevents a false general collapse.

The anti-symplectic endpoint retains exact matrix, Euclidean, volume, and
absolute-symplectic-feature controls. No authoritative local theorem checked by
this packet establishes capacity or `sys` invariance under that endpoint, so
that part remains explicitly proof-pending and its target arms must not be
collapsed.

## Reproduce and check

Generation is fail-closed on an exact clean source revision. The retained
provenance binds that whole-repository revision/tree, the copy-local producer,
all checked local sources, and each artifact byte string. Replay also requires
the exact matrix/row/witness/provenance schemas, nonempty collapse rules, exact
source/artifact/output path sets, revision-tree agreement, clean flags, and
matching byte counts and hashes. Extra or omitted output-tree entries fail.
From the final packet commit, the cheap replay check regenerates the matrix and
witnesses in memory and compares bytes and hashes:

```bash
python3 experiments/sys-datascience/methods/generator-equivalence-regression/produce.py --check
python3 -m unittest experiments/sys-datascience/methods/generator-equivalence-regression/test_produce.py
```

To regenerate after a deliberate source change, first commit the source with a
clean tracked tree, then run:

```bash
REV=$(git rev-parse HEAD)
python3 experiments/sys-datascience/methods/generator-equivalence-regression/produce.py \
  --expected-revision "$REV"
```

Do not hand-edit generated artifacts. This packet contains deterministic
semantic witnesses, not Monte Carlo evidence, a population comparison, a
capacity computation, or a `sys` result.
