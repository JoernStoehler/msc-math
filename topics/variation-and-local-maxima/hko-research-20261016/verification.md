# Verification and integration record

16 October 2026, assigned checkout and branch `claude/hko-research-2026-10-16`.
The run began with a clean tracked/untracked status. No thesis source was edited.

- `python3 -O .../generic-counterexample/verify.py --output-dir /tmp/...` passed;
  regenerated fixture/certificate match retained bytes. The independent reviewer
  also reconstructed determinants and vertices by separate formulas.
- `cargo run --offline --release --locked -p exp-hko-local-maximum --bin
  hko-generic-counterexample -- FIXTURE OUTPUT` compiled and ran successfully.
  Production capacity bounds, exact volume and outward rational ratio rounding
  are retained. Root independently rechecked the rounding using Python Fractions.
- `cargo fmt --all -- --check` passed. The only Cargo interface change adds the
  experiment binary to the existing HKO package; no production algorithm changed.
- The partner owner executed the pinned NumPy/SciPy controls twice; all 20 passed
  with identical scientific output. Its independent reviewer inspected the proof,
  script, report and scope, without rerunning already-passed controls.
- The multicut owner executed 12 cases and exact geometry predicates. A separate
  reviewer replayed the exact predicates with `python -O`. Script hashes match
  both retained multicut reports; the numerical estimates are not enclosures.
- Local Markdown link checks passed; `git diff --check` passed.
- `scripts/render-workflow-graph.py --check` and
  `scripts/serve-project-dashboard.py --check` passed after adding explicit serving
  entries for the three result accounts. The existing scientific/PDF baseline
  reports unreconciled older sources; this run did not refresh that baseline or
  select/promote manuscript claims.

Native read-only reviewer `/root/reviewer` approved the final integrated research
index, generic-construction note and both topic-account additions. Native reviewer
`/root/hko_map` approved the generic construction and multicut argument; the
partner theorem was reviewed independently by `/root/reviewer`. Their substantive
findings and limits are preserved with each scientific packet. Root owns all
integration and the commit; workers made no commits. These checks establish the
stated finite evidence and review history, not external novelty or human acceptance.
