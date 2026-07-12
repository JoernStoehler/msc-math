import importlib.util
import json
import os
import subprocess
import sys
from pathlib import Path

SCRIPT = Path(__file__).parents[1] / "scripts" / "collect_process_events.py"
spec = importlib.util.spec_from_file_location("collector", SCRIPT)
c = importlib.util.module_from_spec(spec)
spec.loader.exec_module(c)
KEY = b"test-secret-key"


def write(path, rows):
    path.write_text("\n".join(json.dumps(x) for x in rows) + "\n")


def actions(command, cwd=None):
    return c.symbolic_actions(command, KEY, cwd)[0]


def test_hmac_domain_separation_and_typed_lifecycle():
    assert c.pseudonym(KEY, "session", "x") != c.pseudonym(KEY, "branch", "x")
    assert c.pseudonym(KEY, "session", "x") != c.pseudonym(b"other", "session", "x")
    lifecycle = actions(
        "git -C /repo worktree add -b topic /tree main; git -C/repo worktree remove --force /tree; git worktree prune"
    )
    assert [a["action"] for a in lifecycle] == [
        "git_worktree_add",
        "git_worktree_remove",
        "git_worktree_prune",
    ]
    add = lifecycle[0]
    assert {
        "repo_path_id",
        "worktree_path_id",
        "branch_ref_id",
        "start_point_ref_id",
    } <= add.keys()
    assert "/repo" not in json.dumps(lifecycle) and "topic" not in json.dumps(lifecycle)
    merge = actions("git merge --no-ff topic && git cherry-pick -m 1 abc")
    assert all(a["git_ref_ids"] for a in merge)
    transfer = actions("cp -a /src/private /dst/private")[0]
    assert {"src_path_ids", "dst_path_id"} <= transfer.keys()
    created = actions("git worktree add -b topic ../trees/w main", "/repo/main")[0]
    removed = actions("git worktree remove ../trees/w", "/repo/main")[0]
    assert created["worktree_path_id"] == removed["worktree_path_id"]
    assert created["branch_ref_id"] == actions("git merge topic")[0]["git_ref_ids"][0]


def test_multi_source_transfer_and_worktree_options():
    plain = actions("cp a b dst", "/repo")[0]
    targeted = actions("cp -t dst a b", "/repo")[0]
    assert (
        len(plain["src_path_ids"]) == 2
        and plain["dst_path_id"] == targeted["dst_path_id"]
    )
    rsync = actions("rsync --exclude '*.tmp' --include '*.txt' a b dst", "/repo")[0]
    assert len(rsync["src_path_ids"]) == 2
    add = actions(
        "git worktree add --lock --reason audit --orphan topic tree", "/repo"
    )[0]
    assert add["worktree_path_id"] == c.path_id(KEY, "tree", "/repo")


def test_dynamic_operands_do_not_create_join_ids():
    parsed, stats = c.symbolic_actions(
        "git worktree add -b $branch $wt main; cp *.txt $dst", KEY, "/repo"
    )
    add, copy = parsed
    assert "branch_ref_id" not in add and "worktree_path_id" not in add
    assert add["confidence"] == "medium" and copy["confidence"] == "medium"
    assert stats["dynamic_operands"] == 4


def test_conservative_shell_split_false_positives():
    command = (
        "git merge-base a b; echo 'git worktree remove /quoted'; git worktree list"
    )
    assert [a["action"] for a in actions(command)] == ["git_worktree_list"]
    assert actions('printf "x;y && z"; git status')[0]["action"] == "git_status"
    assert actions("echo $(git worktree remove /dynamic); git status") == []
    heredoc = "apply_patch <<'PATCH'\n*** text git worktree remove /fake; git merge x\nPATCH\ngit status"
    parsed, stats = c.symbolic_actions(heredoc, KEY)
    assert parsed == [] and stats["heredoc_skipped"] == 1
    assert actions("echo `git merge x`; git status") == []
    assert actions("cat <<< value; git status")[-1]["action"] == "git_status"
    assert actions("echo '<<EOF'; git status")[-1]["action"] == "git_status"
    post = "cat <<EOF\nbody\nEOF\ngit status"
    assert c.symbolic_actions(post, KEY)[0] == []


