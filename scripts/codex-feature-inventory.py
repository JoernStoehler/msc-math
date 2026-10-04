#!/usr/bin/env python3
"""Generate a release-matched flag inventory without model calls or config edits."""
import argparse
import collections
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]

# Release-specific first-pass project screening, not a runtime validation.
# Unknown flags stay visibly unreviewed after upgrades rather than inheriting
# a recommendation from their maturity stage.
REVIEW_GROUPS = {
    'time/context': ('Consider separately; reminders, context tools and hard stopping have different consequences.', '''
        goals token_budget context_management rollout_budget reasoning_effort_override
        current_time_reminder nonfatal_clock_read_errors retain_client_developer_messages
        compaction_image_budget'''),
    'orchestration': ('Keep selected controls; evaluate mailbox/board changes against a concrete coordination problem.', '''
        multi_agent multi_agent_v2 defer_mailbox_preemption agent_message_board'''),
    'execution': ('Preserve current execution; select a bounded trial only for an observed tool or shell problem.', '''
        shell_tool view_image sleep_tool unified_exec unified_exec_tty shell_zsh_fork
        shell_snapshot powershell_shell_version shell_snapshot_v2 deferred_executor
        code_mode code_mode_host code_mode_prewarm code_mode_interrupt instant_interrupt
        code_mode_only apply_patch_streaming_events apply_patch_preserve_line_endings'''),
    'permissions/review': ('No selected permission-policy change; runtime tool approval does not implement project-config edit approval.', '''
        exec_permission_approvals write_stdin_approval request_permissions_tool
        guardian_approval guardian_reuse_parent_compaction guardian_root_handoff_context
        guardian_enhanced_node_repl_transcripts guardian_node_repl_transcript_images
        guardian_conversation_history_tools guardianv2 tool_call_mcp_elicitation'''),
    'host/platform': ('Host owner; select transport/auth/platform changes only for a demonstrated need.', '''
        daemon_auto_start secret_auth_storage windows_sandbox_service prefer_mxc
        api_key_model_discovery enable_request_compression unbounded_connection_retries
        network_proxy respect_system_proxy system_proxy_fallback psp mcp_oauth_refresh_coordination
        use_xaa auth_elicitation bedrock_setup_wizard use_agent_identity'''),
    'storage/memory': ('Separate retention, compatibility and resource decision; do not activate for prompt inspection alone.', '''
        memories external_agent_memory_import local_thread_store_compression
        background_paginated_rollout_migration chronicle'''),
    'tools/discovery': ('Preserve selected integrations; namespace/deferred inventory is relevant to the handed-off prompt view.', '''
        hooks worktrees apps enable_mcp_apps mcp_2026_07_28 codex_apps_mcp_2026_07_28
        deferred_tool_world_state non_prefixed_mcp_tool_names tool_suggest recommended_plugins
        plugins executor_capability_discovery skip_host_skill_discovery remote_plugin
        plugin_sharing skill_mcp_dependency_install skill_search workspace_dependencies
        standalone_web_search image_generation artifact'''),
    'ui/observability': ('UI/telemetry or presentation choice; no evidence that it repairs the reported communication failure.', '''
        analytics_plan_history cwd_relative_turn_diffs content_item_kinds executed_tool_call_metadata
        runtime_metrics omit_app_server_notification_media image_resize_notice unified_image_budget
        concurrent_reasoning_summaries mentions_v2 default_mode_request_user_input
        send_message_to_user_async terminal_visualization_instructions fast_mode step_model_switching
        realtime_conversation prevent_idle_sleep'''),
    'requirements': ('Requirements-only capability gates; do not set these as project user preferences.', '''
        in_app_browser in_app_chat in_app_dictation in_app_local_automation in_app_updates
        browser_use browser_use_full_cdp_access browser_use_external computer_use'''),
}


def screening(key, stage):
    if stage in {'removed', 'deprecated'}:
        return 'legacy', 'Inspect replacement/compatibility behavior; a removed flag can still be advertised or compose with other flags. Do not add it as a new control.'
    for group, (disposition, names) in REVIEW_GROUPS.items():
        if key in names.split():
            return group, disposition
    return 'unreviewed', 'New/unmapped flag: inspect its consumer before recommending a project choice.'


def command(*args, cwd=ROOT):
    return subprocess.check_output(args, cwd=cwd, text=True).strip()


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None


