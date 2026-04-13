<!--
Purpose: provide an auditable reference for the Codex CLI configuration surface.
Context: this is the authoritative artifact being built for a user who wants the complete surface and wants later fact-checkers to distinguish source statements from local observations and inferences.
-->

# Codex CLI configuration reference

Date: 2026-04-13  
Local binary checked: `codex-cli 0.120.0`

## Status

This file is the authoritative reference artifact.

Current coverage in this revision:

- configuration layers and precedence model
- CLI flags and subcommands
- top-level keys in `config.toml`, expanded against the published config-reference key table
- table namespaces and documented subkeys
- feature-flag surface
- non-TOML repo config
- managed / enterprise config

Still weaker than I want:

- not every documented feature flag has a one-line semantic gloss yet
- the file includes a top-level schema-key appendix, but not a full appendix of every dotted key from the published config-reference table
- the discrepancies/open-questions section is intentionally short because only a small number of unresolved items remain

## Audit conventions

- `Doc`: directly supported by an OpenAI documentation page listed in the section's evidence block.
- `Local`: directly observed from the installed binary on this machine.
- `Inference`: conclusion derived from one or more documented or locally observed facts. The reasoning is stated explicitly.
- `Unverified`: plausible but not yet confirmed in the current pass. Avoid relying on these items until upgraded.

The goal is not “citations near every sentence”; the goal is that a later reader can efficiently check each claim category and know what still needs checking.

## Section 1: What counts as configuration

### Claim set

- `Doc`: Codex has a user config file at `~/.codex/config.toml`.
- `Doc`: Codex also supports project-scoped `.codex/config.toml` layers.
- `Doc`: Codex behavior is additionally shaped by project docs and repo-local assets such as `AGENTS.md`, rules, and skills.
- `Doc`: enterprise-managed files `managed_config.toml` and `requirements.toml` are also part of the effective configuration surface.
- `Inference`: the complete configuration surface is larger than `config.toml`; it includes CLI overrides, repo-local `.codex/` layers, non-TOML project assets, local persisted state, and managed policy.

### Evidence

- Config basics: <https://developers.openai.com/codex/config-basics>
- Advanced config: <https://developers.openai.com/codex/config-advanced>
- Managed configuration: <https://developers.openai.com/codex/enterprise/managed-configuration>
- Changelog: <https://developers.openai.com/codex/changelog>

## Section 2: Layering and precedence

### Effective layers

1. `Local`: CLI flags such as `--config`, `--profile`, `--sandbox`, `--ask-for-approval`, `--model`, `--search`.
2. `Doc`: user config, typically `~/.codex/config.toml`.
3. `Doc`: project and ancestor `.codex/config.toml` layers.
4. `Doc`: project assets such as `AGENTS.md`, `.codex/rules/`, `.codex/skills/`, `.codex/hooks.json`.
5. `Doc`: managed defaults and constraints via `managed_config.toml` and `requirements.toml`.
6. `Inference`: persisted local state such as auth credentials, MCP OAuth credentials, history, logs, and SQLite state must also be treated as part of the effective runtime configuration, because they change behavior without requiring a prompt-time flag.

### Precedence statements

- `Doc`: the changelog entry for Team Config says shared `.codex/` layers are loaded from the current working directory, parent directories, repo root, user location, and system location.
- `Doc`: higher-precedence locations override lower-precedence locations.
- `Doc`: `requirements.toml` can still override defaults.
- `Doc`: managed configuration docs distinguish managed defaults from hard requirements.
- `Inference`: a correct mental model is “preferences stack, then requirements constrain the result.”

### Evidence

- Config basics: <https://developers.openai.com/codex/config-basics>
- Advanced config: <https://developers.openai.com/codex/config-advanced>
- Managed configuration: <https://developers.openai.com/codex/enterprise/managed-configuration>
- Changelog, Team Config entry: <https://developers.openai.com/codex/changelog>

## Section 3: CLI flags

### Top-level `codex` flags

- `Local` `--config`, `-c`: one-off override for a config key. In `0.120.0`, the help text says the value is parsed as TOML; if TOML parsing fails, the raw string is used literally.
- `Local` `--enable`: equivalent to setting `features.<name> = true` for the current run.
- `Local` `--disable`: equivalent to setting `features.<name> = false` for the current run.
- `Local` `--remote`: connect the TUI to a remote app-server websocket endpoint.
- `Local` `--remote-auth-token-env`: env var containing the bearer token used with `--remote`.
- `Local` `--image`, `-i`: attach one or more images to the initial prompt.
- `Local` `--model`, `-m`: choose the model for the run.
- `Local` `--oss`: use the local open-source provider path.
- `Local` `--local-provider`: choose which local provider to use, such as `lmstudio` or `ollama`.
- `Local` `--profile`, `-p`: load a named profile from `config.toml`.
- `Local` `--sandbox`, `-s`: choose the command sandbox mode.
- `Local` `--ask-for-approval`, `-a`: choose when command execution requires approval.
- `Local` `--full-auto`: convenience alias for low-friction sandboxed automatic execution. The local help maps this to sandboxed execution, not to unsandboxed execution.
- `Local` `--dangerously-bypass-approvals-and-sandbox`: disable approval prompts and sandboxing.
- `Local` `--cd`, `-C`: set the working root.
- `Local` `--search`: enable live web search.
- `Local` `--add-dir`: grant write access to additional directories.
- `Local` `--no-alt-screen`: disable alternate-screen mode and keep the TUI inline in the main terminal buffer.

