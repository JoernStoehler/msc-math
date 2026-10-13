# Project skill ownership

The selected replacements, authorized by Jörn on 5 October 2026 and refined with his design judgment on 6 October, are:

- `problem-structuring`: develop problems, goals and decisions for useful autonomy; independent investigation and user interaction where needed.
- `external-agent-communication`: independent-context exchange, replacing project `cross-session-messaging` and `chatgpt-pro`, including the complete Pro round trip and inline technical channel usage.
- `harness-engineering`: stable invocation name; former revision label retained as string `metadata.version: "2"`. Includes optional companion design memory at `references/design-notes.md`.

The retained project packages `human-prose-review`, `live-task-graph` and `review-files` received bounded pre-sweep repairs on 6 October. `codex-usage`, `incident-log` and `lesswrong-fetch` originated as preservation copies of the installed agent-skills cache revision `0.1.0+codex.20260927080601`. They are not the semantic adaptations proposed in September. Their original external prerequisites and caveats remain; this rollout makes no fresh-clone execution promise for them.

## Provenance and deliberate changes

Upstream comparison source is [Jörn's agent-skills](https://github.com/JoernStoehler/agent-skills), installed cache at `/home/joern/.codex/plugins/cache/agent-skills/agent-skills/0.1.0+codex.20260927080601/skills`. At the initial preservation check, operational files and modes matched the cache; the incident-log interface invocation selector changed from `$agent-skills:incident-log` to `$incident-log` and a maintenance-only design-note link was later added. Its intake instructions were preserved at that checkpoint; the later unavailable-owner fallback is recorded below. Subsequent bounded repairs to other packages are summarized below. The harness replacement derives from its `harness-engineering-v2` package; changes add design-memory guidance, stable naming, common metadata and rationale. The source package in `/workspaces/agent-skills/plugins/agent-skills/skills/harness-engineering/` receives the same semantic amendment; the project copy uses its local invocation selector. No plugin cache is edited or global deployment performed.

Problem structuring deliberately retires compulsory structure maps, predictions and numbered claim batches while retaining decomposition, probes, readiness, independent investigation, goal/problem co-development and useful handoff contracts. Communication preserves incoming/outgoing message semantics, requested receipts through agent routes, Pro authority/prompt/commands, inbox preservation and return integration. [Analysis and incident limits](../../meta-topics/agent-workflow/collaboration-guidance-design-2026-10-05.md) own the case for these choices; per-skill design notes retain maintenance context.

## Discovery and maintenance

Canonical owners are these repository packages. `.claude/skills/<name>` links to each owner; root `CLAUDE.md` imports `AGENTS.md`. Project Codex configuration disables `agent-skills@agent-skills`; Claude settings disable known marketplace identities (`agent-skills@jstoehler` and `agent-skills@agent-skills`) and the observed ChatGPT sync identity (`agent-skills@synced`). This prevents competing old owners while retaining unrelated capabilities locally. Host settings, installations and the separate canary plugin are unchanged. Existing sessions retain their already-loaded context.

Updates are reviewed here, not automatically synchronized. Historical inactive candidates are evidence, not discovery owners. Structural checks and observed discovery are recorded in [validation](../../meta-topics/agent-workflow/skill-replacement-validation-20261005.md); neither establishes behavioral benefit.

The communication package currently keeps runtime technical usage and work guidance in one sectioned `SKILL.md`. Only design memory is separate. This follows Jörn's staged design preference: establish useful behavior before cost-driven splitting; the current structural/discovery checks do not establish that behavioral baseline or near-optimality. The harness records this preference and asks reviews to assess actual usage and maintenance costs.

Companion audit (6 October): all nine packages now have linked design notes. The five additions retain specific pre-sweep findings and unresolved workflow questions; notes remain conditional on useful maintenance context, not a file-per-package requirement. [Incident assessment](../../docs/history/incident-log-evaluation-20261006/README.md) and [full-set pre-sweep](../../docs/history/skill-presweep-20261006/README.md) record evidence and limits.

## Pre-sweep repairs, 6 October 2026

The full-set review corrected generated-graph ownership, obsolete review handoffs, unknown-device delivery, unconditional Pandoc dependency, wrapper zero timeouts/temporary cleanup, bootstrap preservation of unexpected session objects, and fetch fallback/output/feed wording. It added contextual transfer of prior expert advice to the harness and retained meaningful maintenance notes. The preserved packages are therefore no longer byte-identical upstream copies. Bootstrap still installs/links only within authorized setup; per-grant login authority and the sandbox default remain. No current cloud execution, global rollout or service change was performed. Later bounded public query checks are retained in the pre-sweep report. Following independent content/application review, the project now integrates `codex-resources` and renames `codex-usage` to `codex-cli-claude-cloud`, migrating the Claude discovery link and maintained runtime caller. Earlier candidates remain historical. The resource package supplies guidance and inspection routes, not a persistent reset controller. Incident intake now has an unavailable-owner fallback. Existing loaded catalogs may retain earlier triggers. The retained discovery checks precede these repairs and do not establish fresh discovery of the revised source.

The 6 October inventory was ten packages: the original nine, with the cloud CLI package renamed, plus resource guidance. The [integrated review](../../docs/history/skill-presweep-20261006/README.md#integrated-sweep-package) owns that checkpoint's dispositions and remaining limits.

## Exact-text message approval, 8 October 2026

Jörn requested actionable communication guidance: prepared messages identify a separate subagent that approved their exact text using a prepared workflow. The eleventh package, [message-review](message-review/SKILL.md), supplies the context-first reviewer prompt and versioned-candidate approval procedure. Root AGENTS.md owns the mandatory trigger; the skill owns its boundaries and procedure, and the Claude link points to the same owner. Reviewer identity and approval are checked against native returns by the sender. The initial schema/hash verifier was retired after Jörn challenged its cost and review constraints; prior packets retain its evidence. This is an instruction gate, not platform interception or established behavioral reliability. No Codex config edit or global installation is part of this change.

## Shell app-server client, 8 October 2026

Jörn requested [codex-app-server](codex-app-server/SKILL.md) for Claude-cloud and other shell agents lacking `codex_tui.*` tools. Its daemon-backed client supports persistent thread operations with explicit setup and per-grant device login. The Claude discovery link points to this owner; no plugin or host settings are installed. The existing one-shot `codex-cli-claude-cloud` workflow remains available. [Design/validation notes](codex-app-server/references/design-notes.md) distinguish workstation integration and offline tests from the untested fresh-cloud install/login/lifetime.

## Agent-platform reference, 13 October 2026

Jörn authorized [agent-platform-reference](agent-platform-reference/SKILL.md) as the repository-owned products reference. It retains the acquired corpus, provenance, curated corrections, disclosed-case demonstrations and negative evidence; `.claude/skills/agent-platform-reference` links to this owner. [The adoption record](agent-platform-reference/evidence/repository-adoption-20261013.json) identifies the received capsule and bounded tooling/retention changes. [REVIEW.md](agent-platform-reference/REVIEW.md) owns demonstrated usefulness and its limits. No deployment outside this repository is included.
