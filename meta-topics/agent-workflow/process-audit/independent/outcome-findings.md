# Independent outcome-artifact audit

Frozen initial findings, 2026-09-15. Scope: frozen 86-page thesis, human feedback records, frozen pre-Pro self-review, and unchanged Pro review package. I did not read the prior process audit's REPORT.md, BRIEF.md, assignment, or other auditors' findings. This is independent of that process audit, **not** independent of the Pro mathematical diagnosis: I deliberately used it as outcome evidence and attribute its discoveries below. No logs inspected, no thesis/configuration modifications, no verification campaign run.

## Central diagnosis

The available evidence supports a gap between producing defensible local components and choosing/composing a defensible whole thesis. HKO and Chapter 4 PASS judgments were correctly scoped in the contemporaneous feedback records; those records do not themselves establish that the coordinator mistakenly promoted them to whole-thesis approval. The failure is better investigated as allocation and approach selection than as a simple verdict bookkeeping error. A precise causal claim about why particular tasks displaced others needs the trajectory audit.

The later review's highest-value contributions change the mathematical and empirical approach, not just execution speed: an analytic proof replaces an enumeration-dependent proof, and an exact descriptor law links chapters that otherwise end with an unfulfilled research question. This is evidence that incumbent completion was not the only valuable target. It is not proof that an earlier agent would reliably have found these alternatives, nor that the numerical work was worthless.

## Evidence and findings

### O1. Review scope failed to cover a real user-valued dimension before human reading

**Direct record:** `.git/codex/review-calibration/whole-thesis-reading-1700.md`, subsection “Review after switching to the fixed 17:30 PDF”: Jörn rejected the preliminaries opener and asked whether Codex should have caught the broader prose class. The coordinator withdrew implied prose readiness, acknowledging that preflight covered mathematical prerequisites/inference defects and explicitly excluded style inventory. The subsequent Chapter 4 PASS included “borderline” writing quality.

**Inference:** a proof-focused preflight was an inadequate readiness proxy for an intended mathematical reader. This is a concrete mismatch between review contract and product success; it does not imply every reviewer needs an exhaustive style inventory. Earlier task prompts could have required judging the chapter as a reader's explanation, with the return stating both covered and uncovered success dimensions. Evidence supports an earlier check being useful; it does not quantify elapsed-time savings.

**Preserve:** fixed PDFs during human reading and restrained interpretation of unfinished/partial feedback. `hko-reading-feedback.md` carefully separates initial praise, later annotations, prospective predictions, and completed chapter PASS. That evidence discipline is valuable.

### O2. Correctness validation could coexist with a substantially inferior proof approach

**Printed thesis:** pp. 59–62 and Appendix B rely on a classifier for Theorem 9.1. Page 59 already prints angle-dependent facet rows and a constant positive feasible weight vector whose objective is `cos(theta)/(2(1+cos(pi/5))^2)`.

**Reviewer contribution:** `pro-review-2026-09-15/thesis_review/03_mathematical_audit.md`, §2 proves the same profile from angle-independent feasible weights, harmonic objectives, equal known endpoint maxima, and positive trigonometric interpolation. I checked the reasoning against the displayed thesis candidate: rotation preserves factor closure; mixed pairings are first harmonics; nonnegative endpoint interpolation bounds every feasible candidate; the printed candidate attains the bound. This is a compelling replacement route, although I did not reproduce the full external source normalization audit.

**Earlier plausible alternative:** at the point a costly exhaustive classifier became the proposed proof, give a bounded independent challenge the definitions, known endpoint values, and candidate—not classifier implementation—and ask whether all competitors admit a uniform bound. That information is sufficient in retrospect. Discovery probability at that earlier point is unknown. The correct lesson is a selective challenge at expensive approach commitments, not a mandatory competing-proof search for every lemma.

**Cost evidence:** the frozen pre-Pro self-review reports a retained full classifier run of about 2,614 seconds. This is reported duration of that run only, not total avoidable cost, and cannot be assigned as savings: the computation may have supplied discovery evidence and confidence needed to attempt the proof.

### O3. Cross-chapter synthesis would have generated new mathematical value

**Printed thesis:** p. 52 §8.4 asks for a restricted family where the edge-and-width descriptor has a predictable relation to capacity, then says current evidence does not supply it. Chapter 9 supplies the rotated-pentagon capacity profile. Chapter 8 already supplies the product-ridge formula.

**Reviewer contribution:** mathematical audit §3 combines those ingredients into `S(K_theta)=16 sin(pi/5) cos(d(theta))` and `sys(K_theta) S(K_theta)^2 = 16(3+sqrt(5))`. The derivation follows by pairing the five absolute-cosine terms; it does not explain the pooled correlation or imply universal monotonicity. The thesis's cube counterexample remains correct.

**Inference:** separation of local chapter responsibilities can leave useful relations undiscovered even when all inputs exist. This is a particularly strong candidate for a lightweight integrator task: inspect open questions against results/definitions elsewhere in the manuscript. A review that merely asks whether each chapter's claims are supported would miss this opportunity.