### `codex exec`-specific flags

- `Local` `--skip-git-repo-check`: allow execution outside a Git repository.
- `Local` `--ephemeral`: run without persisting session files to disk.
- `Local` `--output-schema`: path to a JSON schema constraining the final response shape.
- `Local` `--color`: control color mode in output.
- `Local` `--json`: print JSONL events to stdout.
- `Local` `--output-last-message`: write the final assistant message to a file.

### Related subcommands

- `Local` `exec`: non-interactive agent execution.
- `Local` `review`: non-interactive code review mode.
- `Local` `login`, `logout`: authentication state management.
- `Local` `mcp`: manage external MCP servers.
- `Local` `mcp-server`: run Codex as an MCP server over stdio.
- `Local` `app-server`: experimental app-server tooling.
- `Local` `completion`: generate shell completion scripts.
- `Local` `sandbox`: run commands inside Codex's sandbox implementation.
- `Local` `debug`: debugging tools for prompt input and app-server behavior.
- `Local` `apply`: apply the latest produced diff to the local working tree.
- `Local` `resume`, `fork`: reopen or branch an existing session.
- `Local` `cloud`: experimental cloud task browser / apply flow.
- `Local` `exec-server`: experimental standalone exec-server entrypoint.
- `Local` `features`: inspect and toggle feature flags.

### Evidence

- Local command: `codex --help`
- Local command: `codex exec --help`
- Local command: `codex mcp --help`
- Local command: `codex sandbox --help`
- Local command: `codex login --help`
- CLI reference: <https://developers.openai.com/codex/cli/reference>

## Section 4: Top-level and dotted keys outside table-only sections

This section covers documented keys from the config reference that are not already accounted for purely as table namespaces. After the later key-table scrape, this section is broader than the original “top-level scalar keys” pass.

### Model and provider selection

- `Doc` `model`: default model for ordinary interactive or exec runs.
- `Doc` `review_model`: model specifically used for review mode when separated from normal runs.
- `Doc` `model_provider`: which provider implementation resolves the model name.
- `Doc` `openai_base_url`: base URL override for the built-in OpenAI provider.
- `Doc` `model_context_window`: context-window size, in tokens, available to the active model.
- `Doc` `model_auto_compact_token_limit`: token threshold that triggers automatic history compaction.
- `Doc` `model_catalog_json`: path to a JSON model catalog loaded on startup.
- `Doc` `service_tier`: provider-specific service-tier selection.
- `Doc` `oss_provider`: default local provider used when `--oss` is selected.
- `Doc` `personality`: default communication style for models that support personality controls.

### Reasoning and verbosity

- `Doc` `model_reasoning_effort`: default reasoning budget for supported models.
- `Doc` `plan_mode_reasoning_effort`: reasoning budget used when Codex is in plan mode.
- `Doc` `model_reasoning_summary`: control for whether reasoning summaries are shown and at what level.
- `Doc` `model_verbosity`: output verbosity control for supported models.
- `Doc` `model_supports_reasoning_summaries`: manual override for reasoning-summary capability detection.

### Instruction and prompt overrides

- `Doc` `developer_instructions`: extra developer-style instructions injected before project docs.
- `Doc` `compact_prompt`: inline override for the context-compaction prompt.
- `Doc` `commit_attribution`: override or disable the built-in commit attribution trailer.
- `Doc` `model_instructions_file`: file-based replacement for the built-in base instructions.
- `Doc` `experimental_compact_prompt_file`: file-based replacement for the compaction prompt.

### Approval and sandbox

- `Doc` `approval_policy`: top-level approval mode. This can be a named mode or a structured object.
- `Doc` `allow_login_shell`: whether tool execution may use login-shell semantics.
- `Doc` `sandbox_mode`: overall sandbox preset.
- `Doc` `rules`: admin-enforced command rules merged with `.rules` files.
- `Doc` `default_permissions`: default named permissions profile to apply from the `[permissions]` table.

### Authentication and login

- `Doc` `cli_auth_credentials_store`: storage backend for Codex authentication credentials.
- `Doc` `chatgpt_base_url`: base URL used for the ChatGPT login flow.
- `Doc` `forced_chatgpt_workspace_id`: restrict ChatGPT login to one workspace UUID.
- `Doc` `forced_login_method`: force one login method instead of prompting.
- `Doc` `mcp_oauth_credentials_store`: storage backend for MCP OAuth credentials.
- `Doc` `mcp_oauth_callback_port`: fixed local port for the MCP OAuth callback listener.
- `Doc` `mcp_oauth_callback_url`: externally visible callback URL override for MCP OAuth.

