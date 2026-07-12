import importlib.util
import json
import subprocess
import sys
from pathlib import Path

SCRIPT = Path(__file__).parents[1] / "scripts" / "derive_process_relations.py"
spec = importlib.util.spec_from_file_location("relations", SCRIPT)
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)


def call(s, c, t, actions, status="succeeded", nested=False):
    for action in actions:
        action.setdefault("tool_success_implies_action_success", not nested)
    x = {
        "record_type": "tool_call",
        "source": "codex",
        "source_log_id": "log",
        "event_id": "ev" + c,
        "session_id": s,
        "call_id": c,
        "command_id": "cmd" + c,
        "timestamp": t,
        "actions": [],
    }
    if nested:
        x["nested_commands"] = [{"command_id": "nested" + c, "actions": actions}]
    else:
        x["actions"] = actions
    if status == "wrapper":
        x["outcome"] = {"wrapper_status": "completed"}
    elif status:
        x["outcome"] = {"status": status}
    return x


def a(op, repo="repo", tree="tree", **kw):
    return {
        "action": op,
        "confidence": "high",
        "repo_path_id": repo,
        "worktree_path_id": tree,
        **kw,
    }


def typ(xs, k):
    return [x for x in xs if x["relation_type"] == k]


def test_episode_chronology_dedup_cleanup_and_reuse():
    rows = [
        call(
            "owner",
            "1",
            "2026-01-01T00:00:01Z",
            [a("git_worktree_add", branch_ref_id="br")],
        ),
        call(
            "user",
            "2",
            "2026-01-01T00:00:02Z",
            [{"action": "git_status", "confidence": "high", "repo_path_id": "tree"}],
        ),
        call(
            "user",
            "3",
            "2026-01-01T00:00:03Z",
            [{"action": "git_status", "confidence": "high", "repo_path_id": "tree"}],
        ),
        call("cleaner", "4", "2026-01-01T00:00:04Z", [a("git_worktree_remove")]),
        call(
            "late",
            "5",
            "2026-01-01T00:00:05Z",
            [{"action": "git_status", "confidence": "high", "repo_path_id": "tree"}],
        ),
        call("owner2", "6", "2026-01-01T00:00:06Z", [a("git_worktree_add")]),
    ]
    got = r.derive(rows)
    use = typ(got, "worktree_used_by_session")
    assert (
        len(use) == 1
        and use[0]["session_id"] == "user"
        and use[0]["use_event_count"] == 2
    )
    cleanup = typ(got, "cleanup_ownership_transfer")
    assert sorted((x["episode"], x["cleanup_class"]) for x in cleanup) == [
        (1, "other"),
        (2, "unresolved"),
    ]
    assert (
        next(x for x in cleanup if x["episode"] == 1)["eligible"]
        and not next(x for x in cleanup if x["episode"] == 2)["eligible"]
    )


def test_failed_and_wrapper_attempts_do_not_mutate():
    rows = [
        call("o", "1", "2026-01-01T00:00:01Z", [a("git_worktree_add")]),
        call("x", "2", "2026-01-01T00:00:02Z", [a("git_worktree_remove")], "failed"),
        call(
            "x",
            "3",
            "2026-01-01T00:00:03Z",
            [a("git_worktree_remove")],
            "wrapper",
            True,
        ),
        call(
            "u",
            "4",
            "2026-01-01T00:00:04Z",
            [{"action": "git_status", "confidence": "high", "repo_path_id": "tree"}],
        ),
    ]
    got = r.derive(rows)
    cleanup = typ(got, "cleanup_ownership_transfer")[0]
    assert cleanup["cleanup_class"] == "failed-attempt" and not cleanup["eligible"]
    assert typ(got, "worktree_used_by_session")[0]["session_id"] == "u"
    nested = next(
        x
        for x in typ(got, "worktree_removed")
        if x["evidence"][0]["evidence_status"].get("enclosing_wrapper_completed")
    )
    assert nested["evidence"][0]["evidence_status"] == {
        "issued": True,
        "command_outcome": "unknown",
        "enclosing_wrapper_completed": True,
    }


