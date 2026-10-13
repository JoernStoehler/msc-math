# msc-math surface check — draft for coordinator

The assigned branch was cloned into a fresh task directory. Observed HEAD equals expected commit `2c88aaa58b63acbafc0a81c7046508051f607875`; the worktree is clean. Required README, AGENTS, runbook and cloud-workflow were read. Git, direct curl and connector source reads succeeded. This disclosed check does not accept the whole cloud workflow or its efficiency.

Independent review and human delivery remain with the coordinator. `/root/research_routes_review` is the supplied marker, not a verified local reviewer or approval. No other model session was started.

## Capability evidence

| Capability | Status | Evidence / limit |
| --- | --- | --- |
| Shell source/revision | Exposed and used successfully | Fresh shallow Git checkout; HEAD and branch match; no changes |
| Direct pinned source read | Exposed and used successfully | curl exit 0, 3767-byte runbook |
| GitHub connector source read | Exposed and used successfully | Pinned runbook, AGENTS, README and workflow |
| GitHub connector encoded branch URL | Exposed but actual attempt failed | Literal bounded error below; local Git settles revision |
| GitHub branch search | Exposed and used successfully | Exact branch found; search does not supply HEAD |
| Tool discovery | Exposed and used successfully | 318 ALL_TOOLS entries and 11 direct entries enumerated |
| Local report/ZIP | Exposed and used successfully after packaging validation | REPORT.md and RECEIPT.json only; delivery receipt remains unverified |
| Persistent ZIP upload | Exposed but untested at package freeze | Attempt follows packaging; carrier response reports outcome |
| Browser, collaboration, image, other connectors | Exposed but untested | No inference of failure or permission from exposure |
| codex_tui / codex_app tools | Not exposed in inspected interface | Documentation names are not effective tools here |
| Other backend/deferred tools | Unknown | Only current registry enumerated; irrelevant schemas not expanded |

## Actual error and limitations

```text
HTTPError: 400: GitHub Fetch URL contains an invalid repository path. (Response: None)
```

The encoded branch API URL was rejected by the connector's URL validator. This was an unintended invalid-path request, not an artificial failure test, and does not establish a source-access blocker. No shell source access failure occurred. Initial filename lookup outside the task root and a process check were unnecessarily broad; unrelated details are excluded. Two auxiliary workflow reads were partially truncated and are not claimed complete.

## Discovery and source correspondence

Inventory scope is this agent's current harness: the ALL_TOOLS metadata plus directly exposed functions, collaboration and browser-control namespaces. All current names/descriptions were enumerated without invoking every tool; only task-relevant schemas were expanded. No separate tool_search appeared. Exposure does not establish account permission. No release-matched Codex implementation was inspected, so source correspondence is **unknown for every listed tool**; unfamiliar names are not evidence of absence from source.

Measured time from the timer set inside the first orchestration call to package freeze: **117.193 seconds**. This excludes initial overhead and subsequent upload; exact end-to-end duration is not measured. No packages, producers, publication, account changes, /goal or additional sessions were used.

## Shell programs (separate from harness tools)

Only programs actually used were located; this is not a full PATH inventory. Python's standard-library zipfile supplies the archive; no zip executable is required.

| Program | Observed path |
| --- | --- |
| bash | /usr/bin/bash |
| git | /usr/local/bin/git |
| curl | /usr/bin/curl |
| rg | /opt/codex/codex-path/rg |
| cat | /usr/bin/cat |
| head | /usr/bin/head |
| ps | /usr/bin/ps |
| python3 | /opt/codex/runtimes/codex-primary-runtime/dependencies/python/bin/python3 |

## Maintained correction for coordinator integration

Record this dated surface's successful shallow checkout, pinned source reads and ZIP return route in the focused product evidence note if selected. Keep shell/Git access distinct from connector context. Preserve the encoded-branch URL rejection as a narrow validator result, rather than documenting branch access as unavailable. Do not generalize capability or efficiency from this one check. No project documentation or coordination state was edited here; the coordinator owns integration and cleanup.

## Effective harness inventory

Short descriptions below come from exposed tool metadata; they describe purpose, not tested guarantees. Status is per tool except github_fetch, which also has the failed request above. Source correspondence for all entries remains unknown.

### functions

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `functions.exec` | JavaScript orchestration, ALL_TOOLS discovery, and result forwarding. | exposed and used successfully |
| `functions.wait` | Resume a yielded orchestration cell. | exposed but untested |
| `functions.request_user_input` | Optional short user-input questions. | exposed but untested |

### collaboration

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `collaboration.spawn_agent` | Create a child agent. | exposed but untested |
| `collaboration.followup_task` | Assign follow-up work to an existing child. | exposed but untested |
| `collaboration.interrupt_agent` | Interrupt an existing agent. | exposed but untested |
| `collaboration.list_agents` | List agents in the current tree. | exposed but untested |
| `collaboration.send_message` | Send a message to an existing agent. | exposed but untested |
| `collaboration.wait_agent` | Wait for agent updates. | exposed but untested |

