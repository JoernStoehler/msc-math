# Probing Viterbo's conjecture

Jörn Stöhler's MSc thesis combines mathematical proofs, computational symplectic
geometry and empirical investigations. The manuscript is unfinished and has not
been submitted for final grading.

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
- **Coordinate or continue:** [project management](meta-topics/project-management/README.md)
  routes lasting decisions and current [coordination](docs/coordination/README.md).
  Open this checkout's `docs/dashboard/index.html` through the
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