def test_unknown_tied_chronology_and_branch_scope():
    tied = [
        call("o", "1", "2026-01-01T00:00:01Z", [a("git_worktree_add")]),
        call("x", "2", "2026-01-01T00:00:01Z", [a("git_worktree_remove")]),
    ]
    c = typ(r.derive(tied), "cleanup_ownership_transfer")[0]
    assert (
        c["cleanup_class"] == "ambiguous"
        and c["exclusion_reason"] == "ambiguous_chronology"
    )
    rows = [
        call(
            "o",
            "3",
            "2026-01-01T00:00:01Z",
            [a("git_worktree_add", repo="r1", branch_ref_id="b")],
        ),
        call(
            "m",
            "4",
            "2026-01-01T00:00:02Z",
            [
                {
                    "action": "git_merge",
                    "confidence": "high",
                    "repo_path_id": "r2",
                    "git_ref_ids": ["b"],
                }
            ],
        ),
    ]
    assert not typ(r.derive(rows), "branch_created_then_merged")


def test_file_candidates_lineage_unique_ids_and_recursive_privacy():
    rows = [
        {
            "record_type": "lineage",
            "session_id": "child",
            "parent_session_id": "parent",
            "timestamp": "2026-01-01T00:00:00Z",
            "source_log_id": "log",
        },
        call(
            "s",
            "1",
            "2026-01-01T00:00:01Z",
            [
                {
                    "action": "file_transfer",
                    "confidence": "high",
                    "src_path_ids": ["src"],
                    "dst_path_id": "dst",
                },
                {
                    "action": "file_delete",
                    "confidence": "high",
                    "file_path_ids": ["old"],
                },
            ],
        ),
    ]
    got = r.derive(rows)
    assert len({x["relation_id"] for x in got}) == len(got)
    assert typ(got, "file_transferred")[0]["record_type"] == "candidate_relation"

    def strings(x):
        if isinstance(x, str):
            yield x
        elif isinstance(x, dict):
            for k, v in x.items():
                yield k
                yield from strings(v)
        elif isinstance(x, list):
            for v in x:
                yield from strings(v)

    assert not any("/secret" in s for s in strings(got))


def test_cli_manifest_validation_propagation_filter(tmp_path):
    inp = tmp_path / "e.jsonl"
    out = tmp_path / "r.jsonl"
    man = tmp_path / "m.json"
    inp.write_text(
        json.dumps(
            call(
                "s",
                "1",
                "2026-01-01T00:00:01Z",
                [
                    {
                        "action": "file_delete",
                        "confidence": "high",
                        "file_path_ids": ["p"],
                    }
                ],
            )
        )
        + "\n"
    )
    source = {
        "schema": r.EVENT_SCHEMA,
        "output_hash": r.file_hash(inp),
        "window": {"start": "x"},
        "key_fingerprint": "k",
        "input_inventory": {"hash": "i"},
        "script_hash": "collector",
        "coverage": {"x": 1},
    }
    man.write_text(json.dumps(source))
    subprocess.run(
        [
            sys.executable,
            str(SCRIPT),
            "--input",
            str(inp),
            "--input-manifest",
            str(man),
            "--output",
            str(out),
            "--relation-type",
            "file_deleted",
        ],
        check=True,
    )
    m = json.loads((tmp_path / "r.jsonl.manifest.json").read_text())
    assert m["source"]["window"] == {"start": "x"} and m["relation_types"] == [
        "file_deleted"
    ]
    source["output_hash"] = "sha256:bad"
    man.write_text(json.dumps(source))
    p = subprocess.run(
        [
            sys.executable,
            str(SCRIPT),
            "--input",
            str(inp),
            "--input-manifest",
            str(man),
            "--output",
            str(out),
        ],
        capture_output=True,
    )
    assert p.returncode and b"does not match" in p.stderr


def test_cli_outputs_are_private_under_umask_and_replace_public_targets(tmp_path):
    inp, source_manifest = tmp_path / "events.jsonl", tmp_path / "events.manifest.json"
    out, manifest = tmp_path / "relations.jsonl", tmp_path / "relations.manifest.json"
    inp.write_text("")
    source_manifest.write_text(
        json.dumps({"schema": r.EVENT_SCHEMA, "output_hash": r.file_hash(inp)})
    )
    out.write_text("public old output")
    manifest.write_text("public old manifest")
    out.chmod(0o644)
    manifest.chmod(0o644)
    subprocess.run(
        [
            sys.executable,
            str(SCRIPT),
            "--input",
            str(inp),
            "--input-manifest",
            str(source_manifest),
            "--output",
            str(out),
            "--manifest",
            str(manifest),
        ],
        check=True,
        preexec_fn=lambda: __import__("os").umask(0o022),
    )
    assert out.stat().st_mode & 0o777 == 0o600
    assert manifest.stat().st_mode & 0o777 == 0o600
    assert out.read_text() == "" and json.loads(manifest.read_text())["rows"] == 0