### mcp__cua_repl

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__cua_repl.js` | Browser UI automation through a persistent JavaScript runtime; native computer APIs disabled in this interface. | exposed but untested |
| `mcp__cua_repl.js_reset` | Reset browser-control JavaScript bindings. | exposed but untested |

### functions nested core

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `apply_patch` | The `apply_patch` tool can be used to edit files. | exposed and used successfully |
| `exec_command` | Runs a command in a PTY, returning output or a session ID for ongoing interaction. | exposed and used successfully |
| `list_mcp_resource_templates` | Lists resource templates provided by MCP servers. | exposed but untested |
| `list_mcp_resources` | Lists resources provided by MCP servers. | exposed but untested |
| `read_mcp_resource` | Read a specific resource from an MCP server given the server name and resource URI. | exposed but untested |
| `update_plan` | Updates the task plan. | exposed but untested |
| `view_image` | View a local image file from the filesystem when visual inspection is needed. | exposed but untested |
| `write_stdin` | Writes characters to an existing unified exec session and returns recent output. | exposed and used successfully |

### image_gen

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `image_gen__imagegen` | Tools in the image_gen namespace. | exposed but untested |

### mcp__codex_apps__automations

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__automations_create` | Scheduling clarification: | exposed but untested |
| `mcp__codex_apps__automations_discover_webhook_schema` | Scheduling clarification: | exposed but untested |
| `mcp__codex_apps__automations_list` | Scheduling clarification: | exposed but untested |
| `mcp__codex_apps__automations_list_event_sources` | Scheduling clarification: | exposed but untested |
| `mcp__codex_apps__automations_peek` | Scheduling clarification: | exposed but untested |
| `mcp__codex_apps__automations_run_now` | Scheduling clarification: | exposed but untested |
| `mcp__codex_apps__automations_update` | Scheduling clarification: | exposed but untested |

