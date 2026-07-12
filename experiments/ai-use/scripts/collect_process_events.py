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
import sqlite3
import tempfile
import unicodedata
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


def normalize_visible_text(text: str) -> str:
    return " ".join(unicodedata.normalize("NFKC", text).split())


def message_tokens(text: str) -> list[str]:
    return re.findall(r"\w+|[^\w\s]", text.casefold(), flags=re.UNICODE)


def winnowed_fingerprints(
    tokens: list[str], key: bytes, shingle_size: int = 5, window: int = 4
) -> list[str]:
    if len(tokens) < shingle_size:
        return []
    hashes = [
        pseudonym(key, "message_shingle", "\0".join(tokens[i : i + shingle_size]))
        for i in range(len(tokens) - shingle_size + 1)
    ]
    if len(hashes) <= window:
        return sorted(set(hashes))
    selected: list[tuple[int, str]] = []
    for i in range(len(hashes) - window + 1):
        chunk = hashes[i : i + window]
        minimum = min(chunk)
        right = i + max(j for j, value in enumerate(chunk) if value == minimum)
        if not selected or selected[-1][0] != right:
            selected.append((right, minimum))
    return [value for _, value in selected]


SUBAGENT_ENVELOPE = re.compile(
    r"^Message Type: (?:NEW_TASK|MESSAGE|FINAL_ANSWER)\s*\n"
    r"Task name: .+\nSender: .+\nPayload:\s*\n",
    re.MULTILINE,
)
SYSTEM_REMINDER = re.compile(r"</?system-reminder>|<system-reminder\b", re.I)
CODEX_AGENTS_CONTEXT = re.compile(
    r"^(?:<user_instructions>\s*# AGENTS\.md\b.*?</user_instructions>|"
    r"# AGENTS\.md instructions for .+?<INSTRUCTIONS>.*?</INSTRUCTIONS>"
    r"(?:\s*<environment_context>.*?</environment_context>)?)\s*$",
    re.DOTALL,
)


def classify_message_origin(role: str, text: str, source: str) -> tuple[str, str]:
    if role == "agent":
        return "agent", "agent_output"
    # The multiline envelope is checked before whitespace normalization.
    if source == "codex" and SUBAGENT_ENVELOPE.match(text.strip()):
        return "nonhuman_agent", "subagent_delivery"
    if source == "codex" and CODEX_AGENTS_CONTEXT.match(text.strip()):
        return "nonhuman_injected", "system_injection"
    if source == "claude" and SYSTEM_REMINDER.search(text):
        stripped = text.strip().casefold()
        if stripped.startswith("<system-reminder") and stripped.endswith(
            "</system-reminder>"
        ):
            return "nonhuman_injected", "system_injection"
        return "mixed_or_injected", "mixed_system_injection"
    return "human_user_candidate", "direct_user_prompt"


def model_era(model: str) -> str:
    gpt = re.search(r"\bgpt-(\d+(?:\.\d+)?)", model, re.I)
    if gpt:
        return "gpt-" + gpt.group(1)
    claude = re.search(r"\bclaude-(?:[a-z]+-)?(\d+)[-.](\d+)", model, re.I)
    if claude:
        return f"claude-{claude.group(1)}.{claude.group(2)}"
    return "unknown"


def safe_provider(provider: str) -> str:
    normalized = provider.casefold()
    return (
        normalized if normalized in {"openai", "anthropic", "azure_openai"} else "other"
    )


