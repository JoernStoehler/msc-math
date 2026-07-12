#!/usr/bin/env python3
"""Extract keyed-pseudonymous structural process events from agent JSONL."""

from __future__ import annotations
import argparse
import hashlib
import hmac
import json
import os
import re
import shlex
from collections import Counter
from datetime import datetime
from pathlib import Path
from typing import Any

SCHEMA = "ai-use-process-events-v2"
SHELL_TOOLS = {"exec_command", "Bash", "bash", "shell"}


def pseudonym(key: bytes, domain: str, value: object) -> str:
    return (
        "hmac256:"
        + hmac.new(
            key, (domain + "\0" + str(value)).encode(), hashlib.sha256
        ).hexdigest()[:24]
    )


def canonical_path(value: str, cwd: str | None) -> str | None:
    expanded = os.path.expanduser(value)
    if not os.path.isabs(expanded):
        if not cwd:
            return None
        expanded = os.path.join(cwd, expanded)
    return os.path.normpath(expanded)


def path_id(key: bytes, value: str, cwd: str | None) -> str | None:
    normalized = canonical_path(value, cwd)
    return pseudonym(key, "path", normalized) if normalized else None


def dynamic_operand(value: str) -> bool:
    return bool(re.search(r"\$|[*?\[]", value))


def action_identity(
    key: bytes,
    source_log_id: str,
    ordinal: int,
    nested_index: int | None,
    action_index: int,
) -> str:
    coordinate = f"{source_log_id}\0{ordinal}\0{nested_index if nested_index is not None else 'direct'}\0{action_index}"
    return pseudonym(key, "action_event", coordinate)


def parse_time(value: object) -> datetime | None:
    if not isinstance(value, str):
        return None
    try:
        result = datetime.fromisoformat(value.replace("Z", "+00:00"))
        if result.tzinfo is None:
            raise ValueError("timezone required")
        return result
    except ValueError:
        return None


def strict_time(value: str | None, label: str) -> datetime | None:
    if value is None:
        return None
    parsed = parse_time(value)
    if parsed is None:
        raise argparse.ArgumentTypeError(f"{label} must be timezone-aware ISO-8601")
    return parsed


def json_object(value: object) -> dict[str, Any]:
    if isinstance(value, dict):
        return value
    if isinstance(value, str):
        try:
            decoded = json.loads(value)
            return decoded if isinstance(decoded, dict) else {}
        except json.JSONDecodeError:
            pass
    return {}


def command_from(name: str, arguments: object) -> tuple[str | None, str | None]:
    args = json_object(arguments)
    if name not in SHELL_TOOLS:
        return None, None
    command = args.get("cmd", args.get("command"))
    cwd = args.get("workdir")
    return (
        command if isinstance(command, str) else None,
        cwd if isinstance(cwd, str) else None,
    )


def _balanced_object(text: str, start: int) -> str | None:
    depth, quote, escaped = 0, None, False
    for i in range(start, len(text)):
        ch = text[i]
        if quote:
            if escaped:
                escaped = False
            elif ch == "\\":
                escaped = True
            elif ch == quote:
                quote = None
            continue
        if ch in {'"', "'", "`"}:
            quote = ch
        elif ch == "{":
            depth += 1
        elif ch == "}" and (depth := depth - 1) == 0:
            return text[start : i + 1]
    return None