### Project-doc loading

- `Doc` `project_doc_max_bytes`: byte limit for injected project docs.
- `Doc` `project_doc_fallback_filenames`: fallback filenames treated like `AGENTS.md`.
- `Doc` `project_root_markers`: files used to detect where the project root starts.

### UI, output, notifications, updates

- `Doc` `notify`: external notifier command, expressed as argv.
- `Doc` `file_opener`: URI scheme used for clickable file citations.
- `Doc` `hide_agent_reasoning`: suppress reasoning events in UI/output.
- `Doc` `show_raw_agent_reasoning`: expose raw reasoning content when available.
- `Doc` `disable_paste_burst`: disable burst-paste handling in the TUI.
- `Doc` `windows_wsl_setup_acknowledged`: Windows onboarding acknowledgement state.
- `Doc` `check_for_update_on_startup`: whether Codex checks for updates on launch.
- `Doc` `notice.hide_full_access_warning`: acknowledgement state for the full-access warning prompt.
- `Doc` `notice.hide_rate_limit_model_nudge`: acknowledgement state for the rate-limit model-switch reminder.
- `Doc` `notice.hide_world_writable_warning`: acknowledgement state for the world-writable warning prompt.
- `Doc` `notice.hide_gpt5_1_migration_prompt`: acknowledgement state for a GPT-5.1 migration prompt.
- `Doc` `notice.hide_gpt-5.1-codex-max_migration_prompt`: acknowledgement state for a GPT-5.1 Codex Max migration prompt.
- `Doc` `notice.model_migrations`: notice-state bucket for model migration prompts.
- `Doc` `feedback.enabled`: whether `/feedback` submission is enabled.
- `Doc` `analytics.enabled`: whether analytics are enabled for the machine/profile.

### Search, profile activation, warnings

- `Doc` `web_search`: whether search is disabled, cached, or live.
- `Doc` `tools.web_search`: object-form configuration for the web-search tool, including context size, allowed domains, and approximate location.
- `Doc` `profile`: active default profile name.
- `Doc` `suppress_unstable_features_warning`: suppress unstable-feature warning banners.

### Runtime state locations and timing

- `Doc` `background_terminal_max_timeout`: maximum empty polling window for background terminals.
- `Doc` `log_dir`: location for log files.
- `Doc` `sqlite_home`: location for SQLite-backed runtime state.
- `Doc` `tool_output_token_limit`: token budget for storing individual tool outputs in history.

### Shell environment policy

- `Doc` `shell_environment_policy.inherit`: baseline environment inheritance when spawning subprocesses.
- `Doc` `shell_environment_policy.set`: explicit environment overrides injected into every subprocess.
- `Doc` `shell_environment_policy.include_only`: whitelist of patterns; when set, only matching variables are kept.
- `Doc` `shell_environment_policy.exclude`: glob patterns removed after default excludes are applied.
- `Doc` `shell_environment_policy.ignore_default_excludes`: keep variables containing names like `KEY`, `SECRET`, or `TOKEN` before later filters run.
- `Doc` `shell_environment_policy.experimental_use_profile`: whether subprocesses should use the user's shell profile.

### Requirement-only keys from the published key table

- `Doc` `allowed_approval_policies`: requirements-side allowlist for approval-policy values.
- `Doc` `allowed_sandbox_modes`: requirements-side allowlist for sandbox modes.
- `Doc` `allowed_web_search_modes`: requirements-side allowlist for web-search modes.
- `Doc` `mcp_servers`: in `requirements.toml`, allowlist of MCP servers that may be enabled.

### Legacy and deprecated compatibility keys

- `Doc` `instructions`: reserved for future use; docs say to prefer `model_instructions_file` or `AGENTS.md`.
- `Doc` `experimental_use_unified_exec_tool`: legacy name for unified exec; docs prefer `[features].unified_exec` or `--enable unified_exec`.

### Evidence

- Config reference: <https://developers.openai.com/codex/config-reference>
- Sample config: <https://developers.openai.com/codex/config-sample>
- Advanced config: <https://developers.openai.com/codex/config-advanced>
- Config schema: <https://developers.openai.com/codex/config-schema.json>

## Section 4A: Additional top-level keys exposed by the official JSON schema

This section exists because the JSON schema exposes top-level keys that are harder to inventory from the prose config-reference page alone. These are still official sources of truth because the schema is published by OpenAI as the latest schema for `config.toml`.

### Keys surfaced in the schema

