# Project skill migration proposal

**Recommendation:** Keep `.agents/skills` as the project-owned source, retain the
four existing workflows, and port the five additional upstream workflows with
the adaptations below. Expose the resulting nine skills to Claude through
relative links inside this checkout. Fresh clones should need neither a second
repository nor a plugin install to discover project guidance.

**Status:** Reviewable plan only. No skills, discovery paths, global settings,
plugins, credentials or environment setup have been changed. Implementation and
activation remain outside this planning assignment. Prepared for Jörn and the
coordinator of thread `01a0f18c-14b4-78e2-a7ef-91e0d7b23f0a` on
2026-09-30; source inspection completed at approximately 10:20 UTC.

## Decisions worth reviewing

1. Adopt all six upstream capabilities locally, counting the retained and adapted
   `chatgpt-pro` as one of them. Preserve `human-prose-review`, `live-task-graph`
   and `review-files`. These are project contracts, not stale copies to replace.
2. Use ordinary checked-in skill packages rather than another project plugin.
   Add nine per-skill links under `.claude/skills` and a root `CLAUDE.md` importing
   `AGENTS.md`. This adds discovery without maintaining a second instruction set.
3. Replace cross-repository destinations and executable dependencies with local
   owners. Keep host-only capabilities conditional: a fresh cloud clone can
   discover a transfer or review workflow without possessing Tailscale, host
   credentials, a host bridge or Hypothesis access.
4. Adapt upstream `codex-usage` so status inspection is separate from installation
   and session relocation. Do not bootstrap automatically on discovery or startup.
   This is an intentional semantic change, not a byte-identical port.
5. Omit upstream canary deployment machinery and plugin distribution tools.
   Validate the repository's actual discovery paths instead. Do not change or
   disable the separately installed upstream plugins as part of this migration.

An alternative is to port only `harness-engineering-v2`, `incident-log` and
`problem-structuring`, leaving `lesswrong-fetch` and `codex-usage` to the upstream
plugin. That reduces maintenance but makes the latter capabilities unavailable
in a clone without account/plugin setup. A project plugin is another option,
but adds installation, naming and refresh mechanics without a demonstrated need
for connectors or hooks here. The nine-skill local package best satisfies the
fresh-clone objective; it accepts reviewed local maintenance in return.

## Source baseline and limits

- Project Git baseline: `45ebc516c54c3a8b77a0eee512e8be6f55222b1a`.
  Inspection used the **working tree**, including the coordinator's uncommitted
  `AGENTS.md`, commentary and environment-document edits. The baseline commit
  alone does not contain all current instructions. Re-read affected files before
  integration and preserve concurrent changes.
