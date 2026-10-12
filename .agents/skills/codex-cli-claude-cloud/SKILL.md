---
name: codex-cli-claude-cloud
description: Set up and run bounded Codex CLI tasks from a Claude cloud session. Handle authorized installation and output sharing, per-grant login, then collect the final result and execution evidence. Use for that CLI workflow, not account quota tracking or reset management.
---

# Codex CLI from Claude cloud

For persistent background threads and shell equivalents of cross-session `codex_tui.*` operations, use [codex-app-server](../codex-app-server/SKILL.md). This package retains the bounded one-shot `codex exec` workflow and its historical cloud caveats.

Resolve scripts beside the loaded skill's actual filesystem path, not by taking the first match from a filesystem-wide search. In this repository, run from the checkout root:
```sh
codex_scripts="$PWD/.agents/skills/codex-cli-claude-cloud/scripts"
sh "$codex_scripts/bootstrap.sh"      # idempotent: installs codex if missing, links sessions, prints login status
sh "$codex_scripts/codex-run.sh" -t 600 -C <workdir> [-w] [-e low|medium|high] "<prompt>"   # or @promptfile
```

## 1. Setup (bootstrap.sh)
Bootstrap installs software and may publish session rollouts into the cloud output directory; it is not a read-only status check. For status alone use `command -v codex`, `codex --version`, and `codex login status`. Run bootstrap only when that setup and output-sharing scope is authorized. Existing unexpected session-path objects are preserved.
Install: official installer `curl -fsSL https://chatgpt.com/codex/install.sh | sh` (reachable from the VM, verified 2026-09-29; installs to `~/.local/bin` and edits `~/.bashrc`); falls back to `npm i -g @openai/codex` (registry is allowlisted) if that fails. Also links `~/.codex/sessions` → `/mnt/attach/outputs/codex-sessions` when writable (secrets stay local). It never logs in.

## 2. Login (only if bootstrap says "NOT logged in")
Needs a live user message naming the exact action; no workaround is worth trying before it arrives.
1. Ask the user (short, numbered) to send: **"Start `codex login --device-auth` in this session, and complete that one grant in the built-in browser"** (or "… I will approve it myself").
2. Until then do NOT run `codex login` in any form and try no workarounds (other tools, background jobs, copying `auth.json`, injecting tokens). The Claude cloud auto-mode classifier blocked an unapproved grant in the September trial and sometimes affected later actions; that is a dated surface observation. If automatic review rejects an action, report it rather than retrying through a workaround.
3. After the message: run `codex login --device-auth` in the background, capture URL + one-time code, complete the grant in the built-in browser if asked (follow the current chooser/consent/code-entry UI; the September trial used nine separate boxes and a displayed browser pane). Verify: `codex login status`, then a `codex-run.sh` "reply with exactly: ok".
4. Never print token values, copy `auth.json` between sandboxes, or reuse the approval for a later grant. Background: the separately owned [interop document](https://github.com/JoernStoehler/agent-skills/blob/main/codex-claude-interop.md) §2–§4 (workstation checkout `/workspaces/agent-skills/codex-claude-interop.md`; not a bundled runtime prerequisite).

## 3. Run a task (codex-run.sh)
- Final message → stdout; JSONL event log → `$LOG` (default `./codex-run.<pid>.jsonl`, `.err` beside it). Prompt goes via stdin (no size limit, no option injection). `-t` positive seconds (default 300; exit 124 = timeout, SIGKILL 10 s later). Long jobs: run in background, poll the log.
- `-w` enables Codex's live web-search tool. Its shell may lack DNS (observed `EAI_AGAIN` in one run; a later probe resolved hosts) — don't depend on shell network; put fetching in the web tool or do it yourself.
- Measured once: trivial prompt ≈ 6 s wall (2026-09-29).
- Why these flags (each failed without): stdin/`</dev/null` (else blocks on stdin), `timeout` (`unbounded_connection_retries` on), `--sandbox workspace-write` (default here; works in this VM as of 2026-09-29 — an earlier reference claim that nested sandboxes fail was reproduced only in Codex's isolated-CODEX_HOME probes and is not general; set `SANDBOX=danger-full-access` if a task needs it), `--skip-git-repo-check`, `-c 'web_search="live"'` (exec has no `--search`).

## 4. Prompting Codex
- Sandwich: instructions / material or file paths / output contract. Say where to write files and what the final message must contain.
- Codex has no memory of this chat; give paths. It reads files under `-C`.
- Ask for verifiable output (claims table: command + expected regex) and re-run those commands yourself. Codex's summary is testimony to assess; inspect consequential raw evidence and verify claims proportionately. Command output also requires interpretation and does not alone establish the intended result.
- Effort/model: `-e low` for log-reading/NLP, high for deep judgment. Available models depend on the account; test before relying on `-m`.

## 5. Safety
- Never read/print/copy `~/.codex/auth.json` or tokens; metadata only (`stat -c '%a'`, expect 600).
- The wrapper defaults to `workspace-write`; `SANDBOX` selects another supported policy. Choose it within the task authority and hand Codex only tasks whose files you would let a subagent touch. For open-ended tasks, skim the log for surprising commands (grepping for `auth.json` is a weak check: false positives, and it can't prove non-access).
- Inherited hooks: `hooks` feature is on; the sandbox bypass does not decide hook trust (`--dangerously-bypass-hook-trust` is separate); untrusted hook definitions are skipped.

## 6. Config, state, version skew
- Installed parser beats docs: e.g. `model_supports_reasoning_summaries`, `tools.view_image` are rejected by 0.159.0. Probe a key with a scratch `CODEX_HOME`: `timeout 20 codex --strict-config exec --ephemeral x </dev/null` (use an outer timeout; a valid config can proceed to a real run).
- State: sessions/rollouts under `$CODEX_HOME/sessions`, but history is moving to paginated SQLite (`codex migrate-rollouts`); don't treat JSONL as complete. `--ephemeral` still writes SQLite files. Resume: `codex exec resume --last` filters by cwd (`--all` disables); use the same `-C`.
- `codex --version` may print a PATH-alias read-only-FS warning (seen in one environment, not another).
- Optional drift evidence: the separately owned agent-skills canary has `scripts/verify-codex.py` and `references/codex-reference.md` under `plugins/canary/skills/canary/` (workstation checkout `/workspaces/agent-skills`). It is not bundled here. Follow its invocation policy and current support contract; report discrepancies to its owner rather than assuming authority to synchronize independent packages.

When maintaining this skill, read [design notes](references/design-notes.md) for evidence, scope and unresolved choices. These are maintenance context, not another execution-policy owner.