### mcp__codex_apps__chatgpt_space

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__chatgpt_space_attach_automation_to_page` | Attach an existing hosted automation you own to a writable Page. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_create_canvas` | Recover an earlier canvas creation after an uncertain response. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_create_controller_automation` | Create a hosted automation managed by a Page's current auto-update controller. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_create_page` | Create a text Page. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_create_page_visualization` | Upload HTML (maximum 256 KiB UTF-8) to a Page's authorized Library container, Project Files, or personal Library and insert a standalone ![title](visualize:file_id) embed. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_create_presentation` | Recover an earlier presentation creation after an uncertain response. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_create_space` | Create a new Drive Space with a new root Page when enabled for your workspace. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_create_spreadsheet` | Recover an earlier spreadsheet creation after an uncertain response. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_edit_page` | Edit an accessible Page. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_execute_artifact_code` | Edit an existing Spaces native artifact with an async JavaScript callback using the @oai/artifact-tool API. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_find_pages` | Find accessible Pages matching a query, optionally within one Space. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_get_artifact_execution_status` | Check whether an earlier execute_artifact_code request committed its changes. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_get_page_auto_update` | Read a Page's attached auto-update controller and whether you can enable or disable it. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_get_page_sharing` | Read a Page's sharing policy and your permissions. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_get_sharing_availability` | Check whether sharing management is enabled for the active account. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_get_space` | Read an accessible Drive-backed Space and its canonical metadata. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_get_space_sharing` | Call get_sharing_availability first. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_inspect_artifact` | Inspect an existing Spaces native spreadsheet, presentation, or Canvas with an async JavaScript callback using the @oai/artifact-tool API. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_inspect_page_reference` | Inspect an exact Page file reference before reading when its format or size is uncertain. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_list_page_automations` | List the canonical automations attached to an accessible Page, including its controller and workers. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_list_page_comments` | List collaborator comments and thread/message IDs for an accessible Page. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_list_pages` | List accessible Pages, optionally within a Space. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_list_spaces` | List Drive-backed Spaces available to your authenticated ChatGPT account. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_manage_page_comment` | For replies, prefer reply_page_comment. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_move_page` | Move an existing Page beneath another Page only when requested. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_patch_page` | Replace literal text in ordinary Markdown blocks. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_progress_update` | Show a short progress message at observed block IDs while working on an open Page. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_read_page` | Read an accessible Page's current content. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_read_page_changes` | Read Page changes as markdown_diff with block IDs, hashes, actors and optional comments. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_read_page_reference` | Read an exact project-file:, library-file:, or visualize: reference from an accessible Page. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_read_page_transcript` | Read the best available source transcript for a Page with namespace meeting_notes, including during recording. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_reply_page_comment` | Post a reply to an existing Page comment thread. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_run_page_auto_update` | Request a run of an enabled Page auto-update controller. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_search_sharing_recipients` | Call get_sharing_availability first. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_set_page_auto_update` | Enable or disable Page auto-update when necessary. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_trash_page` | Move a Page and its nested Pages to Trash. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_update_page_sharing` | Call get_sharing_availability first. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_update_space_sharing` | Call get_sharing_availability first. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_upload_artifact_images` | Upload images from this task's environment to an existing writable Spaces spreadsheet, presentation, or Canvas before inserting them. | exposed but untested |
| `mcp__codex_apps__chatgpt_space_write_page_reference` | Upload a selected file (up to 10 MiB) to a writable Page's canonical Files destination. | exposed but untested |

### mcp__codex_apps__codex_security_cloud

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__codex_security_cloud_defense_factory_bootstrap` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_environments_list` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_environments_search` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_findings_close` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_findings_get` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_findings_list` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_github_access` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_github_get` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_github_repositories` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_github_search` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_usage_get` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_workflow_cancel` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_workflow_definition` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_workflow_get` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_workflow_launch` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_workflow_list` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_workflow_output` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_defense_factory_workflow_repositories` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |
| `mcp__codex_apps__codex_security_cloud_open_defense_factory` | Only use for an explicitly requested Codex Security Cloud task. | exposed but untested |

### mcp__codex_apps__github

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__github_add_comment_to_issue` | Create a top-level PR Conversation comment (Issue comment). | exposed but untested |
| `mcp__codex_apps__github_add_issue_assignees` | Add assignees to an issue or pull request. | exposed but untested |
| `mcp__codex_apps__github_add_issue_labels` | Add labels to an issue or pull request. | exposed but untested |
| `mcp__codex_apps__github_add_reaction_to_issue_comment` | Add a reaction to an issue comment. | exposed but untested |
| `mcp__codex_apps__github_add_reaction_to_pr` | Add a reaction to a GitHub pull request. | exposed but untested |
| `mcp__codex_apps__github_add_reaction_to_pr_review_comment` | Add a reaction to a pull request review comment. | exposed but untested |
| `mcp__codex_apps__github_add_review_to_pr` | Add a review to a GitHub pull request. | exposed but untested |
| `mcp__codex_apps__github_compare_commits` | Compare two commits/refs and return per-file stats plus compare metadata. | exposed but untested |
| `mcp__codex_apps__github_convert_pull_request_to_draft` | Convert an open pull request back to draft state. | exposed but untested |
| `mcp__codex_apps__github_create_blob` | Create a blob in the repository and return its SHA. | exposed but untested |
| `mcp__codex_apps__github_create_branch` | Create a new branch from exactly one existing commit SHA or base ref. | exposed but untested |
| `mcp__codex_apps__github_create_commit` | Create a commit pointing to tree_sha with one or more parents. | exposed but untested |
| `mcp__codex_apps__github_create_file` | Create a new UTF-8 text file through GitHub's contents API. | exposed but untested |
| `mcp__codex_apps__github_create_issue` | Create a GitHub issue. | exposed but untested |
| `mcp__codex_apps__github_create_pull_request` | Open a pull request in the repository. | exposed but untested |
| `mcp__codex_apps__github_create_tree` | Create a tree object in the repository from the given elements. | exposed but untested |
| `mcp__codex_apps__github_delete_file` | Delete a file through GitHub's contents API. | exposed but untested |
| `mcp__codex_apps__github_dismiss_pull_request_review` | Dismiss a submitted pull request review. | exposed but untested |
| `mcp__codex_apps__github_download_user_content` | Download a GitHub private user image attachment URL. | exposed but untested |
| `mcp__codex_apps__github_download_workflow_artifact` | Download a GitHub Actions workflow artifact ZIP archive. | exposed but untested |
| `mcp__codex_apps__github_enable_auto_merge` | Enable auto-merge for a pull request. | exposed but untested |
| `mcp__codex_apps__github_fetch` | Fetch approved public GitHub repository resources and repository files. | exposed and used successfully |
| `mcp__codex_apps__github_fetch_blob` | Fetch blob content by SHA from the given repository. | exposed but untested |
| `mcp__codex_apps__github_fetch_commit` | Fetch a commit with its metadata, diff, and canonical URL. | exposed but untested |
| `mcp__codex_apps__github_fetch_commit_workflow_runs` | Fetch GitHub Actions workflow runs associated with a commit SHA. | exposed but untested |
| `mcp__codex_apps__github_fetch_file` | Fetch file content by repository path, using the default branch when ref is omitted. | exposed and used successfully |
| `mcp__codex_apps__github_fetch_issue` | Fetch a GitHub issue. | exposed but untested |
| `mcp__codex_apps__github_fetch_issue_comments` | Fetch comments for a GitHub issue across all pages. | exposed but untested |
| `mcp__codex_apps__github_fetch_pr` | Fetch a pull request with its diff, metadata, and optionally comments. | exposed but untested |
| `mcp__codex_apps__github_fetch_pr_comments` | Fetch a merged PR discussion timeline. | exposed but untested |
| `mcp__codex_apps__github_fetch_pr_file_patch` | Fetch the patch for one validated changed file in an accessible pull request. | exposed but untested |
| `mcp__codex_apps__github_fetch_pr_patch` | Fetch the patch for a GitHub pull request across all changed-file pages. | exposed but untested |
| `mcp__codex_apps__github_fetch_workflow_job_logs` | Fetch decoded logs for a GitHub Actions workflow job. | exposed but untested |
| `mcp__codex_apps__github_fetch_workflow_job_steps` | Fetch steps for a GitHub Actions workflow job. | exposed but untested |
| `mcp__codex_apps__github_fetch_workflow_run_artifacts` | Fetch artifacts for a GitHub Actions workflow run. | exposed but untested |
| `mcp__codex_apps__github_fetch_workflow_run_jobs` | Fetch jobs for a GitHub Actions workflow run. | exposed but untested |
| `mcp__codex_apps__github_get_commit_combined_status` | Fetch the combined CI status and individual status checks for a commit. | exposed but untested |
| `mcp__codex_apps__github_get_issue_comment_reactions` | Fetch reactions for an issue comment. | exposed but untested |
| `mcp__codex_apps__github_get_pr_diff` | Fetch just the diff or patch text for a pull request. | exposed but untested |
| `mcp__codex_apps__github_get_pr_info` | Get metadata (title, description, refs, and status) for a pull request. | exposed but untested |
| `mcp__codex_apps__github_get_pr_reactions` | Fetch reactions for a GitHub pull request. | exposed but untested |
| `mcp__codex_apps__github_get_pr_review_comment_reactions` | Fetch reactions for a pull request review comment. | exposed but untested |
| `mcp__codex_apps__github_get_profile` | Retrieve the GitHub profile for the authenticated user. | exposed but untested |
| `mcp__codex_apps__github_get_repo` | Retrieve metadata for a GitHub repository. | exposed but untested |
| `mcp__codex_apps__github_get_repo_collaborator_permission` | Return the collaborator permission level for a user on a repository. | exposed but untested |
| `mcp__codex_apps__github_get_user_login` | Return the GitHub login for the authenticated user. | exposed but untested |
| `mcp__codex_apps__github_get_users_recent_prs_in_repo` | List the user's recent GitHub pull requests in a repository. | exposed but untested |
| `mcp__codex_apps__github_label_pr` | Label a pull request. | exposed but untested |
| `mcp__codex_apps__github_list_installations` | List installations, optionally limited to managed setup account types. | exposed but untested |
| `mcp__codex_apps__github_list_installed_accounts` | List all accounts that the user has installed our GitHub app on. | exposed but untested |
| `mcp__codex_apps__github_list_pr_changed_filenames` | List changed filenames for a PR across all paginated file-list pages. | exposed but untested |
| `mcp__codex_apps__github_list_pull_request_review_threads` | List inline review threads on a pull request, including resolved state. | exposed but untested |
| `mcp__codex_apps__github_list_pull_request_reviews` | List review submissions on a pull request. | exposed but untested |
| `mcp__codex_apps__github_list_recent_issues` | Return the most recent GitHub issues the user can access. | exposed but untested |
| `mcp__codex_apps__github_list_repositories` | List repositories accessible to the authenticated user. | exposed but untested |
| `mcp__codex_apps__github_list_repositories_by_affiliation` | List repositories accessible to the authenticated user filtered by affiliation. | exposed but untested |
| `mcp__codex_apps__github_list_repositories_by_installation` | List repositories accessible to the authenticated user. | exposed but untested |
| `mcp__codex_apps__github_list_user_org_memberships` | List the authenticated user's organization memberships. | exposed but untested |
| `mcp__codex_apps__github_list_user_orgs` | List organizations the authenticated user is a member of. | exposed but untested |
| `mcp__codex_apps__github_lock_issue_conversation` | Lock an issue or pull request conversation. | exposed but untested |
| `mcp__codex_apps__github_mark_pull_request_ready_for_review` | Mark a draft pull request as ready for review. | exposed but untested |
| `mcp__codex_apps__github_merge_pull_request` | Merge a pull request immediately. | exposed but untested |
| `mcp__codex_apps__github_remove_issue_assignees` | Remove assignees from an issue or pull request. | exposed but untested |
| `mcp__codex_apps__github_remove_issue_label` | Remove one label from an issue or pull request. | exposed but untested |
| `mcp__codex_apps__github_remove_pull_request_reviewers` | Remove individual or team reviewer requests from a pull request. | exposed but untested |
| `mcp__codex_apps__github_remove_reaction_from_issue_comment` | Remove a reaction from an issue comment. | exposed but untested |
| `mcp__codex_apps__github_remove_reaction_from_pr` | Remove a reaction from a GitHub pull request. | exposed but untested |
| `mcp__codex_apps__github_remove_reaction_from_pr_review_comment` | Remove a reaction from a pull request review comment. | exposed but untested |
| `mcp__codex_apps__github_reply_to_review_comment` | Reply to an inline review comment on a PR (Files changed thread). | exposed but untested |
| `mcp__codex_apps__github_request_pull_request_reviewers` | Request individual or team reviewers on a pull request. | exposed but untested |
| `mcp__codex_apps__github_rerun_failed_workflow_run_jobs` | Re-run all failed jobs in a GitHub Actions workflow run. | exposed but untested |
| `mcp__codex_apps__github_rerun_workflow_job` | Re-run one GitHub Actions workflow job. | exposed but untested |
| `mcp__codex_apps__github_resolve_review_thread` | Resolve an inline pull request review thread. | exposed but untested |
| `mcp__codex_apps__github_search` | Search GitHub files and return matching excerpts when available. | exposed but untested |
| `mcp__codex_apps__github_search_branches` | Search GitHub branches within a repository. | exposed and used successfully |
| `mcp__codex_apps__github_search_commits` | Search GitHub commits globally, by organization, or optionally by repository. | exposed but untested |
| `mcp__codex_apps__github_search_installed_repositories_streaming` | Search for a repository (not a file) by name or description. | exposed but untested |
| `mcp__codex_apps__github_search_installed_repositories_v2` | Search repositories within the user's installations using GitHub search. | exposed but untested |
| `mcp__codex_apps__github_search_issues` | Search one repository or every repository the linked account can access. | exposed but untested |
| `mcp__codex_apps__github_search_prs` | Search GitHub pull requests globally, by organization, or optionally by repository. | exposed but untested |
| `mcp__codex_apps__github_search_repositories` | Search for a repository (not a file) by name or description. | exposed but untested |
| `mcp__codex_apps__github_unlock_issue_conversation` | Unlock an issue or pull request conversation. | exposed but untested |
| `mcp__codex_apps__github_unresolve_review_thread` | Mark an inline pull request review thread as unresolved. | exposed but untested |
| `mcp__codex_apps__github_update_file` | Replace a UTF-8 text file through GitHub's contents API. | exposed but untested |
| `mcp__codex_apps__github_update_issue` | Update a GitHub issue, including title/body, state, labels, assignees, or milestone. | exposed but untested |
| `mcp__codex_apps__github_update_issue_comment` | Update a top-level PR Conversation comment (Issue comment). | exposed but untested |
| `mcp__codex_apps__github_update_pull_request` | Update PR metadata, base branch, or open/closed state. | exposed but untested |
| `mcp__codex_apps__github_update_ref` | Move a branch ref to a commit, rejecting a mismatched expected_sha. | exposed but untested |
| `mcp__codex_apps__github_update_review_comment` | Update an inline review comment (or a reply) on a PR. | exposed but untested |