- Live upstream `HEAD`, fetched by a depth-one clone:
  [`0f365dbc1ab7f845ee2f42f15d6ce854f1ea39aa`](https://github.com/JoernStoehler/agent-skills/tree/0f365dbc1ab7f845ee2f42f15d6ce854f1ea39aa).
  Sources are `plugins/agent-skills/skills/<name>/`; the separate
  `plugins/canary` package currently declares canary 16. This is a source
  observation, not a claim about what every client has installed.
- Read all six upstream skill bodies, their supporting scripts and interface
  metadata, the upstream README, canary skill body and distribution/check tools.
  Read all four local skill bodies and inspected their executable dependencies.
  The upstream harness body matches the installed cached body used for this
  planning assignment.
- Current local Codex reports `codex-cli 0.159.2`. A Claude executable resolves,
  but its version and fresh-session discovery were not tested. No bootstrap,
  login, canary probe, model evaluation, producer or credential access was run.
- Upstream source was staged outside the checkout in a temporary inspection
  directory. It is not an ongoing project dependency. No broad history audit
  was performed. No license declaration was found in the inspected skill
  frontmatter; do not invent an SPDX license for Jörn's own source or redistribute
  unrelated vendor packages under this proposal.

The implementation should retain the upstream repository URL, pinned commit,
imported file list and each intentional adaptation in a local provenance record,
proposed at `.agents/skills/README.md`. Upstream is a comparison source for future
reviewed edits, not a runtime dependency or automatic synchronization authority.

## Exact skill dispositions

The table names canonical destinations. Claude adapters point to these same
directories; they contain no additional skill bodies.

| Package | Port and preserve | Required adaptation |
| --- | --- | --- |
| `.agents/skills/harness-engineering-v2/` | Upstream `SKILL.md` and `agents/openai.yaml`. Preserve outcomes, authority boundaries, meaning-preservation checks, separation of revision and activation, and structural versus behavioral evidence. | Extend scope wording to project harness work for Codex and Claude. Keep Codex mechanics explicitly labeled; add the Claude discovery/import route below rather than attributing Codex behavior to Claude. Replace the required home-installed validator path with the local checker plus an optional bundled validator when available. Change interface prompt to `$harness-engineering-v2`. |
| `.agents/skills/incident-log/` | Upstream body and `agents/openai.yaml`. Preserve cheap intake, bounded evidence, unknowns, separate observation/explanation/hypothesis, no diagnosis or prompt fixes, immutable source evidence. | Set the project default to `docs/history/incidents/intake/<UTC-timestamp>-<slug>.md`; add `docs/history/incidents/README.md` explaining intake ownership. Do not write to the separate `agent-skills` checkout. Cross-project promotion, when useful, is a later explicit assignment. Use `$incident-log` in interface metadata. |
| `.agents/skills/problem-structuring/` | Upstream `SKILL.md`. Preserve structuring/solving co-evolution, coupled goals and decisions, high-bandwidth claims, cheap probes, interface-based packet handoff and optional trial report. | Use `docs/coordination/structure.md` when a structure map is actually needed, not a new root `STRUCTURE.md`. Selected assignments/resources stay in `current.json`; accepted scope stays with its existing owner. The map owns unresolved decomposition and predictions. Keep orchestration ownership and generated concise human views. Explicitly distinguish predicted marks from evidence-supported probabilities; do not turn claims elicitation into a replacement for Jörn's spending decision or a gate for already-authorized small work. |
| `.agents/skills/lesswrong-fetch/` | Upstream `SKILL.md`, including post/comment/shortform workflows, bounded comment retrieval, independent versus dependent calls and null/error handling. | Describe HTTP POST through an available HTTP client instead of assuming a tool named `web_fetch`. Resolve the body's contradiction between “never” using HTML and its network-error fallback: GraphQL first; a labeled HTML fallback only when GraphQL fails. Retrieved full text is research input, not automatic permission to reproduce it in an answer. Do not silently fetch all comments or broaden this to ordinary paper retrieval. |
| `.agents/skills/codex-usage/` | Upstream `SKILL.md`, `scripts/bootstrap.sh` and `scripts/codex-run.sh`. Preserve prompt via stdin, explicit output contract, timeout/kill-after, log and exit-status reporting, model selection preservation and credential boundaries. | Use checkout-relative script paths, not a filesystem-wide `find`. Make bootstrap default to version/login-status inspection; explicit `--install` and `--link-sessions` actions perform the documented mutations. Preserve existing real session directories and links to different destinations. Replace external interop/canary links with a short local `references/claude-codex.md` covering necessary dated observations and current verification routes. Correct the “full access” blanket claim to the requested/effective sandbox mode; retain `workspace-write` default and do not switch modes merely after failure. Preserve the observed Claude cloud login boundary, qualified to that surface rather than claiming a universal classifier rule. |
| `.agents/skills/chatgpt-pro/` | **Retain the project body and `agents/openai.yaml` as the starting point**, rather than replacing them with upstream. Preserve focused `GOAL.md` ZIP, exact successful human prompt, observed latency, target discovery, fresh receive directory, unrelated inbox arrivals and independent result review. | Incorporate upstream's host-only transfer boundary and unavailable-bridge outcome. Retain self-contained host Taildrop commands; do not require dotfiles' `share-files`. A selected handoff authorizes the transfer, subject to actual host access; migration approval does not authorize sending a dossier now. Keep `$chatgpt-pro` metadata. |
| `.agents/skills/live-task-graph/` | Retain its project storage and coordination contract, outcome/crux/hedge modeling, source/render ownership, terminal width checks, optional Herdr view and narrow review publishing. | Keep `current.json` authoritative and graph files under `docs/coordination/graphs`. Check its `review-files` caller and clarify that a cloud agent needs a reachable host owner to publish. Do not substitute an upstream generic graph or revive old assignments. |
| `.agents/skills/human-prose-review/` | Retain `SKILL.md`, `scripts/open_review.py`, `scripts/fetch_review.py`; preserve frozen artifacts, pinned viewer/hash, eight-hour view, account/document-limited annotation retrieval and raw-feedback retention. | Replace the unconditional “through Herdr” host handoff wording with an available host coordination channel. Discovery does not establish host serving capability. Preserve missing-credential/manual-export behavior; do not provision credentials, download the viewer or launch a review during migration. |
| `.agents/skills/review-files/` | Retain `SKILL.md`, interface metadata and all three scripts. Preserve explicit-file exposure, expiry, stable host index, raw-source view, sandbox mapping and publishing boundaries. | Replace required `share-files` and dotfiles setup callers with the local transfer/environment documentation below. Change the helper's missing-mapping error to the same local owner. Preserve implementation behavior and existing configured mappings. |

Do not add interface metadata merely for symmetry. Preserve existing supported
metadata and invocation policy. New copied metadata must name the local skill,
not `$agent-skills:<name>`. No skill should become a startup bootstrap or a
universal requirement to run its workflow.

## Discovery on both clients

**Canonical files:** `.agents/skills/<name>/SKILL.md`, with short discriminating
descriptions. Codex discovers repository skills along the path from the working
directory to the repository root. Same-named skills do not merge, so a global
or plugin entry is not an override mechanism. The actual selector and source
path must be checked. [Official OpenAI skill documentation](https://learn.chatgpt.com/docs/build-skills).

**Claude adapters:** Commit nine symlink entries:

```text
.claude/skills/<name> -> ../../.agents/skills/<name>
```

Use one link per skill, not a link to a host directory or another checkout.
Claude documents per-skill symlinks, project skills in `.claude/skills`, and
cloud discovery from the cloned repository. Account plugins and locally
configured plugins have different availability; the plan therefore does not
depend on plugin synchronization. Personal same-name skills can take precedence
over project skills and must be reported if encountered.
[Official Claude skill documentation](https://code.claude.com/docs/en/skills).

**Project instructions:** Add root `CLAUDE.md` containing only `@AGENTS.md`.
Current Claude can load `AGENTS.md` directly in some configurations; the explicit
import also supports sessions where that support is unavailable. It preserves
one project owner. Keep the commentary's existing conditional reading triggers;
do not import it unconditionally.
[Official Claude instruction documentation](https://code.claude.com/docs/en/memory).

Use fresh sessions for acceptance, at repository root and a nested directory.
Confirm sources in the clients, not just filesystem presence. Existing sessions
may retain old instruction context. Preserve personal/plugin installations and
report duplicate entries; resolving that wider estate is a separate decision.
Linux clones with Git symlinks enabled are the proposed acceptance target.
Windows or a cloud checkout that materializes links as text requires a tested
copy-generation adapter; do not claim those targets pass from a Linux check.

## Remove cross-repository dependencies

The proposed implementation also owns these **narrow** documentation changes:

- `.agents/skills/README.md`: local inventory, source pin, adaptation rationale,
  discovery adapters, optional dependencies and maintenance contract. Updates
  are reviewed here; no sync daemon, Git submodule, host mount, cherry-pick
  requirement or remote fetch on startup.
- `INSTALL.md`: replace the current instruction that the host estate coordinates
  skill copies between repositories. Add a compact host transfer procedure
  using installed Tailscale target discovery and a fresh receive directory,
  plus prerequisites and unavailable-host behavior. Point retained-copy callers
  there. The existing Pro workflow already supplies the essential commands.
- `INSTALL.md` and `docs/development-environments.md`: replace present-tense
  cross-repository update instructions with local ownership. Keep dated sandbox
  recovery facts explicitly historical. Do not revive the missing Herdr skill
  merely because old setup text refers to it.
- Browser-review setup text: document the helper's actual contract locally:
  `REVIEW_FILES_PUBLIC_BASE_URL` / `review-files.publicBase`, and
  `REVIEW_FILES_SANDBOX_PORT` / `review-files.sandboxPort` (default port 8765).
  The base is an already-provisioned HTTP(S) origin that forwards to the sandbox
  port. A host operator supplies it under the host's ordinary authorization;
  the skill reports a missing mapping instead of assuming dotfiles exists.
  Document the eight-hour lifetime and new-request replacement behavior.
  Port 8765 also appears in dashboard instructions: report a collision rather
  than claiming a listener can always start there.
- `.codex/config.toml`: if touched, correct only its inaccurate comment promising
  discovery of “only” project skills. Preserve the historical disable entries
  and all current agent/resource settings. They do not disable all host plugins.

External tools and services remain real prerequisites. Removing cross-repo sync
does not mean installing Tailscale in a sandbox, forwarding credentials, editing
host network policy or guaranteeing a host-agent bridge in Claude cloud.

**Omit:** `.agents/plugins/marketplace.json`, `.claude-plugin` / `.codex-plugin`
manifests, canary hooks/nonce/assumption baselines, the 96 KB Codex reference,
upstream incident records, `tools/build-zips.sh`, `tools/bump-canary.py` and the
upstream-refresh runbook. `tools/check.py` checks plugin-manifest agreement;
copying it would not validate this design. Use a project checker instead. Do not
port vendor document/browser skills or remove global skills as cleanup.

## Validation and observable completion

### Cheap checks before activation

Proposed new `scripts/check-project-skills.py` validates the nine expected
packages, required frontmatter, name/folder correspondence, interface prompts,
executable file modes, local references, and all nine relative Claude targets.
Check it with a minimal malformed fixture only where failure would otherwise
hide broken discovery. If an installed bundled skill validator is available,
run it too; its absence must not force a global installation.

Run `bash -n` on changed Bash helpers, `sh -n` on the two Codex shell scripts,
and Python syntax checks with cache output in scratch storage. Search maintained
callers for `agent-skills:`, dotfiles, external skill paths, canary references,
plugin path discovery and copy/sync instructions; classify legitimate provenance
and historical text rather than blindly deleting matches. Review a source to
candidate diff for all adaptations in the table.

Use a fake Codex executable for meaningful wrapper checks: prompt-file input
with spaces, a failing run's exit/log propagation, timeout termination and
temporary-file cleanup. Mock installation and session directories for bootstrap
checks: status must not install or move files; explicit actions must preserve
nonempty directories and unrelated symlinks. No real login or model call is
needed for these checks. Retained review servers need no full service rerun if
only caller prose/error text changes.

### Fresh-clone acceptance after selection

Create a disposable checkout of the reviewed candidate, including the current
project instructions. Do not use a clone of the old HEAD and call it a test of
the uncommitted candidate. Prefer a fresh environment with its own normal
runtime settings over changing Jörn's global configuration or borrowing auth
files. Record client versions, commit/candidate identity, selected skill paths
and results.

One bounded Codex TUI session and one bounded Claude fresh-clone session should
check listing, explicit invocation and these nearby cases:

| Input | Observable expected behavior |
| --- | --- |
| Design a small project guidance change | Reads the local harness package and preserves the supplied scope; makes no global/plugin change. |
| User requests incident intake | Writes to the project intake owner with bounded evidence and unknowns; does not start diagnosis or fetch another repo. |
| Vague project fork, then a clear already-authorized small edit | Structuring uses local coordination ownership when useful; the small edit proceeds without an invented spending gate. |
| Prepare a Pro dossier from a cloud session without a host bridge | Produces the dossier and reports the pending host transfer capability; does not install Tailscale or move credentials. No actual transfer is required for this test. |
| Claude asks to inspect Codex readiness | Runs status only; requests the needed authorization if installation/login is actually required, and does not infer it from skill discovery. |

Inspect the rendered human view or final return, not just successful skill
loading. A small live LessWrong/EA GraphQL check is optional and should fetch
bounded content only; record network/schema restrictions rather than declaring
both endpoints compatible from copied examples. No thesis producers, full
certificates, full repository tests or thesis build are warranted by this change.

Acceptance distinguishes: structural validity, actual discovery, successful
script mechanics, Jörn's selected semantics, and behavioral observations. Two
sessions provide bounded evidence, not proof that future agents will comply.

## Execution packet, effort and handoff

**Proposed implementation sequence:**

1. One implementation owner stages candidate packages and documentation diffs
   outside loaded discovery paths, proposed at
   `docs/coordination/skill-migration-candidate/`. Re-read concurrent work first.
   Do not put unsettled skills into `.agents/skills` or `.claude/skills`.
2. Present the source/candidate diffs, preservation dispositions, structural
   checks and precise activation file list to Jörn/coordinator. Optional bounded
   fresh-session tests can use an explicitly selected disposable candidate.
3. After the receiving owner has obtained the selection/activation authorization,
   integrate the reviewed packages, adapters and local callers. Commit only that
   owned change, preserving unrelated work. The coordinator updates
   `current.json` and continuation constraints; this planning session does not.
4. Record fresh-clone results and limitations. Rollback removes the new adapters
   and restores the prior project skill/caller files from the focused migration
   commit's parent, leaving unrelated edits intact. No plugin uninstall is needed.

**Planning estimates, not measured cost or authorization:** candidate ports and
caller edits roughly 45–90 minutes of agent work; script/structural verification
roughly 20–40 minutes; fresh-client checks roughly 15–30 minutes plus client or
environment startup. Jörn's semantic review should take about 10–15 minutes,
focused on the five decisions at the top. No parallel implementation swarm,
paid experiment, Astra Pro run or model comparison is proposed. Behavioral
acceptance adds the two bounded client sessions above; token usage, subscription
headroom and monetary cost are unknown until the selected runtime reports them.
If installation, authentication, unavailable cloud access or network setup turns
these checks into substantial work, return a concrete revised packet rather than
expanding this migration implicitly.

**Receiving owner:** originating coordinator, with Jörn selecting the migration
and activation scope. Review ownership does not assign implementation or cleanup
to this planner. The plan leaves no changes to thesis claims, scientific scope,
evidence producers or the thesis dashboard to reconcile.
