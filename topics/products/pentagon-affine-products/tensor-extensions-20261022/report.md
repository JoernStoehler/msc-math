# Exact tensor-containment extensions: 7×7 and 5×9

Bounded result for the 22 October research sprint. The exact proof packet is retained here after independent review and fresh replay. The selected manuscript was not changed. The maintained
source inspected was
[`topics/products/pentagon-affine-products/research.tex`](/workspaces/msc-math/topics/products/pentagon-affine-products/research.tex),
SHA-256 `82863ee8f129fff607b774d12d48d45692842b37bab18500a3859b955f0ee4e`.
Its sparse reduction and tensor decomposition are at equations (six-tensor),
(Tfour), and (decomp), source lines 123–210.

## Mathematical implication

For both pairs `(7,7)` and `(5,9)`, the exact interval check below establishes
`T6(P_n,P_m) ⊂ T4(P_n,P_m)`. The maintained decomposition theorem therefore
gives the independent-affine capacity formula

\[
c_{EHZ}(G P_n\times_L H P_m)=
\frac{(1+\cos(\pi/n))(1+\cos(\pi/m))}
{\max_{i,j}|(n_i^{(n)})^T G^{-1}H^{-T}n_j^{(m)}|}
\]

for every nonsingular real `G,H`, for these two regular-factor pairs. The
general sparse reduction is recalled in the source; this packet establishes the
new finite tensor-containment premises. It does not establish containment for
all odd pairs.

For the 7×7 pair, the same formula yields a sharp independent-affine systolic
ratio. Set `θ=π/7`, `M=G⁻¹H⁻ᵀ`, and
`D₇=conv{±n_i^(7)}`. Its denominator is
`m(M)=max_{x,y∈D₇}|xᵀMy|`; after scaling to `m(M)=1`, this is equivalent to
`M D₇ ⊂ D₇°`. Hence

\[
|\det M|\leq\frac{\operatorname{area}(D_7^\circ)}
{\operatorname{area}(D_7)}=\sec^2(\pi/14).
\]

Equality is attained by mapping the circumradius-one regular 14-gon `D₇` onto
its polar. Equality in the area inclusion forces `M D₇=D₇°`; the equality
maps under `m(M)=1` are the scale `sec(π/14)` times the rotation by `π/14`,
composed with a dihedral symmetry (including orientation-reversing
symmetries). Without that normalization, the optimizer class also has an
arbitrary positive scalar factor. Since
`area(P₇)=7 sin(2π/7)/2`, the resulting sharp ratio is

\[
\frac{(1+\cos(\pi/7))^3}
{49\sin^2(\pi/7)\cos^2(\pi/7)}.
\]

The equality statement concerns `M`; factorization into `G,H` is nonunique.

## Exact finite certificates

The interval check reconstructs the exact trigonometric normals and closure
weights from rational enclosures for `π`, sine and cosine. For each factor it
classifies every sorted triple by signs of its barycentric weights; no zero or
undecided case is accepted. It then checks every positive triple pair and all
twelve alternating six-block orders. The weights were first proposed by a
small floating-point LP producer, but the proof check does not trust those
LPs: for every row it rebuilds the exact target and generators, verifies the
rational certificate plus an exact five-coordinate correction, proves the
five selected augmented generators invertible by interval pivots, and proves
all corrected coordinates positive. The affine identity holds algebraically
by the definition of the correction. Untouched rational coordinates are at
least `1/100000`.

| Pair | Positive triples | Rejected triples | Six-block tensors | Signed four-block generators | Numerical hull facets | Exact result |
|---|---:|---:|---:|---:|---:|---|
| 7×7 | 14×14 | 21 per factor | 2,352 | 98 | 594 | PASS |
| 5×9 | 5×30 | 5 and 54 | 1,800 | 90 | 810 | PASS |

The checker certifies a common weight lower bound of
`9999998546888671934107081/10^30 ≈ 9.9999985469×10⁻⁶` for 7×7 and
`4999998884966221784544471/(5×10^29) ≈ 9.9999977699×10⁻⁶` for 5×9. These
are minima after correcting the five basis coordinates; the remaining
coordinates have lower bound `10⁻⁵`. The strict positivity also places each
target in the interior of `T4`, since the signed generators affinely span the
four-dimensional matrix space.

The 5×9 factor includes the scalene closure type with cyclic gaps `(2,3,4)`;
its classification is replayed by the exact interval checker rather than
assumed from a numerical run. A bounded floating-point hull discriminator
preceded both certificates. Its worst facet residuals were `−0.00402642163`
for 7×7 and `−0.0021162236354` for 5×9; those margins are diagnostic only.

An independent read-only review found no flaw in either checker's enumeration,
generator indexing, augmented-basis correction, or positivity logic. The
reviewer then replayed both checkers from fresh `/tmp` copies: 11.62 s for
7×7 and 9.42 s for 5×9, with all selected output fields matching the retained
results. The checker does not verify the certificate JSON's source-results
hash metadata; this metadata is not used in the proof. The maintained source supplies the general reduction and
tensor identities, but the new closure classifications and pair-specific
containments live in the standalone checker/certificate here. Human proof
acceptance, literature novelty and publication readiness remain open.

## Packet and replay

The retained proof packet contains `check77.py`, `weights77.json`,
`check-result77.json`, `check59.py`, `weights59.json`, and `check-result59.json`.
Floating-point proposals and diagnostic hull runs remain disposable sprint
material; the exact verifier reconstructs all mathematical inputs independently.
The exact checkers use only Python's standard library. To replay without
replacing retained evidence, copy a checker and its weights JSON into a new
empty directory and run the checker there. For example, from repository root:

```sh
mkdir -p /tmp/msc-math/tensor-replay/NEW-RUN
cp topics/products/pentagon-affine-products/tensor-extensions-20261022/check77.py /tmp/msc-math/tensor-replay/NEW-RUN/
cp topics/products/pentagon-affine-products/tensor-extensions-20261022/weights77.json /tmp/msc-math/tensor-replay/NEW-RUN/
python /tmp/msc-math/tensor-replay/NEW-RUN/check77.py
```

Use `check59.py` and `weights59.json` for the other pair. The checkers refuse
to overwrite an existing result. Independent fresh replay passed both checks.

Certificate SHA-256: 7×7 `7ee77d62a4aa08e84e63d9fd83390ab7322dd23ed00a64a8dba130051ade7bcd`;
5×9 `17d0740a352c4b04319c2768ba49e5db350889fe68e33a0c0b8c27cf8db7cefd`.