def test_tool_success_action_implication():
    semicolon = actions("git worktree remove /x; echo done")[0]
    conjunction = actions("git worktree remove /x && echo done")[0]
    single = actions("git worktree remove /x")[0]
    alternative = actions("git worktree remove /x || echo failed")[0]
    assert semicolon["tool_success_implies_action_success"] is False
    assert alternative["tool_success_implies_action_success"] is False
    assert conjunction["tool_success_implies_action_success"] is True
    assert single["tool_success_implies_action_success"] is True


def test_nested_exec_fixture_and_wrapper_outcome():
    js = """const r=await Promise.all([tools.exec_command({"cmd":"git status","workdir":"/repo"}),tools.exec_command({cmd: "git cherry-pick abc", workdir: "/repo"})]);"""
    nested, dynamic = c.nested_exec_commands(js)
    assert len(nested) == 2 and dynamic == 0
    assert [a["action"] for cmd, cwd in nested for a in actions(cmd, cwd)] == [
        "git_status",
        "git_cherry_pick",
    ]
    result = c.outcome([{"type": "input_text", "text": "Script completed\nOutput:\n"}])
    assert result == {"wrapper_status": "completed", "confidence": "medium"}
    assert c.outcome('{"metadata":{"exit_code":0}}')["status"] == "succeeded"


def test_source_order_ids_nested_and_duplicate_calls_across_logs(tmp_path):
    js = 'const r=await Promise.all([tools.exec_command({"cmd":"git status"}),tools.exec_command({"cmd":"git worktree list"})]);'
    fixtures = []
    for n in (1, 2):
        path = tmp_path / f"log{n}.jsonl"
        fixtures.append(path)
        write(
            path,
            [
                {
                    "timestamp": "2026-01-01T00:00:00Z",
                    "type": "session_meta",
                    "payload": {"id": f"s{n}", "cwd": "/repo"},
                },
                {
                    "timestamp": "2026-01-01T00:00:01Z",
                    "payload": {
                        "type": "custom_tool_call",
                        "name": "exec",
                        "call_id": "duplicate",
                        "input": js,
                    },
                },
            ],
        )
    first, _ = c.extract(fixtures[0], "codex", KEY)
    second, _ = c.extract(fixtures[1], "codex", KEY)
    a = next(r for r in first if r["record_type"] == "tool_call")
    b = next(r for r in second if r["record_type"] == "tool_call")
    assert a["source_log_id"] != b["source_log_id"] and a["call_id"] != b["call_id"]
    assert a["event_ordinal"] == 2
    nested = a["nested_commands"]
    assert [n["nested_command_index"] for n in nested] == [0, 1]
    ids = [action["action_event_id"] for n in nested for action in n["actions"]]
    assert len(ids) == len(set(ids)) == 2 and all(
        n["actions"][0]["action_index"] == 0 for n in nested
    )