### mcp__codex_apps__gmail

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__gmail_apply_labels_to_emails` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_archive_emails` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_batch_modify_email` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_batch_read_email` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_batch_read_email_threads` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_bulk_label_matching_emails` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_create_draft` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_create_label` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_delete_emails` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_forward_emails` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_get_profile` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_list_drafts` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_list_labels` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_read_attachment` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_read_email` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_read_email_thread` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_search_email_ids` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_search_emails` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_send_draft` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_send_email` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |
| `mcp__codex_apps__gmail_update_draft` | Gmail tools for label counts, searching and reading emails/threads/attachments, reviewing drafts, and explicit mail changes like send, draft, forward, archive, Trash, and label actions. | exposed but untested |

### mcp__codex_apps__google_calendar

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__google_calendar_batch_read_event` | Read multiple Google Calendar events by ID. | exposed but untested |
| `mcp__codex_apps__google_calendar_create_event` | Create a new Google Calendar event and return its details. | exposed but untested |
| `mcp__codex_apps__google_calendar_delete_event` | Remove a Google Calendar event. | exposed but untested |
| `mcp__codex_apps__google_calendar_fetch` | Get details for a single Google Calendar event. | exposed but untested |
| `mcp__codex_apps__google_calendar_get_availability` | Look up busy windows on one or more calendars before scheduling a meeting. | exposed but untested |
| `mcp__codex_apps__google_calendar_get_colors` | Return Google Calendar calendar and event color palettes. | exposed but untested |
| `mcp__codex_apps__google_calendar_get_profile` | Return the current Google Calendar user's profile information. | exposed but untested |
| `mcp__codex_apps__google_calendar_list_calendars` | List calendars visible to the authenticated user. | exposed but untested |
| `mcp__codex_apps__google_calendar_list_event_labels` | List named event labels defined on the requested calendar. | exposed but untested |
| `mcp__codex_apps__google_calendar_read_event` | Read a Google Calendar event by ID. | exposed but untested |
| `mcp__codex_apps__google_calendar_respond_event` | Respond to a Google Calendar event invitation on behalf of the authenticated user. | exposed but untested |
| `mcp__codex_apps__google_calendar_search` | Search Google Calendar events within a time window. | exposed but untested |
| `mcp__codex_apps__google_calendar_search_events` | Look up Google Calendar events using various filters. | exposed but untested |
| `mcp__codex_apps__google_calendar_set_event_label_silently` | Set only a primary-calendar event's private label without notifying attendees. | exposed but untested |
| `mcp__codex_apps__google_calendar_update_event` | Update an existing Google Calendar event. | exposed but untested |

### mcp__codex_apps__google_contacts

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__google_contacts_get_profile` | Return the authenticated Google account profile. | exposed but untested |
| `mcp__codex_apps__google_contacts_read_contact` | Read one contact by resource ID. | exposed but untested |
| `mcp__codex_apps__google_contacts_search_contacts` | Search Google Contacts and directory entries matching ``query``. | exposed but untested |

