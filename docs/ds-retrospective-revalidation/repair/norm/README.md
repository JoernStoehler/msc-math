# Exact dyadic norm-repair inputs

`inputs.jsonl` contains 53 requests selected from the 54 primal-norm failures.
`selection.json` records source hashes, exact selection checks, and each exponent.
Regenerate from this worktree root with:

```sh
python3 docs/ds-retrospective-revalidation/repair/norm/prepare.py
python3 docs/ds-retrospective-revalidation/repair/norm/diagonal_probe.py
```

For each body the script selects the smallest absolute integer `k` satisfying
all retained rational primal and dual infinity-norm bounds [1/1000, 1000].
The binary64 dual coordinates satisfy **exactly**
`dual_scaled = dual_original / 2^k`, verified using rational arithmetic.
The source rational dual coordinates are also checked against their original
binary64 values. Input metadata, including old target values, remains original;
it must not be compared directly to the scaled evaluator output.

## Transfer of results

The transformation is `K_scaled = 2^k K`, with positive dilation.
Symplectic capacity is homogeneous of degree two and four-dimensional volume
of degree four. Thus

- `c_original = c_scaled / 2^(2k)`;
- `V_original = V_scaled / 2^(4k)`;
- `sys_original = sys_scaled`.

Both endpoints of a returned capacity interval must be multiplied by the same
positive factor. Preserve intervals and typed failures, not only central values.
Use exact dyadic multiplication, or verify representability/outward rounding if
converting endpoints. Never overwrite a historical scalar with a scaled value.

These inputs **do not establish fresh geometry completeness or capacity**.
The evaluator must reconstruct the geometry and enforce its ordinary policies.
The retained primal vertices here serve selection only. Source IDs intentionally
remain original for joining; the explicit transform and input hash identify the
new computation and distinguish it from the failed original request.

## One unresolved body

`4ab390657ba9bde151d076a3a1ad6e1a1e454a84c430b819e417040646526fd7`,
`random_F8_s3_45`, has 8 dual and 18 retained primal vertices.
Its retained minimum/maximum primal infinity norms are approximately
1.00667117 and 1,607,236.06. The former requires dilation at least
0.0009933730385; the latter requires dilation at most 0.0006221861412.
The exact rational inequalities in `selection.json` prove that **no positive
uniform scale** can satisfy both for these vertices, not merely no power of two.

A bounded floating-point screen of diagonal symplectic transformations
`diag(2^a,2^b,2^-a,2^-b)` with `a,b` in [-12,12], combined with uniform
`2^k` for `k` in [-24,24], found no candidate (`diagonal-probe.json`).
This screening is a negative heuristic result, not an exact impossibility theorem.
The coordinate convention is `(q1,q2,p1,p2)` as in
`crates/symplectic/src/geom/symplectic_form.rs`.

A bounded next option is small integer symplectic shears, followed by the same
norm-balancing screen. Shears can change the alignment of the extreme vertex,
which diagonal maps cannot. Any candidate needs exact transformed duals,
verified exact binary64 representability (integer sums are not automatically
representable), fresh geometry, and all normal policy checks. If no candidate
is found, retain this as unresolved rather than changing the geometry or bypassing
the policy. No capacity computation was performed for this preparation.
