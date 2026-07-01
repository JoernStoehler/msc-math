#!/usr/bin/env python3
"""
Extract a compact, chat-level text view from a Codex rollout JSONL file.

The PaperOrchestra aggregator wants experiment/research signal, not raw tool
noise. This helper keeps user messages, user-visible assistant messages,
assistant text messages, compact subagent completions, and short tool-call
summaries while dropping large tool outputs by default.
"""

import argparse
import json
from pathlib import Path


def text_from_content(content) -> str:
    if isinstance(content, str):
        return content
    if not isinstance(content, list):
        return ""
    parts: list[str] = []
    for item in content:
        if isinstance(item, dict):
            text = item.get("text")
            if isinstance(text, str):
                parts.append(text)
    return "\n".join(parts)


def clip(text: str, limit: int) -> str:
    text = text.strip()
    if len(text) <= limit:
        return text
    return text[:limit].rstrip() + "\n[... clipped ...]"


def is_bootstrap_user_message(text: str) -> bool:
    stripped = text.lstrip()
    return (
        stripped.startswith("# AGENTS.md instructions for ")
        or stripped.startswith("<environment_context>")
    )


def extract(
    path: Path,
    include_tool_calls: bool,
    include_tool_outputs: bool,
    max_text_chars: int,
) -> str:
    chunks: list[str] = []
    seen_messages: set[tuple[str, str]] = set()
    with path.open("r", encoding="utf-8", errors="replace") as f:
        for line in f:
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue
            timestamp = event.get("timestamp", "")
            payload = event.get("payload")
            if not isinstance(payload, dict):
                continue
            ptype = payload.get("type")

            if event.get("type") == "session_meta":
                session_id = payload.get("id", "")
                cwd = payload.get("cwd") or (payload.get("git") or {}).get("cwd", "")
                forked = payload.get("forked_from_id", "")
                chunks.append(
                    f"\n## session_meta {timestamp}\n"
                    f"- id: {session_id}\n- cwd: {cwd}\n- forked_from: {forked}"
                )
                continue

            if ptype == "turn_context":
                cwd = payload.get("cwd", "")
                if cwd:
                    chunks.append(f"\n## turn_context {timestamp}\n- cwd: {cwd}")
                continue

            if ptype == "user_message":
                message = payload.get("message", "")
                if is_bootstrap_user_message(str(message)):
                    continue
                dedupe_key = ("user", str(message).strip())
                if dedupe_key in seen_messages:
                    continue
                seen_messages.add(dedupe_key)
                chunks.append(f"\n## user {timestamp}\n{clip(str(message), max_text_chars)}")
                continue

            if ptype == "agent_message":
                phase = payload.get("phase", "")
                message = payload.get("message", "")
                dedupe_key = ("assistant", str(message).strip())
                if dedupe_key in seen_messages:
                    continue
                seen_messages.add(dedupe_key)
                chunks.append(
                    f"\n## assistant_visible {timestamp} {phase}\n"
                    f"{clip(str(message), max_text_chars)}"
                )
                continue

            if ptype == "message":
                role = payload.get("role", "")
                if role in {"assistant", "user"}:
                    phase = payload.get("phase", "")
                    text = text_from_content(payload.get("content"))
                    if role == "user" and is_bootstrap_user_message(text):
                        continue
                    if text.strip():
                        dedupe_key = (role, text.strip())
                        if dedupe_key in seen_messages:
                            continue
                        seen_messages.add(dedupe_key)
                        chunks.append(
                            f"\n## {role}_message {timestamp} {phase}\n"
                            f"{clip(text, max_text_chars)}"
                        )
                continue

            if ptype == "function_call":
                if not include_tool_calls:
                    continue
                name = payload.get("name", "")
                call_id = payload.get("call_id", "")
                args = payload.get("arguments", "")
                chunks.append(
                    f"\n## tool_call {timestamp}\n"
                    f"- name: {name}\n- call_id: {call_id}\n"
                    f"- arguments: {clip(str(args), 1200)}"
                )
                continue

            if include_tool_outputs and ptype == "function_call_output":
                call_id = payload.get("call_id", "")
                output = payload.get("output", "")
                chunks.append(
                    f"\n## tool_output {timestamp}\n"
                    f"- call_id: {call_id}\n{clip(str(output), max_text_chars)}"
                )

    return "\n".join(chunks).strip() + "\n"


def main() -> None:
    parser = argparse.ArgumentParser(description="Extract chat-level text from a Codex rollout JSONL")
    parser.add_argument("rollout", help="Path to rollout-*.jsonl")
    parser.add_argument("--include-tool-calls", action="store_true",
                        help="Include clipped tool call names/arguments. Default omits them.")
    parser.add_argument("--include-tool-outputs", action="store_true",
                        help="Include clipped tool outputs. Default omits them.")
    parser.add_argument("--max-text-chars", type=int, default=6000,
                        help="Maximum characters kept per text payload")
    args = parser.parse_args()

    print(
        extract(
            Path(args.rollout),
            args.include_tool_calls,
            args.include_tool_outputs,
            args.max_text_chars,
        ),
        end="",
    )


if __name__ == "__main__":
    main()
