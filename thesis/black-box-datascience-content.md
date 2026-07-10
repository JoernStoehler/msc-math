# Black-Box Data-Science Content Notes

Status: section-local content companion for `thesis/08-black-box-datascience.tex`.
Not source truth.

Purpose: gather the data-science search-result writing inventory, result
taxonomy, and open decisions before final prose is written.

Overruled by: `experiments/sys-datascience/`,
`experiments/sys-datascience/methods/`,
`experiments/sys-landscape/legacy-ascent-continuation-debt.md`,
generated tables/figures, and Jörn/Kai review.

Lifecycle: keep while the section is being assembled. After the section is
stable, delete this file or reduce it to a short maintenance index.

Update rule: add or change a claim only with a source pointer or an explicit
`needs source` marker.

## Active Result Shape

Current active slice: random polytopes and random Lagrangian-product polytopes
only.

Source pointers:

- `experiments/sys-datascience/README.md`;
- `experiments/sys-datascience/produce/README.md`;
- `experiments/sys-datascience/prepare/README.md`;
- `experiments/sys-datascience/methods/README.md`;
- `experiments/sys-datascience/methods/trusted-random-product-closure-summary.md`;
- `experiments/sys-datascience/methods/trusted-random-product-method-dispositions.md`;
- relevant method-packet READMEs.

The thesis-facing claim should be bounded:

- retained random/product table: `14336` rows, `0` rows with `sys > 1`;
- generic random rows: `4096` rows, `0` positives;
- random Lagrangian-product rows: `10240` rows, `0` positives;
- retained in-table EDA, scalar associations, projections, supervised
  prediction, and tail-rule diagnostics found structure but no validated new
  `sys > 1` row;
- frozen scalar generated-candidate packet over `100000` random-product
  candidates evaluated `1675` selected-or-baseline rows after selection was
  frozen, and found no evaluated candidate with `sys > 1`;
- ridge/mechanism diagnostics give empirical hints for future frozen rules, but
  post-`sys` diagnostic splits are not candidate-proposer claims.

Current chapter role: report a controlled failure mode of ordinary random search
and data-science wrappers, together with the strongest positive structure they
did recover. The main transferable pattern is bucket-local: low total or mean
symplectic area of the primal two-faces, normalized by `sqrt(volume)`, selects
generated random-product candidates with higher mean `sys` than matched
baselines. It does not lift the selected maximum beyond the observed
sub-threshold plateau. Source:
`experiments/sys-datascience/methods/ridge-mechanism-discriminator/` and its
generated scalar-proposer inputs. This is not a mathematical nonexistence
theorem or a demonstrated route to `sys > 1`.

Exploration closed on 2026-07-10 without selecting another dataset. The dormant
same-generator replication plan answers no remaining research question.
Post-target ridge-area concentration/entropy splits remain future-rule seeds,
not a reason to delay demonstration or writing. Cross-method interpretation and
the reopening boundary live in
`experiments/sys-datascience/coordination/exploration-result.md`.

## Active Producers

Source pointer: `experiments/sys-datascience/produce/README.md`.

- Generic random rows: facet counts `F=5..12`, `512` accepted samples per
  facet count, height interval `[0.8, 1.2]`, seed `42`, rejection until valid.
- Random Lagrangian-product rows: polygon-pair buckets `3 <= k <= m <= 6`,
  `1024` accepted samples per bucket, height interval `[0.8, 1.2]`, seed `42`,
  rejection until valid.
- Total retained random/product rows: `14336`.
- Known HKO reference rows are reference/holdout rows, not part of the retained
  random/product production counts.

## Active Rows

Define rows as retained polytopes in the random/product prepared table.

Active row families:

- generic random polytopes from the retained finite production run;
- random Lagrangian products from the retained finite production run;
- generated random-product candidates in the scalar-proposer packet only when
  discussing generated-candidate evidence separately from the retained table.

Inactive or out-of-scope for this section unless Jörn explicitly reopens them:

- old fixed-F ascent endpoints;
- old product ascent endpoints;
- old variable-F continuation endpoints;
- endpoint stability, attractors, basins, local-behavior panels;
- perturbation panels.

Reason: active sys-datascience notes say the old ascent/continuation surfaces
are context only and do not support local-maximality, exhaustive-search, or
candidate-proposer claims.

## Active Columns

Source pointer: `experiments/sys-datascience/prepare/README.md`.

The method-facing table exports identity/target/source fields and active
invariant feature columns. Current covariates include:

- identity/target/metadata: `poly_id`, `sys`, `capacity_source`;
- combinatorial invariants from the face lattice;
- symplectic two-face area invariants, normalized by `volume.sqrt()`;
- source/provenance metadata kept separate for leakage and provenance checks.

The v1 active table intentionally excludes raw dual vertices, capacity, volume,
Euclidean representative features, omega magnitudes of normalized dual rows,
transition features, and cutoff/sign features from the method-facing covariates.

Important thesis wording: a method may learn source/provenance rather than
geometry. Treat metadata-only baselines and provenance overlays as caveats and
controls, not as candidate-proposer inputs.

## Result Types

- `not applicable`: the method does not fit the available data or search
  interface.
- `not run within the stated bound`: implementation, runtime, or data
  requirements exceeded the stated thesis bound.
- `implementation bug; no method verdict`: a run artifact exists but is not
  interpretable as method evidence.
- `ran with no candidate-proposer and no new validated row`: the method ran but
  recorded neither a candidate-proposer nor a validated new `sys > 1` row.
- `candidate-proposer`: a reproducible rule proposes candidate polytopes or
  rows before their `sys` values are evaluated. For retained-table diagnostics,
  it must not use post-hoc target information or provenance leakage. A generated
  proposer may be explicitly scoped to a predeclared generator or bucket
  contract, as long as selection is frozen before `sys` evaluation.
- `validated candidate`: a run produced a new row with verified `sys > 1`.
- `generated-candidate evidence`: a rule freezes generated candidates before
  `sys` evaluation and then evaluates those candidates. This is stronger than
  retained-table ranking, but it is still scoped to its generator, rule set, and
  candidate budget.
- If a real positive result appears, report it honestly and escalate to Jörn. It
  may falsify the current negative main result and justify follow-up.
- Filter obvious false positives, for example a model finding a within-table
  association because `sys` was regressed against `sys`.

## Methods

- Batch attempted methods by role: direct scan, descriptive tail diagnostics,
  scalar association/projection/ranking/rule diagnostics, generated-candidate
  scalar proposer, and mechanism/reference diagnostics.
- Put detailed figures, parameters, and method-specific rows in the
  data-science appendix or method READMEs, not in the main text.
- Methods come from the current random/product method packets, not old
  ascent/continuation archaeology.

## Forbidden Or Risky Wording

- Do not claim no `sys > 1` polytopes exist.
- Do not claim no random distribution can produce `sys > 1`.
- Do not claim exhaustive search.
- Do not claim the retained table validates a generated-candidate proposer.
- Do not claim the 100k scalar-proposer packet validates a near-counterexample
  source; it found no positive row and max evaluated `sys` was about `0.868`.
- Do not import old ascent/continuation/local-behavior packets into the active
  chapter without explicitly reopening and reviewing that separate surface.
- Do not present ridge/concentration mechanism diagnostics as proof of a
  mechanism; they are empirical packet labels and future-rule seeds.
