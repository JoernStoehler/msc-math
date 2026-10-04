# Non-HKO ten-facet local maximum

This packet owns the exact proof candidate for the rational hexagon--quadrilateral
product (P_*). It is separate from the HKO certificate because it proves a
different local result: (P_*) has ratio (1), is not affinely equivalent to
HKO, and locally maximizes the ratio among convex four-polytopes with ten
genuine facets.

The reader-facing theorem is in the selected thesis, in the subsection
`A second ten-facet local maximum` of `thesis/chapters/07-hko.tex`. The detailed
proof is [PROOF.md](PROOF.md); the fixed rational witness is
`certificate/certificate_base.json`.

## Trust boundary

The verifier uses exact rational and rational-function arithmetic. Its feasible
HK sections provide upper bounds on the actual ratio; the product enumeration
is used separately to establish equality on the displayed two-parameter family.
The analytic uniform-neighborhood argument is part of `PROOF.md`, not supplied
by exact arithmetic alone. The independent checker in `code/independent_audit.py`
recomputes the geometry, volume derivatives, section gradients and exceptional
quadratic direction without importing the submitted verifier modules. This is
independent implementation evidence, not proof-assistant formalization or human
refereeing.

No numerical neighborhood radius is claimed. The result does not prove a
global maximum, uniqueness among all ten-facet bodies, or any theorem when the
facet count changes.

## Reproduction

From this directory, write the results to a fresh temporary directory. Keep
assertions enabled for both implementations; the independent audit uses Python
assertions for its proof-facing checks.

```sh
non_hko_run_dir=$(mktemp -d)
PYTHONOPTIMIZE=0 uv run --with sympy==1.14.0 python verify.py \
  --output-dir "$non_hko_run_dir/exact"
PYTHONOPTIMIZE=0 uv run --with sympy==1.14.0 python code/independent_audit.py \
  --source . --rerun "$non_hko_run_dir/exact" \
  --output "$non_hko_run_dir/independent.json"
```

The verifier refuses a nonempty output directory. The independent audit reads
the new exact run's proposed weight jets and checks them independently; its
output file is separate from the retained evidence. This reproduces the exact
predicates, not the analytic neighborhood implication by itself.

The 24 September 2026 rerun is retained under `verification/`, including the
summary and console logs. Its exact outcomes include 624 product orderings,
24 positive sections, gradient rank 22, two equality parameters modulo
symmetries, full chart rank 40, and the verified upper-section second derivative
(-13/432) in the remaining transverse direction.

## Historical evidence

The original packet is preserved inside
[the consolidation archive](../../../docs/pro-handoffs/msc-math-consolidation-20260924/README.md):
repository file
`docs/pro-handoffs/msc-math-consolidation-20260924/msc_math_consolidation_codex.zip`
contains the member `incoming/non_hko_f10_certified_result.zip`.
Paths beginning `evidence/` or `discovery/`, and `RUN_LEDGER.md`, refer to the
root of that nested ZIP, not this checkout directory. They include the original
exact run, clean-extraction reproduction and numerical discovery records.
[PROVENANCE.json](PROVENANCE.json) records both archive hashes and the current
verification paths. No discovery run is needed for the certificate checks above.

To inspect a historical member without unpacking or replacing any evidence,
run this from the repository root:

```sh
python3 - <<'PY'
from io import BytesIO
from zipfile import ZipFile

archive = 'docs/pro-handoffs/msc-math-consolidation-20260924/msc_math_consolidation_codex.zip'
with ZipFile(archive) as outer:
    packet = outer.read('incoming/non_hko_f10_certified_result.zip')
with ZipFile(BytesIO(packet)) as inner:
    print(inner.read('RUN_LEDGER.md').decode())
PY
```
