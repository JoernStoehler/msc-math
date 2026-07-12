import importlib.util
import json
import subprocess
import sys
from pathlib import Path


SCRIPT = Path(__file__).parents[1] / "scripts" / "prefilter_labor_tasks.py"
spec = importlib.util.spec_from_file_location("labor_prefilter", SCRIPT)
p = importlib.util.module_from_spec(spec)
spec.loader.exec_module(p)
KEY = b"labor-test-key"


def write_jsonl(path: Path, rows: list[dict]) -> None:
    path.write_text("".join(json.dumps(row) + "\n" for row in rows))


def test_rules_overlap_and_casefold_without_matched_evidence():
    labels = p.classify_message(
        "PROVE the theorem, CHECK the proof, find a counterexample, "
        "formalize the definition, explain the paper, implement the Rust code, "
        "benchmark numerical performance, analyze the experiment data, review "
        "the implementation, and prioritize the next steps."
    )
    assert len(labels) >= 10
    assert set(labels) == set(p.LABOR_LABELS)
    assert labels == p.classify_message(
        "PROVE THE THEOREM, CHECK THE PROOF, FIND A COUNTEREXAMPLE, "
        "FORMALIZE THE DEFINITION, EXPLAIN THE PAPER, IMPLEMENT THE RUST CODE, "
        "BENCHMARK NUMERICAL PERFORMANCE, ANALYZE THE EXPERIMENT DATA, REVIEW "
        "THE IMPLEMENTATION, AND PRIORITIZE THE NEXT STEPS."
    )


def test_common_nonlabor_boundaries():
    assert "proof_search_generation" not in p.classify_message(
        "Please proofread this paragraph."
    )
    assert "proof_checking" not in p.classify_message("Check the process status.")
    assert "conjecture_example_counterexample" not in p.classify_message(
        "Give an example JSON response."
    )
    assert "formalizing_intuition_definitions" not in p.classify_message(
        "Define the done state for this ticket."
    )
    assert "code_math_review_interpretation" not in p.classify_message(
        "Review the meeting notes."
    )


def _codex_fixture(path: Path) -> None:
    write_jsonl(
        path,
        [
            {
                "timestamp": "2026-01-01T00:00:00Z",
                "type": "session_meta",
                "payload": {"id": "secret-session", "model": "gpt-5.6", "cwd": "/private/repo"},
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "payload": {"type": "user_message", "message": "Prove the theorem and check the proof."},
            },
            {
                "timestamp": "2026-01-01T00:00:02Z",
                "payload": {"type": "agent_message", "message": "I implemented the numerical benchmark."},
            },
            {
                "timestamp": "2026-01-01T00:00:03Z",
                "payload": {
                    "type": "user_message",
                    "message": "Message Type: NEW_TASK\nTask name: child\nSender: parent\nPayload:\nProve the theorem.",
                },
            },
            {
                "timestamp": "2026-01-01T00:00:04Z",
                "payload": {
                    "type": "user_message",
                    "message": "# AGENTS.md instructions for /private/repo\n<INSTRUCTIONS>Prove the theorem.</INSTRUCTIONS>",
                },
            },
        ],
    )


def test_codex_default_origin_exclusion_and_agent_opt_in(tmp_path):
    log = tmp_path / "codex.jsonl"
    _codex_fixture(log)
    default, stats = p._candidate_rows(log, "codex", KEY, None, None, False, 1)
    assert len(default) == 1
    assert default[0]["role"] == "user"
    assert default[0]["model_era_at_event"] == "gpt-5.6"
    assert stats["messages_excluded_by_origin"] == 3
    with_agent, _ = p._candidate_rows(log, "codex", KEY, None, None, True, 1)
    assert {row["role"] for row in with_agent} == {"user", "agent"}
    serialized = json.dumps(with_agent)
    assert not any(
        secret in serialized
        for secret in (
            "Prove the theorem",
            "numerical benchmark",
            "/private/repo",
            "secret-session",
        )
    )
    assert all("matched" not in row for row in with_agent)


def test_claude_fixture_and_window(tmp_path):
    log = tmp_path / "claude.jsonl"
    write_jsonl(
        log,
        [
            {
                "timestamp": "2025-12-31T23:59:59Z",
                "sessionId": "claude-session",
                "message": {"role": "user", "content": "Prove the theorem."},
            },
            {
                "timestamp": "2026-01-01T00:00:01Z",
                "sessionId": "claude-session",
                "message": {
                    "role": "user",
                    "model": "claude-sonnet-4-5",
                    "content": "Analyze the experiment data and explain the paper.",
                },
            },
            {
                "timestamp": "2026-01-01T00:00:02Z",
                "sessionId": "claude-session",
                "message": {
                    "role": "assistant",
                    "model": "claude-sonnet-4-5",
                    "content": [{"type": "text", "text": "Review the code."}],
                },
            },
        ],
    )
    rows, _ = p._candidate_rows(
        log,
        "claude",
        KEY,
        p.collector.parse_time("2026-01-01T00:00:00Z"),
        p.collector.parse_time("2026-01-02T00:00:00Z"),
        True,
        1,
    )
    assert len(rows) == 2
    assert {row["source"] for row in rows} == {"claude"}
    assert rows[0]["model_era_at_event"] == "claude-4.5"


def test_cli_atomic_private_outputs_and_key_permission(tmp_path):
    root = tmp_path / "logs"
    root.mkdir()
    _codex_fixture(root / "codex.jsonl")
    key = tmp_path / "key"
    key.write_bytes(KEY)
    key.chmod(0o600)
    out = tmp_path / "candidates.jsonl"
    out.write_text("old\n")
    out.chmod(0o644)
    subprocess.run(
        [
            sys.executable,
            str(SCRIPT),
            "--codex-root",
            str(root),
            "--key-file",
            str(key),
            "--out",
            str(out),
            "--start",
            "2026-01-01T00:00:00Z",
            "--end",
            "2026-01-02T00:00:00Z",
        ],
        check=True,
    )
    manifest_path = out.with_suffix(out.suffix + ".manifest.json")
    assert out.stat().st_mode & 0o777 == 0o600
    assert manifest_path.stat().st_mode & 0o777 == 0o600
    manifest = json.loads(manifest_path.read_text())
    assert manifest["schema"] == p.SCHEMA
    assert manifest["rule_hash"] == p.RULE_HASH
    assert manifest["output_hash"] == "sha256:" + __import__("hashlib").sha256(out.read_bytes()).hexdigest()
    assert "/private/repo" not in out.read_text() + manifest_path.read_text()

    bad_key = tmp_path / "bad-key"
    bad_key.write_bytes(KEY)
    bad_key.chmod(0o644)
    failed = subprocess.run(
        [sys.executable, str(SCRIPT), "--codex-root", str(root), "--key-file", str(bad_key), "--out", str(tmp_path / "x")],
        capture_output=True,
        text=True,
    )
    assert failed.returncode != 0