### mcp__codex_apps__google_drive

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__google_drive_batch_update_document` | Apply raw Google Docs batchUpdate requests to document content, not Drive file metadata. | exposed but untested |
| `mcp__codex_apps__google_drive_batch_update_presentation` | Apply raw Google Slides batchUpdate requests to presentation content, not Drive file metadata. | exposed but untested |
| `mcp__codex_apps__google_drive_batch_update_spreadsheet` | Apply raw Google Sheets batchUpdate requests to spreadsheet content, not Drive file metadata. | exposed but untested |
| `mcp__codex_apps__google_drive_bulk_update_file_comments` | Create, reply to, and resolve Drive file comments in one bulk tool call. | exposed but untested |
| `mcp__codex_apps__google_drive_copy_file` | Copy a Drive file and return the URL of the new copy. | exposed but untested |
| `mcp__codex_apps__google_drive_create_file` | Create a native Google Doc, Sheet, or Slide file. | exposed but untested |
| `mcp__codex_apps__google_drive_create_folder` | Create a folder in Google Drive, optionally under a parent folder. | exposed but untested |
| `mcp__codex_apps__google_drive_create_presentation_from_template` | Copy a Google Slides template to create a new deck. | exposed but untested |
| `mcp__codex_apps__google_drive_delete_file` | Permanently delete a Drive file. | exposed but untested |
| `mcp__codex_apps__google_drive_duplicate_sheet_in_new_spreadsheet` | Duplicate an existing sheet into a newly created spreadsheet file. | exposed but untested |
| `mcp__codex_apps__google_drive_export_file` | Export a native Google Doc, Sheet, or Slide to the requested MIME type. | exposed but untested |
| `mcp__codex_apps__google_drive_fetch` | With default options, return readable file text. | exposed but untested |
| `mcp__codex_apps__google_drive_fetch_file_revision` | Fetch text and revision-level author metadata from one Drive revision. | exposed but untested |
| `mcp__codex_apps__google_drive_find_document_text_range` | Find the index range of an exact text match in a Google Doc. | exposed but untested |
| `mcp__codex_apps__google_drive_get_document` | Get a native Google Doc, including tab content. | exposed but untested |
| `mcp__codex_apps__google_drive_get_document_comments` | Read user comments and replies on a Google Doc for additional review context. | exposed but untested |
| `mcp__codex_apps__google_drive_get_document_paragraph_range` | Resolve the paragraph range containing a given document index. | exposed but untested |
| `mcp__codex_apps__google_drive_get_document_tables` | Return table structures and cell text from a Google Doc. | exposed but untested |
| `mcp__codex_apps__google_drive_get_document_text` | Return text and indexes from a native Google Doc. | exposed but untested |
| `mcp__codex_apps__google_drive_get_file_comments` | Read comments and replies on an arbitrary Drive file. | exposed but untested |
| `mcp__codex_apps__google_drive_get_file_metadata` | Return metadata for a Google Drive file or folder without downloading contents. | exposed but untested |
| `mcp__codex_apps__google_drive_get_presentation` | Get a native Google Slides presentation. | exposed but untested |
| `mcp__codex_apps__google_drive_get_presentation_comments` | Read user comments and replies on a Google Slides deck for additional review context. | exposed but untested |
| `mcp__codex_apps__google_drive_get_presentation_outline` | Return a compact slide outline for stable slide targeting. | exposed but untested |
| `mcp__codex_apps__google_drive_get_presentation_tables` | Return Google Slides table structures with row and column coordinates preserved. | exposed but untested |
| `mcp__codex_apps__google_drive_get_presentation_text` | Get text from a native Google Slides presentation. | exposed but untested |
| `mcp__codex_apps__google_drive_get_profile` | Return the current Google Drive user's profile information. | exposed but untested |
| `mcp__codex_apps__google_drive_get_slide` | Get a single slide by object ID. | exposed but untested |
| `mcp__codex_apps__google_drive_get_slide_thumbnail` | Return slide metadata plus an inline thumbnail image for visual layout questions. | exposed but untested |
| `mcp__codex_apps__google_drive_get_spreadsheet_cells` | Read CellData from bounded native Google Sheets ranges. | exposed but untested |
| `mcp__codex_apps__google_drive_get_spreadsheet_comments` | Read user comments and replies on a Google Sheets spreadsheet for additional review context. | exposed but untested |
| `mcp__codex_apps__google_drive_get_spreadsheet_metadata` | Get metadata for a native Google Sheet. | exposed but untested |
| `mcp__codex_apps__google_drive_get_spreadsheet_range` | Read plain cell values from a native Google Sheet. | exposed but untested |
| `mcp__codex_apps__google_drive_import_document` | Upload a local DOC/DOCX/ODT/RTF/HTML/TXT file to Drive, defaulting to native Google Docs. | exposed but untested |
| `mcp__codex_apps__google_drive_import_presentation` | Upload a local PPT/PPTX/ODP file to Drive, defaulting to native Google Slides. | exposed but untested |
| `mcp__codex_apps__google_drive_import_spreadsheet` | Upload a spreadsheet file to Drive, defaulting to native Google Sheets conversion. | exposed but untested |
| `mcp__codex_apps__google_drive_list_drives` | List shared drives accessible to the user. | exposed but untested |
| `mcp__codex_apps__google_drive_list_file_revisions` | List version-history revisions for a Google Drive file. | exposed but untested |
| `mcp__codex_apps__google_drive_list_folder` | List the items directly contained in a Google Drive folder. | exposed but untested |
| `mcp__codex_apps__google_drive_recent_documents` | Return the most recently modified documents accessible to the user. | exposed but untested |
| `mcp__codex_apps__google_drive_search` | Search Google Drive and return file or folder metadata. | exposed but untested |
| `mcp__codex_apps__google_drive_search_spreadsheet_rows` | Search a native Google Sheet's existing cell bounds. | exposed but untested |
| `mcp__codex_apps__google_drive_share_file` | Share a Drive file with a user or anyone at the company. | exposed but untested |
| `mcp__codex_apps__google_drive_update_file` | Update an existing Drive file. | exposed but untested |
| `mcp__codex_apps__google_drive_upload_file` | Upload a file reference as a new Google Drive file, not rename or move an existing file. | exposed but untested |

### mcp__codex_apps__hotline

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__hotline_get_local_hotline` | Look up local helpline information for the user based on country inferred from the conversation. | exposed but untested |