def nested_exec_commands(script: str) -> tuple[list[tuple[str, str | None]], int]:
    commands, dynamic = [], 0
    for match in re.finditer(r"tools\.exec_command\s*\(\s*", script):
        start = match.end()
        literal = (
            _balanced_object(script, start)
            if start < len(script) and script[start] == "{"
            else None
        )
        if not literal:
            dynamic += 1
            continue
        obj = json_object(literal)
        command, cwd = obj.get("cmd"), obj.get("workdir")
        if not isinstance(command, str):
            cm = re.search(r'(?:["\']cmd["\']|\bcmd)\s*:\s*(["\'])', literal)
            if not cm:
                dynamic += 1
                continue
            q, pos, escaped, chars = cm.group(1), cm.end(), False, []
            while pos < len(literal):
                ch = literal[pos]
                if escaped:
                    chars.append(ch)
                    escaped = False
                elif ch == "\\":
                    escaped = True
                    chars.append(ch)
                elif ch == q:
                    break
                else:
                    chars.append(ch)
                pos += 1
            command = "".join(chars)
            wm = re.search(
                r'(?:["\']workdir["\']|\bworkdir)\s*:\s*(["\'])(.*?)\1', literal
            )
            cwd = wm.group(2) if wm else None
        if isinstance(command, str):
            commands.append((command, cwd if isinstance(cwd, str) else None))
    return commands, dynamic


def shell_segments(command: str) -> tuple[list[str], dict[str, int]]:
    """Conservatively split top-level shell lists; never inspect heredoc bodies."""
    stats = Counter()
    segments, chars = [], []
    quote, escaped, paren = None, False, 0
    heredoc_at, q, esc = None, None, False
    for j, ch in enumerate(command[:-1]):
        if q:
            if esc:
                esc = False
            elif ch == "\\" and q == '"':
                esc = True
            elif ch == q:
                q = None
            continue
        if ch in {'"', "'"}:
            q = ch
            continue
        if command[j : j + 2] == "<<" and command[j : j + 3] != "<<<":
            heredoc_at = j
            break
    scan = (
        command[: command.find("\n", heredoc_at)]
        if heredoc_at is not None and "\n" in command[heredoc_at:]
        else command
    )
    if heredoc_at is not None:
        stats["heredoc_skipped"] += 1
    i = 0
    while i < len(scan):
        ch = scan[i]
        if quote:
            chars.append(ch)
            if escaped:
                escaped = False
            elif ch == "\\" and quote == '"':
                escaped = True
            elif ch == quote:
                quote = None
            i += 1
            continue
        if ch in {'"', "'"}:
            quote = ch
            chars.append(ch)
            i += 1
            continue
        if ch == "`" or (ch == "$" and i + 1 < len(scan) and scan[i + 1] == "("):
            stats["dynamic_segments"] += 1
            return [], dict(stats)
        if ch == "(":
            paren += 1
        elif ch == ")" and paren:
            paren -= 1
        if paren == 0 and (
            (ch == "|" and scan[i : i + 2] != "||")
            or (ch == "&" and scan[i : i + 2] != "&&")
        ):
            stats["pipeline_or_background"] += 1
        sep = paren == 0 and (ch in ";\n" or scan[i : i + 2] in {"&&", "||"})
        if sep:
            if "".join(chars).strip():
                segments.append("".join(chars).strip())
                separator = scan[i : i + 2] if scan[i : i + 2] in {"&&", "||"} else ch
                width = 2 if separator in {"&&", "||"} else 1
                if separator != "&&" and scan[i + width :].strip():
                    stats["non_conjunctive_chain"] += 1
            chars = []
            i += 2 if scan[i : i + 2] in {"&&", "||"} else 1
            continue
        chars.append(ch)
        i += 1
    if quote or paren:
        stats["unparseable_segments"] += 1
        return [], dict(stats)
    if "".join(chars).strip():
        segments.append("".join(chars).strip())
    return segments, dict(stats)


def _value_after(tokens: list[str], names: set[str]) -> str | None:
    for i, token in enumerate(tokens[:-1]):
        if token in names:
            return tokens[i + 1]
    return None


def _positional(tokens: list[str], value_options: set[str]) -> list[str]:
    result, skip = [], False
    for token in tokens:
        if skip:
            skip = False
            continue
        if token in value_options:
            skip = True
            continue
        if token.startswith("-"):
            continue
        result.append(token)
    return result


