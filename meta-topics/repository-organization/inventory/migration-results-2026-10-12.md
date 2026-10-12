# Repository consolidation, 12 October 2026

This is the assembled route and limitation ledger for the [approved execution
plan](parallel-plan.md). The topic accounts and their source packets own the
mathematics and code contracts; this record maps changed ownership and bounded
checks. No new theorem was selected for the thesis, no full empirical producer
was rerun, and no release was made.

## Current route

| Former owner | Current owner | Disposition |
| --- | --- | --- |
| `formal/hk2017-qp-*.tex`, `ehz-kkt-system.tex`, pruning/precision packets | [`topics/capacity-and-qp/`](../../../topics/capacity-and-qp/README.md) | Canonical proof development; formal input shims remain for existing labels. |
| `docs/{numerics-correspondence-check,qp-derivation-review,qp-flow-integration,imported-theorem-contracts}/` | [`topics/capacity-and-qp/evidence/`](../../../topics/capacity-and-qp/README.md) | Dated evidence/proposals retained with source limits. |
| Five geometry/flow proof packets in `formal/`; `docs/flow-runtime-closure/` | [`topics/polytope-geometry/`](../../../topics/polytope-geometry/README.md), [`topics/flow-graphs/`](../../../topics/flow-graphs/README.md) | Canonical proof and runtime-closure owners; formal shims preserve inputs. |
| `experiments/regular-products/`, product proof packets and `formal/pentagon-affine-products/` | [`topics/products/`](../../../topics/products/README.md) | Product proof, computation and retained evidence together. |
| `experiments/hko-local-maximum/`, `docs/variation-interface-check/` | [`topics/variation-and-local-maxima/`](../../../topics/variation-and-local-maxima/README.md) | Exact local theorem/non-HKO packets and bounded empirical controls. |
| `docs/empirical-viterbo-design/desk/{pentagon-arbitrary-partner,product-position-research}/` | [`topics/products/`](../../../topics/products/README.md) | Symmetric-partner and product-position research accounts; remaining scientific desk now belongs to empirical discovery-design. |
| `docs/empirical-viterbo-design/` remaining scientific records | [discovery account](../../../topics/empirical-studies/discovery.md) and [producer/evidence desk](../../../topics/empirical-studies/discovery-design/README.md) | 52 scientific files retained; two temporary startup notes removed; ridge, moment, representation and zonotope results distinguished from historical proposals. |
| `docs/{ds-evidence-closure,ds-family-theorem-mapping,ds-retrospective-revalidation}/`; `experiments/{polytope-datasets,polytope-invariant-table,sys-datascience,sys-landscape}/` | [`topics/empirical-studies/`](../../../topics/empirical-studies/README.md) | Evidence/interpretation and four coupled producer packages moved; frozen outputs and artifact identities preserved. |
| `papers/`, `docs/{open-thesis-literature,algorithm-prior-art-closure}/` | [`topics/literature/`](../../../topics/literature/README.md) | Source collection and historical prior-art search; project implications stay with scientific topics. |
| `INSTALL.md`, `docs/{development-environments,codex}.md`, `submit/`, selected process/retention packets | [`meta-topics/`](../README.md) | Five lasting purpose owners. Conventional root, skill, build and thesis interfaces stay at their discovery paths. |
| `ARCHITECTURE.md`; `docs/coordination/message-reviews/` | Topic/meta-topic accounts and [retention disposition](../../agent-workflow/message-review-retention.md) | Useful contracts transferred; redundant owner retired. Disposable exact reviews moved to temporary checked recovery copies, with consequential findings retained. |
| `docs/inventory/` | [Repository-organization inventory](README.md) | Source records and generated browser view moved together; source paths follow current topic owners. The 10 October physical census remains an explicitly dated baseline. |

The detailed meta path map and byte-check evidence are in
[`meta-migration-review-2026-10-12.md`](../meta-migration-review-2026-10-12.md).
An independent completion audit found two remaining migration holes after the
first assembly. The discovery desk now has a source-backed
[scientific account](../../../topics/empirical-studies/discovery.md), five
inventory rows and preserved producer/evidence bytes. Expired October launch
packets have a [dated disposition](../../project-management/expired-window-retention-2026-10-12.md):
useful evidence moved to history, eight named dispatch/prompt wrappers were
Git-retired, and the registry no longer presents their deadlines or spending
questions as live. A targeted independent recheck approved closure of these
two findings. Its scope did not certify new mathematics, the whole thesis or
the entire repository.
The physical [file census](file-census.md) remains a dated pre-migration survey,
not a current count of this checkout. The semantic inventory is a
selective source map, not proof that every repository file was read.

## What was checked

- Capacity/QP: [bounded mathematical/API reviews](../../../topics/capacity-and-qp/reviews/migration-2026-10-12.md), formal build and label checks, preserved packet hashes and focused formatting.
- Geometry/flow: [independent math/API review](../../../topics/polytope-geometry/reviews/2026-10-12-math-api.md), determinant/polar/flow tests and frozen witness checker. One genuinely identical integer determinant now has a shared Euclidean implementation; generic exact, rational and f64 policies remain distinct.
- Products/variation: [source-routing reviews](../../../topics/products/reviews/2026-10-12-source-routing.md), focused Cargo, Sage/LaTeX and packet-hash checks. The HKO certificate and non-HKO theorem retain their separate proof/evidence boundaries.
- Empirical: [data/method/landscape/discovery reviews](../../../topics/empirical-studies/migration-review-2026-10-12.md), fixture and schema checks, retained reanalysis comparisons, and hashes of frozen moved evidence. Full producers were not rerun.
- Meta/literature: [retention review](../meta-migration-review-2026-10-12.md), source-byte checks, report verifiers and link checks. The cached paper sources match their original hashes.
- Assembled consumers: Cargo workspace metadata/check, formal build, selected thesis build, semantic inventory generator and coordination dashboard checks passed. The current thesis PDF has 102 pages; the 4 October frozen candidate has 101 and a bounded page/source review. An independent selected-source/PDF trace found path/layout changes without a claim or caveat change. No complete thesis visual or human PASS review existed before this migration or was added by it, so the [thesis dashboard](../../../docs/dashboard/thesis.html) keeps the older bounded page map and an unreconciled newer-source signal.

