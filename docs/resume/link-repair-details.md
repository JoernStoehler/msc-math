# Maintained Markdown link repair

The consolidation left 21 local links in maintained Markdown pointing at paths
removed during cleanup. Four links depended on worktrees retired before commit
`35f29db4`; the other 17 pointed at repository paths moved by the later layout
migration. Commit `9a976751` repaired the links, and commit `7b8a1b03` preserved
the original branch blobs for edited retained sources.

| # | Source and original line | Removed target | Replacement target |
|---:|---|---|---|
| 1 | `experiments/dev-gradient-ascent/METHOD-CANDIDATE.md:5` | `../../memories/todos.md` | `../../docs/WORK_REMAINING.md` |
| 2 | `experiments/dev-gradient-ascent/README.md:8` | `../../memories/todos.md` | `../../docs/WORK_REMAINING.md` |
| 3 | `experiments/sys-datascience/README.md:7` | `../../memories/todos.md` | `../../docs/WORK_REMAINING.md` |
| 4 | `experiments/dev-gradient-ascent/optimizer-comparison/README.md:35` | `../../../thesis/08-black-box-datascience-finite-budget-optimization.tex` | `../../../thesis/chapters/08-data-science.tex` |
| 5 | `experiments/sys-datascience/coordination/README.md:6` | `../../../memories/todos.md` | `../../../docs/WORK_REMAINING.md` |
| 6 | `experiments/sys-datascience/coordination/next-session-candidates.md:6` | `../../../memories/todos.md` | `../../../docs/WORK_REMAINING.md` |
| 7 | `experiments/sys-datascience/coordination/datascience-review-2026-09-14.md:90` | `../../../thesis/08-black-box-datascience.tex` | `../../../thesis/chapters/08-data-science.tex` |
| 8 | `experiments/sys-datascience/coordination/datascience-review-2026-09-14.md:91` | `../../../thesis/08-black-box-datascience-finite-budget-optimization.tex` | `../../../thesis/chapters/08-data-science.tex` |
| 9 | `experiments/sys-datascience/coordination/datascience-review-2026-09-14.md:92` | `../../../thesis/08-black-box-datascience-local-maxima-check.tex` | `../../../thesis/chapters/08-data-science.tex` |
| 10 | `experiments/sys-datascience/coordination/datascience-review-2026-09-14.md:93` | `../../../thesis/a-datascience-results.tex` | `../../../thesis/appendices/data-science.tex` |
| 11 | `experiments/sys-datascience/coordination/feature-completion-contract.md:5` | `../../../memories/todos.md` | `../../../docs/WORK_REMAINING.md` |
| 12 | `docs/ds-family-theorem-mapping/README.md:29` | `/workspaces/msc-math/.worktrees/ds-evidence-closure/docs/ds-evidence-closure/README.md` | `../ds-evidence-closure/README.md` |
| 13 | `docs/ds-family-theorem-mapping/README.md:62` | `/workspaces/msc-math/.worktrees/ds-evidence-closure/docs/ds-evidence-closure/current-panels.md` | `../ds-evidence-closure/current-panels.md` |
| 14 | `docs/open-thesis-literature/README.md:28` | `../literature-closure-plan/README.md` | `../history/literature-closure-plan/README.md` |
| 15 | `docs/empirical-viterbo-design/README.md:9` | `../../docs/review-evidence/human-feedback/ds-reading-2026-09-14.md` | `../history/review-evidence/human-feedback/ds-reading-2026-09-14.md` |
| 16 | `docs/empirical-viterbo-design/README.md:41` | `../../../ds-evidence-closure/docs/ds-evidence-closure/README.md` | `../ds-evidence-closure/README.md` |
| 17 | `docs/empirical-viterbo-design/desk/discovery-toolbox.md:143` | `/workspaces/msc-math/.worktrees/open-thesis-literature/docs/open-thesis-literature/README.md` | `../../open-thesis-literature/README.md` |
| 18 | `docs/empirical-viterbo-design/desk/research-agenda.md:17` | `../../task-graph/25-empirical-research.svg` | `../../history/task-graph/25-empirical-research.svg` |
| 19 | `formal/pentagon-affine-products/README.md:39` | `../../docs/pentagon-chapter-v2/chapter.tex` | `../../docs/history/pentagon-chapter-v2/chapter.tex` |
| 20 | `formal/pentagon-affine-products/README.md:66` | `../../thesis/candidate/pentagon-affine-draft/README.md` | `../../thesis/chapters/10-affine-pentagons.tex` |
| 21 | `formal/product-rotation/README.md:4` | `../../thesis/candidate/product-rotation-draft/section.tex` | `../../thesis/chapters/11-product-position.tex` |

The verification scan examined 372 local links in maintained Markdown, excluding
`docs/history/**`, `docs/resume/**`, writing-quality provenance copies, and frozen
writing-quality run packets. It found zero broken links. `docs/resume/verify.py`
also passes after checking all 3,370 branch records and retained snapshots, the
frozen PDF, and 51 selected build inputs.