- `Doc` `approvals_reviewer`: configures who escalated approval requests are routed to for review.
- `Doc` `audio`: machine-local realtime audio device preferences used by realtime voice.
- `Doc` `experimental_realtime_start_instructions`: experimental realtime start-instruction override.
- `Doc` `experimental_realtime_ws_backend_prompt`: experimental override for realtime websocket transport instructions.
- `Doc` `experimental_realtime_ws_base_url`: experimental override for the realtime websocket base URL.
- `Doc` `experimental_realtime_ws_model`: experimental selection of the realtime websocket model/snapshot.
- `Doc` `experimental_realtime_ws_startup_context`: experimental replacement for synthesized realtime startup context.
- `Doc` `experimental_use_freeform_apply_patch`: schema-published experimental boolean key.
- `Doc` `ghost_snapshot`: settings for ghost snapshots used for undo.
- `Doc` `include_apps_instructions`: whether to inject the `<apps_instructions>` developer block.
- `Doc` `include_environment_context`: whether to inject the `<environment_context>` user block.
- `Doc` `include_permissions_instructions`: whether to inject the `<permissions instructions>` developer block.
- `Doc` `js_repl_node_module_dirs`: ordered list of directories to search for Node modules in `js_repl`.
- `Doc` `js_repl_node_path`: optional absolute path to the Node runtime used by `js_repl`.
- `Doc` `memories`: memories subsystem settings.
- `Doc` `permissions`: named permissions profiles.
- `Doc` `plugins`: user-level plugin configuration keyed by plugin name.
- `Doc` `realtime`: experimental realtime websocket session selection.
- `Doc` `tool_suggest`: additional discoverable tools that can be suggested for installation.
- `Doc` `zsh_path`: optional absolute path to the patched zsh used by zsh-exec-bridge-backed shell execution.

### Interpretation note

- `Inference`: these keys are part of the official configuration surface even when the prose guides emphasize them less, because the published JSON schema is explicitly offered as the latest schema for `config.toml`.

### Evidence

- Config schema: <https://developers.openai.com/codex/config-schema.json>

## Section 5: `config.toml` tables and documented subkeys

This section covers table-shaped config. A top-level table is part of the configuration surface even when some of its leaves are optional or sparsely documented.

### `[history]`

- `Doc` `[history]`: namespace for transcript persistence settings.
- `Doc` `persistence`: whether session transcripts are saved at all.
- `Doc` `max_bytes`: size cap for saved history before old entries are dropped.

### `[tui]`

- `Doc` `[tui]`: namespace for terminal UI behavior.
- `Doc` `tui.alternate_screen`: controls whether Codex uses the terminal alternate screen buffer. The CLI docs tie `--no-alt-screen` to this setting.
- `Doc` `tui.animations`: enable or disable terminal animations.
- `Doc` `tui.notification_method`: terminal-notification method for unfocused notifications.
- `Doc` `tui.notifications`: enable notifications, optionally restricted to specific event types.
- `Doc` `tui.show_tooltips`: show onboarding tooltips on the welcome screen.
- `Doc` `tui.status_line`: ordered list of footer status-line item identifiers; `null` disables the status line.
- `Doc` `tui.theme`: syntax-highlighting theme override.
- `Doc` `tui.model_availability_nux.<model>`: internal startup-tooltip state keyed by model slug.

### `[agents]` and `[agents.<role>]`

- `Doc` `[agents]`: namespace for spawned-agent limits and defaults.
- `Doc` `agents.max_threads`: maximum concurrently open agent threads.
- `Doc` `agents.max_depth`: maximum nesting depth for spawned agents.
- `Doc` `agents.job_max_runtime_seconds`: default timeout for worker jobs.
- `Doc` `[agents.<role>]`: named role definition for subagent tooling.
- `Doc` `agents.<role>.description`: human-readable role description.
- `Doc` `agents.<role>.config_file`: config file loaded for that role.
- `Doc` `agents.<role>.nickname_candidates`: suggested display names for that role.

### `[[skills.config]]`

- `Doc` `[[skills.config]]`: repeated block for per-skill config overrides.
- `Doc` `skills.config.path`: location of the skill definition.
- `Doc` `skills.config.enabled`: whether that skill is available.

### `[sandbox_workspace_write]`

- `Doc` `[sandbox_workspace_write]`: extra policy details that refine `sandbox_mode = "workspace-write"`.
- `Doc` `sandbox_workspace_write.exclude_tmpdir_env_var`: if true, do not automatically grant write access via `$TMPDIR`.
- `Doc` `sandbox_workspace_write.exclude_slash_tmp`: if true, do not automatically grant write access to `/tmp`.
- `Doc` `sandbox_workspace_write.writable_roots`: extra writable paths in workspace-write mode.
- `Doc` `sandbox_workspace_write.network_access`: whether outbound network is allowed in workspace-write mode.

### `[model_providers.<name>]`