def test_codex_schemas_lineage_outcome_privacy_and_bounds(tmp_path):
    path = tmp_path / "codex.jsonl"
    write(
        path,
        [
            {
                "timestamp": "2026-01-01T00:00:00Z",
                "type": "session_meta",
                "payload": {"id": "child", "forked_from_id": "parent", "cwd": "/repo"},
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "payload": {
                    "type": "function_call",
                    "name": "exec_command",
                    "call_id": "a",
                    "arguments": json.dumps({"cmd": "git merge feature"}),
                },
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "payload": {
                    "type": "function_call_output",
                    "call_id": "a",
                    "output": json.dumps({"metadata": {"exit_code": 1}}),
                },
            },
            {
                "timestamp": "2026-01-01T00:00:03Z",
                "payload": {
                    "type": "custom_tool_call",
                    "name": "exec_command",
                    "call_id": "b",
                    "input": {"cmd": "cp /private/a /private/b"},
                },
            },
            {
                "timestamp": "2026-01-01T00:00:04Z",
                "payload": {
                    "type": "custom_tool_call_output",
                    "call_id": "b",
                    "result": json.dumps({"metadata": {"exit_code": 0}}),
                },
            },
            {
                "payload": {
                    "type": "function_call",
                    "name": "exec_command",
                    "call_id": "unknown",
                    "arguments": '{"cmd":"git status"}',
                }
            },
        ],
    )
    rows, stats = c.extract(
        path,
        "codex",
        KEY,
        c.parse_time("2026-01-01T00:00:00Z"),
        c.parse_time("2026-02-01T00:00:00Z"),
    )
    calls = [r for r in rows if r["record_type"] == "tool_call"]
    assert (
        any(r["record_type"] == "lineage" for r in rows)
        and calls[0]["outcome"]["status"] == "failed"
    )
    assert (
        calls[1]["outcome"]["status"] == "succeeded"
        and stats["unknown_timestamps_excluded"] == 1
    )
    assert stats["outcome_failed"] == 1 and stats["outcome_succeeded"] == 1
    serialized = json.dumps(rows)
    assert not any(
        x in serialized
        for x in ['"child"', '"parent"', "/private", '"feature"', "/repo"]
    )


def test_claude_fixture_and_empty_optional(tmp_path):
    path = tmp_path / "claude.jsonl"
    write(
        path,
        [
            {
                "timestamp": "2026-01-01T00:00:00Z",
                "sessionId": "s",
                "message": {
                    "content": [
                        {
                            "type": "tool_use",
                            "id": "e",
                            "name": "Bash",
                            "input": {"command": "echo hi"},
                        },
                        {
                            "type": "tool_use",
                            "id": "u",
                            "name": "Bash",
                            "input": {"command": "git cherry-pick abc"},
                        },
                    ]
                },
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "sessionId": "s",
                "message": {
                    "content": [
                        {
                            "type": "tool_result",
                            "tool_use_id": "u",
                            "content": "bad",
                            "is_error": True,
                        }
                    ]
                },
            },
        ],
    )
    rows, stats = c.extract(path, "claude", KEY)
    call = next(r for r in rows if r["record_type"] == "tool_call")
    assert (
        call["outcome"]["status"] == "failed" and stats["empty_tool_calls_omitted"] == 1
    )
    all_rows, _ = c.extract(path, "claude", KEY, include_empty_tool_calls=True)
    assert len([r for r in all_rows if r["record_type"] == "tool_call"]) == 2


def test_claude_outcome_tristate():
    assert c.outcome("", False) == {"status": "succeeded", "confidence": "high"}
    assert c.outcome("", True) == {"status": "failed", "confidence": "high"}
    assert c.outcome("", None) == {"status": "reported", "confidence": "low"}


def test_explicit_roots_and_strict_dates(tmp_path):
    codex = tmp_path / "claude-in-name"
    claude = tmp_path / "codex-in-name"
    codex.mkdir()
    claude.mkdir()
    (codex / "a.jsonl").write_text("")
    (claude / "b.jsonl").write_text("")
    assert [source for _, source in c.log_paths([codex], [claude])] == [
        "claude",
        "codex",
    ]
    assert c.parse_time("2026-01-01") is None


def test_lineage_metadata_before_window_is_preserved(tmp_path):
    path = tmp_path / "old-meta.jsonl"
    write(
        path,
        [
            {
                "timestamp": "2025-01-01T00:00:00Z",
                "type": "session_meta",
                "payload": {"id": "child", "forked_from_id": "parent", "cwd": "/repo"},
            },
            {
                "timestamp": "2026-01-02T00:00:00Z",
                "payload": {
                    "type": "function_call",
                    "name": "exec_command",
                    "call_id": "c",
                    "arguments": json.dumps({"cmd": "git status"}),
                },
            },
        ],
    )
    rows, _ = c.extract(
        path,
        "codex",
        KEY,
        c.parse_time("2026-01-01T00:00:00Z"),
        c.parse_time("2026-02-01T00:00:00Z"),
    )
    lineage = next(r for r in rows if r["record_type"] == "lineage")
    assert lineage["metadata_outside_window"] is True


