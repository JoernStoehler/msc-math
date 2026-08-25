# Environment health validation

This playbook validates one exact `msc-math` environment instance. It combines
deterministic, read-only checks with observations that only the running Codex
actor can make.

The central distinction is:

```text
supported != configured != authenticated != exposed != callable != persistent
```

## Support and prior-art gate

Before adding credentials, changing an environment, or running a live canary,
ask whether someone has probably attempted the same product, surface, and
capability combination. A focused search often costs seconds and can avoid
minutes or hours of redundant work.

1. Search the exact product, execution surface, and capability together.
2. Check the vendor's current supported-surface documentation.
3. Search the primary issue tracker and credible firsthand reports.
4. Record the residual unknown.
5. Test only that unknown with the smallest reversible experiment.

When the chance of existing work is material and search cost is low, searching
is the default. Skipping even that cost/probability assessment is a process
failure. Investigation prompts must require this gate; correct a prompt that
fails to elicit it before reuse.

## Deterministic check

Run:

```bash
bash scripts/environment-health.sh
```

The script emits one JSON object. It never prints credential values and never
creates, updates, uploads, pushes, or deletes external data. Its default mode
performs authenticated read-only GitHub and R2 probes when the relevant clients
and credentials are available.

For a filesystem-and-process-only snapshot:

```bash
bash scripts/environment-health.sh --offline
```

## Internal Codex validation prompt

Start a fresh Codex thread in the environment under test and provide:

```text
Validate this exact msc-math environment. Treat AGENTS.md as authoritative.

1. Apply the support and prior-art gate in ENVIRONMENT_HEALTH.md before
   proposing any provisioning or live canary.
2. Inventory the native, connector, MCP, browser, and repository tools actually
   available to this thread. Do not infer tools from repository files.
3. Run `bash scripts/environment-health.sh` and interpret its JSON output.
4. Reconcile configured, authenticated, exposed, callable, and persistent
   states. A configured client or credential is not an exposed actor tool.
5. Do not push, open a pull request, write R2 objects, or mutate any external
   service. Mutation and persistence canaries require a separate instruction
   naming a disposable target.
6. Never print tokens, authorization headers, cookies, secret values, remote
   URLs containing credentials, or credential-file contents.
7. Return exactly one YAML document matching ENVIRONMENT_HEALTH.md's response
   contract. Use null for unavailable facts and explain false/null acceptance
   fields in blockers.
```

## Response contract

```yaml
schema_version: 1
environment: "<label or null>"
run_id: "<operator correlation ID or null>"
repository:
  expected: "JoernStoehler/msc-math"
  identity_ok: true
  branch: "<branch or DETACHED>"
  head_sha: "<full SHA>"
  worktree_clean: true
tool_inventory:
  native: []
  connectors: []
  mcp_servers: []
  browser: []
  repository_tools: []
automated:
  command: "bash scripts/environment-health.sh"
  exit_code: 0
  output_interpreted: true
github:
  client_installed: true
  authenticated: true
  repository_read: true
  mutation_attempted: false
r2:
  client_installed: true
  configured: true
  config_mode_600: true
  snapshots_read: true
  mutation_attempted: false
runtime:
  python_3_12: true
  uv: true
  rust_1_94: true
  cargo: true
  rustfmt: true
  latexmk: true
  biber: true
secret_lifecycle:
  setup_secrets_absent_from_agent: true
persistence:
  fresh_process_verified: false
  fresh_environment_verified: false
overall: "pass | partial | fail"
blockers:
  - check: "<field path>"
    classification: "unsupported | not_configured | unauthenticated | not_exposed | call_failed | not_run | policy_blocked | unknown"
    reason: "<concise observed reason>"
```

Arrays contain only tools the actor actually sees. A successful read does not
prove write access, and a successful mutation does not prove persistence.

