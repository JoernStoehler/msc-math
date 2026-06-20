# Notation Help: `sys_sigma` Branches

Status: provisional notation helper for the `sys(a)` local-behavior work. This
is not thesis source text and not a theorem source. Use it to keep experiment
reports, draft prose, and later thesis notation aligned.

Recovered from the sys local-behavior source session rollout at
`2026-06-16T09:51:43Z`; adjusted to preserve the later active-germ warning in
`research/sys-first-order-local-behavior.md`.

## Objects

- A cyclic partial permutation `sigma` is the combinatorial label used by the
  HK-style `sys_sigma` branch.
- A branch is the local action/capacity function attached to such a `sigma`,
  written informally as `sys_sigma`.
- Avoid calling these objects "words" in thesis-facing prose when discussing
  this data-science/local-behavior layer. "Words" collides with CH2021
  terminology and does not describe the current `sigma` labels well.

Use "branch" for the object we care about analytically, and "cyclic partial
permutation" when the combinatorial `sigma` itself needs to be named.

## Strict Branches

The strict branch set at `a` is

```text
Branches_{beta>0}(a)
:=
{
  sigma:
  KKT(a,sigma) has a solution (beta,mu,xi)
  with beta > 0 and Q(a,sigma,beta) > 0
}.
```

Here `beta > 0` means every component of `beta` is strictly positive. The
condition `Q > 0` selects the positive-action orientation, so that the
associated branch value is finite and positive.

The strict minimizing branches are

```text
MinBranches_{beta>0}(a)
:=
{
  sigma in Branches_{beta>0}(a):
  sys_sigma(a) = sys(a)
}.
```

These are the primary objects for infinitesimal local behavior at generic
points. When the relevant KKT solution is nondegenerate, each such `sigma`
determines a smooth local branch of `sys`.

## Nonnegative-Beta Candidate Branches

For nongeneric points and branch birth/death, use the closed beta condition as
an over-approximation bucket:

```text
Branches_{beta>=0}(a)
:=
{
  sigma:
  KKT(a,sigma) has a solution (beta,mu,xi)
  with beta >= 0 and Q(a,sigma,beta) >= 0
}.
```

This set is not meant to say that every element is realized as a nearby strict
branch. Its purpose is more conservative:

```text
limiting strict branches at a
  subseteq Branches_{beta>=0}(a).
```

Likewise,

```text
MinBranches_{beta>=0}(a)
:=
{
  sigma in Branches_{beta>=0}(a):
  sys_sigma(a) = sys(a)
}
```

should be read as a candidate set containing all limiting strict minimizing
branches, possibly with extra boundary candidates.

If a nonnegative-beta solution has zero components, deleting those entries and
passing to the positive support is enough for the base HK value only when a
minimum-support or dominance argument applies. It is not automatically safe for
first-order local behavior: a zero component can become positive along a
right-hand branch germ and carry a different nearby slope. This is why
`beta >= 0` is useful for local one-sided prediction, but should not be
conflated with the strict smooth branch set.

## Branch Statuses

Do not build negative definiteness into the definition of `Branches`.

Negative definiteness, KKT nonsingularity, rank deficiency, and second-order
degeneracy are statuses of a branch at `a`. They are important diagnostics, but
they should be recorded separately from branch existence:

- beta status: `beta > 0`, boundary `beta >= 0`, or infeasible;
- Q status: `Q > 0`, `Q = 0`, or `Q < 0`;
- KKT status: nonsingular, rank deficient, or no solution;
- second-order status: negative definite, degenerate, or indefinite;
- action gap relative to `sys(a)`.

For data-science branch cartography, it is useful to return nearby-action
candidates even when the negative-definite filter fails. The certified capacity
path may still filter aggressively; exploratory local-behavior producers should
classify rather than discard.

## Naming In Artifacts

Prefer these labels in reports and data columns:

- `min_branches_beta_pos`
- `min_branches_beta_nonneg`
- `branches_beta_pos`
- `branches_beta_nonneg`
- `support_reduced_min_branches_beta_nonneg`
- `branch_status_at_a0`
- `branch_status_at_a`

Avoid:

- `winner`: suggests an arbitrary representative minimizer.
- `M(a)`: too semantically weak for the beta distinction.
- `argmin(a)` without saying which branch universe is being minimized over.
- `A` or `mathcal A` for active sets, because `A` is naturally action in
  symplectic geometry.

## Current Working Convention

Use

```text
Branches_{beta>0}(a),
Branches_{beta>=0}(a),
MinBranches_{beta>0}(a),
MinBranches_{beta>=0}(a).
```

In prose, say:

- "strict branches" for `Branches_{beta>0}`;
- "nonnegative-beta candidate branches" or "closed-beta candidate branches" for
  `Branches_{beta>=0}`;
- "strict minimizing branches" for `MinBranches_{beta>0}`;
- "nonnegative-beta minimizing branch candidates" for
  `MinBranches_{beta>=0}`.
