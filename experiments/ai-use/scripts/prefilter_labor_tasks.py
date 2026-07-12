#!/usr/bin/env python3
"""Prefilter visible session messages into candidate mathematical-labor rows.

This is deliberately a high-recall, deterministic filter.  Its labels are
overlapping rule hits and are not semantic classifications.  The output keeps
only keyed pointers and aggregate text shape; message text, prompts, paths,
commands, and matched terms never leave the process.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import tempfile
import unicodedata
from collections import Counter
from datetime import datetime
from pathlib import Path
from typing import Any, Iterable

import importlib.util


_COLLECTOR_PATH = Path(__file__).with_name("collect_process_events.py")
_SPEC = importlib.util.spec_from_file_location("ai_use_process_collector", _COLLECTOR_PATH)
if _SPEC is None or _SPEC.loader is None:  # pragma: no cover - import failure
    raise RuntimeError("cannot load collect_process_events.py")
collector = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(collector)


SCHEMA = "ai-use-labor-prefilter-v1"
RULE_VERSION = "labor-rules-v1"

LABOR_LABELS = (
    "proof_search_generation",
    "proof_checking",
    "conjecture_example_counterexample",
    "formalizing_intuition_definitions",
    "explanation_literature_synthesis",
    "code_implementation_debugging",
    "numerical_performance",
    "experiment_design_data_analysis",
    "code_math_review_interpretation",
    "research_prioritization_integration",
)


def _rx(*patterns: str) -> tuple[re.Pattern[str], ...]:
    # Inputs are casefolded before matching; avoiding IGNORECASE keeps matching
    # deterministic for Unicode edge cases.
    return tuple(re.compile(pattern) for pattern in patterns)


MATH_CUE = _rx(
    r"\b(?:math(?:ematics|ematic)?|theorem|lemma|proposition|corollary|proof|"
    r"conjecture|polytope|symplectic|geometry|algebra|topolog\w*|equation|"
    r"inequal\w*|manifold|capacity|viterbo|convex|simplex|facet|formula|"
    r"derivative|gradient|flow|group|ring|field|category|cohomolog\w*|"
    r"homolog\w*|axiom|claim|statement)\b",
)


# Patterns are intentionally broad.  Context gates below keep common words
# such as ``example`` and ``check`` from becoming candidates on their own.
RULES: dict[str, tuple[re.Pattern[str], ...]] = {
    "proof_search_generation": _rx(
        r"\bproof\b",
        r"\b(?:prove|proving|proven|theorem|lemma|proposition|corollary|derive|"
        r"derivation|proof\s+strategy|proof\s+sketch|show\s+that)\b",
    ),
    "proof_checking": _rx(
        r"\b(?:proof\s*check(?:ing)?|proof\s+assistant|qed|soundness|"
        r"completeness|missing\s+case|proof\s+gap|verify\s+the\s+(?:proof|"
        r"theorem|lemma)|check\s+the\s+(?:proof|theorem|lemma))\b",
        r"\b(?:check|verify|validate|audit|correct(?:ness)?|sound)\b",
    ),
    "conjecture_example_counterexample": _rx(
        r"\b(?:conjecture|counter[- ]?example|disprove|disproof|falsif\w*)\b",
        r"\b(?:example|construct|illustrat\w*)\b",
    ),
    "formalizing_intuition_definitions": _rx(
        r"\b(?:formaliz\w*|formalise\w*|formal\s+proof|"
        r"notation|axiom|type\s+theory|lean|coq|isabelle|specification)\b",
        r"\b(?:define|defined|definition|notation)\b",
    ),
    "explanation_literature_synthesis": _rx(
        r"\b(?:explain|explanation|intuition|why\s+(?:does|is|are)|"
        r"literature|paper|citation|reference|related\s+work|summari[sz]\w*|"
        r"synthesi[sz]\w*|background|article|source)\b",
    ),
    "code_implementation_debugging": _rx(
        r"\b(?:implement\w*|code|coding|program\w*|function|class|module|"
        r"crate|rust|python|script|debug\w*|bug|fix|test\w*|compile\w*|"
        r"build|refactor\w*|api|algorithm)\b",
    ),
    "numerical_performance": _rx(
        r"\b(?:numeric\w*|floating[- ]point|f64|exact\s+arithmetic|"
        r"precision|stabil\w*|converg\w*|runtime|performance|benchmark\w*|"
        r"profil\w*|speed|memory|optimiz\w*|complexity|timing)\b",
    ),
    "experiment_design_data_analysis": _rx(
        r"\b(?:experiment\w*|data(?:set)?s?|sample|sampling|analy[sz]\w*|"
        r"figure|plot|table|measure\w*|ablation|reproduc\w*|"
        r"simulation\w*|run\s+the\s+experiment)\b",
    ),
    "code_math_review_interpretation": _rx(
        r"\b(?:review|inspect|audit|interpret\w*|assess|evaluate|critique|"
        r"read)\b",
        r"\b(?:check|verify)\s+(?:the\s+)?(?:code|implementation|result|"
        r"calculation|argument|claim)\b",
    ),
    "research_prioritization_integration": _rx(
        r"\b(?:prioriti[sz]\w*|roadmap|next\s+steps?|plan|scope|integrat\w*|"
        r"trade[- ]?off|which\s+(?:approach|method)|decide|recommend\w*|"
        r"connect\s+the\s+results?|research\s+question|agenda|thesis)\b",
    ),
}


def _rule_payload() -> dict[str, Any]:
    return {
        "version": RULE_VERSION,
        "labels": list(LABOR_LABELS),
        "patterns": {
            label: [pattern.pattern for pattern in RULES[label]]
            for label in LABOR_LABELS
        },
        "math_cue": [pattern.pattern for pattern in MATH_CUE],
    }


RULE_HASH = "sha256:" + hashlib.sha256(
    json.dumps(_rule_payload(), sort_keys=True, separators=(",", ":")).encode()
).hexdigest()


def normalize_for_matching(text: str) -> str:
    """Normalize exactly as the collector does, then casefold for rules."""
    return " ".join(unicodedata.normalize("NFKC", text).split()).casefold()


def _matches(patterns: Iterable[re.Pattern[str]], text: str) -> bool:
    return any(pattern.search(text) for pattern in patterns)


def classify_message(text: str) -> list[str]:
    """Return overlapping candidate labels without exposing rule evidence."""
    normalized = normalize_for_matching(text)
    math = _matches(MATH_CUE, normalized)
    labels: list[str] = []

    if _matches(RULES["proof_search_generation"], normalized):
        labels.append("proof_search_generation")

    # Generic checking/review words are only useful with a mathematical cue;
    # proof-specific phrases above remain unconditional for high recall.
    if _matches(RULES["proof_checking"][:1], normalized) or (
        math and _matches(RULES["proof_checking"][1:], normalized)
    ):
        labels.append("proof_checking")

    if _matches(RULES["conjecture_example_counterexample"][:1], normalized) or (
        math and _matches(RULES["conjecture_example_counterexample"][1:], normalized)
    ):
        labels.append("conjecture_example_counterexample")

    if _matches(RULES["formalizing_intuition_definitions"][:1], normalized) or (
        math and _matches(RULES["formalizing_intuition_definitions"][1:], normalized)
    ):
        labels.append("formalizing_intuition_definitions")

    # Explanation and planning are intentionally broad candidate surfaces: a
    # later human pass decides whether an explanation was mathematical.
    for label in (
        "explanation_literature_synthesis",
        "code_implementation_debugging",
        "numerical_performance",
        "experiment_design_data_analysis",
    ):
        if _matches(RULES[label], normalized):
            labels.append(label)

    code_cue = _matches(RULES["code_implementation_debugging"], normalized)
    if _matches(RULES["code_math_review_interpretation"][1:], normalized) or (
        (math or code_cue)
        and _matches(RULES["code_math_review_interpretation"][:1], normalized)
    ):
        labels.append("code_math_review_interpretation")

    if _matches(RULES["research_prioritization_integration"], normalized):
        labels.append("research_prioritization_integration")

    return labels


def _event_in_window(
    timestamp: object, start: datetime | None, end: datetime | None
) -> bool:
    if start is None and end is None:
        return True
    when = collector.parse_time(timestamp)
    if when is None:
        return False
    return not ((start and when < start) or (end and when >= end))


def _message_text_by_id(
    path: Path,
    source: str,
    key: bytes,
    source_log_id: str,
    start: datetime | None,
    end: datetime | None,
    needed_ids: set[str],
) -> dict[str, str]:
    """Read visible text only long enough to classify keyed collector rows."""
    result: dict[str, str] = {}
    with path.open(encoding="utf-8", errors="replace") as handle:
        for line_no, line in enumerate(handle, 1):
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue
            timestamp = event.get("timestamp")
            if not _event_in_window(timestamp, start, end):
                continue
            messages, _ = collector.visible_messages(event, source)
            for message_index, (role, text, _origin, _delivery) in enumerate(messages):
                message_id = collector.pseudonym(
                    key,
                    "message",
                    f"{source_log_id}\0{line_no}\0{message_index}\0{role}",
                )
                if message_id in needed_ids:
                    result[message_id] = text
    return result


def _candidate_rows(
    path: Path,
    source: str,
    key: bytes,
    start: datetime | None,
    end: datetime | None,
    include_agent_outputs: bool,
    min_chars: int,
) -> tuple[list[dict[str, Any]], Counter[str]]:
    digest = collector.file_hash(path)
    source_log_id = collector.pseudonym(
        key, "source_log", source + "\0" + digest
    )
    rows, extracted_stats = collector.extract(
        path,
        source,
        key,
        start=start,
        end=end,
        include_empty_tool_calls=False,
        include_message_fingerprints=True,
        message_min_chars=min_chars,
        message_fingerprint_mode="task-frame",
        content_hash=digest,
    )
    stats: Counter[str] = Counter(extracted_stats)
    needed_ids = set()
    for row in rows:
        if row.get("record_type") != "message_fingerprint":
            continue
        direct_user = (
            row.get("role") == "user"
            and row.get("message_origin") == "human_user_candidate"
        )
        agent_output = (
            row.get("role") == "agent"
            and row.get("message_origin") == "agent"
            and row.get("delivery_kind") == "agent_output"
        )
        if direct_user or (include_agent_outputs and agent_output):
            needed_ids.add(row["message_id"])
    texts = _message_text_by_id(
        path, source, key, source_log_id, start, end, needed_ids
    )
    candidates: list[dict[str, Any]] = []
    for row in rows:
        if row.get("record_type") != "message_fingerprint":
            continue
        origin = row.get("message_origin")
        direct_user = row.get("role") == "user" and origin == "human_user_candidate"
        agent_output = (
            row.get("role") == "agent"
            and origin == "agent"
            and row.get("delivery_kind") == "agent_output"
        )
        if not direct_user and not (include_agent_outputs and agent_output):
            stats["messages_excluded_by_origin"] += 1
            continue
        text = texts.get(row.get("message_id", ""))
        if text is None:
            stats["messages_without_joinable_text"] += 1
            continue
        labels = classify_message(text)
        if not labels:
            stats["messages_without_labor_label"] += 1
            continue
        candidate_id = collector.pseudonym(
            key,
            "labor_candidate",
            row["source_log_id"] + "\0" + row["message_id"],
        )
        candidate = {
            "record_type": "labor_task_candidate",
            "candidate_id": candidate_id,
            "source": row["source"],
            "source_log_id": row["source_log_id"],
            "session_id": row["session_id"],
            "event_ordinal": row["event_ordinal"],
            "timestamp": row.get("timestamp"),
            "role": row["role"],
            "message_origin": origin,
            "delivery_kind": row.get("delivery_kind"),
            "message_id": row["message_id"],
            "normalized_text_id": row["normalized_text_id"],
            "char_count": row["char_count"],
            "token_count": row["token_count"],
            "labels": labels,
            "model_era_at_event": row.get("model_era_at_event", "unknown"),
        }
        if row.get("model_provider_at_event"):
            candidate["model_provider_at_event"] = row["model_provider_at_event"]
        candidates.append(candidate)
        stats["labor_candidates_emitted"] += 1
        for label in labels:
            stats["label_" + label] += 1
    return candidates, stats


def _atomic_write(path: Path, data: bytes) -> None:
    """Atomically replace a private file, including a permissive old target."""
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix="." + path.name + ".", dir=path.parent)
    temporary_path = Path(temporary)
    try:
        os.fchmod(fd, 0o600)
        with os.fdopen(fd, "wb") as handle:
            handle.write(data)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary_path, path)
        try:
            directory_fd = os.open(path.parent, os.O_DIRECTORY)
        except OSError:
            directory_fd = None
        if directory_fd is not None:
            try:
                os.fsync(directory_fd)
            finally:
                os.close(directory_fd)
    finally:
        temporary_path.unlink(missing_ok=True)
    os.chmod(path, 0o600)


def _strict_time(value: str | None, label: str) -> datetime | None:
    if value is None:
        return None
    parsed = collector.parse_time(value)
    if parsed is None:
        raise argparse.ArgumentTypeError(f"{label} must be timezone-aware ISO-8601")
    return parsed


def run(
    codex_roots: list[Path],
    claude_roots: list[Path],
    key: bytes,
    out: Path,
    start: datetime | None = None,
    end: datetime | None = None,
    manifest: Path | None = None,
    include_agent_outputs: bool = False,
    min_chars: int = 1,
) -> dict[str, Any]:
    if not codex_roots and not claude_roots:
        raise ValueError("at least one explicit source root is required")
    if start and end and start >= end:
        raise ValueError("start must precede end")
    if min_chars < 1:
        raise ValueError("min_chars must be positive")
    totals: Counter[str] = Counter()
    candidates: list[dict[str, Any]] = []
    inventory: list[tuple[str, str]] = []
    scanned, included = Counter(), Counter()
    for path, source in collector.log_paths(codex_roots, claude_roots):
        digest = collector.file_hash(path)
        inventory.append((source, digest))
        scanned[source] += 1
        rows, stats = _candidate_rows(
            path,
            source,
            key,
            start,
            end,
            include_agent_outputs,
            min_chars,
        )
        candidates.extend(rows)
        # Collector schema-coverage maps are useful to its own manifest but
        # are not scalar counters and would make Counter.update ambiguous.
        stats.pop("schema_variants_scanned", None)
        stats.pop("schema_variants_included", None)
        totals.update(stats)
        if rows:
            included[source] += 1
    candidates.sort(
        key=lambda row: (
            row.get("timestamp") or "",
            row["source"],
            row["session_id"],
            row["event_ordinal"],
            row["candidate_id"],
        )
    )
    lines = b"".join(
        (json.dumps(row, sort_keys=True, separators=(",", ":")) + "\n").encode()
        for row in candidates
    )
    output_hash = "sha256:" + hashlib.sha256(lines).hexdigest()
    _atomic_write(out, lines)
    by_label = {
        label: totals.get("label_" + label, 0) for label in LABOR_LABELS
    }
    per_source = {
        source: {
            "count": sum(item_source == source for item_source, _ in inventory),
            "hash": collector.inventory_hash(
                [(item_source, digest) for item_source, digest in inventory if item_source == source]
            ),
        }
        for source in sorted(scanned)
    }
    manifest_obj: dict[str, Any] = {
        "schema": SCHEMA,
        "rule_version": RULE_VERSION,
        "rule_hash": RULE_HASH,
        "interpretation": "Overlapping deterministic high-recall candidates; manual validation is required.",
        "window": {
            "start": start.isoformat() if start else None,
            "end": end.isoformat() if end else None,
        },
        "sources": {"scanned": dict(sorted(scanned.items())), "included": dict(sorted(included.items()))},
        "input_inventory": {
            "count": len(inventory),
            "hash": collector.inventory_hash(inventory),
            "by_source": per_source,
        },
        "config": {
            "include_agent_outputs": include_agent_outputs,
            "message_min_chars": min_chars,
            "direct_user_prompt_default": True,
        },
        "key_fingerprint": "sha256:" + hashlib.sha256(key).hexdigest()[:16],
        "script_hash": collector.file_hash(Path(__file__)),
        "output_hash": output_hash,
        "rows": len(candidates),
        "by_label": by_label,
        "coverage": dict(sorted(totals.items())),
    }
    target = manifest or out.with_suffix(out.suffix + ".manifest.json")
    if target == out:
        raise ValueError("--manifest must differ from --out")
    _atomic_write(
        target,
        (json.dumps(manifest_obj, indent=2, sort_keys=True) + "\n").encode(),
    )
    return manifest_obj


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--codex-root", action="append", type=Path, default=[])
    parser.add_argument("--claude-root", action="append", type=Path, default=[])
    parser.add_argument("--key-file", required=True, type=Path)
    parser.add_argument("--start")
    parser.add_argument("--end")
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--include-agent-outputs", action="store_true")
    parser.add_argument("--message-min-chars", type=int, default=1)
    args = parser.parse_args()
    if not args.codex_root and not args.claude_root:
        parser.error("at least one explicit source root is required")
    try:
        start, end = _strict_time(args.start, "--start"), _strict_time(args.end, "--end")
    except argparse.ArgumentTypeError as exc:
        parser.error(str(exc))
    try:
        stat = args.key_file.stat()
    except OSError as exc:
        parser.error(f"cannot stat --key-file: {exc}")
    if stat.st_mode & 0o077:
        parser.error("--key-file must not be group- or world-readable; chmod it to 0600")
    key = args.key_file.read_bytes()
    if not key:
        parser.error("--key-file must not be empty")
    try:
        run(
            args.codex_root,
            args.claude_root,
            key,
            args.out,
            start,
            end,
            args.manifest,
            args.include_agent_outputs,
            args.message_min_chars,
        )
    except ValueError as exc:
        parser.error(str(exc))


if __name__ == "__main__":
    main()