def test_cli_manifest_provenance_and_no_paths(tmp_path):
    root = tmp_path / "logs"
    root.mkdir()
    key = tmp_path / "key"
    key.write_bytes(KEY)
    key.chmod(0o600)
    out = tmp_path / "events.jsonl"
    write(
        root / "rollout.jsonl",
        [
            {
                "timestamp": "2026-01-01T00:00:00Z",
                "type": "session_meta",
                "payload": {"id": "s", "cwd": "/secret/repo"},
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "payload": {
                    "type": "function_call",
                    "name": "exec_command",
                    "call_id": "c",
                    "arguments": json.dumps(
                        {"cmd": "git worktree add -b b /secret/tree main"}
                    ),
                },
            },
        ],
    )
    subprocess.run(
        [
            sys.executable,
            str(SCRIPT),
            "--codex-root",
            str(root),
            "--key-file",
            str(key),
            "--start",
            "2026-01-01T00:00:00Z",
            "--end",
            "2026-02-01T00:00:00Z",
            "--out",
            str(out),
        ],
        check=True,
    )
    manifest = json.loads((tmp_path / "events.jsonl.manifest.json").read_text())
    assert {
        "key_fingerprint",
        "script_hash",
        "output_hash",
        "window",
        "sources",
        "coverage",
    } <= manifest.keys()
    assert (
        manifest["output_hash"]
        == "sha256:" + __import__("hashlib").sha256(out.read_bytes()).hexdigest()
    )
    assert manifest["config"]["include_message_fingerprints"] is False
    assert manifest["config"]["message_min_chars"] == 40
    assert "/secret" not in out.read_text() and "/secret" not in json.dumps(manifest)
    first = manifest["input_inventory"]["hash"]
    with (root / "rollout.jsonl").open("a") as handle:
        handle.write("{}\n")
    out2 = tmp_path / "events2.jsonl"
    subprocess.run(
        [
            sys.executable,
            str(SCRIPT),
            "--codex-root",
            str(root),
            "--key-file",
            str(key),
            "--out",
            str(out2),
        ],
        check=True,
    )
    second = json.loads((tmp_path / "events2.jsonl.manifest.json").read_text())[
        "input_inventory"
    ]["hash"]
    assert first != second