def test_stress_linear_episode_shape():
    rows = []
    for i in range(200):
        rows += [
            call(
                "o",
                f"a{i}",
                f"2026-01-01T00:{i // 60:02}:{i % 60:02}Z",
                [a("git_worktree_add", tree=f"t{i}")],
            ),
            call(
                "o",
                f"r{i}",
                f"2026-01-02T00:{i // 60:02}:{i % 60:02}Z",
                [a("git_worktree_remove", tree=f"t{i}")],
            ),
        ]
    assert len(typ(r.derive(rows), "cleanup_ownership_transfer")) == 200


def test_july_like_wrapper_episode_is_visible_but_never_eligible():
    rows = [
        call(
            "owner",
            "wa",
            "2026-07-01T00:00:01Z",
            [a("git_worktree_add")],
            "wrapper",
            True,
        ),
        call(
            "worker",
            "wu",
            "2026-07-01T00:00:02Z",
            [{"action": "git_status", "confidence": "high", "repo_path_id": "tree"}],
            "wrapper",
            True,
        ),
        call(
            "cleaner",
            "wr",
            "2026-07-01T00:00:03Z",
            [a("git_worktree_remove")],
            "wrapper",
            True,
        ),
    ]
    got = r.derive(rows)
    cleanup = typ(got, "cleanup_ownership_transfer")
    assert len(cleanup) == 1 and cleanup[0]["cleanup_class"] == "other"
    assert cleanup[0]["transition_evidence"] == "wrapper_completed_unverified"
    assert (
        cleanup[0]["eligible"] is False
        and cleanup[0]["exclusion_reason"] == "command_outcome_unknown"
    )
    use = typ(got, "worktree_used_by_session")
    assert len(use) == 1 and use[0]["session_id"] == "worker" and not use[0]["eligible"]


def test_tool_outcome_requires_action_implication_claude_semicolon_vs_and():
    semicolon = a("git_worktree_add", tree="semi")
    semicolon["tool_success_implies_action_success"] = False
    conjunction = a("git_worktree_add", tree="and")
    conjunction["tool_success_implies_action_success"] = True
    rows = [
        call("claude", "s", "2026-01-01T00:00:01Z", [semicolon], "succeeded"),
        call("claude", "a", "2026-01-01T00:00:02Z", [conjunction], "succeeded"),
    ]
    created = typ(r.derive(rows), "worktree_created")
    semi = next(x for x in created if x["worktree_path_id"] == "semi")
    yes = next(x for x in created if x["worktree_path_id"] == "and")
    assert semi["evidence"][0]["evidence_status"] == {
        "issued": True,
        "command_outcome": "unknown",
        "enclosing_tool_outcome": "succeeded",
    }
    assert not semi["eligible"] and yes["eligible"]


def test_wrapper_branch_candidate_is_scoped_deduped_and_ineligible():
    add = a("git_worktree_add", repo="repo", branch_ref_id="br")
    merge = {
        "action": "git_merge",
        "confidence": "high",
        "repo_path_id": "repo",
        "git_ref_ids": ["br"],
    }
    rows = [
        call("owner", "1", "2026-07-01T00:00:01Z", [add], "wrapper", True),
        call("integrator", "2", "2026-07-01T00:00:02Z", [merge], "wrapper", True),
    ]
    got = typ(r.derive(rows), "branch_created_then_merged")
    assert len(got) == 1 and not got[0]["eligible"]
    assert got[0]["transition_evidence"] == "wrapper_completed_unverified"
    assert got[0]["exclusion_reason"] == "command_outcome_unknown"


def msg(mid, role, session, log, time, norm, shingles, tokens=10):
    return {
        "record_type": "message_fingerprint",
        "source_log_id": log,
        "event_ordinal": 1,
        "session_id": session,
        "timestamp": time,
        "role": role,
        "message_origin": "agent"
        if role in {"agent", "assistant"}
        else "human_user_candidate",
        "delivery_kind": "agent_output"
        if role in {"agent", "assistant"}
        else "direct_user_prompt",
        "message_id": mid,
        "normalized_text_id": norm,
        "char_count": 100,
        "token_count": tokens,
        "reuse_eligible": True,
        "winnowed_fingerprint_ids": shingles,
    }