def classify_segment(
    segment: str, key: bytes, cwd: str | None = None
) -> dict[str, Any] | None:
    try:
        words = shlex.split(segment, comments=True)
    except ValueError:
        return None
    while words and re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*=.*", words[0]):
        words.pop(0)
    if words[:1] in (["sudo"], ["command"]):
        words.pop(0)
    if not words:
        return None
    action: dict[str, Any] = {"confidence": "high"}

    def put(field: str, domain: str, value: str | None) -> None:
        if value is not None:
            if dynamic_operand(value):
                action["_dynamic_operands"] = action.get("_dynamic_operands", 0) + 1
                action["confidence"] = "medium"
            else:
                action[field] = pseudonym(key, domain, value)

    def put_path(field: str, value: str | None, base_cwd: str | None) -> None:
        if value is None:
            return
        if dynamic_operand(value):
            action["_dynamic_operands"] = action.get("_dynamic_operands", 0) + 1
            action["confidence"] = "medium"
            return
        pid = path_id(key, value, base_cwd)
        if pid:
            action[field] = pid

    if cwd:
        put_path("repo_path_id", cwd, cwd)
    base = Path(words[0]).name
    if base == "git":
        i, repo = 1, None
        while i < len(words) and words[i].startswith("-"):
            token = words[i]
            if token == "-C" and i + 1 < len(words):
                repo = words[i + 1]
                i += 2
            elif token.startswith("-C") and len(token) > 2:
                repo = token[2:]
                i += 1
            elif token in {"-c", "--git-dir", "--work-tree", "--namespace"}:
                i += 2
            else:
                i += 1
        effective_cwd = canonical_path(repo, cwd) if repo else cwd
        if repo:
            put_path("repo_path_id", repo, cwd)
        if i >= len(words):
            return None
        sub, tail = words[i], words[i + 1 :]
        if sub == "worktree" and tail and tail[0] in {"add", "remove", "prune", "list"}:
            op, args = tail[0], tail[1:]
            action["action"] = f"git_worktree_{op}"
            pos = _positional(
                args, {"-b", "-B", "--orphan", "--reason", "--expire", "--format"}
            )
            if op == "add":
                branch = _value_after(args, {"-b", "-B", "--orphan"})
                put("branch_ref_id", "git_ref", branch)
                if pos:
                    put_path("worktree_path_id", pos[0], effective_cwd)
                if len(pos) > 1:
                    put("start_point_ref_id", "git_ref", pos[1])
            elif op == "remove" and pos:
                put_path("worktree_path_id", pos[-1], effective_cwd)
            return action
        if sub in {"merge", "cherry-pick"}:
            refs = _positional(
                tail,
                {"-m", "--mainline", "-X", "--strategy-option", "-s", "--strategy"},
            )
            action["action"] = "git_" + sub.replace("-", "_")
            action["git_ref_ids"] = []
            for ref in refs:
                if dynamic_operand(ref):
                    action["_dynamic_operands"] = action.get("_dynamic_operands", 0) + 1
                    action["confidence"] = "medium"
                else:
                    action["git_ref_ids"].append(pseudonym(key, "git_ref", ref))
            return action
        if sub in {"commit", "status"}:
            action["action"] = f"git_{sub}"
            return action
        if sub == "branch" and ({"-d", "-D", "--delete"} & set(tail)):
            action["action"] = "git_branch_delete"
            action["git_ref_ids"] = []
            for ref in _positional(tail, set()):
                if dynamic_operand(ref):
                    action["_dynamic_operands"] = action.get("_dynamic_operands", 0) + 1
                    action["confidence"] = "medium"
                else:
                    action["git_ref_ids"].append(pseudonym(key, "git_ref", ref))
            return action
        return None
    if base in {"cp", "mv", "rsync", "install"}:
        tail = words[1:]
        target = _value_after(tail, {"-t", "--target-directory"})
        pos = _positional(
            tail,
            {
                "--exclude",
                "--include",
                "--chmod",
                "--chown",
                "-t",
                "--target-directory",
            },
        )
        sources = pos if target else pos[:-1]
        target = target or (pos[-1] if pos else None)
        if sources and target:
            action["action"] = "file_transfer"
            action["src_path_ids"] = []
            for x in sources:
                if dynamic_operand(x):
                    action["_dynamic_operands"] = action.get("_dynamic_operands", 0) + 1
                    action["confidence"] = "medium"
                elif pid := path_id(key, x, cwd):
                    action["src_path_ids"].append(pid)
            put_path("dst_path_id", target, cwd)
            return action
    if base in {"rm", "unlink"}:
        action["action"] = "file_delete"
        action["file_path_ids"] = []
        for x in _positional(words[1:], set()):
            if dynamic_operand(x):
                action["_dynamic_operands"] = action.get("_dynamic_operands", 0) + 1
                action["confidence"] = "medium"
            elif pid := path_id(key, x, cwd):
                action["file_path_ids"].append(pid)
        return action
    return None