### O4. Empirical caveats were doing work that primary evidence and composition needed to do

**Printed thesis:** pp. 50–51 report correlations −0.938/−0.205 and random-forest performance with 39 descriptors, but provide no inspectable scatter plot of the central association or full feature/split/configuration specification. Pages 52–57 contain many experiments whose differing source laws, controls, budgets, and estimands impose substantial reading load. Appendix empirical evidence restrictions are not a substitute for reproducible primary claims.

**Corroboration:** frozen pre-Pro findings F1–F3 and Pro findings R07–R09/R13–R19 agree on these concrete deficits. This agreement is not an independent model ranking: the self-review knew the user's FAIL and prior feedback; review budgets and context differed.

**Earlier plausible alternative:** choose a small set of thesis-facing claims early and require a claim's figure/table plus recoverable producer/inputs before promoting it to primary evidence. Demote unavailable historical results to discovery material. A smaller reproducible experiment may dominate adding more comparisons; whether it could be completed before the deadline requires data/pipeline evidence not assessed here.

**Preserve:** prospective selection, shared starts, controls, distinctions between numerical misses and proofs, and honest disclosure of admission conditioning. The frozen chapter provides these repeatedly. Simplification must retain the distinctions needed to interpret results rather than replacing careful science with a cleaner but false story.

### O5. A source-checked patch still introduced a stale field-level assertion

**Human-feedback record:** the 17:00 reading record says a primary-source search found HKO Remark 1.4(v) leaving EHZ/cylindrical equality open, and that the candidate distinguished this from Edtmair's local result.

**Outcome:** Pro R01 identifies the resulting p. 5 open-problem assertion as false. I independently opened [Abbondandolo–Edtmair–Kang, arXiv:2412.01777](https://arxiv.org/abs/2412.01777), submitted 2 December 2024: the abstract states cylindrical capacity coincides with minimal action for convex bodies in R4. The primary source predates the sprint.

**Inference:** verifying that an older source says a problem is open is not verifying current open status. An explicit current-status check is warranted for consequential claims such as “remains open”, “first”, or “unknown”; blanket bibliography recrawls would have higher overhead. This error is a concrete available-information miss, without establishing why the search missed it or what causal effect it had on the user's FAIL.

## Small prioritized intervention candidates

These are scoped prompt/task-design proposals under harness-engineering-v2, not activated instructions. No changes to loaded guidance are made.

| Priority | Decision and surface | Observable bounded test | Overhead / adverse effects |
|---|---|---|---|
| 1 | Whole-thesis integrator prompt: judge the submitted reader experience, evidence package, and unresolved questions against results elsewhere before declaring a candidate review-ready. | Give a fresh reviewer an unseen manuscript with one locally correct but disconnected result and one unsupported primary empirical claim. Judge whether it identifies both and proposes a concrete reorganization/demotion; include a coherent negative-control manuscript to penalize invented restructuring. | One bounded review at an integration checkpoint. Risk: generic structural complaints or needless rewrites; require passage-to-result anchors. |
| 2 | Expensive-proof commitment prompt: when an exhaustive proof becomes a major remaining cost, briefly challenge whether existing symmetry, convexity, interpolation, or invariant structure can bound all competitors. Keep the incumbent available. | Use held-out mathematical tasks with and without structural shortcuts; evaluate valid alternative proofs and appropriate continuation of computation, not number of suggestions. | Bounded challenge only at consequential commitments. Risk: speculative shortcuts derail a correct proof; require derivation before switching. |
| 3 | Empirical chapter's claim/evidence inventory, owned with research artifacts: primary claim, reader-visible output, producer/input identity, and disposition if unavailable. | Have a fresh worker reconstruct one headline claim from supplied release material. Success is reconstruction or correct demotion, not merely existence of a manifest. | Small per-primary-claim bookkeeping; potential heavy repair becomes explicit early. Avoid inventorying every exploratory run. |
| 4 | Literature task prompt: verify current status of consequential open/priority claims against a dated primary source and recent citing/updating work. | A stale-but-accurately-quoted open-problem passage plus an actually open nearby claim; reviewer must correct the former while preserving uncertainty for the latter. | Targeted searches, not whole-field surveillance. Risk: false confidence from search absence; require support for positive claims and qualify unsettled scope. |

Tests are proposed, not performed. The second-day task can exercise them prospectively on actual remaining work; no paid replay campaign is needed to obtain initial behavioral evidence. Do not make success on these historical examples proof of generalization.

## Limits

No institutional grading rubric, complete account of Jörn's FAIL reasons, full mathematical software audit, or trajectory-level allocation evidence was inspected. The Pro report recommends major revision and withholds a numerical mark; it should not be silently equated to Jörn's binary verdict or treated as adjudicated truth on every editorial preference. I inspected extracted thesis passages rather than rendering all pages, and did not independently reproduce the HKO certificate. Its positive exact verification is attributed to the supplied review. These limits permit a strong diagnosis of outcome gaps, but not numerical causal shares, model rankings, or a guaranteed path to PASS.
