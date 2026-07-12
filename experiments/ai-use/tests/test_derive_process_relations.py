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