### mcp__codex_apps__library

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__library_create_library_file` | Create persistent ChatGPT Library files from local Codex files. | exposed but untested at package freeze; upload result belongs to carrier receipt |
| `mcp__codex_apps__library_finalize_uploads` | Finalize a batch of completed upload sessions into ChatGPT Library create or replace writes. | exposed but untested |
| `mcp__codex_apps__library_find` | Find Library text. | exposed but untested |
| `mcp__codex_apps__library_list` | List persistent ChatGPT Library file and folder metadata. | exposed but untested |
| `mcp__codex_apps__library_manage_library` | Mutate persistent ChatGPT Library state. | exposed but untested |
| `mcp__codex_apps__library_prepare_materialize` | Copy known Library files into the model workspace for programmatic use. | exposed but untested |
| `mcp__codex_apps__library_prepare_uploads` | Prepare local files that will become new Library items or versions of existing items. | exposed but untested |
| `mcp__codex_apps__library_read` | Read Library. | exposed but untested |
| `mcp__codex_apps__library_replace_library_file` | Replace an existing ChatGPT Library file with a local Codex file. | exposed but untested |
| `mcp__codex_apps__library_search` | Search Library. | exposed but untested |
| `mcp__codex_apps__library_share` | Share one native ChatGPT Library file or folder you own or can edit with one eligible recipient by email or everyone in the current workspace, or set revoke=true to remove an existing share. | exposed but untested |

### mcp__codex_apps__personal_context

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__personal_context_search` | The personal_context tool retrieves user-specific personal context gathered from multiple underlying sources (e.g., linked accounts, prior interactions, and other personal context streams). | exposed but untested |