def test_message_fingerprints_roles_dedup_edits_and_privacy(tmp_path):
    path = tmp_path / "messages.jsonl"
    base = "Please inspect the exact worktree lifecycle and preserve every useful artifact before cleanup."
    edited = "Please carefully inspect the exact worktree lifecycle and preserve every useful artifact before final cleanup."
    write(
        path,
        [
            {
                "timestamp": "2026-01-01T00:00:00Z",
                "type": "session_meta",
                "payload": {"id": "s"},
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "payload": {"type": "user_message", "message": base},
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "payload": {
                    "type": "message",
                    "role": "user",
                    "content": [
                        {
                            "type": "input_text",
                            "text": "  Please inspect the exact worktree lifecycle\n and preserve every useful artifact before cleanup.  ",
                        }
                    ],
                },
            },
            {
                "timestamp": "2026-01-01T00:00:03Z",
                "payload": {"type": "agent_message", "message": edited},
            },
            {
                "timestamp": "2026-01-01T00:00:04Z",
                "payload": {
                    "type": "message",
                    "role": "developer",
                    "content": [
                        {
                            "type": "input_text",
                            "text": "private developer instruction that must never appear",
                        }
                    ],
                },
            },
            {
                "timestamp": "2026-01-01T00:00:05Z",
                "payload": {
                    "type": "reasoning",
                    "encrypted_content": "secret encrypted reasoning",
                },
            },
            {
                "timestamp": "2026-01-01T00:00:06Z",
                "payload": {"type": "function_call_output", "output": base},
            },
        ],
    )
    rows, stats = c.extract(
        path, "codex", KEY, include_message_fingerprints=True, message_min_chars=20
    )
    messages = [r for r in rows if r["record_type"] == "message_fingerprint"]
    assert [r["role"] for r in messages] == ["user", "agent"]
    assert (
        stats["duplicate_message_representations"] == 1
        and stats["message_blocks_excluded"] >= 2
    )
    assert messages[0]["duplicate_schema_occurrences"][0]["event_ordinal"] == 3
    assert set(messages[0]["winnowed_fingerprint_ids"]) & set(
        messages[1]["winnowed_fingerprint_ids"]
    )
    serialized = json.dumps(rows)
    assert not any(
        raw in serialized
        for raw in [
            "worktree lifecycle",
            "developer instruction",
            "encrypted reasoning",
        ]
    )
    again, _ = c.extract(
        path, "codex", KEY, include_message_fingerprints=True, message_min_chars=20
    )
    other, _ = c.extract(
        path,
        "codex",
        b"different-key",
        include_message_fingerprints=True,
        message_min_chars=20,
    )
    assert rows == again
    assert (
        messages[0]["normalized_text_id"]
        != next(r for r in other if r["record_type"] == "message_fingerprint")[
            "normalized_text_id"
        ]
    )


def test_message_origin_envelopes_and_repeat_chronology(tmp_path):
    path = tmp_path / "origin.jsonl"
    envelope = "Message Type: NEW_TASK\nTask name: /root/a\nSender: /root\nPayload:\nPerform the bounded extraction task and report evidence."
    ordinary = (
        "Please perform the bounded extraction task and report the evidence clearly."
    )
    write(
        path,
        [
            {
                "timestamp": "2026-01-01T00:00:00Z",
                "type": "session_meta",
                "payload": {"id": "s"},
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "payload": {"type": "user_message", "message": envelope},
            },
            {
                "timestamp": "2026-01-01T00:00:02Z",
                "payload": {"type": "user_message", "message": ordinary},
            },
            {
                "timestamp": "2026-01-01T00:00:03Z",
                "payload": {"type": "user_message", "message": ordinary},
            },
            {
                "timestamp": "2026-01-01T00:00:04Z",
                "payload": {
                    "type": "agent_message",
                    "message": "I completed the bounded extraction task and preserved the requested evidence.",
                },
            },
        ],
    )
    rows, stats = c.extract(
        path, "codex", KEY, include_message_fingerprints=True, message_min_chars=20
    )
    messages = [r for r in rows if r["record_type"] == "message_fingerprint"]
    assert [(r["message_origin"], r["delivery_kind"]) for r in messages] == [
        ("nonhuman_agent", "subagent_delivery"),
        ("human_user_candidate", "direct_user_prompt"),
        ("human_user_candidate", "direct_user_prompt"),
        ("agent", "agent_output"),
    ]
    repeats = [
        r
        for r in messages
        if r["normalized_text_id"] == messages[1]["normalized_text_id"]
    ]
    assert [r["event_ordinal"] for r in repeats] == [3, 4]
    assert stats["message_delivery_subagent_delivery"] == 1