- `Doc` `[model_providers.<name>]`: one provider/backend definition.
- `Doc` `name`: human-readable provider label.
- `Doc` `base_url`: root HTTP endpoint for the provider.
- `Doc` `wire_api`: request protocol Codex should speak to the provider. The sample config shows `responses`.
- `Doc` `env_key`: env var containing the provider API key.
- `Doc` `env_key_instructions`: user-facing text shown when the key is missing.
- `Doc` `query_params`: fixed query parameters appended to provider requests.
- `Doc` `http_headers`: static headers attached to provider requests.
- `Doc` `env_http_headers`: headers populated from environment variables at runtime.
- `Doc` `request_max_retries`: retry budget for non-streaming requests.
- `Doc` `stream_max_retries`: retry budget for streaming requests.
- `Doc` `stream_idle_timeout_ms`: idle timeout for streaming responses.
- `Doc` `supports_websockets`: whether websocket transport is supported where applicable.
- `Doc` `requires_openai_auth`: whether the provider should use OpenAI auth semantics rather than only an arbitrary API key.
- `Doc` `experimental_bearer_token`: direct bearer token override for development/testing use.

### `[mcp_servers.<name>]`

- `Doc` `[mcp_servers.<name>]`: one MCP server definition.
- `Doc` `enabled`: whether Codex should attempt to load the server.
- `Doc` `required`: whether startup or resume should fail if the server cannot initialize.
- `Doc` `command`: stdio transport command.
- `Doc` `args`: stdio transport arguments.
- `Doc` `env`: environment variables injected into the MCP server process.
- `Doc` `cwd`: working directory for the MCP server process.
- `Doc` `url`: HTTP transport endpoint for a streamable MCP server.
- `Doc` `bearer_token_env_var`: env var whose contents become an Authorization bearer token.
- `Doc` `http_headers`: static headers sent to the HTTP MCP server.
- `Doc` `env_http_headers`: headers populated from environment variables.
- `Doc` `startup_timeout_sec`: timeout for MCP server startup.
- `Doc` `tool_timeout_sec`: timeout for individual MCP tool calls.
- `Doc` `startup_timeout_ms`: millisecond alias for `startup_timeout_sec`.
- `Doc` `enabled_tools`: allow-list of tools exposed from the server.
- `Doc` `disabled_tools`: deny-list of tools hidden from the server.
- `Doc` `scopes`: OAuth scopes requested for the server.
- `Doc` `env_vars`: additional environment variables whitelisted for an MCP stdio server.
- `Doc` `oauth_resource`: optional RFC 8707 OAuth resource parameter during MCP login.

### Requirements-side MCP allowlisting

- `Doc` `mcp_servers.<id>.identity`: identity rule for one allowed MCP server in `requirements.toml`.
- `Doc` `mcp_servers.<id>.identity.command`: allow a stdio MCP server when its configured command matches.
- `Doc` `mcp_servers.<id>.identity.url`: allow a streamable HTTP MCP server when its configured URL matches.
- `Inference`: requirements-side `mcp_servers` is not the same object as ordinary runtime `mcp_servers.<name>` config; it is a policy allowlist keyed by name plus identity.

### `[apps]`, `[apps._default]`, `[apps.<app>]`, `[apps.<app>.tools."<tool id>"]`

- `Doc` `[apps]`: namespace for app / connector configuration.
- `Doc` `[apps._default]`: defaults applied to all apps unless overridden.
- `Doc` `[apps.<app>]`: per-app policy override block.
- `Doc` `enabled`: whether the app is enabled.
- `Doc` `destructive_enabled`: whether destructive-hint tools are permitted for that app.
- `Doc` `open_world_enabled`: whether externally acting or open-world behavior is permitted for that app.
- `Doc` `default_tools_enabled`: whether the app's default tools are exposed.
- `Doc` `default_tools_approval_mode`: baseline approval behavior for tools in that app.
- `Doc` `[apps.<app>.tools."<tool id>"]`: per-tool override block within one app.
- `Doc` `enabled`: per-tool enabled state is also documented for app tools.
- `Doc` `approval_mode`: tool-specific approval behavior overriding the app default.

### `[profiles.<name>]`

- `Doc` `[profiles.<name>]`: named config overlay.
- `Doc` Profiles can override ordinary config keys rather than introducing a new category of setting.
- `Doc` The sample config explicitly shows profile-scoped overrides such as `model`, `model_provider`, `approval_policy`, `sandbox_mode`, `service_tier`, `oss_provider`, reasoning controls, prompt-file overrides, app/tool controls, and `features = { ... }`.

### `[projects."/absolute/path"]`

- `Doc` `[projects."/absolute/path"]`: trust configuration for one explicit project path.
- `Doc` `trust_level`: whether that path is treated as trusted or untrusted.
- `Inference`: this matters operationally because project-level `.codex/` layers are skipped when the project is untrusted.

### `[tools]`

- `Doc` `[tools]`: namespace for tool-specific toggles not scoped to one app.
- `Doc` `view_image`: whether the image-viewing tool is exposed.
- `Doc` `web_search`: optional web-search tool configuration. The docs explicitly say the legacy boolean form is accepted, but the object form allows `context_size`, `allowed_domains`, and approximate `location`.

### `[otel]`, `[otel.exporter."<kind>"]`, `[otel.trace_exporter."<kind>"]`, `[otel.metrics_exporter."<kind>"]`

