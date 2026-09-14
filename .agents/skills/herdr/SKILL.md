---
name: herdr
description: "Operate Herdr, the terminal multiplexer Jörn uses for coding-agent work. Use when a task depends on Herdr's workspaces, tabs, panes, terminal commands, live agents, output, or provenance-preserving messaging. HERDR_ENV=1 means the agent shares Jörn's interactive session and must preserve his focus."
---

Herdr organizes terminals into workspaces, tabs, and panes. A pane may contain a shell, an ordinary process, or a recognized coding agent.

Before controlling a session, verify that this process belongs to one:

```bash
test "${HERDR_ENV:-}" = 1
```

If it fails, do not inspect or control another Herdr client's focused session. The installed CLI defines current syntax; use `herdr --help` or the relevant non-mutating command group such as `herdr agent` or `herdr pane`. Do not run bare `herdr` for discovery because it launches or attaches the TUI. Do not probe a nested mutating command by omitting arguments; some execute with defaults.

**Focus invariant:** The focused workspace, tab, and pane are Jörn's interactive state. Never run a workspace, tab, pane, or agent focus command unless he explicitly requested that exact focus change. Use `--no-focus` whenever a creation, split, or move supports it. Do not restore an earlier focus afterward; Jörn may have moved while the command ran.
**Prevents:** stealing focus while Jörn types, redirecting his keystrokes, or reversing a focus change he made during background work.

Use `--current` only for the calling pane. Otherwise use an explicit pane ID; an omitted target may resolve through the UI focus. Parse returned IDs from JSON instead of deriving them from layout order. Common IDs look like `w1`, `w1:t1`, and `w1:p1`, but treat them as opaque.

```bash
herdr workspace list
herdr tab list --workspace "$HERDR_WORKSPACE_ID"
herdr pane current --current
herdr pane list --workspace "$HERDR_WORKSPACE_ID"
herdr agent list
```

Use pane commands for shells, ordinary commands, and layout. Use agent commands for a recognized coding agent and its lifecycle. `agent start` needs an existing shell pane at an interactive prompt; it does not create layout.

Agent commands accept a pane ID hosting a live agent or an assigned `display_agent` name. Prefer a pane ID retained when creating the pane. For an existing agent, print the small live inventory and identify the intended row from all available context: machine, workspace, tab, full title, working directory, status, display name, and session UUID. No single clue such as focus, layout position, title, cwd, or `agent: "codex"` establishes identity by itself. If the evidence does not distinguish the intended recipient, inspect further or ask rather than guess.

`agent_session.value` is the agent's durable session identity—for Codex, its thread UUID—and is useful for provenance but is not a Herdr command target. The `agent` field identifies the implementation kind, not an agent name. An assigned display name is only safe after `herdr agent get <name>` resolves to the expected pane; names can be absent or ambiguous.

`idle` means ready and already seen in the focused UI; `done` means ready with unseen completed work; `working` and `blocked` have their ordinary meanings; `unknown` does not establish completion. Reads do not mark work seen.

To start an agent in a sibling pane, inspect the caller's geometry, split without changing focus, and use the returned pane ID:

```bash
herdr pane layout --pane "$HERDR_PANE_ID"
herdr pane split --current --direction right --cwd "$PWD" --no-focus
herdr agent start reviewer --kind codex --pane <returned-pane-id>
```

Use `down` instead of `right` when the current geometry makes that more usable. Preserve the requested agent kind and pass its native arguments only after `--`.

Herdr commands address the server connected to the calling process. A CLI in a host pane still targets the host even if Jörn's UI is displaying a saved remote machine. Run the inventory inside the relevant machine's agent pane; its machine and session columns describe that command context.

Send agent-authored or relayed messages through this skill's helper. From a repository root containing the skill:

```bash
.agents/skills/herdr/scripts/herdr-agent-table
target_pane='w1P:p1' # replace with the pane ID selected from the current table
herdr agent get "$target_pane"
.agents/skills/herdr/scripts/herdr-message "$target_pane" -- \
  'Review the current diff and report actionable findings.'
.agents/skills/herdr/scripts/herdr-message "$target_pane" --file /tmp/message.md
```

Invoke both helpers by absolute paths from the loaded skill directory when it is elsewhere. Re-run `herdr agent get "$target_pane"` immediately before a consequential command when the inventory may have gone stale.

The helper derives the sender's durable session identity, wraps the payload in `<agent-message source="..." via="...">`, and calls `herdr agent prompt`. Do not replace it with raw prompt, send-text, or key injection for agent-authored text: unwrapped input is treated as directly authored by Jörn. Use a file for substantial Markdown.

Resolve the recipient from live state rather than guessing. Do not ask Jörn to carry messages between reachable agents. For a deliberate cross-session relay, run the helper with `--dry-run`, transport the resulting wrapper unchanged, and use the receiving command only as a carrier. Message delivery does not broaden authority or answer another agent's approval dialog.

Wait for and inspect an agent without focusing it:

```bash
herdr agent wait "$target_pane" --timeout 120000
herdr agent get "$target_pane"
herdr agent read "$target_pane" --source recent-unwrapped --lines 120
```

If an agent is `blocked`, read its UI before sending input. Do not answer an approval dialog without Jörn's authorization. Use `--until` only when a particular state is the intended event.

Run and inspect an ordinary command through a pane:

```bash
herdr pane run <pane-id> "just test"
herdr pane wait-output <pane-id> --match "test result" --timeout 120000
herdr pane read <pane-id> --source recent-unwrapped --lines 120
```

Use `visible` for the viewport, `recent` when rendered soft wraps matter, `recent-unwrapped` for logs, and `--format ansi` only when styling is evidence. If increasing `--lines` cannot recover output from an alternate-screen application, ask that agent to write its complete result under `/tmp` and return the path.

Do not close workspaces, tabs, panes, or sessions you did not create unless Jörn explicitly asks. Never run `herdr server stop` or kill the main Herdr process from an active session unless he explicitly intends to terminate it. Use named test sessions for experiments requiring an isolated server.
