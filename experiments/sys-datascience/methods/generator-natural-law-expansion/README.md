# Natural-law expansion smoke

This packet owns the two high-value planar mechanisms that were still missing
after the line's first generator zoo:

- **Shared-latent factors (wishlist item 13).** For equal side counts, draw
  centered Gaussian latent vectors for angular-gap logits and log supports.
  The second factor is `rho` times the first plus an independent component,
  with `rho in [0,1]`; exponentiating and normalizing the gap logits gives a
  logistic-normal angular law, and `exp(sigma*z)` gives positive supports.
  One global rotation is shared (a gauge). `rho=0` is independent shape
  variation; `rho=1` is the congruent-shape endpoint. `sigma` controls
  scale-free support variation. Every draw is conditioned on all facets being
  active, area-normalized separately to one, and assigned a deterministic
  seed/attempt/pairing identity.

- **Centroid-centered polar coupling (wishlist item 14).** Draw a current-law
  factor `Q`, area-normalize it, compute its area centroid `c`, and construct
  `(Q-c)^o` from the exact support formula
  `vertices = u_i/(h_i-u_i dot c)`. The polar vertices are convex-hulled,
  rotated by the explicit relative angle `phi`, converted back to an H
  representation, and area-normalized. Raw-origin polarity is never used;
  the center choice is part of the law and is recorded as `center=centroid`.

The binary performs only exact product reconstruction and a positive-volume
smoke. It never calls the target backend or selects on retained values. The
report records all 21 wishlist dispositions: existing faithful laws are
adapters/reuse references, item 13 and 14 are implemented here, the faithful
Poisson/Crofton cell (item 17) is explicitly deferred because finite-window
conditioning would be a different law, and items 18--21 remain outside this
planar owner.

The polar arm is a marked pushforward, not an additional independent breadth
claim: the existing equal-support/inscribed constructions are related by
origin polarity. The tests therefore calibrate the geometry with a
double-origin-polar incidence witness and a finite positive Mahler product;
later atlas work should compare the marked polar image against its source
shape rather than pool it as a fresh unconstrained population. The bounded
cross-seed test also checks deterministic IDs, replayability, and all-active
facet conditioning. These tests do not establish the logistic-normal joint
law or any population-level exchangeability; a larger PIT/rank audit is a
follow-up if this mechanism survives the atlas review.

## Scientific handoff

The shared-latent survivor adds dependence between the two factor shape laws,
which is absent from independent products and distinct from a deterministic
congruent pair except at `rho=1`. A later atlas comparison should use exact
side-count strata and joint factor-shape summaries (including support and gap
coordinates) to decide whether intermediate `rho` values add coverage or are
redundant with the independent/congruent endpoints.

The polar survivor adds support/vertex duality around a declared non-origin
center. A later atlas comparison should keep it marked as a pushforward and
compare its source/image shape metrics and polar Mahler/incidence witnesses
against equal-support and inscribed controls; it should not be pooled as a
third independent marginal law. No tiny-smoke ranking is claimed here.

## Command

```text
cargo test -p exp-sys-landscape --bin sys-datascience-generator-natural-law-expansion
cargo run -p exp-sys-landscape --release --bin sys-datascience-generator-natural-law-expansion -- \
  --out-dir experiments/sys-datascience/methods/generator-natural-law-expansion/artifacts \
  --seed 20260715 --attempts 128 --rows-per-law 2
```

The default smoke has three `rho` levels for shared-latent factors and two
relative rotations for polar coupling, over `3x3`, `4x4`, and `6x6` equal-side
products. `smoke-rows.jsonl` records one row per requested
law/parameter/bucket/row: an accepted row has the number of proposals in its
`attempts` field through the accepted proposal, while an exhausted row records
the bounded proposal count and `low acceptance` status. Each retained row also
records factor metric witnesses, exact product volume, and timing;
rejected proposals are not emitted as separate rows. `batch-report.json`
records the complete repository `HEAD` revision and tree, plus a tracked-clean
predicate from `git status --porcelain=v1 --untracked-files=no`. That source
snapshot is captured before output creation, so report files cannot make their
own provenance dirty; untracked/ignored files are intentionally excluded.
The producer writes the report before failing closed with exit status 1 if the
requested-row contract contains any nonterminal status or row-count mismatch.
Exhaustion is a terminal bounded outcome and does not itself fail the process.
The explicit interpretation boundary is recorded in the report.