def test_prompt_reuse_exact_normalized_and_light_edit():
    rows = [
        msg(
            "a",
            "agent",
            "s1",
            "l1",
            "2026-01-01T00:00:01Z",
            "same",
            ["1", "2", "3", "4"],
        ),
        msg(
            "u",
            "user",
            "s2",
            "l2",
            "2026-01-01T00:00:02Z",
            "same",
            ["1", "2", "3", "4"],
        ),
        msg(
            "a2",
            "assistant",
            "s3",
            "l3",
            "2026-01-01T00:00:03Z",
            "old",
            ["5", "6", "7", "8"],
        ),
        msg(
            "u2",
            "user",
            "s4",
            "l4",
            "2026-01-01T00:00:04Z",
            "new",
            ["5", "6", "7", "9"],
        ),
    ]
    got = typ(
        r.derive(
            rows,
            reuse_config={
                "min_overlap": 3,
                "min_similarity": 0.5,
                "max_shingle_df": 20,
                "min_token_ratio": 0.5,
            },
        ),
        "prompt_reuse_candidate",
    )
    assert {x["reuse_label"] for x in got} == {"exact", "edited"}
    assert all(
        not x["eligible"] and x["exclusion_reason"] == "pending_review" for x in got
    )


def test_prompt_reuse_chronology_session_and_common_template_controls():
    common = ["c1", "c2", "c3"]
    rows = [
        msg("a1", "agent", "s1", "l1", "2026-01-01T00:00:02Z", "n1", common + ["x"]),
        msg("a2", "agent", "s2", "l2", "2026-01-01T00:00:02Z", "n2", common + ["y"]),
        msg("early", "user", "s3", "l3", "2026-01-01T00:00:01Z", "n1", common + ["x"]),
        msg("same", "user", "s1", "l4", "2026-01-01T00:00:03Z", "n1", common + ["x"]),
        msg(
            "target",
            "user",
            "s4",
            "l4",
            "2026-01-01T00:00:03Z",
            "other",
            common + ["z"],
        ),
    ]
    assert (
        r.prompt_reuse(
            rows,
            min_overlap=3,
            min_similarity=0.5,
            max_shingle_df=1,
            min_token_ratio=0.5,
        )
        == []
    )


def test_prompt_reuse_broadcast_group_and_scalability():
    rows = [
        msg(
            f"a{i}",
            "agent",
            f"s{i}",
            f"l{i}",
            "2026-01-01T00:00:01Z",
            "same",
            ["1", "2", "3"],
        )
        for i in range(30)
    ]
    rows += [
        msg(
            f"u{i}",
            "user",
            f"t{i}",
            f"m{i}",
            "2026-01-01T00:00:02Z",
            "same",
            ["1", "2", "3"],
        )
        for i in range(30)
    ]
    rows += [
        msg(
            f"noise{i}",
            "user",
            f"n{i}",
            f"z{i}",
            "2026-01-01T00:00:03Z",
            f"n{i}",
            [f"q{i}"],
            2,
        )
        for i in range(1000)
    ]
    got = r.prompt_reuse(rows)
    assert (
        len(got) == 1
        and len(got[0]["source_message_ids"]) == 30
        and len(got[0]["target_message_ids"]) == 30
    )


def test_prompt_reuse_subagent_delivery_is_not_a_human_target():
    source = msg(
        "a", "agent", "s1", "l1", "2026-01-01T00:00:01Z", "same", ["1", "2", "3"]
    )
    delivered = msg(
        "u", "user", "s2", "l2", "2026-01-01T00:00:02Z", "same", ["1", "2", "3"]
    )
    delivered["message_origin"] = "subagent_delivery"
    assert r.prompt_reuse([source, delivered]) == []


def test_prompt_reuse_5000_by_5000_exact_broadcast_suppressed_without_pairs():
    rows = [
        msg(
            f"a{i}",
            "agent",
            f"s{i}",
            f"l{i}",
            "2026-01-01T00:00:01Z",
            "same",
            ["1", "2", "3"],
        )
        for i in range(5000)
    ]
    rows += [
        msg(
            f"u{i}",
            "user",
            f"t{i}",
            f"m{i}",
            "2026-01-01T00:00:02Z",
            "same",
            ["1", "2", "3"],
        )
        for i in range(5000)
    ]
    stats = __import__("collections").Counter()
    assert r.prompt_reuse(rows, exact_frequency_ceiling=1000, stats=stats) == []
    assert stats["exact_frequency_ceiling_suppressed"] == 1


