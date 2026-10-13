  397     /// Optional verbosity control for GPT-5 models (Responses API `text.verbosity`).
  398     pub model_verbosity: Option<Verbosity>,
  399 
  400     /// Optional path to a JSON model catalog (applied on startup only).
  401     /// Per-thread `config` overrides are accepted but do not reapply this (no-ops).
  402     pub model_catalog_json: Option<AbsolutePathBuf>,