## Remaining gaps and boundaries

**Pre-existing scientific/acceptance gaps:** The fixed-regular-pentagon with
arbitrary centrally symmetric partner result is selected in
[`project-facts.md`](../../../docs/project-facts.md) but absent from both the frozen and
working thesis PDFs. Its [agent-reviewed research
packet](../../../topics/products/pentagon-arbitrary-partner/README.md) does not
establish human proof or prose acceptance. Product stream coverage, broader
nonsymmetric regular-factor containment, complete flow theorem/runtime
correspondence, QP exhaustive certificate verification and historical empirical
reproduction have their topic-local limits. The data-science method scope and
human thesis PASS remain unresolved. The missing HKO certificate-slope
calibration packet in the quotient diagnostic was already absent before this
migration; its README no longer treats it as retained evidence.

**Migration regressions repaired:** The HKO, regular-product and empirical-package moves initially
broke Cargo/workspace consumers, selected-thesis paths, formal inputs and some
research links. Current Cargo/formal/thesis checks and independent result traces
support those repairs. Frozen reports, public-commit path statements and dated
absolute worktree references retain their historical spelling where changing
them would misrepresent provenance. Other active experimental packages remain
at their documented `experiments/` homes; topic accounts route to them without
implying that every producer was executed during this migration.

The approved execution had a wall-time and account-wide quota ceiling. Those
resource limits bound this cleanup, not the scientific validity or later thesis
work.


## Proposed checkpoint wrap-up, 12 October

**Planning only; execution awaits Jörn's selection.** At 11:30 UTC Jörn
requested a planning/questions phase of up to fifteen minutes and said a later
execution phase could receive up to sixty minutes. This does not extend the
expired migration window or restart thesis production.

The migration is committed on `codex/topic-migration-20261012` as `2f1fc5e2`,
followed by storage-guidance clarification `88240a2d`. The small recommended
follow-up is:

1. Reconcile live completion/decision records with the checkpoint and expired
   windows. Keep unresolved substantive obligations, but remove obsolete launch
   language from current next steps. Inspect the still-open
   `oct03-resource-scope` and `oct04-workflow-fix-window` decisions against their
   source dispositions; their old windows are not current questions for Jörn.
2. Repair the missing-Pandoc diagnostic in `scripts/serve-files.py`, which still
   points to the removed `INSTALL.md` instead of the maintained installation
   workflow.
3. Clarify the inventory's coverage arithmetic. Its generator intersects
   current semantic source paths with the frozen 10 October physical census;
   the resulting count is unchanged-path overlap, not current repository
   coverage. Label that precise meaning or omit the misleading counts. Preserve
   the dated census rather than rerunning it for cosmetic freshness.
4. Retain a concise integration map for the occupied worktrees, identifying
   moved-path counterparts and confirmed or unknown ownership. Do not apply
   their changes or alter those worktrees during this wrap-up. Bound ownership
   lookup to ten minutes; unresolved owners become an explicit handoff limit.

Read-only Git observations at 11:32–11:34 UTC: `main` is `844fc5be`, an exact
ancestor of this branch, with zero commits of divergence and two migration
commits ahead. Main nevertheless has 100 dirty/untracked status entries and
18 tracked changed paths that overlap migration changes, including moves and
deletions. Worktrees `337f` and `fde8` have nine and fifteen status entries,
with six and seven overlaps respectively. These counts are dated observations,
not file ownership or proof of running sessions. A direct update of occupied
main is therefore not the selected route. Broader consolidation would include
independent cloud/harness/research work and needs its own disposition.

**Execution proposal:** allow up to sixty minutes from an explicit launch,
with roughly fifteen to thirty minutes expected for this narrow package;
that estimate is judgment, not a measured runtime. Root owns current-state
reconciliation, integration map, final commit and delivery. A bounded worker
can own the two script/presentation corrections; a separate reviewer checks
the assembled diff and completion claim. No paid API calls, scientific
producer/certificate reruns, storage-backend comparison, thesis rewrite,
push, main merge or unrelated session consolidation is included. No new
quota allowance is inferred from the expired two-hour programme.

**Completion evidence:** the two corrected interfaces point to existing owners
and describe their counts accurately; generated inventory, coordination and
graph checks pass; the integration map records preservation and unknowns;
independent review finds no consequential objection; the follow-up is committed
on this branch. Stop once those outputs are complete. If integration ownership
cannot be established, leave it explicitly unknown instead of broadening the
work or representing main as consolidated. Thesis and human acceptance gaps
remain at their existing owners.

**Alternatives:** stop with the current committed migration and retain the
small defects; run the recommended wrap-up; or separately select consolidation
of other sessions' work. The bounded inspection supports the middle option;
it does not justify another hour of general migration or research cleanup.

Independent bounded plan review by `/root/wrapup_gaps` approved this proposal
on 12 October: the demonstrated fixes, ten-minute ownership limit, preservation
boundary and focused completion checks fit the planning request and proposed
execution ceiling. This is agent review of a proposal, not Jörn's execution
approval or a claim that the fixes have been made.