### mcp__codex_apps__pets

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__pets_adopt` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |
| `mcp__codex_apps__pets_create_pet` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |
| `mcp__codex_apps__pets_delete_pet` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |
| `mcp__codex_apps__pets_get_pet_download_link` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |
| `mcp__codex_apps__pets_list_pets` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |
| `mcp__codex_apps__pets_prepare_pet_upload` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |
| `mcp__codex_apps__pets_select_pet` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |
| `mcp__codex_apps__pets_share_pet` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |
| `mcp__codex_apps__pets_unshare_pet` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |
| `mcp__codex_apps__pets_update_pet` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |
| `mcp__codex_apps__pets_validate_pet_spritesheet` | Create and manage the user's animated companion pets inside ChatGPT Work mode. | exposed but untested |

### mcp__codex_apps__plugin_creator

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__plugin_creator_create_plugin` | Use create_plugin to create a PRIVATE plugin in the authenticated user's active workspace, or a personal plugin without an active workspace. | exposed but untested |
| `mcp__codex_apps__plugin_creator_get_owned_plugin_archive` | Use create_plugin to create a PRIVATE plugin in the authenticated user's active workspace, or a personal plugin without an active workspace. | exposed but untested |
| `mcp__codex_apps__plugin_creator_get_plugin_files` | Use create_plugin to create a PRIVATE plugin in the authenticated user's active workspace, or a personal plugin without an active workspace. | exposed but untested |
| `mcp__codex_apps__plugin_creator_get_plugin_metadata` | Use create_plugin to create a PRIVATE plugin in the authenticated user's active workspace, or a personal plugin without an active workspace. | exposed but untested |
| `mcp__codex_apps__plugin_creator_list_owned_personal_plugins` | Use create_plugin to create a PRIVATE plugin in the authenticated user's active workspace, or a personal plugin without an active workspace. | exposed but untested |
| `mcp__codex_apps__plugin_creator_list_plugin_releases` | Use create_plugin to create a PRIVATE plugin in the authenticated user's active workspace, or a personal plugin without an active workspace. | exposed but untested |
| `mcp__codex_apps__plugin_creator_update_plugin` | Use create_plugin to create a PRIVATE plugin in the authenticated user's active workspace, or a personal plugin without an active workspace. | exposed but untested |

