# DS reviewer discrimination and live transfer, 11 October 2026

This packet retains a bounded early check of one fresh Sol reviewer. The historical task used two complete blinded DS subsections with a common factual context. `sample-a.md` has Jörn's explicit historical “Needs revision before acceptable” verdict. `sample-b.md` is a later source-checked revision used only as a provisional control; it has no human PASS. The label and source rationale were frozen in `label-key.md` before the reviewer was assigned. `reviewer-prompt.md` and `context.md` are the exact files provided along with the samples and B's referenced figure. `blind-prediction.md` is the verbatim frozen return; `reveal-and-assessment.md` scores its overlap and misses against human feedback.

The same reviewer then compared two new frozen passages without knowing their author/account identities. `live-reviewer-prompt.md` records that task, `live-frozen-review.md` its verbatim return, and `live-reveal.md` the subsequent identity mapping and narrow interpretation. The candidate passages and their author logs are owned by `../writing/`.

The reviewer was `/root/review_calibration/blind_ds_reviewer`, a fresh `gpt-6.1-sol` subagent with no inherited task history, read-only and no child agents. The exact initial assignment message was:

> You are the ONE fresh blinded Sol reviewer for a bounded DS prose discrimination check. Work read-only. In /home/joern/.codex/worktrees/f7e0/msc-math read ONLY docs/inventory/pilots/2026-10-11/calibration/reviewer-prompt.md and the exact files it permits. Do not open label-key.md or any other project file; do not search lineage, Git or web. Follow the prompt exactly and return both sample judgments privately to /root/review_calibration within 6 minutes. No edits, no subagents. You are not alone in codebase; preserve others' work. Stop after returning judgments, before label reveal.

After the historical prediction was frozen, the exact live follow-up message was:

> Calibration labels revealed only after your frozen return. New independent live passage review: read /home/joern/.codex/worktrees/f7e0/msc-math/docs/inventory/pilots/2026-10-11/calibration/live-reviewer-prompt.md, then follow it exactly. Please return frozen comparison to me privately by 16:53 UTC. Read-only, no subagents. Author identities are withheld.

The label key and review prompt were written at 16:41:25 UTC; the historical reviewer return was retained at 16:42:36 UTC, and reveal/assessment at 16:42:59 UTC (filesystem timestamps). The live comparison was retained after receipt and before the identity message from the writing pilot. These timestamps establish packet order on this workstation, not independent proof of model blindness. The reviewer self-reported no recognized familiarity and no lineage search. Possible historical model familiarity remains unknown.