def symbolic_actions(
    command: str, key: bytes, cwd: str | None = None
) -> tuple[list[dict[str, Any]], dict[str, int]]:
    segments, stats = shell_segments(command)
    actions = []
    for segment in segments:
        found = classify_segment(segment, key, cwd)
        if found:
            stats["dynamic_operands"] = stats.get("dynamic_operands", 0) + found.pop(
                "_dynamic_operands", 0
            )
            actions.append(found)
        else:
            stats["unmatched_segments"] = stats.get("unmatched_segments", 0) + 1
    implies_success = not any(
        stats.get(label, 0)
        for label in (
            "non_conjunctive_chain",
            "pipeline_or_background",
            "heredoc_skipped",
            "dynamic_segments",
            "unparseable_segments",
        )
    )
    for action in actions:
        action["tool_success_implies_action_success"] = implies_success
    return actions, stats


def outcome(output: object, claude_error: bool | None = None) -> dict[str, Any]:
    decoded = json_object(output)
    metadata = (
        decoded.get("metadata") if isinstance(decoded.get("metadata"), dict) else {}
    )
    code = metadata.get("exit_code")
    if isinstance(code, int):
        return {
            "status": "succeeded" if code == 0 else "failed",
            "exit_code": code,
            "confidence": "high",
        }
    if claude_error is True:
        return {"status": "failed", "confidence": "high"}
    if claude_error is False:
        return {"status": "succeeded", "confidence": "high"}
    text = (
        "\n".join(str(x.get("text", "")) for x in output if isinstance(x, dict))
        if isinstance(output, list)
        else str(output)
    )
    if text.startswith("Script completed"):
        return {"wrapper_status": "completed", "confidence": "medium"}
    return {"status": "reported", "confidence": "low"}