- `Doc` `[otel]`: namespace for telemetry behavior and exporter selection.
- `Doc` `otel.log_user_prompt`: whether prompt text is included in telemetry logs.
- `Doc` `otel.environment`: environment label attached to telemetry.
- `Doc` `otel.exporter`: log-exporter backend selection.
- `Doc` `otel.trace_exporter`: trace-exporter backend selection.
- `Doc` `otel.metrics_exporter`: metrics-exporter backend selection.
- `Doc` Exporter blocks can include `endpoint`, `protocol`, `headers`, and TLS-related material such as CA certificate, client certificate, and client private key.

### `[windows]`

- `Doc` `[windows]`: Windows-specific settings namespace.
- `Doc` `windows.sandbox`: native Windows sandbox mode selection.
- `Doc` `windows.sandbox_private_desktop`: whether the final sandboxed child runs on a private desktop by default on native Windows.

### `[rules]`

- `Doc` `[rules]`: namespace for admin-enforced command rules merged with `.rules` files.
- `Doc` `rules.prefix_rules`: list of enforced prefix rules.
- `Doc` `rules.prefix_rules[].decision`: required decision for a rule; docs show `prompt | forbidden`.
- `Doc` `rules.prefix_rules[].justification`: optional rationale surfaced in approval prompts or rejection messages.
- `Doc` `rules.prefix_rules[].pattern`: command prefix expressed as pattern tokens.
- `Doc` `rules.prefix_rules[].pattern[].any_of`: list of allowed alternative tokens at one position.
- `Doc` `rules.prefix_rules[].pattern[].token`: one literal token at one position.

### `[features]`

- `Doc` `[features]`: pinned feature values keyed by canonical feature names.
- `Doc` `features.<name>`: boolean per-feature override.

### `[permissions]`, `[plugins]`, `[memories]`, `[audio]`, `[realtime]`, `[ghost_snapshot]`, `[tool_suggest]`

- `Doc` `[permissions]`: named permissions profiles.
- `Doc` `[plugins]`: user-level plugin config entries keyed by plugin name.
- `Doc` `[memories]`: memories subsystem settings.
- `Doc` `[audio]`: machine-local realtime audio device preferences.
- `Doc` `[realtime]`: experimental realtime websocket session selection.
- `Doc` `[ghost_snapshot]`: ghost-snapshot settings used for undo.
- `Doc` `[tool_suggest]`: additional discoverable tools that can be suggested for installation.

### Evidence

- Config reference: <https://developers.openai.com/codex/config-reference>
- Sample config: <https://developers.openai.com/codex/config-sample>
- Advanced config: <https://developers.openai.com/codex/config-advanced>
- CLI reference, for `tui.alternate_screen` via `--no-alt-screen`: <https://developers.openai.com/codex/cli/reference>
- Config schema: <https://developers.openai.com/codex/config-schema.json>

## Section 6: Feature-flag surface

### What is configurable

- `Local` `codex features list` reports the set of feature names known to the installed binary, together with stage and effective state.
- `Local` feature flags can be changed by `--enable FEATURE`, `--disable FEATURE`, and `[features]` in config.
- `Doc` feature maturity is documented separately from the core config pages.

### Feature names observed on this machine

- `Local` `apply_patch_freeform`
- `Local` `apps`
- `Local` `artifact`
- `Local` `child_agents_md`
- `Local` `code_mode`
- `Local` `code_mode_only`
- `Local` `codex_git_commit`
- `Local` `codex_hooks`
- `Local` `collaboration_modes`
- `Local` `default_mode_request_user_input`
- `Local` `elevated_windows_sandbox`
- `Local` `enable_fanout`
- `Local` `enable_request_compression`
- `Local` `exec_permission_approvals`
- `Local` `experimental_windows_sandbox`
- `Local` `fast_mode`
- `Local` `general_analytics`
- `Local` `guardian_approval`
- `Local` `image_detail_original`
- `Local` `image_generation`
- `Local` `js_repl`
- `Local` `js_repl_tools_only`
- `Local` `memories`
- `Local` `multi_agent`
- `Local` `multi_agent_v2`
- `Local` `personality`
- `Local` `plugins`
- `Local` `prevent_idle_sleep`
- `Local` `realtime_conversation`
- `Local` `remote_control`
- `Local` `remote_models`
- `Local` `request_permissions_tool`
- `Local` `request_rule`
- `Local` `responses_websockets`
- `Local` `responses_websockets_v2`
- `Local` `runtime_metrics`
- `Local` `search_tool`
- `Local` `shell_snapshot`
- `Local` `shell_tool`
- `Local` `shell_zsh_fork`
- `Local` `skill_env_var_dependency_prompt`
- `Local` `skill_mcp_dependency_install`
- `Local` `sqlite`
- `Local` `steer`
- `Local` `tool_call_mcp_elicitation`
- `Local` `tool_search`
- `Local` `tool_suggest`
- `Local` `tui_app_server`
- `Local` `undo`
- `Local` `unified_exec`
- `Local` `use_legacy_landlock`
- `Local` `use_linux_sandbox_bwrap`
- `Local` `web_search_cached`
- `Local` `web_search_request`