def visible_messages(
    event: dict[str, Any], source: str
) -> tuple[list[tuple[str, str, str, str]], Counter[str]]:
    """Extract visible user/agent prose, excluding instructions and opaque blocks."""
    raw_found: list[tuple[str, str]] = []
    coverage: Counter[str] = Counter()
    if source == "codex":
        payload = event.get("payload") if isinstance(event.get("payload"), dict) else {}
        ptype, role = payload.get("type"), payload.get("role")
        if ptype == "user_message" and isinstance(payload.get("message"), str):
            raw_found.append(("user", payload["message"]))
            coverage["message_schema_codex_user_message"] += 1
        elif ptype == "agent_message" and isinstance(payload.get("message"), str):
            raw_found.append(("agent", payload["message"]))
            coverage["message_schema_codex_agent_message"] += 1
        elif ptype == "message" and role in {"user", "assistant"}:
            mapped = "agent" if role == "assistant" else "user"
            content = payload.get("content", [])
            if isinstance(content, list):
                texts = [
                    block.get("text")
                    for block in content
                    if isinstance(block, dict)
                    and block.get("type") in {"input_text", "output_text", "text"}
                    and isinstance(block.get("text"), str)
                ]
                if texts:
                    raw_found.append((mapped, "\n".join(texts)))
                    coverage["message_schema_codex_role_message"] += 1
        elif ptype in {
            "developer_message",
            "system_message",
            "reasoning",
            "encrypted_content",
        } or role in {"developer", "system"}:
            coverage["message_blocks_excluded"] += 1
    else:
        message = event.get("message") if isinstance(event.get("message"), dict) else {}
        role = message.get("role")
        if role in {"user", "assistant"}:
            mapped = "agent" if role == "assistant" else "user"
            content = message.get("content")
            if isinstance(content, str):
                raw_found.append((mapped, content))
                coverage["message_schema_claude_string"] += 1
            elif isinstance(content, list):
                texts = []
                for block in content:
                    if not isinstance(block, dict):
                        continue
                    if block.get("type") == "text" and isinstance(
                        block.get("text"), str
                    ):
                        texts.append(block["text"])
                    elif block.get("type") in {
                        "tool_result",
                        "tool_use",
                        "thinking",
                        "redacted_thinking",
                    }:
                        coverage["message_blocks_excluded"] += 1
                if texts:
                    raw_found.append((mapped, "\n".join(texts)))
                    coverage["message_schema_claude_blocks"] += 1
        elif role in {"system", "developer"}:
            coverage["message_blocks_excluded"] += 1
    found = [
        (role, text, *classify_message_origin(role, text, source))
        for role, text in raw_found
    ]
    if (
        source == "claude"
        and event.get("isSidechain") is True
        and isinstance(event.get("agentId"), str)
    ):
        found = [
            (role, text, "nonhuman_agent", "subagent_delivery")
            if role == "user"
            else (role, text, origin, delivery)
            for role, text, origin, delivery in found
        ]
        coverage["message_schema_claude_sidechain"] += sum(
            role == "user" for role, *_ in found
        )
    return found, coverage


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
    include_message_fingerprints: bool = False,
    message_min_chars: int = 40,
    message_fingerprint_mode: str = "reuse",
) -> tuple[list[dict[str, Any]], dict[str, Any]]:
    if message_fingerprint_mode not in {"task-frame", "reuse"}:
        raise ValueError("message_fingerprint_mode must be 'task-frame' or 'reuse'")
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
    session_repo_identity: str | None = None
    explicit_models: set[str] = set()
    explicit_providers: set[str] = set()
    current_model: str | None = None
    current_provider: str | None = None
    current_model_provenance: str | None = None
    session_is_sidechain = False
    source_log_id = pseudonym(
        key, "source_log", source + "\0" + (content_hash or file_hash(path))
    )
    bounded = start is not None or end is not None
    metadata_before_window_used = False
    seen_messages: dict[tuple[str, str, str], int] = {}
    # Some older Codex rollouts place the initial user request before the first
    # session_meta row. Establish identity and event-time model metadata before
    # assigning those messages to a session.
    if source == "codex":
        with path.open(encoding="utf-8", errors="replace") as metadata_handle:
            for metadata_line_no, metadata_line in enumerate(metadata_handle, 1):
                try:
                    metadata_event = json.loads(metadata_line)
                except json.JSONDecodeError:
                    continue
                if metadata_event.get("type") != "session_meta":
                    continue
                metadata_timestamp = metadata_event.get("timestamp")
                metadata_when = parse_time(metadata_timestamp)
                if end is not None and (
                    metadata_when is None or metadata_when >= end
                ):
                    continue
                metadata_payload = metadata_event.get("payload")
                if not isinstance(metadata_payload, dict):
                    continue
                metadata_git = metadata_payload.get("git")
                if not isinstance(metadata_git, dict):
                    metadata_git = {}
                session_id = str(metadata_payload.get("id") or session_id)
                session_timestamp = metadata_timestamp
                session_ordinal = metadata_line_no
                session_cwd = metadata_payload.get("cwd") or metadata_git.get("cwd")
                repository_url = metadata_git.get("repository_url")
                if isinstance(repository_url, str):
                    session_repo_identity = repository_url
                provider = metadata_payload.get("model_provider") or metadata_payload.get(
                    "provider"
                )
                if isinstance(provider, str):
                    explicit_providers.add(provider)
                    current_provider = provider
                meta_model = metadata_payload.get("model")
                if isinstance(meta_model, str):
                    explicit_models.add(meta_model)
                    current_model = meta_model
                    current_model_provenance = "codex_session_meta"
                break
    with path.open(encoding="utf-8", errors="replace") as handle:
        for line_no, line in enumerate(handle, 1):
            stats["events_scanned"] += 1
            if bounded:
                prefix = line[:512]
                timestamp_match = re.match(r'\{"timestamp":"([^"]+)"', prefix)
                prefix_when = parse_time(timestamp_match.group(1)) if timestamp_match else None
                if prefix_when is not None:
                    after_window = end is not None and prefix_when >= end
                    before_window = start is not None and prefix_when < start
                    codex_baseline = source == "codex" and (
                        '"type":"session_meta"' in prefix or '"type":"turn_context"' in prefix
                    )
                    if after_window or (before_window and not codex_baseline):
                        stats["events_outside_window_prefiltered"] += 1
                        continue
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
            metadata_allowed = not bounded or (
                when is not None and (end is None or when < end)
            )
            metadata_before = bool(metadata_allowed and start and when and when < start)
            metadata_event = False
            if (
                source == "codex"
                and event.get("type") == "session_meta"
                and metadata_allowed
            ):
                metadata_event = True
                git_meta = (
                    payload.get("git") if isinstance(payload.get("git"), dict) else {}
                )
                session_id = str(payload.get("id") or session_id)
                session_cwd = payload.get("cwd") or git_meta.get("cwd")
                if session_timestamp is None:
                    session_timestamp = timestamp
                    session_ordinal = line_no
                repository_url = git_meta.get("repository_url")
                if isinstance(repository_url, str):
                    session_repo_identity = repository_url
                provider = payload.get("model_provider") or payload.get("provider")
                if isinstance(provider, str):
                    explicit_providers.add(provider)
                    current_provider = provider
                meta_model = payload.get("model")
                if isinstance(meta_model, str):
                    explicit_models.add(meta_model)
                    current_model, current_model_provenance = (
                        meta_model,
                        "codex_session_meta",
                    )
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
            if (
                source == "codex"
                and event.get("type") == "turn_context"
                and metadata_allowed
            ):
                metadata_event = True
                context_cwd = payload.get("cwd")
                if isinstance(context_cwd, str):
                    session_cwd = context_cwd
                context_model = payload.get("model")
                if isinstance(context_model, str):
                    explicit_models.add(context_model)
                    current_model, current_model_provenance = (
                        context_model,
                        "codex_turn_context",
                    )
            if source == "claude" and metadata_allowed:
                metadata_event = True
                root_session = event.get("sessionId")
                sidechain = event.get("isSidechain") is True and isinstance(
                    event.get("agentId"), str
                )
                if sidechain:
                    session_is_sidechain = True
                    session_id = str(event["agentId"])
                    if isinstance(root_session, str):
                        known_parent = root_session
                elif isinstance(root_session, str):
                    session_id = root_session
                if session_timestamp is None:
                    session_timestamp, session_ordinal = timestamp, line_no
                claude_cwd = event.get("cwd")
                if isinstance(claude_cwd, str):
                    session_cwd = claude_cwd
                message_obj = (
                    event.get("message")
                    if isinstance(event.get("message"), dict)
                    else {}
                )
                claude_model = message_obj.get("model") or event.get("model")
                if isinstance(claude_model, str):
                    explicit_models.add(claude_model)
                    current_model, current_model_provenance = (
                        claude_model,
                        "claude_message",
                    )
                explicit_providers.add("anthropic")
                current_provider = "anthropic"
            if metadata_event:
                if metadata_before:
                    metadata_before_window_used = True
                    stats["metadata_before_window"] += 1
                else:
                    stats["metadata_in_window"] += 1
            if bounded and when is None:
                stats["unknown_timestamps_excluded"] += 1
                continue
            in_range = not ((start and when < start) or (end and when >= end))
            if (
                known_parent
                and in_range
                and (event.get("type") == "session_meta" or source == "claude")
            ):
                parent_in_range = True
            if not in_range:
                continue
            stats["events_in_range"] += 1
            if variant:
                included_variants[variant] += 1
            if include_message_fingerprints:
                messages, message_coverage = visible_messages(event, source)
                stats.update(message_coverage)
                for message_index, (
                    role,
                    text,
                    message_origin,
                    delivery_kind,
                ) in enumerate(messages):
                    stats["message_candidates"] += 1
                    normalized = normalize_visible_text(text)
                    above_reuse_threshold = len(normalized) >= message_min_chars
                    reuse_eligible = message_fingerprint_mode == "reuse" and above_reuse_threshold
                    if not above_reuse_threshold:
                        stats["messages_below_threshold"] += 1
                    normalized_id = pseudonym(key, "normalized_message", normalized)
                    # Collapse only parallel schema representations at the same
                    # timestamp. Later repetitions remain separate chronology.
                    dedup_key = (
                        role,
                        normalized_id,
                        str(timestamp)
                        if timestamp is not None
                        else f"ordinal:{line_no}",
                    )
                    message_id = pseudonym(
                        key,
                        "message",
                        f"{source_log_id}\0{line_no}\0{message_index}\0{role}",
                    )
                    if dedup_key in seen_messages:
                        stats["duplicate_message_representations"] += 1
                        rows[seen_messages[dedup_key]].setdefault(
                            "duplicate_schema_occurrences", []
                        ).append(
                            {
                                "message_id": message_id,
                                "event_ordinal": line_no,
                                "timestamp": timestamp,
                            }
                        )
                        continue
                    tokens = message_tokens(normalized)
                    rows.append(
                        {
                            "record_type": "message_fingerprint",
                            "source": source,
                            "source_log_id": source_log_id,
                            "event_ordinal": line_no,
                            "session_id": pseudonym(key, "session", session_id),
                            "timestamp": timestamp,
                            "role": role,
                            "message_origin": message_origin,
                            "delivery_kind": delivery_kind,
                            "message_id": message_id,
                            "normalized_text_id": normalized_id,
                            "char_count": len(normalized),
                            "token_count": len(tokens),
                            "reuse_eligible": reuse_eligible,
                            "shingle_size": 5 if message_fingerprint_mode == "reuse" else None,
                            "winnow_window": 4 if message_fingerprint_mode == "reuse" else None,
                            "winnowed_fingerprint_ids": winnowed_fingerprints(
                                tokens, key
                            )
                            if reuse_eligible
                            else [],
                        }
                    )
                    message_row = rows[-1]
                    if message_fingerprint_mode == "task-frame":
                        message_row.pop("shingle_size", None)
                        message_row.pop("winnow_window", None)
                        message_row.pop("winnowed_fingerprint_ids", None)
                    if current_model:
                        message_row["model_id_at_event"] = pseudonym(
                            key, "model", current_model
                        )
                        message_row["model_era_at_event"] = model_era(current_model)
                        message_row["model_metadata_provenance"] = (
                            current_model_provenance
                        )
                        stats["message_model_at_event_present"] += 1
                    else:
                        stats["message_model_at_event_missing"] += 1
                    if current_provider:
                        message_row["model_provider_at_event"] = safe_provider(
                            current_provider
                        )
                        message_row["model_provider_id_at_event"] = pseudonym(
                            key, "model_provider", current_provider
                        )
                        stats["message_provider_at_event_present"] += 1
                    else:
                        stats["message_provider_at_event_missing"] += 1
                    seen_messages[dedup_key] = len(rows) - 1
                    stats["message_fingerprints_emitted"] += 1
                    stats[f"message_mode_{message_fingerprint_mode}_emitted"] += 1
                    stats[f"message_origin_{message_origin}"] += 1
                    stats[f"message_delivery_{delivery_kind}"] += 1
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
        session_row = {
            "record_type": "session",
            "source": source,
            "source_log_id": source_log_id,
            "event_ordinal": session_ordinal,
            "session_id": pseudonym(key, "session", session_id),
            "timestamp": session_timestamp,
            "confidence": "high" if session_timestamp else "medium",
            "model_ids": sorted(
                pseudonym(key, "model", model) for model in explicit_models
            ),
            "model_eras": sorted({model_era(model) for model in explicit_models}),
            "model_providers": sorted(
                {safe_provider(provider) for provider in explicit_providers}
            ),
            "model_provider_ids": sorted(
                pseudonym(key, "model_provider", provider)
                for provider in explicit_providers
            ),
            "metadata_before_window": metadata_before_window_used,
            "session_kind": "sidechain"
            if session_is_sidechain
            else "primary_or_unknown",
        }
        if session_cwd:
            session_row["cwd_path_id"] = path_id(key, session_cwd, session_cwd)
            marker = f"{os.sep}.worktrees{os.sep}"
            if marker in os.path.normpath(session_cwd):
                repo_root, worktree_tail = os.path.normpath(session_cwd).split(
                    marker, 1
                )
                worktree_root = repo_root + marker + worktree_tail.split(os.sep, 1)[0]
                session_row["repo_path_id"] = path_id(key, repo_root, repo_root)
                session_row["worktree_path_id"] = path_id(
                    key, worktree_root, worktree_root
                )
            else:
                session_row["repo_path_id"] = path_id(key, session_cwd, session_cwd)
        if session_repo_identity:
            session_row["repository_id"] = pseudonym(
                key, "repository", session_repo_identity
            )
        rows.insert(
            0,
            session_row,
        )
        stats[
            "sessions_model_present" if explicit_models else "sessions_model_missing"
        ] += 1
        stats[
            "sessions_provider_present"
            if explicit_providers
            else "sessions_provider_missing"
        ] += 1
        stats["sessions_cwd_present" if session_cwd else "sessions_cwd_missing"] += 1
        stats[
            "sessions_sidechain"
            if session_is_sidechain
            else "sessions_primary_or_unknown"
        ] += 1
    for label in (
        "outcome_succeeded",
        "outcome_failed",
        "outcome_reported",
        "outcome_wrapper_completed",
        "unmatched_outputs",
        "sessions_model_present",
        "sessions_model_missing",
        "sessions_provider_present",
        "sessions_provider_missing",
        "sessions_cwd_present",
        "sessions_cwd_missing",
        "sessions_sidechain",
        "sessions_primary_or_unknown",
        "metadata_before_window",
        "metadata_in_window",
        "message_model_at_event_present",
        "message_model_at_event_missing",
        "message_provider_at_event_present",
        "message_provider_at_event_missing",
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
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return "sha256:" + digest.hexdigest()


def inventory_hash(items: list[tuple[str, str]]) -> str:
    payload = "".join(
        f"{source}\0{digest}\n" for source, digest in sorted(items)
    ).encode()
    return "sha256:" + hashlib.sha256(payload).hexdigest()


def private_write_text(path: Path, text: str) -> None:
    """Replace text with mode 0600, including an already permissive target."""
    path.parent.mkdir(parents=True, exist_ok=True)
    flags = (
        os.O_WRONLY
        | os.O_CREAT
        | getattr(os, "O_CLOEXEC", 0)
        | getattr(os, "O_NOFOLLOW", 0)
    )
    fd = os.open(path, flags, 0o600)
    try:
        os.fchmod(fd, 0o600)
        os.ftruncate(fd, 0)
        with os.fdopen(fd, "w", encoding="utf-8", closefd=False) as handle:
            handle.write(text)
            handle.flush()
            os.fsync(handle.fileno())
    finally:
        os.close(fd)


def private_write_lines(path: Path, lines: Any) -> tuple[str, int]:
    """Stream private text lines and return their SHA-256 and count."""
    path.parent.mkdir(parents=True, exist_ok=True)
    flags = os.O_WRONLY | os.O_CREAT | getattr(os, "O_CLOEXEC", 0) | getattr(os, "O_NOFOLLOW", 0)
    fd = os.open(path, flags, 0o600)
    digest, count = hashlib.sha256(), 0
    try:
        os.fchmod(fd, 0o600)
        os.ftruncate(fd, 0)
        with os.fdopen(fd, "wb", closefd=False) as handle:
            for line in lines:
                encoded = line.encode("utf-8")
                handle.write(encoded)
                digest.update(encoded)
                count += 1
            handle.flush()
            os.fsync(handle.fileno())
    finally:
        os.close(fd)
    return "sha256:" + digest.hexdigest(), count


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
    p.add_argument("--include-message-fingerprints", action="store_true")
    p.add_argument("--message-min-chars", type=int, default=40)
    p.add_argument(
        "--message-fingerprint-mode",
        choices=("task-frame", "reuse"),
        default="reuse",
    )
    args = p.parse_args()
    if not args.codex_root and not args.claude_root:
        p.error("at least one explicit source root is required")
    try:
        start, end = strict_time(args.start, "--start"), strict_time(args.end, "--end")
    except argparse.ArgumentTypeError as exc:
        p.error(str(exc))
    if start and end and start >= end:
        p.error("--start must precede --end")
    if args.message_min_chars < 1:
        p.error("--message-min-chars must be positive")
    try:
        key_stat = args.key_file.stat()
    except OSError as exc:
        p.error(f"cannot stat --key-file: {exc}")
    if args.key_file.is_file() and key_stat.st_mode & 0o077:
        p.error("--key-file must not be group- or world-readable; chmod it to 0600")
    key = args.key_file.read_bytes()
    if not key:
        p.error("--key-file must not be empty")
    args.out.parent.mkdir(parents=True, exist_ok=True)
    spool_fd, spool_name = tempfile.mkstemp(prefix=".process-events-", suffix=".sqlite", dir=args.out.parent)
    os.fchmod(spool_fd, 0o600)
    os.close(spool_fd)
    spool = sqlite3.connect(spool_name)
    spool.execute("PRAGMA journal_mode=OFF")
    spool.execute("PRAGMA synchronous=OFF")
    spool.execute("CREATE TABLE rows (timestamp TEXT, source TEXT, session_id TEXT, record_type TEXT, call_id TEXT, source_log_id TEXT, event_ordinal INTEGER, sequence INTEGER PRIMARY KEY, json TEXT)")
    sequence = 0
    totals = Counter()
    variants = Counter()
    included_variants = Counter()
    files_by_source = Counter()
    included_by_source = Counter()
    record_types = Counter()
    action_types = Counter()
    action_confidences = Counter()
    inventory = []
    for path, source in log_paths(args.codex_root, args.claude_root):
        digest = file_hash(path)
        inventory.append((source, digest))
        files_by_source[source] += 1
        rows, stats = extract(
            path,
            source,
            key,
            start,
            end,
            args.include_empty_tool_calls,
            digest,
            args.include_message_fingerprints,
            args.message_min_chars,
            args.message_fingerprint_mode,
        )
        for row in rows:
            encoded = json.dumps(row, sort_keys=True) + "\n"
            spool.execute(
                "INSERT INTO rows VALUES (?,?,?,?,?,?,?,?,?)",
                (
                    row.get("timestamp") or "",
                    row["source"],
                    row["session_id"],
                    row["record_type"],
                    row.get("call_id", ""),
                    row.get("source_log_id", ""),
                    int(row.get("event_ordinal") or 0),
                    sequence,
                    encoded,
                ),
            )
            sequence += 1
            record_types[row["record_type"]] += 1
            row_actions = list(row.get("actions", [])) + [
                action
                for nested in row.get("nested_commands", [])
                for action in nested.get("actions", [])
            ]
            action_types.update(action["action"] for action in row_actions)
            action_confidences.update(action["confidence"] for action in row_actions)
        spool.commit()
        if rows:
            included_by_source[source] += 1
        variants.update(stats.pop("schema_variants_scanned", {}))
        included_variants.update(stats.pop("schema_variants_included", {}))
        totals.update(stats)
    ordered = spool.execute(
        "SELECT json FROM rows ORDER BY timestamp,source,session_id,record_type,call_id,source_log_id,event_ordinal,sequence"
    )
    output_hash, output_rows = private_write_lines(args.out, (row[0] for row in ordered))
    spool.close()
    os.unlink(spool_name)
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
        "config": {
            "include_empty_tool_calls": args.include_empty_tool_calls,
            "include_message_fingerprints": args.include_message_fingerprints,
            "message_min_chars": args.message_min_chars,
            "message_fingerprint_mode": args.message_fingerprint_mode,
            "message_shingle_size": 5,
            "message_winnow_window": 4,
        },
        "key_fingerprint": "sha256:" + hashlib.sha256(key).hexdigest()[:16],
        "script_hash": file_hash(Path(__file__)),
        "output_hash": output_hash,
        "rows": output_rows,
        "coverage": dict(sorted(totals.items())),
        "schema_variants_scanned": dict(sorted(variants.items())),
        "schema_variants_included": dict(sorted(included_variants.items())),
        "by_record_type": dict(sorted(record_types.items())),
        "by_action": dict(sorted(action_types.items())),
        "by_action_confidence": dict(sorted(action_confidences.items())),
    }
    target = args.manifest or args.out.with_suffix(args.out.suffix + ".manifest.json")
    private_write_text(target, json.dumps(manifest, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
