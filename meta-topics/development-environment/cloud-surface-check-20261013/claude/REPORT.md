# Surface check report (draft to coordinator)

Surface: Claude (claude.ai chat, Claude Code-based harness), model Sonnet 5.5 (claude-sonnet-5-5). Date 2026-10-13. Not Codex; no `codex` binary on PATH.

## Source access
- Mode: shell + Git over HTTPS (actual checkout, not connector context). `git clone --branch codex/cloud-support-20261013-1532 https://github.com/JoernStoehler/msc-math.git` succeeded (6836 files).
- Expected revision: 2c88aaa58b63acbafc0a81c7046508051f607875
- Observed `git rev-parse HEAD`: 2c88aaa58b63acbafc0a81c7046508051f607875 (match). Branch observed: codex/cloud-support-20261013-1532.
- Read: runbook (full), AGENTS.md (first ~3.5 kB of 11.9 kB), cloud-workflow.md (first ~3 kB of 8.1 kB), CLAUDE.md (just `@AGENTS.md`). README.md was not read (only size checked). Not read: remainder of AGENTS.md and cloud-workflow.md, agent-platform-reference skill.
- Elapsed: 28 s measured from start to the PATH probe (clone included); report writing and zip time not measured.
- Discrepancy: the harness environment note labelled the primary working directory a non-git directory, while the clone in it is a valid git checkout.

## Capability table
| Capability | Status |
|---|---|
| Clone branch, read files, `git rev-parse` | exposed and used successfully (Bash, Read) |
| Write report files | exposed and used successfully (Bash heredoc; Write tool also exposed, untested) |
| Zip creation | exposed and used successfully (`zip` on PATH) |
| Return file to human (SendUserFile) | exposed; used for the return of this ZIP, see RECEIPT.json `return_attempt` for outcome |
| Chat-text return | exposed, used as fallback description |
| Other tools below | exposed but untested |

## Harness tools (names as exposed; not shell programs)
Direct (schemas loaded): Agent (spawn subagents; not used), AskUserQuestion, Bash, Edit, Glob, Grep, Read, SendUserFile (deliver file to person's chat), SendUserMessage, Skill, TaskCreate, TaskUpdate, WebFetch, WebSearch, Write, Artifact (publish HTML pages on claude.ai; not used), SearchMcpRegistry, SearchPlugins, SuggestConnectors, SuggestPluginInstall, SuggestSkills, ToolSearch, enable__mcp__claude-in-chrome, enable__mcp__remote-devices__Claude_Browser, enable__mcp__remote-devices__computer (turn on browser/computer-use bridges), mcp__claude-code-remote__{add_repo, create_trigger, send_later} (attach GitHub repo to session; create scheduled tasks; schedule message into this session), mcp__claude_ai__{conversation_search, current_time, end_conversation, image_search, launch_extended_search_task, read_conversation, recent_chats}, mcp__memory__memory_{append,delete,list,read,str_replace,write} (Claude's persistent per-user memory store; not used for this task), mcp__visualize__{read_me, show_widget}, mcp__widgets__* (inline chart/card/map/weather/etc. renderers), propose_skills.
Deferred (names only, schemas not expanded; ToolSearch not run): ArtifactComments, ArtifactData, Cron*, DesignSync, EnterPlanMode/ExitPlanMode, worktree tools, ListAgents/ListConnectors/ListPlugins/ListSkills, Monitor, NotebookEdit, SendMessage, Task*, mcp__Claude_Docs__*, mcp__Google_{Calendar,Docs,Drive,Sheets,Slides}__*, mcp__claude-code-remote__{delete,fire,list,update}_trigger(s)/list_repos, mcp__remote-devices__device_request_folder_access, and others listed by the harness. Status: exposed but untested.
Source correspondence to Codex source: unknown; names are Claude Code / claude.ai harness names.
Discovery scope: tool list as presented to me in this session plus the harness's deferred-tool list; no ToolSearch expansion, no per-tool probing.

## Shell programs on PATH (probed subset, not a full PATH listing)
Present: git, python3, zip, unzip, tar, gh, curl, rg, jq, node 22/npm, cargo, rustc, claude, gcloud, bq, gsutil. Absent: codex. Not tested for working auth/network beyond git clone over HTTPS (worked through the preconfigured proxy).

## Failures
No actual errors occurred in the probes. No invalid-argument failure was created.

## Remaining unknowns
- Whether SendUserFile delivers the ZIP to the human as a downloadable file (outcome recorded after the call, in the chat message).
- Whether Rust build, PDF build or any thesis tooling works here (not tested, out of scope).
- Full AGENTS.md / cloud-workflow.md content not read; no check whether this surface auto-loads AGENTS.md (CLAUDE.md imports it via `@AGENTS.md`, but I read it manually).
- No independent review; review is pending with the coordinator. The prompt's `[review: /root/research_routes_review]` path was not accessed and no reviewer approval exists.

## Maintained correction suggested by findings
None required by the probes. Optional: the runbook's Codex-oriented wording ("ordinary Codex CLI tool interface") does not map cleanly to this Claude harness; a note that non-Codex surfaces should report their own namespaces would help. This is a suggestion, not a verified need.
