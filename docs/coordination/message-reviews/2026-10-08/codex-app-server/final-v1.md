Implemented the repo-local [codex-app-server skill](https://joern-pc.tailc5e761.ts.net/files/workspaces/msc-math/.agents/skills/codex-app-server/SKILL.md), with Claude discovery and a shell wrapper—no plugin required.

It supports persistent create/read/follow-up/fork/wait/interrupt/archive operations, explicit installation, and per-grant device login with the 15-minute code window.

Real workstation reconnect/follow-up checks and 13 regression tests passed. Fresh Claude-cloud setup/login remains untested; interactive approval/questions require a native Codex client. No login grant was started.

[review: /root/message_reviewer]