### Interpretation notes

- `Inference`: the complete feature-flag surface for a given installation is version-specific, so `codex features list` is part of the source of truth, not just the docs.
- `Doc` the config reference does provide meanings for a subset of canonical feature keys, including:
  - `features.apps`: enable ChatGPT Apps/connectors support.
  - `features.codex_hooks`: enable lifecycle hooks loaded from `hooks.json`.
  - `features.enable_request_compression`: compress streaming request bodies with zstd when supported.
  - `features.fast_mode`: the docs describe this as preventing the machine from sleeping while a turn is actively running.
  - `features.multi_agent`: enable the multi-agent collaboration tools.
  - `features.personality`: enable personality-selection controls.
  - `features.shell_snapshot`: snapshot shell environment to speed up repeated commands.
  - `features.skill_mcp_dependency_install`: allow prompting for and installing missing MCP dependencies for skills.
  - `features.smart_approvals`: route eligible approval requests through the guardian reviewer subagent.
  - `features.undo`: enable undo support.
  - `features.unified_exec`: use the unified PTY-backed exec tool.
  - `features.web_search`: deprecated legacy toggle; prefer top-level `web_search`.
- `Doc` the prose table appears internally inconsistent for `features.web_search_cached`: its description matches shell-tool text rather than web-search-caching text.
- `Inference`: because of that inconsistency, this file does not assign a semantic gloss to `web_search_cached` beyond recording the name and the docs issue.
- `Unverified`: this file still does not provide a one-line meaning for every feature flag exposed by `codex features list`.

### Evidence

- Local command: `codex features list`
- Local command: `codex features --help`
- Config reference: <https://developers.openai.com/codex/config-reference>
- Feature maturity: <https://developers.openai.com/codex/feature-maturity>

## Section 7: Non-TOML repo and user config surface

### Items

- `Doc` `AGENTS.md`: repository instruction file injected into the model context.
- `Doc` `.codex/config.toml`: repo-local shared defaults.
- `Doc` `.codex/rules/`: repo-local rules controlling command behavior.
- `Doc` `.codex/skills/`: repo-local reusable skills.
- `Doc` `.codex/hooks.json`: repo-local hook definitions.
- `Doc` `~/.codex/...`: user-level config, skills, and local state.
- `Doc` `/etc/codex/...`: system-level config and policy.

### Interpretation notes

- `Doc` the changelog calls this shared repository/user/system layering “Team Config”.
- `Inference`: these files are part of the configuration surface even though several are not TOML, because they deterministically change model behavior and tool policy.

### Evidence

- Advanced config: <https://developers.openai.com/codex/config-advanced>
- Changelog, Team Config entry: <https://developers.openai.com/codex/changelog>

## Section 8: Managed / enterprise config surface

### Managed files

- `Doc` `managed_config.toml`: managed defaults supplied by the org/admin side.
- `Doc` `requirements.toml`: hard constraints on what users may select.

### Behavioral claims

- `Doc` managed defaults and hard requirements are distinct concepts in the managed-configuration docs.
- `Doc` requirements can override defaults regardless of location.
- `Doc` if configured values conflict with requirements, Codex falls back to a compatible value and notifies the user.
- `Doc` requirements can constrain feature flags via the `[features]` table in `requirements.toml`.
- `Doc` if requirements-side `mcp_servers` is present but empty, all MCP servers are disabled.
- `Doc` managed defaults merge on top of user `config.toml` and take precedence over CLI `--config` overrides at startup.
- `Doc` users can still change those settings during a session; managed defaults are reapplied next time Codex starts.
- `Doc` requirements precedence is documented as cloud-managed requirements, then macOS managed preferences, then system `requirements.toml`.
- `Doc` managed-defaults precedence is documented as managed preferences, then system `managed_config.toml`, then user `config.toml`.
- `Doc` for backwards compatibility, legacy `managed_config.toml` fields `approval_policy` and `sandbox_mode` are also interpreted as requirements.
- `Inference`: on managed installations, local `config.toml` is not the whole truth even if it is the only file the end user edits.

### Evidence

- Managed configuration: <https://developers.openai.com/codex/enterprise/managed-configuration>

## Section 9: Completeness notes from the published config-reference key table

### Machine-extracted key inventory

- `Local`: I extracted the published key names from the config-reference page's embedded table and observed 127 distinct documented keys in that table for this docs revision.
- `Inference`: this gives a stronger completeness check than relying on the prose sample config alone.
- `Doc`: the config-reference page also links the latest published JSON schema for `config.toml`, which exposes additional top-level keys and table namespaces not as easy to inventory from the prose table alone.

### Consequences for this file

