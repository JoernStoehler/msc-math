# Manage Experiment Data And Artifacts

Keep data beside its producer unless an established broader owner applies.
Preserve the command, inputs, parameters, seeds or selection rules, non-obvious
dependencies, and comparison contract needed for the intended downstream use.

Do not patch generated JSONL, CSV, tables, figures, or reports by hand.
Regenerate them or record the missing refresh. Stop and identify unexpected
tracked changes before treating them as evidence. Do not maintain README prose
as a second store of generated metric rows; point to the artifact or generate a
compact readable report.

Use current code, inputs, hashes, manifests, or reviewed verification to
identify an artifact. A recent timestamp is not identity. Recompute only when a
cheaper check cannot resolve an uncertainty material to the current decision.

Provide a cheap smoke path when it materially reduces implementation, review,
or rerun cost and checks the relevant input, build/binary, output schema,
parameter, provenance, or resume boundary. Do not retain smoke artifacts
without a downstream use or let successful plumbing imply research evidence.

For expensive runs, gate execution on the successful current build or an
identified binary in the same command path. Declare full-run and repair-run
budgets. Concurrent producers should write separate shards and merge only
after validation. Preserve partial outputs when their provenance and resume
semantics remain usable.

JSONL often works well for row-oriented heterogeneous records because it
streams and diffs cleanly, composes with Rust row types, and is directly
inspectable by agents. This is a design rationale, not a mandate: inspect
current producers and consumers and choose another format when its task-local
properties are better.

Put local purpose, provenance, interpretation, disposition, and reopen
constraints in the experiment or method owner; detailed records in generated
artifacts/reports; proof development in `formal/`; reusable code contracts in
crate documentation. Promote code or data to the narrowest shared owner after
multiple current consumers need it or duplication creates a concrete risk, not
speculatively.
