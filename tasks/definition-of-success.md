# Definition Of Success

Agents must not replace thesis success with easier checks.

## Core Success

Success means:

- `thesis/build/main.pdf` is a defensible master thesis by Jörn Stöhler.
- Thesis claims have the proof, evidence, caveats, and review required by their
  strength.
- Repo state needed by thesis claims, reproducibility promises, and archive
  promises is true or caveated.
- Required university submission artifacts are submitted or ready at the stage
  where they are required.
- Jörn accepts that no remaining thesis-scope work blocks submission.

Success does not require every interesting side route, broad cleanup program,
publication-grade extension, or post-thesis dissemination idea.

## Authority

- Jörn final acceptance is required for thesis readiness.
- Jörn/Kai approval is required for theorem-strength proof acceptance where the
  thesis route requires it.
- Advisor/context decisions may make a story sufficient, optional, or future;
  they do not by themselves settle theorem wording, proof correctness, final
  prose readiness, or submission readiness.
- If Kai or Elizabeth provide blocker feedback on clarity, proof support,
  scope, or submission readiness, resolve it or record Jörn's explicit
  non-blocking/cut decision before final readiness.
- Agents classify, prepare, verify, and recommend. They must escalate decisions
  that depend on Jörn/Kai mathematical judgment, scope judgment, or advisor
  context.
- Tool checks, tests, and review passes are evidence. They replace Jörn/Kai
  authority only when the success condition is explicitly the check itself.

## Failure Guards

Agents must not count the thesis as successful by:

- accepting theorem-strength wording without required proof, review, or caveat;
- presenting empirical or bounded-search evidence as a density theorem,
  impossibility theorem, or exhaustive proof;
- using legacy thesis prose as current thesis truth without revalidation;
- using `/tmp`, chat, stale maps, deleted worktrees, or unsupported prose as
  source truth for claim-bearing evidence;
- making broad code cleanup, solver formalization, or publication compute a
  default thesis obligation when retained thesis wording does not need it;
- treating administrative submission work as a substitute for thesis-content
  readiness;
- treating a green check or review pass as a Jörn/Kai decision where such a
  decision is required.

## Story Conditions

### HKO

Every theorem-strength HKO claim retained in the thesis needs exact proof
support or honest weakening. If the route requires Jörn/Kai review, the wording
is not accepted until that review happens.

Packet 3 is current evidence toward the HKO condition. Packet 3 closing is not
itself the success condition.

### Hostile Landscape

The hostile-landscape story needs repo-owned evidence with explicit data,
method, compute-budget, and scope caveats.

Bounded negative-search evidence must not be stated as a density theorem,
impossibility theorem, or exhaustive search claim.

### Sys First-Order

If retained thesis/HKO wording relies on arbitrary-polytope first-order
behavior, the accepted route must provide the needed compute-once evaluator or
the thesis must weaken/caveat the claim.

A generic smooth-branch or Danskin-style statement is not a substitute for the
non-generic arbitrary-polytope theorem.

### Numerics And Code

Every numerical/code claim retained in the thesis or final repo promises needs
support at the strength used in the thesis.

Reruns suffice for retained experiments when they support exactly the claim made
in the thesis. A public certified solver claim needs certification support. A
broad solver formalization is not a default requirement.

## Final Acceptance Gate

<!--
Migration-review note: live-test this definition by asking a fresh agent, without
executing the review, what concrete checks it would run before asking Jörn for
final acceptance. A good answer should derive checks from current thesis text
and source truth, including retained claims/stories, HKO, hostile landscape,
sys first-order dependencies, numerics/code claims, provenance/references, repo
promises, PDF build/readability, submission/admin state, and Jörn/Kai-only
decisions. A bad answer treats this file as a standalone checklist, performs
only a cheap TODO/stale-link skim, or asks Jörn for acceptance before preparing
evidence and named caveats. This comment exists because no separate live
repeated-check procedure is retained.
-->
Jörn can declare the thesis done when all conditions below hold, or when Jörn
explicitly accepts a named caveat as non-blocking for submission.

- Every retained thesis story has its proof, interpretation, writeup,
  verification, and cut/weaken obligations closed.
- Moving an obligation to future/cut is valid only if retained thesis wording no
  longer depends on it, or Jörn explicitly accepts the caveat as non-blocking.
- The final thesis PDF builds, has no silent placeholders, and has passed the
  intended readability/proofread review level.
- Claim support has been checked for missing proof, missing evidence, stale
  interpretation, and uncaveated overclaim.
- Bibliography, cross-references, theorem/proof references, figures, tables,
  experiment artifacts, datasets, and code references resolve at the level the
  thesis uses them.
- Every thesis-facing repo, code, command, reproducibility, and archive promise
  is true or caveated.
- External submission prerequisites have been checked; no hidden thesis-content
  work remains behind an administrative unknown.
- Required university forms, uploads, printed copies, archives, and other
  submission artifacts are completed or ready at the stage where they are
  required.
- Remaining open work is non-thesis future/follow-up, external-clock submission
  mechanics, or explicitly accepted by Jörn as non-blocking.
- Jörn says the thesis is ready to submit.

Reusable verification checks are separate from this final acceptance gate. At
migration time no separate live repeated-check procedure is retained. Rebuild or
refresh repeated check definitions from this file, current thesis text, and
source truth before final signoff. Do not expand this file into a full
repeated-check procedure; do not skip repeated checks because this gate is
compact.

Submission and archive work follows thesis done, except earlier external-clock
preparation. Final project closure means no further direct repo-related
master-thesis action remains; the final GitHub archive/read-only action is the
last direct repo action.