def registry(source):
    path = source / 'codex-rs/features/src/lib.rs'
    text = path.read_text()
    enum = text.split('pub enum Feature {', 1)[1].split('\n}', 1)[0]
    descriptions = {}
    comments = []
    for line in enum.splitlines():
        if line.strip().startswith('///'):
            comments.append(line.strip()[3:].strip())
        elif match := re.fullmatch(r'\s*(\w+),\s*', line):
            descriptions[match[1]] = ' '.join(comments)
            comments = []
        elif line.strip() and not line.strip().startswith('#['):
            comments = []
    records = {}
    pattern = re.compile(
        r'FeatureSpec\s*\{\s*id:\s*Feature::(\w+),\s*'
        r'key:\s*"([^"]+)",\s*stage:\s*'
        r'((?:(?!FeatureSpec).)*?)default_enabled:\s*([^,\n]+),\s*\}', re.S)
    for match in pattern.finditer(text):
        variant, key, stage_expression, default = match.groups()
        stage_match = re.match(r'Stage::(\w+)', stage_expression)
        stage = stage_match[1] if stage_match else 'Conditional; use CLI stage'
        records[key] = {
            'variant': variant, 'stage': stage,
            'default': (default == 'true') if default in {'true', 'false'} else default,
            'description': descriptions.get(variant, ''),
            'source_line': text.count('\n', 0, match.start()) + 1,
            'consumer_files': [],
        }
    if not records:
        raise RuntimeError('Feature registry format changed; update parser rather than emit an empty inventory.')
    variants = {record['variant']: record for record in records.values()}
    for path in sorted((source / 'codex-rs').rglob('*.rs')):
        relative = path.relative_to(source).as_posix()
        if '/tests/' in relative or 'test' in path.name or path == source / 'codex-rs/features/src/lib.rs':
            continue
        content = path.read_text(errors='replace')
        for variant in set(re.findall(r'Feature::(\w+)', content)):
            if variant in variants:
                variants[variant]['consumer_files'].append(relative)
    return records


def blame_rows():
    rows = []
    row = {}
    for line in command('git', 'blame', '--line-porcelain', '--', '.codex/config.toml').splitlines():
        if re.fullmatch(r'[0-9a-f]{40} \d+ \d+(?: \d+)?', line):
            row = {'commit': line.split()[0]}
        elif line.startswith('author-time '):
            row['changed_at'] = datetime.datetime.fromtimestamp(int(line.split()[1]), datetime.timezone.utc).isoformat()
        elif line.startswith('summary '):
            row['summary'] = line[8:]
        elif line.startswith('\t'):
            rows.append({**row, 'text': line[1:]})
    return rows