- `Doc` the file now includes the major key families that were previously missing from the first draft, including `model_context_window`, `model_auto_compact_token_limit`, `model_catalog_json`, `personality`, `tools.web_search`, `shell_environment_policy.*`, notice-state keys, feedback/analytics toggles, fuller `[tui]` keys, rules-prefix-rule keys, and requirements-side allowlist keys.
- `Doc` the file now also includes schema-surfaced top-level keys and tables such as `permissions`, `plugins`, `memories`, `audio`, `realtime`, `ghost_snapshot`, `tool_suggest`, and several experimental realtime-related overrides.
- `Unverified` one schema-published key still has only existence-level coverage here: `experimental_use_freeform_apply_patch`.
- `Doc` the file now includes a top-level schema-key appendix as a finite completeness checklist.

### Evidence

- Config reference: <https://developers.openai.com/codex/config-reference>
- Local scrape of the config-reference key table performed on 2026-04-13
- Config schema: <https://developers.openai.com/codex/config-schema.json>

## Appendix A: Top-level keys from the published JSON schema

This appendix is a completeness checklist, not an explanatory section. It lists the top-level keys present in the official schema fetched on 2026-04-13.

- `agents`
- `allow_login_shell`
- `analytics`
- `approval_policy`
- `approvals_reviewer`
- `apps`
- `audio`
- `background_terminal_max_timeout`
- `chatgpt_base_url`
- `check_for_update_on_startup`
- `cli_auth_credentials_store`
- `commit_attribution`
- `compact_prompt`
- `default_permissions`
- `developer_instructions`
- `disable_paste_burst`
- `experimental_compact_prompt_file`
- `experimental_realtime_start_instructions`
- `experimental_realtime_ws_backend_prompt`
- `experimental_realtime_ws_base_url`
- `experimental_realtime_ws_model`
- `experimental_realtime_ws_startup_context`
- `experimental_use_freeform_apply_patch`
- `experimental_use_unified_exec_tool`
- `features`
- `feedback`
- `file_opener`
- `forced_chatgpt_workspace_id`
- `forced_login_method`
- `ghost_snapshot`
- `hide_agent_reasoning`
- `history`
- `include_apps_instructions`
- `include_environment_context`
- `include_permissions_instructions`
- `instructions`
- `js_repl_node_module_dirs`
- `js_repl_node_path`
- `log_dir`
- `mcp_oauth_callback_port`
- `mcp_oauth_callback_url`
- `mcp_oauth_credentials_store`
- `mcp_servers`
- `memories`
- `model`
- `model_auto_compact_token_limit`
- `model_catalog_json`
- `model_context_window`
- `model_instructions_file`
- `model_provider`
- `model_providers`
- `model_reasoning_effort`
- `model_reasoning_summary`
- `model_supports_reasoning_summaries`
- `model_verbosity`
- `notice`
- `notify`
- `openai_base_url`
- `oss_provider`
- `otel`
- `permissions`
- `personality`
- `plan_mode_reasoning_effort`
- `plugins`
- `profile`
- `profiles`
- `project_doc_fallback_filenames`
- `project_doc_max_bytes`
- `project_root_markers`
- `projects`
- `realtime`
- `review_model`
- `sandbox_mode`
- `sandbox_workspace_write`
- `service_tier`
- `shell_environment_policy`
- `show_raw_agent_reasoning`
- `skills`
- `sqlite_home`
- `suppress_unstable_features_warning`
- `tool_output_token_limit`
- `tool_suggest`
- `tools`
- `tui`
- `web_search`
- `windows`
- `windows_wsl_setup_acknowledged`
- `zsh_path`

### Evidence

- Config schema: <https://developers.openai.com/codex/config-schema.json>

## Notes on source-of-truth quality

- `Local`: for immediate behavior of the installed binary, trust `codex --help`, `codex exec --help`, and `codex features list`.
- `Doc`: for the documented key inventory and examples, use the official config reference and sample config.
- `Inference`: when doc text and local help disagree, the operationally safe rule is to record both and treat the local binary as authoritative for immediate behavior on this machine.

## Recorded discrepancy

- `Local`: `codex --help` in `0.120.0` says `--config` values are parsed as TOML and otherwise treated as literal strings.
- `Doc`: the CLI reference page currently describes `--config` parsing differently.
- `Inference`: this is a real doc/binary mismatch and should be kept explicit rather than silently normalized away.
- `Doc`: the config-reference feature table entry for `features.web_search_cached` appears to contain shell-tool text rather than a web-search-caching description.
- `Inference`: that looks like a docs-table defect, so this file does not treat that description as reliable semantics for the flag.

## Open questions

- `Unverified` `experimental_use_freeform_apply_patch` is present in the published schema, but this pass has not yet attached a good prose description.
- `Unverified` this file groups documented feature names and states, but does not yet provide a one-line meaning for every feature flag exposed by `codex features list`.

## Revision policy

If a factual error is found, patch this file. Do not rely on earlier chat messages as the maintained record.
