# AI In The Research Process: Content Companion

Status: evidence-bounded revision companion for `thesis/13-use-of-ai.tex`.
The 2026-08-28 revision has not been reviewed or accepted by Jörn as final
publication prose. This file is maintenance context, not thesis text or
independent evidence.

Purpose: keep the numbered section distinct from the factual disclosure. The
section now presents one reproducible methodological observation: for a known
code defect, a passing regression was informative only when replay showed that
it failed after the defect was restored. It does not attempt the broader
productivity or mathematical-labor analysis that the previous draft could not
support.

## Direct support

- Frozen source state:
  `f3d36cc968716132af582282dbe6c137a2857ec4`.
- Historical repair and regression commits: `94fee3e8` and `e56cf161`.
- Retained case-study contract and limitations:
  `experiments/ai-use/sign-replay/README.md`.
- Executable replay:
  `experiments/ai-use/sign-replay/sign_replay.py`.
- Transformation/contract tests:
  `experiments/ai-use/sign-replay/test_sign_replay.py`.

The replay toggles the reduced-gradient coefficient between the historical
positive sign and the corrected negative sign, injects each reconstructed
regression into a temporary worktree at the frozen base, and runs the named
Rust test. Its expected and reproduced matrix is:

| Recorded condition | Bad sign | Correct sign |
|---|---:|---:|
| GPT-5.5, minimal | fail | pass |
| GPT-5.5, verifier first | fail | pass |
| GPT-5.6-sol, minimal | pass | pass |
| GPT-5.6-sol, verifier first | fail | pass |

The exact prompts are unrecoverable. The contracts are contemporaneous
reconstructions. There was one run per cell, no randomization, no
within-condition replication, and the defect category was supplied. The replay
checks mutation sensitivity of the reconstructed regressions; it does not
reproduce the original interactions or identify a model or prompting effect.

## Claims deliberately removed

The revision omits the earlier retrospective timing and speedup anecdotes,
session and changed-line counts, broad claims about parallel breadth, selected
proof and data-science episodes, and generalized working principles. The
available evidence does not support using those selected cases to estimate:

- active human labor, time saved, cost, or net productivity;
- comparative model or prompt performance;
- success rates or a ranked frontier across mathematical labor;
- causal effects of AI use, review topology, or parallelism;
- idea origin or semantic acceptance from Git history.

The section also avoids first-person retrospective judgments because no newer
Jörn-authored statement was established for this revision.

## Remaining publication gate

Jörn must decide whether the narrow case belongs in the submitted thesis and
whether the revised prose is accurate and thesis-worthy. Agent review, a clean
build, and a reproduced mutation matrix do not establish his acceptance.
