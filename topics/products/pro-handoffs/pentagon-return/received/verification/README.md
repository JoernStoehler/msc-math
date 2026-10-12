# Optional verification

The written proofs in `../math/research.tex` stand on their own. These checks provide a reproducible guard against algebra, normalization, sign, and omitted-word errors; they are not a replacement for the geometric arguments.

Run from this directory:

```sh
python check.py --output results.json
```

The script uses Python 3.10 or newer, NumPy, and SymPy. It does not access the network. Exact versions used for the included run are recorded in `results.json`.

## Exact checks

SymPy verifies the pentagon's normal relations, both tensor identities, all twelve reduced orders, the four symbolic isosceles squared-norm types, the dominance factorization, the pentagon switch contact, and the exact systolic constant. Python rational arithmetic verifies the root-bracketing signs and every rational bound appearing in the heptagon argument. All exact checks passed in the included run.

The script does not mechanically prove the general sparse reduction, width-body lemma, area-containment argument, stability theorem, or completeness of the gap classifications. Those proofs are in the note.

## Floating-point diagnostics

Independently of the reduced six-block expression, the script evaluates the original full word objective for **all 120 cyclic orders** of each six-entry closure pair, including two-, four-, and six-block orders. It obtains positive closure triples by solving all index triples rather than assuming the geometric type classification.

The checked regular pairs are `(3,3)`, `(5,5)`, `(7,7)`, and `(5,7)`: respectively 120, 3,000, 23,520, and 8,400 candidate words. All predicted hull halfspaces are tested, and angular support values are tested at predicted vertex directions, switching directions, and additional fixed angles. The largest observed absolute error in Q was below `3e-16`.

There are also 24 fixed independent-affine pentagon/rotation cases, including reflections, shears, and anisotropic scales. The largest observed absolute Q error was below `3e-16`; all corresponding systolic ratios obeyed the proved bound to the stated floating-point tolerance.

These use ordinary `float64`, not directed rounding or interval arithmetic. Passing these tests is **not** an exact capacity certificate or an exhaustive affine-parameter test. Exact proofs of the statements appear in the mathematical note; no theorem depends on a numerical tolerance.
