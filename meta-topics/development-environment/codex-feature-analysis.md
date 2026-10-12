# Codex feature flags — grouped table

Maintained reference for using Codex in this project. This is a dated snapshot; refresh it when the CLI version, relevant configuration, or implementation changes. [Sources and maintenance notes](#sources-and-maintenance-notes) follow the table.

**CLI 0.160.0 · Linux · settings observed 2 October 2026 · all 154 printed flags.** State is the resolved flag setting, not proof that the current client/model exposes the capability. Links are pinned to the matching upstream source. `S` = stable, `E` = experimental, `D` = under development, `L` = deprecated. Retired flags are collected at the end. Two settings differ from Linux registry defaults: `analytics_plan_history` is on and `shell_snapshot` is off.

## Agent collaboration and user interaction

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`multi_agent`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L206) | On | S | V1 fallback tool family in namespace `multi_agent_v1`: `spawn_agent`, `send_input`, `wait_agent`, `resume_agent`, `close_agent`, addressing thread IDs. Model/session version selection can supersede the fallback. [Tools](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/handlers/multi_agents_spec.rs#L80) |
| [`multi_agent_v2`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L208) | Off | S | V2 tool family: `spawn_agent`, `send_message`, `followup_task`, `interrupt_agent`, `list_agents`, optional `wait_agent`; namespace configurable. Named task paths and `fork_turns` history selection. Messages leave idle agents idle; follow-up tasks start a turn. True explicitly selects V2; false permits model/session selection, including historical V1 compatibility. [Selection](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/config/mod.rs#L1586); [History](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/session/mod.rs#L484) |
| [`agent_message_board`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L215) | Off | D | Adds discussion-board tools shared by an agent tree. This is separate from ordinary V2 direct messaging. |
| [`defer_mailbox_preemption`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L211) | Off | D | Keeps generation running through reasoning/commentary boundaries when agent mail arrives; delivers the mail at the next normal input boundary. |
| [`default_mode_request_user_input`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L321) | Off | D | Allows the request_user_input tool in Default collaboration mode. |
| [`send_message_to_user_async`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L325) | Off | D | Allows root agents to send asynchronous user messages without requiring model-catalog support. |
| [`instant_interrupt`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L213) | Off | D | Preempts responses and yields foreground code-mode observations when new user input arrives. |

## Goals, memories and conversation history

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`goals`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L346) | On | S | Model tools: `get_goal`, `create_goal`, `update_goal`. TUI `/goal` manages persisted goals, including pause/resume/clear; active goals can continue automatically when the thread goes idle. [Tools](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/ext/goal/src/spec.rs#L9); [TUI](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/tui/src/chatwidget/slash_dispatch.rs#L910) |
| [`memories`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L181) | Off | S | Enables the local memory pipeline and model read context/tools. TUI `/memories` controls using memories and generating new ones. Startup extraction is skipped for ephemeral/subagent sessions; separate from ChatGPT web memory. [Pipeline](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/memories/write/src/start.rs#L20); [Settings](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/tui/src/chatwidget.rs#L1085) |
| [`external_agent_memory_import`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L183) | Off | D | Imports project-scoped memory from external agents. |
| [`chronicle`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L192) | Off | D | Enables a Chronicle sidecar for passive screen-context memory. This is distinct from conversation-derived memories. |
| [`local_thread_store_compression`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L186) | Off | D | Compresses cold local conversation files, including shared histories; every reader must support that format. |
| [`background_paginated_rollout_migration`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L190) | Off | D | Migrates older conversation rollout files to paginated history in the background. |

## Command execution, editing and workspaces

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`shell_tool`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L102) | On | S | Enables the shell tool for agent-run commands. Does not grant filesystem/network permissions. |
| [`unified_exec`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L133) | On | S | Agent command runner with retained processes and subsequent input. In this version ordinary user opt-outs are normalized to on; managed requirements can disable it. [Normalization](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/config/managed_features.rs#L150). |
| [`unified_exec_tty`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L135) | On | S | Lets the agent allocate an interactive terminal for a command; useful for programs needing terminal input/output. |
| [`shell_snapshot`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L169) | Off | S | Captures and reuses shell initialization/environment state to reduce repeated startup work; currently disabled in your configuration. |
| [`shell_snapshot_v2`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L173) | Off | D | Keeps policy-filtered shell snapshots entirely in executor memory. |
| [`shell_zsh_fork`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L137) | Off | D | Routes shell-tool execution through the zsh execution bridge. |
| [`powershell_shell_version`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L171) | Off | D | Exposes the selected PowerShell host's bounded major/minor version. |
| [`code_mode`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L121) | Off | D | Lets the agent execute JavaScript that calls and combines tools through code-mode `exec`/`wait`; host operation has a separate flag. |
| [`code_mode_host`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L125) | On | S | Executes code-mode JavaScript in a separate host process. This does not itself select code mode for a session. |
| [`code_mode_prewarm`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L127) | Off | D | Connects to the code-mode host at session startup instead of waiting until later. |
| [`code_mode_interrupt`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L129) | Off | D | Terminates active code-mode cells when their turn is interrupted. |
| [`code_mode_only`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L131) | Off | D | Restricts the tools visible to the model to code-mode exec and wait entrypoints. Dependency normalization also enables code_mode. |
| [`deferred_executor`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L175) | Off | D | Allows turns to begin while selected execution environments are still starting. |
| [`sleep_tool`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L106) | On | S | Allows the timed-wait tool. In model-driven mode, registration also depends on clock-capable model metadata or compatible current-time-reminder settings. [Implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/spec_plan.rs#L1225) |
| [`apply_patch_preserve_line_endings`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L151) | Off | D | Keeps existing line-ending style when `apply_patch` updates files. Patch-tool availability itself depends on a usable environment and model metadata, not this flag. [Implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/spec_plan.rs#L1269) |
| [`apply_patch_streaming_events`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L149) | Off | D | Emits structured progress while `apply_patch` input is generated; does not itself enable the patch tool. |
| [`worktrees`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L200) | On | S | Allows managed worktree creation and repository-aware sessions, giving work a separate Git checkout when selected. |
| [`workspace_dependencies`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L386) | On | S | Allows workspace-dependency runtime integration. The flag does not itself specify a package list or promise an install command/UX. |

## Created outputs and image handling

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`artifact`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L368) | Off | D | No native-artifact tools or TUI output interface are gated by this flag in the published CLI source; its declared capability is [omitted from the config schema](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/config/src/schema.rs#L50). Ordinary document/PPTX/XLSX skills and [TUI file links](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/tui/src/markdown_render/inline_directives.rs#L218) operate independently. Concrete private-desktop artifact tools/UX are not established by this source. |
| [`image_generation`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L299) | On | S | Agent image-generation/editing tool backed by an extension; returns generated images. Availability also requires the provider/model/account checks. |
| [`view_image`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L104) | On | S | Agent image-inspection tool for local files. Lets the model inspect pixels; does not add a browser pane to the TUI. |
| [`image_resize_notice`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L303) | Off | D | Tells the model when an input image was resized and supplies the dimensions. |
| [`unified_image_budget`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L305) | Off | D | Applies a shared pixel/token budget to all images regardless of legacy detail hints. |

## Skills, plugins, connectors and hosted tools

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`apps`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L221) | On | S | Enables app/connector integration. The registry additionally requires ChatGPT authentication for its apps-enabled check. |
| [`plugins`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L249) | On | S | Enables plugin support, including bundles of skills and tool integrations. |
| [`remote_plugin`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L293) | On | S | Enables the service-backed remote plugin catalog. |
| [`plugin_sharing`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L295) | On | S | Enables remote plugin-sharing flows, subject to the supported client and workspace. |
| [`recommended_plugins`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L247) | Off | S | Includes recommended plugins in the context visible to the model. Recommendations can also be enabled by tool_suggest when apps and plugins are on. |
| [`tool_suggest`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L245) | On | S | Adds `list_available_plugins_to_install` and `request_plugin_install` when there are eligible candidates and apps+plugins are enabled. Connector discovery also requires ChatGPT auth. Can activate plugin-recommendation gating; does not itself install plugins. [Tools](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/spec_plan.rs#L1253) |
| [`mentions_v2`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L319) | On | S | Enables the unified mention-selection popup in the terminal UI. |
| [`skill_search`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L315) | On | S | Runs cheap skill-search implementations in shadow mode and emits experiment metrics; it does not switch basic skill discovery on/off. |
| [`skill_mcp_dependency_install`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L313) | On | S | Allows prompting about and installing missing MCP dependencies required by skills. |
| [`hooks`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L108) | On | S | Runs configured lifecycle handlers at session, turn, and tool events. Users manage/review handler trust through `/hooks`; the flag does not configure handlers. |
| [`executor_capability_discovery`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L251) | Off | D | Discovers plugin and skill manifests for selected roots through one high-level exec-server request. |
| [`skip_host_skill_discovery`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L253) | Off | D | Skips host skill snapshots when no registered contributor requires them. |
| [`auth_elicitation`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L362) | On | S | For eligible Codex Apps connector-auth failures, turns the failed MCP tool result into a URL sign-in prompt. Requires an approval policy permitting MCP elicitations; `never` leaves it unchanged. [Implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/mcp_tool_call.rs#L771) |
| [`enable_mcp_apps`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L225) | Off | D | Enables MCP Apps support. This is separate from the stable apps connector flag. |
| [`mcp_2026_07_28`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L227) | Off | D | Enables support for MCP protocol version 2026-07-28. |
| [`codex_apps_mcp_2026_07_28`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L229) | Off | D | Enables MCP protocol version 2026-07-28 for the host-owned Codex Apps server. |
| [`mcp_oauth_refresh_coordination`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L231) | Off | D | Lets the Rust MCP client coordinate OAuth refresh through the configured credential store. |
| [`use_xaa`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L233) | Off | D | Enables enterprise refresh-token authorization for configured MCP resources. |
| [`non_prefixed_mcp_tool_names`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L243) | Off | D | Exposes model-visible MCP namespaces without the older mcp__ prefix. |
| [`deferred_tool_world_state`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L241) | Off | D | Describes deferred tool namespaces in the context visible to the model. |
| [`standalone_web_search`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L164) | Off | D | Exposes an extension-backed standalone web-search tool; separate from the deprecated legacy search flags. |

## Permissions, sandboxes and approval review

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`guardian_approval`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L327) | On | S | Enables eligible automatic approval reviews when the configured reviewer/model supports them. Guardian does not itself bypass the sandbox; approved permission/escalation requests can expand effective permissions. `approval_policy=never` supplies no interactive review request. [Eligibility](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/session/step_settings.rs#L324); [Permission requests](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/tools/approvals.rs#L413) |
| [`guardianv2`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L342) | Off | D | Enables Guardian V2’s asynchronous review path; approval review must also be enabled. [Implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/guardian/decision.rs#L111) |
| [`guardian_reuse_parent_compaction`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L332) | On | S | Reuses encrypted parent compaction when restarting review sessions; otherwise the reviewer retains an independent transcript across parent compaction. |
| [`guardian_root_handoff_context`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L334) | Off | D | Restricts a worker's Guardian root evidence to preceding root communication windows. |
| [`guardian_enhanced_node_repl_transcripts`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L336) | Off | D | Includes completed node_repl/cua_repl code-mode response content in approval reviews. |
| [`guardian_node_repl_transcript_images`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L338) | Off | D | Includes images from completed node_repl/cua_repl responses in approval reviews. |
| [`guardian_conversation_history_tools`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L340) | Off | D | Gives Guardian access to tools for reading the root conversation's message history. |
| [`exec_permission_approvals`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L153) | Off | D | Allows execution tools to request additional permissions while remaining sandboxed. |
| [`write_stdin_approval`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L155) | On | S | Reviews nonempty terminal input when current execution permissions require review, with strict-auto-review exceptions. Empty input and non-TTY Ctrl-C skip review. [Implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/unified_exec/stdin_approval.rs#L186) |
| [`request_permissions_tool`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L157) | Off | D | Exposes the built-in request_permissions tool. |
| [`prefer_mxc`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L410) | Off | D | Selects the local native Windows sandbox when supported by host and network configuration, retaining legacy fallback. No corresponding Linux sandbox UX. |
| [`windows_sandbox_service`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L408) | Off | D | Attempts elevated Windows sandbox provisioning through the installed service. |

## Desktop app and browser policy gates

These are requirements-policy gates exposed to compatible clients. The public Rust source does not establish the private desktop implementation or availability in a specific build. **They do not add browser/GUI panes to the TUI.**

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`in_app_browser`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L259) | On | S | Requirements policy permitting the desktop app's built-in browser pane. Requirements-only policy gate; the Codex TUI has no built-in browser pane. |
| [`in_app_chat`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L263) | On | S | Requirements policy permitting the chat pane in desktop apps. Requirements-only policy gate; not a terminal chat-mode selector. |
| [`in_app_dictation`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L267) | On | S | Requirements policy permitting desktop-app dictation. Requirements-only policy gate. |
| [`in_app_local_automation`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L271) | On | S | Requirements policy permitting desktop apps to run local automations. Requirements-only policy gate. |
| [`in_app_updates`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L275) | On | S | Requirements policy permitting updates through desktop apps. Requirements-only policy gate. |
| [`browser_use`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L279) | On | S | Requirements policy permitting desktop Browser Use agent integration. Source marks this as a requirements-only policy gate; no terminal browser pane. |
| [`browser_use_external`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L287) | On | S | Requirements policy permitting the desktop Browser Use integration to operate external browsers. |
| [`browser_use_full_cdp_access`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L283) | On | S | Requirements policy permitting Browser Use to access the full Chrome DevTools Protocol surface. |
| [`computer_use`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L291) | On | S | Requirements policy permitting Computer Use. Source marks it as a requirements-only gate; this does not add a GUI to the TUI. |

## Terminal UX, model selection and host behavior

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`realtime_conversation`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L374) | On | S | Enables voice conversations in the TUI; the flag does not alone establish runtime/account availability. |
| [`bedrock_setup_wizard`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L364) | Off | D | Adds Amazon Bedrock setup to terminal sign-in onboarding. |
| [`analytics_plan_history`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L95) | On | E | Adds consumer five-hour and weekly allowance history to the TUI `/analytics` view. |
| [`fast_mode`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L370) | On | S | Enables model-supported Fast/service-tier slash commands and a configurable TUI toggle keybinding. Actual request tier depends on current model support and the selected tier; on is not a report of Fast usage. [Implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/tui/src/chatwidget/service_tiers.rs#L36) |
| [`step_model_switching`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L372) | Off | D | Allows explicit per-turn settings updates—including model, reasoning effort, summary and service tier—to apply to later execution steps. [Implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/session/step_activation.rs#L234) |
| [`prevent_idle_sleep`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L376) | Off | E | Keeps the host computer awake while a turn is actively running. |
| [`daemon_auto_start`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L113) | On | S | Automatically starts the shared local daemon for eligible interactive launches. |
| [`cwd_relative_turn_diffs`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L177) | Off | D | Uses the current working directory as the base for paths displayed in turn diffs. |
| [`terminal_visualization_instructions`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L147) | Off | D | Adds terminal-specific visualization guidance to the developer instructions supplied to the model. |

## Authentication and network transport

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`api_key_model_discovery`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L97) | Off | D | Discovers model catalogs when authenticating with an OpenAI API key. |
| [`secret_auth_storage`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L110) | Off | S | Selects the encrypted local secrets backend for CLI auth when keyring storage is configured; otherwise the direct keyring backend is used. Not a blanket encryption switch for all Codex data. |
| [`use_agent_identity`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L384) | Off | D | Uses Agent Identity for ChatGPT-authenticated sessions. |
| [`network_proxy`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L198) | Off | E | Starts restrictions on sandboxed command-network traffic. Does not filter hosted web search, app/connector traffic, or MCP traffic. |
| [`respect_system_proxy`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L202) | Off | D | Uses host system proxy settings for Codex-owned network clients. |
| [`system_proxy_fallback`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L204) | On | S | When `respect_system_proxy` is off, retries eligible bootstrap GET requests through the system proxy after a connection/timeout failure. [Implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/config/mod.rs#L3071) |
| [`psp`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L223) | Off | D | Uses the internal PSP route for first-party ChatGPT backend requests. Changes request routing; no agent tool or user-facing mode is declared. |
| [`enable_request_compression`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L194) | On | S | Compresses streaming request bodies with zstd when sending them to the Codex backend. |
| [`unbounded_connection_retries`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L196) | On | S | Keeps eligible non-internal, non-Bedrock generation turns alive after connection failures, with backoff and a visible reconnect notice, until recovery or interruption. [Implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/core/src/responses_retry.rs#L92) |

## Model context, protocol metadata and diagnostics

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`compaction_image_budget`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L380) | On | S | Counts retained images in the context budget used for remote conversation compaction. |
| [`retain_client_developer_messages`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L382) | Off | D | Keeps client-authored developer messages across compacted context windows. |
| [`context_management`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L350) | Off | D | Enables experimental context management. This is an unfinished internal feature, not an established user-facing workflow. |
| [`token_budget`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L348) | Off | D | Adds current context-window metadata to model-visible context; the declaration does not describe a spending cap. |
| [`rollout_budget`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L352) | Off | D | Tracks and reports a token budget shared across the session's agent threads; distinct from context-window metadata and goal budgets. |
| [`reasoning_effort_override`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L354) | Off | D | Adds trusted response-configuration items when the selected reasoning effort changes. |
| [`current_time_reminder`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L356) | Off | D | Adds current-time reminders to the context visible to the model. |
| [`nonfatal_clock_read_errors`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L358) | Off | D | Reports clock-read failures to the model without failing the turn. |
| [`concurrent_reasoning_summaries`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L311) | Off | D | Protocol behavior: the declaration requests sequential cutoff-summary delivery. No separate agent tool or UI control is declared. |
| [`content_item_kinds`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L117) | On | S | Adds per-content-entry classifications to internal Responses metadata. |
| [`executed_tool_call_metadata`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L119) | Off | D | Records model-attempted tool calls in internal Responses metadata. |
| [`omit_app_server_notification_media`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L301) | Off | D | Omits inline image/audio data from app-server item notifications. |
| [`runtime_metrics`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L179) | Off | D | Enables runtime-metric snapshots through a manual reader. |
| [`tool_call_mcp_elicitation`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L360) | On | S | Routes MCP tool-approval prompts through the MCP elicitation request path. |

## Deprecated settings

| Flag | State | Stage | Agent tools / user-visible or runtime effect |
|---|---|---|---|
| [`web_search_cached`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L162) | Off | L | Deprecated legacy cached-search toggle. Use the top-level web_search setting instead. |
| [`web_search_request`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L159) | Off | L | Deprecated legacy live-search toggle. Use the top-level web_search setting instead. |
| [`transcript_v2`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L99) | Off | L | Use tui.fullscreen_transcript instead; this legacy flag is ignored. |
| [`use_legacy_landlock`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L167) | Off | L | Selects the deprecated Landlock fallback instead of the default bubblewrap pipeline. |

## Removed compatibility flags — all 40

These are retained keys, not useful feature switches. Their printed on/off values do not establish present functionality. Some underlying behaviors are now always on; other implementations were deleted.

| Flag | What remains / replacement or historical purpose |
|---|---|
| [`apply_patch_freeform`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L418) | Retired switch for a deleted patch fallback; it does not enable today's apply_patch tool. |
| [`apps_mcp_path_override`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L235) | Retired override for the legacy Apps MCP path. |
| [`code_mode_buffered_exec`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L123) | Retired configuration of the code-mode exec yield timeout. |
| [`codex_git_commit`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L414) | Retired guidance for legacy Git commit attribution. |
| [`collaboration_modes`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L426) | Plan and Default modes are now always enabled; this compatibility switch no longer selects their availability. |
| [`elevated_windows_sandbox`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L406) | Retired switch for the elevated Windows setup-and-runner sandbox pipeline; not a Linux setting. |
| [`enable_fanout`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L219) | Retired switch for deleted agent-job tools. |
| [`experimental_windows_sandbox`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L404) | Retired switch for the restricted-token Windows sandbox; not a Linux setting. |
| [`external_migration`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L297) | Retained no-op compatibility key. |
| [`guardian_ext`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L344) | Retired unused Guardian extension prototype. |
| [`guardianv2.thread_context`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L329) | Thread-owned Guardian context is always enabled; this compatibility setting is ignored. |
| [`image_detail_original`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L431) | No-op compatibility key. Passing it does not restore a separate original-detail feature. |
| [`item_ids`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L309) | Response item IDs are always enabled; this compatibility switch no longer controls them. |
| [`js_repl`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L393) | Retired switch for the deleted JavaScript REPL feature; not the current code-mode host. |
| [`js_repl_tools_only`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L395) | Retired switch for the deleted JavaScript REPL tool-only mode. |
| [`local_thread_store_shared_compression`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L188) | Superseded by local_thread_store_compression, which controls all rollout files. |
| [`multi_agent_mode`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L217) | Retained no-op compatibility key, not the current multi_agent/multi_agent_v2 selector. |
| [`personality`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L366) | Retained no-op compatibility key; not a current personality selector. |
| [`plugin_hooks`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L255) | Retired lifecycle-hook switch. Current plugin hooks use the active hook system rather than this old flag. |
| [`remote_compaction_v2`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L378) | Retired compatibility key still advertised to the Responses API; not a supported user switch for selecting compaction V2. |
| [`remote_control`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L428) | Retired switch for deleted remote-control functionality. |
| [`remote_models`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L412) | Legacy remote-model compatibility key; not a current model-availability selector. |
| [`request_rule`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L402) | Retired switch for the former request-rule approval flow. Current approval prompts are governed by approval policy. [Implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/prompts/src/permissions_instructions.rs#L29) |
| [`resize_all_images`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L307) | Centralized image preparation is always enabled; this compatibility key no longer controls it. |
| [`responses_websockets`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L438) | Retired rollout key for Responses API WebSocket transport experiments. |
| [`responses_websockets_v2`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L440) | Retired V2 rollout key for Responses API WebSocket transport experiments. |
| [`search_tool`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L397) | Legacy search-tool compatibility flag; not the current top-level web_search setting. |
| [`send_async_message`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L323) | Retired switch for model-enabled asynchronous user messaging. |
| [`skill_env_var_dependency_prompt`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L317) | Retired prompting for skill environment-variable dependencies. |
| [`sqlite`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L416) | Retired rollout-metadata database flag. On does not mean it is an exposed SQL tool for the agent. |
| [`steer`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L423) | Immediate steering submission is always enabled; this compatibility switch no longer chooses that behavior. |
| [`terminal_resize_reflow`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L145) | Terminal transcript reflow on resize is always enabled; this compatibility key no longer controls it. |
| [`tool_search`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L237) | Tool search is always enabled; this old compatibility key no longer controls it. |
| [`tool_search_always_defer_mcp_tools`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L239) | MCP tools are always deferred when tool search is available; this old flag no longer chooses that behavior. |
| [`tui_app_server`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L433) | The TUI always uses the app-server implementation; this old flag no longer selects an implementation. |
| [`unavailable_dummy_tools`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L420) | Retired placeholders for unavailable tools. |
| [`undo`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L391) | Retained no-op so older configurations containing undo can still parse. |
| [`unified_exec_zsh_fork`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L143) | Retired composition gate between unified execution and the zsh bridge; it was not an independent shell-enablement flag. |
| [`use_linux_sandbox_bwrap`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L400) | Retired opt-in for the Linux bubblewrap sandbox; old wrappers can still pass the key. |
| [`workspace_owner_usage_nudge`](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/features/src/lib.rs#L436) | Workspace-owner usage nudges are always enabled; this compatibility flag no longer controls them. |

Descriptions summarize declarations and linked implementation checks. Four Luna source audits checked the major groups and sampled internal consumers; this is not an end-to-end test of every feature. Desktop implementation outside the published CLI source was not inspected. No Codex settings were changed.

## Sources and maintenance notes

### Provenance and scope

- CLI: `codex-cli 0.160.0`, Linux; resolved settings captured on 2026-10-02. Retained [raw CLI output](codex-features/0.160.0-linux-2026-10-02/feature-list.txt) and [parsed registry comparison](codex-features/0.160.0-linux-2026-10-02/feature-catalog.json). The comparison is a derived aid; the CLI output owns the observed settings, and upstream source owns declarations and behavior.
- Public source: tag `rust-v0.160.0`, commit [`a956835d020762cb2b570053af06f643a11c0ecc`](https://github.com/openai/codex/tree/a956835d020762cb2b570053af06f643a11c0ecc). This matches the installed version; it is not a claim of byte-for-byte binary provenance. All implementation links in the table are pinned to this commit.
- The [CLI list implementation](https://github.com/openai/codex/blob/a956835d020762cb2b570053af06f643a11c0ecc/codex-rs/cli/src/main.rs#L1805) prints resolved registry settings. A setting is not evidence that a tool is exposed in this session, that the client implements a UI, or that configuration permits its use.
- The original session artifact is a historical snapshot. This repository file is the maintained owner. Do not depend on the session artifact directory or temporary source checkout for future updates.

### Updating the guide

1. Record `codex --version` and capture `codex features list`; compare keys, stages and settings with the retained snapshot. Mark unverified descriptions explicitly if the new version cannot be checked.
2. Locate source matching that version. Check changed registry entries and follow their actual consumers: tool registration, model/session selection, configuration conditions, approval handling and client UX. Inspect unchanged entries when their consumers changed. Public CLI declarations alone cannot establish private desktop behavior.
3. Update affected rows, pinned links, observation date and provenance together. Keep the functional groupings and one removed-flags subsection; explain concrete tools and UX. Retain a new dated settings snapshot when refreshing it, and distinguish configuration-only changes from implementation changes.
4. Check each observed key appears exactly once, all removed entries are together, table structure and links remain valid, and counts match the snapshot. State which consumer or runtime checks were performed; completeness checks do not validate behavior. Use Git history for earlier explanations rather than keeping competing maintained guides.

Refresh in response to a relevant change or request; this document does not establish a scheduled updater.

### Interpretation and delivery pitfalls

Preserve the table-first format. Search and read the consumers needed for a claim; use bounded source excerpts and, where authorized, cheap delegated audits for independent groups. A direct Markdown file link is sufficient for this reference; rendering or serving is useful only when an actual access requirement calls for it.

A declared `artifact` flag does not imply native artifact tools or UI in the public CLI. Desktop policy gates do not establish a TUI browser pane. A false `multi_agent_v2` setting does not decide the active protocol by itself. Removed flags can represent deleted or always-on behavior. Recheck these conclusions against changed source rather than turning them into timeless rules.

These maintenance notes are scoped to this reference; they do not override current user instructions. Completeness and sampled source checks do not establish exhaustive runtime behavior or the effectiveness of this maintenance process.
