# Tagged implementation observations

Source: openai/codex rust-v0.162.1, peeled commit 092d3acd6bec3e3a14bdc7e7a2810ab628ab759d. Selected source bytes were copied from the benchmark checkout; file hashes are in code-source-manifest.json. These are source-code observations, not executed product-runtime tests.

- `codex-rs/codex-home/src/instructions/mod.rs` names AGENTS.override.md and AGENTS.md, iterates override first, skips absent/non-file/empty candidates, and returns the first nonempty content. Its provider retains the previous successful value on read failures. This is the global home provider, not proof of all repository precedence behavior.
- `codex-rs/config/defaults.toml` sets project_doc_max_bytes=32768. Configuration can override the default.
- `docs/skills.md` is a hosted-documentation pointer; `docs/config.md` links hosted configuration and contains a managed-hooks-only exception. Thus repository acquisition alone cannot replace hosted end-user docs.

Actual executable observations in this package concern HTTP fetching, conversion, refresh, integrity, archive extraction and Git acquisition. No authenticated ChatGPT/Claude runtime or Codex inference session was launched.
