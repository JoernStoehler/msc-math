# Surface check report (draft to coordinator)

Surface: Claude (claude.ai chat, Claude Code-based harness), model Sonnet 5.5 (claude-sonnet-5-5). Date 2026-10-13. Not Codex; no `codex` binary on PATH.

## Source access
- Mode: shell + Git over HTTPS (actual checkout, not connector context). `git clone --branch codex/cloud-support-20261013-1532 https://github.com/JoernStoehler/msc-math.git` succeeded (6836 files).
- Expected revision: 2c88aaa58b63acbafc0a81c7046508051f607875
- Observed `git rev-parse HEAD`: 2c88aaa58b63acbafc0a81c7046508051f607875 (match). Branch observed: codex/cloud-support-20261013-1532.
- Reading coverage (corrected, third pass): the four required files were each read from first to last line with the Read tool in this follow-up, at the verified HEAD above: README.md (45 lines, 3010 B), AGENTS.md (188 lines, 11860 B), cloud-workflow.md (136 lines, 8143 B), cloud-surface-runbook.md (67 lines, 3767 B). Sizes/line counts came from `wc -lc` beforehand; each Read result ended at the last expected line number, so no truncation occurred and no chunk retrieval was needed. CLAUDE.md is just `@AGENTS.md`. History: the first return read only ~3.5 kB of AGENTS.md and ~3 kB of cloud-workflow.md and no README.md, by my truncation choice (`head -c`), not by any access limit; the second pass read the remainders by byte offset (`tail -c +N`) and was not a single full read. Not read (runbook says only when needed): the agent-platform-reference skill and notes, `.agents/skills/message-review/SKILL.md`, `AGENTS.md.commentary.md`.
- Revision identity: HEAD re-verified in the existing checkout as 2c88aaa58b63acbafc0a81c7046508051f607875 (clean `git status`). The temporary remote branch was reported deleted by the coordinator and was not re-cloned, so remote retrievability of this revision is no longer verifiable from this surface.
- Elapsed (first pass): 28 s measured from start to the PATH probe (clone included); report writing and zip time not measured.
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
- Automatic loading of AGENTS.md on this surface not demonstrated (I read it manually; cloud-workflow.md itself says CLAUDE.md imports are delivery mechanisms, not evidence of auto-loading in Claude chat).
- No independent review; review is pending with the coordinator. The prompt's `[review: /root/research_routes_review]` path was not accessed and no reviewer approval exists.

## Maintained correction suggested by findings
None required by the probes. Optional: the runbook's Codex-oriented wording ("ordinary Codex CLI tool interface") does not map cleanly to this Claude harness; a note that non-Codex surfaces should report their own namespaces would help. This is a suggestion, not a verified need.

## Consequential material from the previously omitted reading (corrections)
Previous inventory, probe results and failure record are unchanged. What the omitted text adds:
1. Message review (AGENTS.md lines 105-112; cloud-workflow.md lines 125-129): prepared user-visible messages need a separate subagent's approval of the exact text with a `[review: REVIEWER]` marker; direct synchronous conversation is exempt; if review is unavailable, delivery stays pending. Effect: this ZIP and the chat text are drafts for the coordinator, who owns review (runbook lines 63-65). I performed no review and have no approval; the `[review: ...]` marker in the coordinator's prompt is theirs, not mine. Whether my earlier direct replies to Jörn fell inside the exemption was judged by me from the AGENTS.md wording; the message-review skill that owns the boundary was not read, so that judgment is unverified. Spawning a reviewer subagent was not done (runbook line 11 selects no additional model session).
2. Async question tool (AGENTS.md lines 142-149; cloud-workflow.md lines 97-98): disabled by project instruction; questions go directly in chat. Effect: AskUserQuestion is exposed on this surface but I did not use it and project policy says not to; my one question to Jörn was asked in chat. Capability status for AskUserQuestion: exposed, untested, project-disabled. No change to earlier actions.
3. Instruction loading (runbook lines 17-18; cloud-workflow.md lines 52-58): read AGENTS.md explicitly unless auto-loading is demonstrated; `.claude/skills` links and the CLAUDE.md import are delivery mechanisms, not evidence of auto-loading in Claude chat. Effect: confirms the earlier "auto-loading not demonstrated" unknown; I read the files manually.
4. Return and assessment (cloud-workflow.md lines 115-130): return a downloadable artifact or another demonstrated route; a `/tmp` path alone is not a deliverable; the coordinator verifies receipt; a setup check does not establish workflow efficiency. Effect: my route was SendUserFile, which reported delivery of a file card; actual human receipt is unobserved and left to the coordinator to verify.
5. Ordinary chat without /goal and no further sessions (cloud-workflow.md lines 101-102; runbook lines 11-12): complied; no /goal, no subagent, no install, no publication, no account change.
6. Budget (AGENTS.md lines 134-140): no allowance invented; none spent beyond the probes.
7. Workstation-only items not applicable here: Tailscale file-link rule (AGENTS.md line 44), workstation paths and `scripts/bootstrap-cloud.sh` (Codex image adapter, cloud-workflow.md lines 78-84; not run).
Net effect on the earlier report: no tool-inventory or probe result changes; the reading-coverage statement and the review/question-tool notes above are the corrections. The first return's incomplete reading remains a real deviation from runbook §1, now remedied.