def escape(value):
    return str(value).replace('|', '\\|').replace('\n', ' ')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-root', type=Path, default=Path(os.environ.get('CODEX_SOURCE_DIR', str(Path(os.environ.get('XDG_CACHE_HOME', str(Path.home() / '.cache'))) / 'codex-source'))))
    parser.add_argument('--output-dir', type=Path, default=ROOT / 'docs')
    parser.add_argument('--no-refresh', action='store_true', help='Use already-refreshed source; still require the installed release commit.')
    parser.add_argument('--check', action='store_true', help='Check existing snapshot freshness without fetching or writing.')
    args = parser.parse_args()
    source = args.source_root.resolve()
    if not args.no_refresh and not args.check:
        env = dict(os.environ, CODEX_SOURCE_DIR=str(source))
        subprocess.run(['bash', str(ROOT / 'scripts/update-codex-source.sh')], env=env, check=True)
    version = command('codex', '--version')
    release = 'rust-v' + version.removeprefix('codex-cli ')
    commit = command('git', 'rev-parse', 'HEAD', cwd=source)
    expected = command('git', 'rev-parse', release + '^{commit}', cwd=source)
    if commit != expected:
        raise RuntimeError('Source differs from installed CLI release; run the source refresh helper first.')
    if command('git', 'status', '--porcelain', cwd=source):
        raise RuntimeError('Source checkout is dirty; refuse to describe local modifications as release behavior.')
    daemon = json.loads(command('codex', 'app-server', 'daemon', 'version'))
    features_output = command('codex', 'features', 'list')
    live = {}
    for line in features_output.splitlines():
        match = re.fullmatch(r'(\S+)\s+(stable|experimental|under development|deprecated|removed)\s+(true|false)', line)
        if not match:
            raise RuntimeError('CLI feature-list format changed: ' + line)
        live[match[1]] = {'cli_stage': match[2], 'enabled': match[3] == 'true'}
    provenance_rows = blame_rows()
    # Git assigns a fresh author-time to uncommitted lines on each invocation.
    # Retain their text/commit identity without making every check falsely stale.
    provenance_identity = [
        {'commit': row['commit'], 'text': row['text']}
        for row in provenance_rows
    ]
    fingerprints = {
        'cli': version, 'source_commit': commit,
        'daemon_version': daemon.get('appServerVersion'),
        'registry_sha256': digest(source / 'codex-rs/features/src/lib.rs'),
        'feature_config_sha256': digest(source / 'codex-rs/features/src/feature_configs.rs'),
        'project_config_sha256': digest(ROOT / '.codex/config.toml'),
        'host_config_sha256': digest(Path.home() / '.codex/config.toml'),
        'effective_flags_sha256': hashlib.sha256(features_output.encode()).hexdigest(),
        'generator_sha256': digest(Path(__file__)),
        'project_provenance_sha256': hashlib.sha256(json.dumps(provenance_identity, sort_keys=True).encode()).hexdigest(),
    }
    out_json = args.output_dir / 'codex-features.json'
    out_md = args.output_dir / 'codex-features.md'
    if args.check:
        old = json.loads(out_json.read_text())
        changed = [key for key, value in fingerprints.items() if old['fingerprints'].get(key) != value]
        if changed:
            raise RuntimeError('Snapshot stale: ' + ', '.join(changed) + '. Rerun this script.')
        if digest(out_md) != old.get('markdown_sha256'):
            raise RuntimeError('Generated Markdown differs from snapshot; rerun this script.')
        print('PASS: feature snapshot matches CLI, daemon version, source, configs and generator.')
        return
    features = registry(source)
    if set(features) != set(live):
        raise RuntimeError('CLI/source flag mismatch: ' + str(sorted(set(features) ^ set(live))))
    config = tomllib.loads((ROOT / '.codex/config.toml').read_text())
    for key, record in features.items():
        record.update(live[key])
        record['project_override'] = config.get('features', {}).get(key)
        record['review_group'], record['review_disposition'] = screening(key, record['cli_stage'])
    stamp = datetime.datetime.now(datetime.timezone.utc).isoformat()
    prefix = f'https://github.com/openai/codex/blob/{commit}/'
    stage_counts = collections.Counter(record['cli_stage'] for record in features.values())
    lines = ['# Generated Codex feature inventory', '',
             f'Captured {stamp}; {version}; daemon {daemon.get("appServerVersion")}; source `{commit}`.', '',
             'Regenerate: `python3 scripts/codex-feature-inventory.py`.',
             'Check freshness without fetching: `python3 scripts/codex-feature-inventory.py --check`.', '',
             'This is a generated snapshot of fresh CLI flag resolution, not an active thread prompt or a recommendation to enable every flag. Stage alone is not a usefulness verdict. Consumer candidates are code references, not proof that a path is exercised. Regeneration uses the installed CLI and requires its matching source; no model generation or configuration edit.', '',
             'Counts: ' + ', '.join(f'{key}: {value}' for key, value in sorted(stage_counts.items())) + '.', '',
             'Project review groups are a first-pass screen of registry descriptions. They do not establish every runtime consumer or behavior. New flags are marked **unreviewed** until mapped; the maintained reference contains the deeper findings.', '',
             '| Review group | Flags | Disposition |', '| --- | --- | --- |']
    grouped = collections.Counter(record['review_group'] for record in features.values())
    for group, count in sorted(grouped.items()):
        disposition = next(record['review_disposition'] for record in features.values() if record['review_group'] == group)
        lines.append(f'| {group} | {count} | {disposition} |')
    lines += ['', '| Flag | Stage | Default | CLI value | Project override | Review group | Source description | Consumer candidates |',
              '| --- | --- | --- | --- | --- | --- | --- | --- |']
    for key, record in sorted(features.items()):
        label = f'[{key}]({prefix}codex-rs/features/src/lib.rs#L{record["source_line"]})'
        consumers = record['consumer_files']
        displayed = ', '.join(f'[{Path(path).parent.name}/{Path(path).name}]({prefix}{path})' for path in consumers[:3]) or 'No direct reference found'
        if len(consumers) > 3:
            displayed += f' (+{len(consumers)-3} in JSON)'
        override = record['project_override']
        override_text = str(override).lower() if isinstance(override, bool) else escape(override)
        lines.append('| ' + ' | '.join([label, record['cli_stage'], str(record['default']).lower(), str(record['enabled']).lower(), override_text if override is not None else 'inherit', record['review_group'], escape(record['description'] or 'No enum description'), displayed]) + ' |')
    lines += ['', '## Project setting provenance', '',
              'Git identifies the last change to a committed line, not the Codex version running when it was written. Uncommitted lines are identified explicitly. The existing project overrides were assessed against 0.160.0 in docs/codex.md; historical runtime versions remain unknown unless their sources recorded them.', '',
              '| Line | Setting | Last change | Recorded runtime at that change |',
              '| --- | --- | --- | --- |']
    provenance = []
    for number, row in enumerate(provenance_rows, 1):
        if not re.match(r'^\s*[\w.-]+\s*=', row['text']):
            continue
        working = row['commit'] == '0' * 40
        change = 'working tree' if working else f'{row["commit"][:10]} · {row.get("changed_at", "unknown")} · {row.get("summary", "")}'
        provenance.append({'line': number, **row, 'runtime_version_at_change': None})
        lines.append(f'| {number} | `{escape(row["text"])}` | {escape(change)} | unknown; see dated assessment |')
    lines += ['', 'Interpretation and candidate changes belong to [the maintained reference](codex.md).', '']
    markdown = '\n'.join(lines)
    payload = {'captured_at': stamp, 'fingerprints': fingerprints,
               'features': features, 'project_line_provenance': provenance,
               'markdown_sha256': hashlib.sha256(markdown.encode()).hexdigest()}
    args.output_dir.mkdir(parents=True, exist_ok=True)
    out_md.write_text(markdown)
    out_json.write_text(json.dumps(payload, ensure_ascii=False, indent=2) + '\n')
    print(f'Generated {len(features)} flags: {out_md} and {out_json}')


if __name__ == '__main__':
    try:
        main()
    except (OSError, ValueError, RuntimeError, subprocess.CalledProcessError) as error:
        print(str(error), file=sys.stderr)
        sys.exit(1)
