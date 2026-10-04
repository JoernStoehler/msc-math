# Five-hour task: finish the arbitrary-partner local HKO theorem

Standalone allocation rejected as an unjustified use of the five-hour window.
Retained candidate lane for portfolio comparison; not approved or launched. Jörn accepted the clarity of
the symmetric-partner summary in this conversation on 3 October; that is local
prose feedback, not proof acceptance or permission for substantial execution.
The separate coordinator records symmetric-partner inclusion as selected and
has already produced `docs/history/symmetric-partner-candidate-2026-10-03/`. Preserve that work.

## Value and remaining uncertainty

The existing thesis proves HKO local maximality within ten-facet deformations.
This candidate would prove local maximality among products `K × Q`, with `Q`
fixed regular pentagon and `K` any nearby planar convex body, including smooth
bodies. It removes the partner's facet-count restriction, while keeping the
product and fixed-factor restrictions. It does not settle general four-body
deformations or global optimality.

The candidate proof and exact certificate exist. The substantive remaining work
is independent justification of a local signed-polygon area estimate, followed
by reconstruction of the finite-to-continuum implication. Two preliminary
source audits found no concrete obstruction; neither finished that proof nor
measured delivery time. The theorem may still fail review. This is a better
supported near-term completion candidate than F=11 research, which has unresolved coupled cut cases,
and more sharply defined than choosing between the DS chapter's two explanations.
It also supplies a bounded result that can be explained alongside the selected
symmetric-partner theorem without another broad manuscript rewrite.

Finishing the existing symmetric candidate is the lower-risk writing outcome:
its mathematics is complete and its inclusion selected. This package adds the
local theorem because it removes a substantive restriction for arbitrary convex
partners. The coordinator owns the combined candidate and integration patch,
reusing the symmetric candidate after checking its current owner; an active
owner's work must be coordinated rather than duplicated. Jörn would still need
to assess the new continuum argument, choose whether to promote the theorem,
and judge the complete section. Review time is unknown.

## Launch prompt

Complete and challenge the candidate local HKO theorem for an arbitrary planar
convex partner of a fixed regular pentagon. Deliver a self-contained proof and
readable, typeset candidate section with its exact computational support. The
local proof must be complete, with explicitly imported BMP capacity-cover
theorems; reproving their upstream billiard framework is outside this task. Work
for at most five hours from launch; finishing earlier is welcome. Use available
included Codex agents and installed local tools, with at most four workers
simultaneously plus the coordinator. No paid APIs or new remote compute.

Start from `docs/empirical-viterbo-design/desk/pentagon-arbitrary-partner/`:
`local-cover-rigidity.md`, `local-cover/pentagon_fixture.py`, and
`local-cover/verify_local_cover.py`. Read the capacity-cover input in
Balitskiy–Mitrofanov–Polyanskii, arXiv:2603.12495v2, Theorems 2.5 and 3.3 and
Section 6.1. Reconcile its definitions with the fixture and thesis conventions.
Read `docs/consolidation/2026-09-24/claim-disposition.md` for the precise reason
COVER-L remains a research candidate. Earlier PASS records certify finite
predicates, not the missing continuum implication.

Use parallel assignments with disjoint outputs: one owner for the geometric
lemma; one for the exact certificate, coordinates and capacity/compactness
transfer; and one for the reader-facing account. Assign an independent proof
reviewer once a complete argument exists. Every delegation names scope, output,
deadline and receiving owner; workers must preserve others' edits. Coordinator
integrates and checks pending reviews and dependencies throughout.

For the geometric lemma, prove uniformly that the convex-hull area bounds the
selected loop's signed area up to `C r²` for small vertex displacements. The
supplied loop-deletion argument needs explicit intersection localization,
simultaneous deletion, orientation and degeneracy treatment. Challenge those
points. Compare a replacement proof if it is clearer. An arbitrary signed-area
versus convex-hull assertion is false; a naive support derivative can miss
moving-edge terms. Derive the actual finite row gradients and uniform remainder.

Reconstruct how the positive product distribution and rank 18 imply a uniform
linear area increase after anchoring one triangle. Prove unique base placements
using exposed difference-body vertices; give the compactness argument for all
nearby convex covers and the equality case. Verify one-factor capacity scaling,
continuity and normalization back to systolic ratio. The certificate's fixed
linear coordinate change is a proof device, not a symplectic symmetry of one
factor. State the theorem in physical coordinates. If claiming the reflected
placement at -H as well, show the complete normal-triangle family's permutation
under central reflection and transfer the proof explicitly; the current local
verifier checks H. Distinguish both local statements from any global assertion.

Inspect the verifier's inputs and write effects, then run its exact check into
fresh owned output. Retain the command, timing, input hashes and result. Ensure
assertions remain enabled. Challenge certificate-to-lemma matching rather than
counting another PASS as proof. No large enumeration or retained-data overwrite.

Own new support under `formal/pentagon-local-cover/` and a candidate section
under `thesis/candidates/pentagon-local-cover/`. Preserve the active manuscript
and other candidates. Explain the fixed-pentagon question, the actual local
conclusion, its proof and the remaining global question in a complete reading
context. Use the accepted short symmetric-partner summary recorded in
`docs/coordination/five-hour-task-structure.md` as a clarity reference; its
acceptance does not certify new prose. Reuse and reference the existing symmetric
candidate rather than rewriting it. Coordinator owns a complete combined reading
context and integration patch for the selected symmetric result and completed
local candidate. Check current ownership before preparing it. Supply concrete
insertion locations; do not silently promote an unaccepted theorem.

Read the clock at launch and checkpoints. Return a first substantive proof or
specific obstruction within roughly one hour. Reserve the final hour for
independent reconstruction, correction and artifact checks. Persist usable
derivations throughout so exhaustion leaves evidence. Do not spend the remainder
on generic backlog work if this theorem is blocked: repair its named gap or
retain the strongest checked partial statement with the exact obstruction.

Build and inspect the candidate PDF, including surrounding definitions and
figures. Return the complete theorem/proof, exact run receipt, source/PDF
identities, integration patch and a short `RESULT.md`: what follows, what remains
unproved, and precisely what Jörn must assess. Update coordination and reconcile
the relevant thesis dashboard status without representing the active manuscript
as containing this candidate. Independent review and a build do not confer human
acceptance. Success is a fully supported and readable candidate theorem; if it
cannot be completed, identify the failing implication and its strongest valid
replacement rather than claiming completion.
