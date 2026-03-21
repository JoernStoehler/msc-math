# Backup Investigation Report — 2026-03-18

During host migration, investigated whether `/srv/devhome/.claude/` (the devcontainer's Claude Code data directory) was ever backed up.

## Critical finding

The live data at `/srv/devhome/.claude/` and both archive copies created this session are on the same physical disk (`/dev/sdb1`, per `df`). A disk failure would lose all three. No off-disk backup of this data was found in the sources checked (see item 5 below).

## Was `/srv/devhome/.claude/` ever backed up?

No evidence of a backup was found in the sources checked. Sources not checked are listed in item 5.

This session created what appears to be the first archive of this data (see "What was done this session" below, referred to as "the devcontainer archive" in this section).

1. **5 `.tar.zst` archives in `/home/joern/backups/claude-sessions/` predate the devcontainer archive.** Ran `tar --zstd -tf $file | grep -c 'projects/-workspaces-'` on each. All returned 0 (i.e. none contain paths with the `-workspaces-` prefix that devcontainer sessions use). Also ran `tar --zstd -tf $file | grep 'projects/' | head -20` on each and observed only `-home-joern-*` paths. (The path encoding convention is that Claude Code names project directories after the absolute path with `/` replaced by `-`. Devcontainer sessions used paths under `/workspaces/`, producing `-workspaces-*` prefixes. Host sessions used `/home/joern/workspaces/`, producing `-home-joern-workspaces-*` prefixes.) The 5th archive, `claude-sessions-20260318-101723.tar.zst`, was created earlier this session before the devcontainer archive.

2. **Host bash history (`~/.bash_history`):** Searched for `backup`, `tar`, `zip`, `zstd`, `/srv/devhome/.claude`. Found `cd /srv/devhome/.claude` followed by `./backup-sessions`. Bash history records commands entered, not necessarily commands that ran successfully or at all. The current version of that script uses `$HOME/.claude/` as source. On the host (where this history entry was recorded), `$HOME=/home/joern` (verified via `echo $HOME` during this session; whether it was the same when the history entry was recorded is assumed but not verified). Bash history may be incomplete due to `HISTSIZE`/`HISTFILESIZE` limits and non-interactive shells not writing history.

3. **Container bash history (`/srv/devhome/.bash_history_dir/.bash_history`):** Same search terms. No matching commands found. Same caveats about history completeness and what history entries prove. Note: if the script were run inside the container, the devcontainer configuration (`.devcontainer/devcontainer.json`, line 40) bind-mounts `/srv/devhome/.claude` to `/home/vscode/.claude`, so `$HOME/.claude/` would resolve to the correct data. However, the archive destination (`$HOME/backups/claude-sessions/`) would resolve to `/home/vscode/backups/claude-sessions/`, which is not a bind mount — so archives would be lost if the container was rebuilt. Whether the container was ever rebuilt after a hypothetical in-container invocation is unknown.

4. **Recovered versions of the backup script** from `/home/joern/.claude/file-history/`: 2 file hashes exist — `4209cb8c3dd1b60a` (= `/home/joern/.claude/backup-sessions`, 2 versions under session `f646c679`) and `34c228c280185369` (= `/srv/devhome/.claude/backup-sessions`, 2 versions under session `f646c679`, 8 versions under session `4f7b13a1`). Total: 12 file-history entries. Grepped each for `tar`, `SRC`, `HOME`, `srv`. The `4209cb8c3dd1b60a` versions use `tar -cf - -C "$SRC" projects/ history.jsonl` where `SRC="$HOME/.claude"`. The `34c228c280185369` versions use `tar -cf - -C "$HOME" .claude/`. Neither references `/srv/devhome`. Note: file-history versions start at `@v2`; no `@v0` or `@v1` files exist on disk for either hash. Whether this is Claude Code's numbering convention or evidence of a missing earlier version is unknown.

5. **Not checked:** Unmounted drives (`/dev/sda1`, `/dev/sdc3`, `/dev/sdc4`), cloud storage, other machines, archive formats not searched for (searched `.tar.zst`, `.tar.gz`, `.tar.bz2`, `.tar.xz`, `.tgz`, `.zip`, `.7z`).

## Was any session data lost from `/srv/devhome/.claude/`?

No evidence of loss found, and no way to detect it. A deleted session would leave no trace — both the JSONL file and any index entry would be gone. There is no baseline to compare against.

## How the script ended up with the wrong source path

The Feb 14 agent (session `f646c679`) chose `$HOME/.claude/` as the source path when writing the script. Based on the JSONL transcript, Jörn asked for backup scripts "in each .claude/ folder" and the agent decided how to implement them. The agent wrote both `/home/joern/.claude/backup-sessions` and `/srv/devhome/.claude/backup-sessions` with the same `$HOME/.claude/` source — meaning the script at `/srv/devhome/.claude/` does not back up its own directory.

On Mar 2 (session `4f7b13a1`, Sonnet), Jörn discovered the `KEEP=5` rotation bug and asked the agent to fix and scrutinize the script. The agent fixed the rotation but did not notice the source path issue. Based on a subagent's reading of the JSONL transcript (final exchange corroborated by direct grep): the agent reviewed the script 6 times (lines 111, 125, 128, 171, 176, 213), giving all-clears each time. Final exchange: Jörn asked "Anythign else?", agent replied "Nothing. Looks good."

**Search for other sessions discussing the backup target:** Searched all JSONL transcripts in `/home/joern/.claude/projects/` and `/srv/devhome/.claude/projects/` for `backup-sessions`, `backup.*script`, `backup.*folder`, `right.*backup`, `correct.*backup`, `srv.devhome`, `which path`, `wrong directory`, `wrong folder`. No sessions other than `f646c679` and `4f7b13a1` contain conversations about the backup script.

Source: JSONL transcripts at `/home/joern/.claude/projects/-srv-devhome--claude/f646c679-c28a-4141-b553-a0996d1a83dd.jsonl` and `/home/joern/.claude/projects/-srv-devhome--claude/4f7b13a1-a53a-4573-90d8-2a924d71499c.jsonl`.

## What was done this session

Archived `/srv/devhome/.claude/` (2.6G on disk per `du -sh`) to:
- `/home/joern/backups/claude-sessions/devcontainer-claude-sessions-20260318-102138.tar.zst` (511M, per `du -sh`)
- `/srv/devhome/backups/claude-sessions/claude-sessions-20260318-102138.tar.zst` (511M, verified identical via `md5sum`: both `a3064d2d504d92e3a6bebd4e4ea274c3`)

See "Critical finding" above regarding disk redundancy.
