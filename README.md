# Probing Viterbo's conjecture

Jörn Stöhler's MSc thesis combines mathematical proofs, computational symplectic
geometry and empirical investigations. The last attributed status (Jörn,
23 October 2026) is unfinished and unsubmitted, with failing preliminary checks;
newer institutional status has not been established here.

- **Find current project work:** [PLAN.md](PLAN.md) is the readable work document,
  covering purpose, thesis/admin outcomes, research alternatives and evidence;
  [STATUS.md](STATUS.md) is its short entry view. Use the shared
  `$work-document`
  to initialize, read, search or maintain it. The completed 28–29 October research campaign
  is consolidated in this checkout. Its 56-page report and source/evidence
  archive are linked from the [convex systolic optimization account](topics/capacity-and-qp/convex-systolic-optimization/README.md);
  this is a research result, separate from the unfinished selected thesis.
  The campaign worktree was retired after consolidation. Future research
  allocation remains open and unassigned. The [HKO cut-family handoff](topics/variation-and-local-maxima/hko-cut-family/README.md)
  retains exact conventions and labelled candidate inputs for a future bounded step.
  The [HKO postmortem](docs/history/hko-research-postmortem-20261029/report.html)
  explains the successful body’s causal path, costs and specific shortcuts.
- **Understand the research:** the [research guide](docs/research-guide.md)
  explains the questions and support. Topic accounts cover
  [polytope geometry](topics/polytope-geometry/README.md),
  [capacity and QP](topics/capacity-and-qp/README.md),
  [flow graphs](topics/flow-graphs/README.md),
  [products](topics/products/README.md),
  [variation and local maxima](topics/variation-and-local-maxima/README.md),
  [empirical studies](topics/empirical-studies/README.md), and
  [literature](topics/literature/README.md).
- **Read or edit the thesis:** [thesis/README.md](thesis/README.md).
  Build with `sh thesis/build.sh`; output is `thesis/build/main.pdf`.
- **Find code and proof entry points:** [crates/](crates/README.md) owns
  reusable Rust interfaces; topic accounts identify canonical proofs, producers,
  evidence and caveats. [formal/](formal/README.md) retains the combined proof
  build and compatibility inputs; [experiments/](experiments/README.md) routes
  producer packages that remain at their conventional paths.
- **Set up or reproduce:** [development environment](meta-topics/development-environment/README.md),
  [installation workflow](meta-topics/development-environment/installation-workflow.md),
  [artifact contracts](docs/artifacts.md), and [test scope](docs/algorithm-testing.md).
- **Continue with a cloud agent:** [cloud work and handoff](meta-topics/development-environment/cloud-workflow.md)
  covers source delivery, instruction discovery, task-specific setup, continuation
  and returned work. Product mechanics belong to the repository's
  `$agent-platform-reference`.
- **Coordinate or continue:** start at [PLAN.md](PLAN.md).
  [Project management](meta-topics/project-management/README.md) routes detailed
  obligations and the retained [registry/dashboard contract](docs/coordination/README.md).
  For that legacy view, open this checkout's `docs/dashboard/index.html` through the
  [workstation file viewer](docs/file-viewer.md); its URLs identify the checkout.
- **Maintain the repository and workflow:** [repository organization](meta-topics/repository-organization/README.md),
  the [inventory](meta-topics/repository-organization/inventory/README.md), and
  [agent workflow](meta-topics/agent-workflow/README.md) identify ownership,
  migration coverage and durable process knowledge.
- **Prepare submission:** [thesis submission](meta-topics/thesis-submission/README.md)
  owns administrative requirements and release exclusions.

`thesis/` contains the selected manuscript and its build inputs. Superseded
manuscripts remain in Git history. Topic accounts distinguish established
results, finite evidence, rejected routes and unresolved obligations. Passing a
build or test does not establish mathematical validity, adequate exposition or
thesis acceptance.
