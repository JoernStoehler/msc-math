# Choose The Working Surface

Use this reference when deciding where thesis-related text, state, evidence,
or temporary reasoning belongs.

- Put reader-facing prose, mathematical statements, figure/table environments,
  captions, inclusion commands, labels, and necessary local status comments
  in active `thesis/*.tex`.
- Put chapter-local source routes, interpretations, caveats, fallback wording,
  open decisions, and review state in the owning companion.
- Put proof development and developer-facing mathematics in `formal/`.
- Keep code, data, producer commands, empirical reports, and generated assets
  with the self-contained experiment packet whose question makes them
  interpretable. Record other thesis, method, and implementation relationships
  as links instead of moving the packet into another category.
- Treat `thesis/legacy/` as stale source material, never as active prose.
- Use `/tmp` for disposable reasoning, candidate comparisons, prompts, and
  review packets that do not justify durable maintenance.
- Use `docs/project-status.md` for current whole-project milestone and gate
  state, and `docs/project-facts.md` for still-current accepted project facts.

Keep `thesis/` self-contained at build time. Deliberately copy selected
publication asset files from their producer owners into the thesis tree; keep
thesis-native assets with their thesis owner. Do not create build-time links to
`formal/`, `experiments/`, or `crates/`.

Choose by the future consumer, not by file extension convenience. Do not turn
active TeX into a planning ledger or a companion into substitute source truth.