### mcp__codex_apps__plugin_management

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__plugin_management_get_app_permissions` | Manage plugins, settings, permissions, and connections. | exposed but untested |
| `mcp__codex_apps__plugin_management_get_plugin_dependencies` | Manage plugins, settings, permissions, and connections. | exposed but untested |
| `mcp__codex_apps__plugin_management_search_plugins` | Manage plugins, settings, permissions, and connections. | exposed but untested |
| `mcp__codex_apps__plugin_management_suggest_plugins` | Manage plugins, settings, permissions, and connections. | exposed but untested |
| `mcp__codex_apps__plugin_management_uninstall_app` | Manage plugins, settings, permissions, and connections. | exposed but untested |
| `mcp__codex_apps__plugin_management_update_app_permissions` | Manage plugins, settings, permissions, and connections. | exposed but untested |

### mcp__codex_apps__safety_settings

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__safety_settings_get_family_info` | For ChatGPT Parental Controls (your child or teen's settings, features, Study Mode, quiet hours, family setup) and Trusted Contact (setup, status, privacy). | exposed but untested |
| `mcp__codex_apps__safety_settings_get_parental_controls` | For ChatGPT Parental Controls (your child or teen's settings, features, Study Mode, quiet hours, family setup) and Trusted Contact (setup, status, privacy). | exposed but untested |
| `mcp__codex_apps__safety_settings_get_trusted_contact` | For ChatGPT Parental Controls (your child or teen's settings, features, Study Mode, quiet hours, family setup) and Trusted Contact (setup, status, privacy). | exposed but untested |
| `mcp__codex_apps__safety_settings_prepare_parental_control_update` | For ChatGPT Parental Controls (your child or teen's settings, features, Study Mode, quiet hours, family setup) and Trusted Contact (setup, status, privacy). | exposed but untested |
| `mcp__codex_apps__safety_settings_update_parental_control` | For ChatGPT Parental Controls (your child or teen's settings, features, Study Mode, quiet hours, family setup) and Trusted Contact (setup, status, privacy). | exposed but untested |

### mcp__codex_apps__search_service

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__search_service_web_run` | Tool for accessing the internet. | exposed but untested |

### mcp__codex_apps__sites

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `mcp__codex_apps__sites_add_custom_domain` | Add a custom domain to a published site. | exposed but untested |
| `mcp__codex_apps__sites_change_site_slug` | Change a site's public URL label. | exposed but untested |
| `mcp__codex_apps__sites_create_schedule` | Create and link a cloud task for a published Site when the user has requested scheduling. | exposed but untested |
| `mcp__codex_apps__sites_create_site` | Create a site only when .openai/hosting.json has no project_id. | exposed but untested |
| `mcp__codex_apps__sites_create_source_repository_write_credential` | Create a short-lived source repository write credential when the credential returned by create_site is missing or no longer usable. | exposed but untested |
| `mcp__codex_apps__sites_deploy_private_site_version` | Deploy a saved site version to production for a site created in the current flow whose owner-only access has not changed, or an existing site already known to be owner-private for the selected account. | exposed but untested |
| `mcp__codex_apps__sites_deploy_site_version` | Deploy a saved site version to production when the site is shared, public, cannot be verified as owner-only, or private deployment is unavailable. | exposed but untested |
| `mcp__codex_apps__sites_generate_siwc_bypass_token` | Generate a bearer token for identity-less API requests that bypasses a site's Sign in with ChatGPT gate. | exposed but untested |
| `mcp__codex_apps__sites_get_deployment_status` | Get the current status of a production deployment. | exposed but untested |
| `mcp__codex_apps__sites_get_environment_variables` | Get the production runtime environment variables for a site. | exposed but untested |
| `mcp__codex_apps__sites_get_site` | Get a site and its current access configuration, including external visitors. | exposed but untested |
| `mcp__codex_apps__sites_get_site_version` | Get a saved site version and its source provenance. | exposed but untested |
| `mcp__codex_apps__sites_get_site_worker_logs` | Read recent production Cloudflare Worker logs for a Site when diagnosing why a deployed website is crashing, returning an error, or failing after a click or tap. | exposed but untested |
| `mcp__codex_apps__sites_list_custom_domains` | List custom domains attached to a site. | exposed but untested |
| `mcp__codex_apps__sites_list_site_versions` | List saved site versions in newest-first order for history, deployment, or rollback selection. | exposed but untested |
| `mcp__codex_apps__sites_list_sites` | List Sites you own in the selected account, including personal accounts. | exposed but untested |
| `mcp__codex_apps__sites_read_database_overview` | Inspect the user tables in a deployed site's live Cloudflare D1 database before reading rows. | exposed but untested |
| `mcp__codex_apps__sites_read_database_table_rows` | Read one bounded page of rows from a user table in a deployed site's live Cloudflare D1 database. | exposed but untested |
| `mcp__codex_apps__sites_refresh_custom_domain_status` | Refresh custom domain validation status for a site. | exposed but untested |
| `mcp__codex_apps__sites_remove_custom_domain` | Remove a custom domain from a site. | exposed but untested |
| `mcp__codex_apps__sites_save_site_version` | Save a version of the site's pushed source without deploying it. | exposed but untested |
| `mcp__codex_apps__sites_save_version_and_deploy_private` | For a site created in the current flow whose owner-only access has not changed, or an existing site already known to be owner-private for the selected account, use this instead of save_site_version followed by deploy_private_site_version. | exposed but untested |
| `mcp__codex_apps__sites_update_environment_variables` | Update production runtime environment variables for a site. | exposed but untested |
| `mcp__codex_apps__sites_update_site_access` | Update who can visit a site only when the user asks to change access. | exposed but untested |
| `mcp__codex_apps__sites_update_site_metadata` | Update a site's display title. | exposed but untested |

### skills

| Actual tool name | Short description | Status |
| --- | --- | --- |
| `skills__list` | Tools in the skills namespace. | exposed but untested |
| `skills__read` | Tools in the skills namespace. | exposed and used successfully |

## Remaining unknowns

- Recipient download/extraction and coordinator review
- Tool availability/permissions and reliability beyond safe source/report probes
- Unexpanded irrelevant schemas and backend tools outside registry
- Precise model ID, reasoning effort, cost and usage
- Persistence of checkout/dependencies after inactivity
- Release-matched source correspondence
- Whole cloud workflow correctness or efficiency