def test_task_start_sampling_frame_root_continuation_metadata_and_first_prompt():
    root = {
        "record_type": "session",
        "source_log_id": "lr",
        "event_ordinal": 1,
        "session_id": "root",
        "timestamp": "2026-01-01T00:00:00Z",
        "cwd_path_id": "cwd",
        "repo_path_id": "repo",
        "model_ids": ["model"],
        "model_eras": ["gpt-5.6"],
        "model_providers": ["openai"],
    }
    child = {
        "record_type": "session",
        "source_log_id": "lc",
        "event_ordinal": 1,
        "session_id": "child",
        "timestamp": "2026-01-01T00:01:00Z",
        "worktree_path_id": "tree",
    }
    lineage = {
        "record_type": "lineage",
        "source_log_id": "lc",
        "session_id": "child",
        "parent_session_id": "root",
    }
    first = msg("first", "user", "root", "lr", "2026-01-01T00:00:01Z", "n1", ["1"])
    later = msg("later", "user", "root", "lr", "2026-01-01T00:00:02Z", "n2", ["2"])
    handoff = msg("handoff", "user", "child", "lc", "2026-01-01T00:01:01Z", "n3", ["3"])
    first["model_id_at_event"] = "event-model"
    first["model_era_at_event"] = "gpt-5.6"
    first["model_provider_at_event"] = "openai"
    got = r.derive([root, child, lineage, later, handoff, first])
    start = typ(got, "task_start_candidate")
    continuation = typ(got, "task_continuation_candidate")
    assert (
        len(start) == 1
        and start[0]["message_id"] == "first"
        and not start[0]["eligible"]
        and start[0]["eligible_for_sampling_frame"]
    )
    assert (
        start[0]["root_status"] == "no_parent_observed_in_input"
        and start[0]["cwd_path_id"] == "cwd"
        and start[0]["model_era_at_event"] == "gpt-5.6"
        and start[0]["session_observed_model_ids"] == ["model"]
    )
    assert (
        len(continuation) == 1
        and continuation[0]["parent_session_id"] == "root"
        and continuation[0]["root_status"] == "explicit_non_root"
    )


def test_task_start_excludes_subagent_delivery_and_unknown_prompt_time():
    session = {
        "record_type": "session",
        "source_log_id": "l",
        "event_ordinal": 1,
        "session_id": "s",
        "timestamp": None,
    }
    delivery = msg("d", "user", "s", "l", "2026-01-01T00:00:01Z", "n", ["1"])
    delivery["message_origin"] = "nonhuman_agent"
    delivery["delivery_kind"] = "subagent_delivery"
    unknown = msg("u", "user", "s", "l", None, "n2", ["2"])
    assert r.task_start_candidates([session, delivery, unknown]) == []


def test_task_start_short_prompt_and_duplicate_session_ids_stay_per_log():
    sessions = [
        {
            "record_type": "session",
            "source_log_id": "l1",
            "session_id": "same",
            "timestamp": "2026-01-01T00:00:00Z",
            "cwd_path_id": "c1",
        },
        {
            "record_type": "session",
            "source_log_id": "l2",
            "session_id": "same",
            "timestamp": "2026-01-01T00:00:00Z",
            "cwd_path_id": "c2",
        },
    ]
    p1 = msg("p1", "user", "same", "l1", "2026-01-01T00:00:01Z", "short1", [], 1)
    p1["reuse_eligible"] = False
    p2 = msg("p2", "user", "same", "l2", "2026-01-01T00:00:01Z", "short2", [], 1)
    p2["reuse_eligible"] = False
    got = r.task_start_candidates([*sessions, p1, p2], window_bounded=True)
    assert len(got) == 2 and {x["cwd_path_id"] for x in got} == {"c1", "c2"}
    assert all(
        x["collection_window_bounded"] and x["frame_status"] == "candidate_not_episode"
        for x in got
    )


def test_one_token_exact_is_task_frame_only_not_prompt_reuse():
    source = msg("agent-short", "agent", "a", "la", "2026-01-01T00:00:01Z", "exact-short", [], 1)
    target = msg("user-short", "user", "u", "lu", "2026-01-01T00:00:02Z", "exact-short", [], 1)
    source["reuse_eligible"] = False
    target["reuse_eligible"] = False
    session = {"record_type":"session","source_log_id":"lu","session_id":"u","timestamp":"2026-01-01T00:00:00Z"}
    got = r.derive([session, source, target])
    assert typ(got, "prompt_reuse_candidate") == []
    starts = typ(got, "task_start_candidate")
    assert len(starts) == 1 and starts[0]["message_id"] == "user-short"
    assert starts[0]["eligible_for_sampling_frame"]
