User request, verbatim:
request: claude.ai cloud microVMs would like to use codex. i suggest a repo-local skill, based on "codex app-server daemon" usage (install: |sh one-liner, login: device code (needs jörn to open a url + type in a code within 15min), probably with a script/wrapper that is better than codex exec (i.e. replaces the standard cross-session codex_tui.* tools basically, to be run by anyone, including by agents who don't have the codex_tui.* tools)

Followups, verbatim:
fyi: openai github has a plugin for claude code - but obv we want a skill not a weird plugin
(and it's not actually targeted at agent/cli users)

Scope: implement requested repository-local skill and shell wrapper; no login grant authorized or begun. Evidence is retained in .agents/skills/codex-app-server/references/design-notes.md. Separate exact-text native reviewer /root/message_reviewer had no inherited conversation, received verbatim user text, scoped evidence and the complete candidate. No reviewer tools were used. Gate owner: .agents/skills/message-review/SKILL.md. Sender owns integration.

Final-v1 evidence supplied: canonical skill and Claude discovery link exist; 0.161.0 host create/disconnect/reconnect/follow-up/full paginated reads succeeded; fork/title/archive/restore succeeded and smoke threads archived. Thirteen offline tests, shell syntax, skill YAML, coordination state/served links and graph checks pass. Fresh-cloud install/login/lifetime untested, no new grant or daemon restart. Dashboard separately reports an existing scientific baseline source discrepancy in docs/project-facts.md; this work did not modify thesis sources. Native-client requirement for pending interactive requests is explicit.