def extract(
    path: Path,
    source: str,
    key: bytes,
    start: datetime | None = None,
    end: datetime | None = None,
    include_empty_tool_calls: bool = False,
    content_hash: str | None = None,
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    rows, calls, stats = [], {}, Counter()
    variants, included_variants = Counter(), Counter()
    session_id, session_timestamp, session_cwd, known_parent, parent_in_range = (
        path.stem,
        None,
        None,
        None,
        False,
    )
    session_ordinal = 0
    source_log_id = pseudonym(
        key, "source_log", source + "\0" + (content_hash or file_hash(path))
    )
    bounded = start is not None or end is not None
    with path.open(encoding="utf-8", errors="replace") as handle:
        for line_no, line in enumerate(handle, 1):
            stats["events_scanned"] += 1
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                stats["malformed_lines"] += 1
                continue
            timestamp = event.get("timestamp")
            when = parse_time(timestamp)
            payload = (
                event.get("payload") if isinstance(event.get("payload"), dict) else {}
            )
            ptype = payload.get("type")
            variant = (
                str(ptype)
                if source == "codex"
                and ptype
                in {
                    "function_call",
                    "custom_tool_call",
                    "function_call_output",
                    "custom_tool_call_output",
                }
                else None
            )
            if source == "claude":
                probe = (event.get("message") or {}).get("content", [])
                kinds = (
                    sorted(
                        {
                            str(b.get("type"))
                            for b in probe
                            if isinstance(b, dict)
                            and b.get("type") in {"tool_use", "tool_result"}
                        }
                    )
                    if isinstance(probe, list)
                    else []
                )
                if kinds:
                    variant = "claude_" + "+".join(kinds)
            if variant:
                variants[variant] += 1
            if source == "codex" and event.get("type") == "session_meta":
                git_meta = (
                    payload.get("git") if isinstance(payload.get("git"), dict) else {}
                )
                session_id, session_timestamp, session_cwd = (
                    str(payload.get("id") or session_id),
                    timestamp,
                    payload.get("cwd") or git_meta.get("cwd"),
                )
                session_ordinal = line_no
                parent = payload.get("forked_from_id")
                sub = (
                    payload.get("source", {}).get("subagent", {})
                    if isinstance(payload.get("source"), dict)
                    else {}
                )
                parent = (
                    (sub.get("thread_spawn") or {}).get("parent_thread_id")
                    if isinstance(sub, dict)
                    and isinstance(sub.get("thread_spawn"), dict)
                    else parent
                )
                if parent:
                    known_parent = parent
            if bounded and when is None:
                stats["unknown_timestamps_excluded"] += 1
                continue
            in_range = not ((start and when < start) or (end and when >= end))
            if event.get("type") == "session_meta" and known_parent and in_range:
                parent_in_range = True
            if not in_range:
                continue
            stats["events_in_range"] += 1
            if variant:
                included_variants[variant] += 1
            blocks: list[tuple[str, str, str, object]] = []
            if source == "codex" and ptype in {"function_call", "custom_tool_call"}:
                blocks = [
                    (
                        str(payload.get("call_id") or payload.get("id") or line_no),
                        str(payload.get("name") or "custom"),
                        "call",
                        payload.get("arguments", payload.get("input", {})),
                    )
                ]
            elif source == "claude":
                session_id = str(event.get("sessionId") or session_id)
                content = (event.get("message") or {}).get("content", [])
                if isinstance(content, list):
                    blocks = [
                        (
                            str(b.get("id") or line_no),
                            str(b.get("name") or ""),
                            str(b.get("type")),
                            b,
                        )
                        for b in content
                        if isinstance(b, dict)
                    ]
            for cid, name, btype, raw in blocks:
                if btype == "tool_result":
                    rid = str(raw.get("tool_use_id") or "")
                    if rid in calls:
                        error_flag = (
                            raw.get("is_error")
                            if isinstance(raw.get("is_error"), bool)
                            else None
                        )
                        result = outcome(raw.get("content", ""), error_flag)
                        rows[calls[rid]]["outcome"] = result
                        stats[
                            "outcome_"
                            + (
                                result.get("status")
                                or "wrapper_" + result.get("wrapper_status", "unknown")
                            )
                        ] += 1
                    else:
                        stats["unmatched_outputs"] += 1
                    continue
                arguments = raw.get("input", {}) if btype == "tool_use" else raw
                command, cwd = command_from(name, arguments)
                row = {
                    "record_type": "tool_call",
                    "source": source,
                    "source_log_id": source_log_id,
                    "event_ordinal": line_no,
                    "session_id": pseudonym(key, "session", session_id),
                    "call_id": pseudonym(key, "call", source_log_id + "\0" + cid),
                    "timestamp": timestamp,
                    "tool": name,
                    "confidence": "high",
                }
                if command is not None:
                    actions, coverage = symbolic_actions(
                        command, key, cwd or session_cwd
                    )
                    for ai, action in enumerate(actions):
                        action.update(
                            action_index=ai,
                            action_event_id=action_identity(
                                key, source_log_id, line_no, None, ai
                            ),
                        )
                    row.update(
                        command_id=pseudonym(key, "command", command), actions=actions
                    )
                    stats.update(coverage)
                elif name in {"exec", "functions.exec"} and isinstance(arguments, str):
                    nested, dynamic = nested_exec_commands(arguments)
                    stats["dynamic_exec_calls"] += dynamic
                    packed = []
                    for ni, (cmd, workdir) in enumerate(nested):
                        actions, coverage = symbolic_actions(
                            cmd, key, workdir or session_cwd
                        )
                        stats.update(coverage)
                        for ai, action in enumerate(actions):
                            action.update(
                                action_index=ai,
                                action_event_id=action_identity(
                                    key, source_log_id, line_no, ni, ai
                                ),
                            )
                        packed.append(
                            {
                                "nested_command_index": ni,
                                "command_id": pseudonym(key, "command", cmd),
                                "actions": actions,
                            }
                        )
                    row["nested_commands"] = packed
                has = bool(row.get("actions")) or any(
                    x.get("actions") for x in row.get("nested_commands", [])
                )
                if has or include_empty_tool_calls:
                    calls[cid] = len(rows)
                    rows.append(row)
                else:
                    stats["empty_tool_calls_omitted"] += 1
            if source == "codex" and ptype in {
                "function_call_output",
                "custom_tool_call_output",
            }:
                cid = str(payload.get("call_id") or payload.get("id") or "")
                if cid in calls:
                    # For functions.exec rows this outcome belongs to the JS
                    # wrapper. Nested commands deliberately have no individual
                    # success field unless their own authoritative output exists.
                    result = outcome(payload.get("output", payload.get("result", "")))
                    rows[calls[cid]]["outcome"] = result
                    stats[
                        "outcome_"
                        + (
                            result.get("status")
                            or "wrapper_" + result.get("wrapper_status", "unknown")
                        )
                    ] += 1
                else:
                    stats["unmatched_outputs"] += 1
    if rows:
        if known_parent:
            rows.insert(
                0,
                {
                    "record_type": "lineage",
                    "source": source,
                    "source_log_id": source_log_id,
                    "event_ordinal": session_ordinal,
                    "session_id": pseudonym(key, "session", session_id),
                    "parent_session_id": pseudonym(key, "session", known_parent),
                    "timestamp": session_timestamp,
                    "confidence": "high",
                    "metadata_outside_window": bounded and not parent_in_range,
                },
            )
        rows.insert(
            0,
            {
                "record_type": "session",
                "source": source,
                "source_log_id": source_log_id,
                "event_ordinal": session_ordinal,
                "session_id": pseudonym(key, "session", session_id),
                "timestamp": session_timestamp,
                "confidence": "high" if session_timestamp else "medium",
            },
        )
    for label in (
        "outcome_succeeded",
        "outcome_failed",
        "outcome_reported",
        "outcome_wrapper_completed",
        "unmatched_outputs",
    ):
        stats[label] += 0
    stats["schema_variants_scanned"] = dict(variants)
    stats["schema_variants_included"] = dict(included_variants)
    return rows, dict(stats)


def log_paths(
    codex_roots: list[Path], claude_roots: list[Path]
) -> list[tuple[Path, str]]:
    found = []
    for root, source in [(r, "codex") for r in codex_roots] + [
        (r, "claude") for r in claude_roots
    ]:
        if root.is_file():
            found.append((root, source))
        elif root.exists():
            found.extend((p, source) for p in root.rglob("*.jsonl"))
    return sorted(set(found), key=lambda x: (x[1], str(x[0])))


def file_hash(path: Path) -> str:
    return "sha256:" + hashlib.sha256(path.read_bytes()).hexdigest()


def inventory_hash(items: list[tuple[str, str]]) -> str:
    payload = "".join(
        f"{source}\0{digest}\n" for source, digest in sorted(items)
    ).encode()
    return "sha256:" + hashlib.sha256(payload).hexdigest()


def main() -> None:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--codex-root", action="append", type=Path, default=[])
    p.add_argument("--claude-root", action="append", type=Path, default=[])
    p.add_argument("--key-file", required=True, type=Path)
    p.add_argument("--start")
    p.add_argument("--end")
    p.add_argument("--out", required=True, type=Path)
    p.add_argument("--manifest", type=Path)
    p.add_argument("--include-empty-tool-calls", action="store_true")
    args = p.parse_args()
    if not args.codex_root and not args.claude_root:
        p.error("at least one explicit source root is required")
    try:
        start, end = strict_time(args.start, "--start"), strict_time(args.end, "--end")
    except argparse.ArgumentTypeError as exc:
        p.error(str(exc))
    if start and end and start >= end:
        p.error("--start must precede --end")
    key = args.key_file.read_bytes()
    if not key:
        p.error("--key-file must not be empty")
    all_rows = []
    totals = Counter()
    variants = Counter()
    included_variants = Counter()
    files_by_source = Counter()
    included_by_source = Counter()
    inventory = []
    for path, source in log_paths(args.codex_root, args.claude_root):
        digest = file_hash(path)
        inventory.append((source, digest))
        files_by_source[source] += 1
        rows, stats = extract(
            path, source, key, start, end, args.include_empty_tool_calls, digest
        )
        all_rows.extend(rows)
        if rows:
            included_by_source[source] += 1
        variants.update(stats.pop("schema_variants_scanned", {}))
        included_variants.update(stats.pop("schema_variants_included", {}))
        totals.update(stats)
    all_rows.sort(
        key=lambda r: (
            r.get("timestamp") or "",
            r["source"],
            r["session_id"],
            r["record_type"],
            r.get("call_id", ""),
        )
    )
    args.out.parent.mkdir(parents=True, exist_ok=True)
    data = "".join(json.dumps(r, sort_keys=True) + "\n" for r in all_rows)
    args.out.write_text(data, encoding="utf-8")
    actions = [a for r in all_rows for a in r.get("actions", [])] + [
        a
        for r in all_rows
        for n in r.get("nested_commands", [])
        for a in n.get("actions", [])
    ]
    per_source = {
        source: {
            "count": sum(s == source for s, _ in inventory),
            "hash": inventory_hash([(s, h) for s, h in inventory if s == source]),
        }
        for source in sorted(files_by_source)
    }
    manifest = {
        "schema": SCHEMA,
        "window": {"start": args.start, "end": args.end},
        "sources": {
            "scanned": dict(sorted(files_by_source.items())),
            "included": dict(sorted(included_by_source.items())),
        },
        "input_inventory": {
            "count": len(inventory),
            "hash": inventory_hash(inventory),
            "by_source": per_source,
        },
        "config": {"include_empty_tool_calls": args.include_empty_tool_calls},
        "key_fingerprint": "sha256:" + hashlib.sha256(key).hexdigest()[:16],
        "script_hash": file_hash(Path(__file__)),
        "output_hash": "sha256:" + hashlib.sha256(data.encode()).hexdigest(),
        "rows": len(all_rows),
        "coverage": dict(sorted(totals.items())),
        "schema_variants_scanned": dict(sorted(variants.items())),
        "schema_variants_included": dict(sorted(included_variants.items())),
        "by_record_type": dict(
            sorted(Counter(r["record_type"] for r in all_rows).items())
        ),
        "by_action": dict(sorted(Counter(a["action"] for a in actions).items())),
        "by_action_confidence": dict(
            sorted(Counter(a["confidence"] for a in actions).items())
        ),
    }
    target = args.manifest or args.out.with_suffix(args.out.suffix + ".manifest.json")
    target.write_text(
        json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )


if __name__ == "__main__":
    main()
