# Domain Conventions

Use this file for broad repo conventions that apply across multiple domain
surfaces. Language-specific details live in the matching skills.

## Source And Text

- **File headers:** Module-level source files start with a short purpose/context
  comment block. Small leaf files may rely on module docs and clear names.
  Detailed language-specific header rules live in the relevant convention
  skills.
- **Cross-file references:** Comments and notes should reference neighboring
  surfaces explicitly, e.g. `<file>.tex:\ref{label}`, `<file>.rs:symbol`, or
  `<file>.sage:symbol`.

## Math And Code

- **Feature lifecycle:** New exploratory code starts in the relevant
  `experiments/` subtree. Stable, approved algorithms migrate into `crates/`.
  Validation experiments either become crate tests or remain in `experiments/`.
- **Test/validation boundary:** Crate tests are fast live checks for developer
  feedback and ordinary regressions. Slow mathematical validation, edge-case
  searches, broad random sweeps, and generated evidence datasets live in
  `experiments/`.
- **Math-code correspondence:** Rust code cross-references formal mathematics
  when correctness depends on a formal result. Use labels such as `[lem:label]`,
  `[thm:label]`, or `[def:label]`; pure orchestration does not need a label. The
  matching `\label{...}` lives in `formal/*.tex`.

## Experiments And Data

- **Experiment paths:** Use semantic experiment paths. Do not force balanced
  subtrees when the semantics are asymmetric.
- **Research notes:** Put research-state notes, interpreted analysis, decision
  history, and next-step planning in `research/`. Keep only execution-facing
  packet docs under `experiments/`.
- **Data ownership:** Keep generated data with the producer that writes it.
  Avoid multiple binaries writing to the same tracked output.
- **JSONL / LFS safety:** `.jsonl` files are generated artifacts tracked by Git
  LFS. Trace figure, table, dataset, and experiment-artifact provenance with
  targeted `rg` and local source inspection. There is no repo-wide generated
  dataflow map; rebuild one only if repeated provenance work proves it is worth
  maintaining.