def test_claude_message_blocks_and_short_threshold(tmp_path):
    path = tmp_path / "claude-messages.jsonl"
    write(
        path,
        [
            {
                "timestamp": "2026-01-01T00:00:00Z",
                "sessionId": "s",
                "message": {
                    "role": "user",
                    "content": "This is a sufficiently long visible Claude user request for fingerprint extraction.",
                },
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "sessionId": "s",
                "message": {
                    "role": "assistant",
                    "content": [
                        {"type": "thinking", "thinking": "hidden chain"},
                        {
                            "type": "text",
                            "text": "This is a sufficiently long visible Claude agent response for fingerprint extraction.",
                        },
                        {
                            "type": "tool_use",
                            "id": "x",
                            "name": "Bash",
                            "input": {"command": "secret"},
                        },
                    ],
                },
            },
            {
                "timestamp": "2026-01-01T00:00:02Z",
                "sessionId": "s",
                "message": {
                    "role": "user",
                    "content": [
                        {
                            "type": "tool_result",
                            "tool_use_id": "x",
                            "content": "private output",
                        }
                    ],
                },
            },
            {
                "timestamp": "2026-01-01T00:00:03Z",
                "sessionId": "s",
                "message": {"role": "user", "content": "short"},
            },
        ],
    )
    rows, stats = c.extract(
        path, "claude", KEY, include_message_fingerprints=True, message_min_chars=20
    )
    messages = [r for r in rows if r["record_type"] == "message_fingerprint"]
    assert [r["role"] for r in messages] == ["user", "agent"]
    assert (
        stats["messages_below_threshold"] == 1 and stats["message_blocks_excluded"] == 3
    )
    assert all(r["shingle_size"] == 5 and r["char_count"] >= 20 for r in messages)


def test_message_fingerprints_opt_in_default_unchanged(tmp_path):
    path = tmp_path / "only-message.jsonl"
    write(
        path,
        [
            {
                "timestamp": "2026-01-01T00:00:00Z",
                "payload": {
                    "type": "user_message",
                    "message": "A visible message that is definitely long enough for the configured threshold.",
                },
            }
        ],
    )
    rows, _ = c.extract(path, "codex", KEY)
    assert rows == []


def test_claude_system_reminder_origins(tmp_path):
    path = tmp_path / "claude-origin.jsonl"
    write(
        path,
        [
            {
                "timestamp": "2026-01-01T00:00:00Z",
                "sessionId": "s",
                "message": {
                    "role": "user",
                    "content": "<system-reminder>This injected reminder is structural and not a human-authored request.</system-reminder>",
                },
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "sessionId": "s",
                "message": {
                    "role": "user",
                    "content": "Please inspect the current result. <system-reminder>Injected structural context follows here.</system-reminder>",
                },
            },
        ],
    )
    rows, _ = c.extract(
        path, "claude", KEY, include_message_fingerprints=True, message_min_chars=20
    )
    messages = [r for r in rows if r["record_type"] == "message_fingerprint"]
    assert [(r["message_origin"], r["delivery_kind"]) for r in messages] == [
        ("nonhuman_injected", "system_injection"),
        ("mixed_or_injected", "mixed_system_injection"),
    ]


def test_private_outputs_and_key_permissions(tmp_path):
    root = tmp_path / "logs"
    root.mkdir()
    write(
        root / "rollout.jsonl",
        [
            {
                "timestamp": "2026-01-01T00:00:00Z",
                "payload": {
                    "type": "user_message",
                    "message": "A long visible message for the private output permission test.",
                },
            }
        ],
    )
    key = tmp_path / "key"
    key.write_bytes(KEY)
    out = tmp_path / "events.jsonl"
    manifest = tmp_path / "manifest.json"
    out.write_text("old permissive data")
    manifest.write_text("old permissive manifest")
    out.chmod(0o644)
    manifest.chmod(0o644)
    key.chmod(0o644)
    command = [
        sys.executable,
        str(SCRIPT),
        "--codex-root",
        str(root),
        "--key-file",
        str(key),
        "--out",
        str(out),
        "--manifest",
        str(manifest),
    ]
    rejected = subprocess.run(command, text=True, capture_output=True)
    assert rejected.returncode != 0 and "group- or world-readable" in rejected.stderr
    key.chmod(0o600)
    old_umask = os.umask(0o022)
    try:
        subprocess.run(command, check=True)
    finally:
        os.umask(old_umask)
    assert (out.stat().st_mode & 0o777) == 0o600
    assert (manifest.stat().st_mode & 0o777) == 0o600
