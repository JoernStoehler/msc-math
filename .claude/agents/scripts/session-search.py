#!/usr/bin/env python3
"""Search and inspect Claude Code session JSONL transcripts.

Subcommands:
    list      List sessions for this project with dates and titles
    structure Show session structure: compaction points, message counts, agent launches
    search    Search for text pattern in messages, return file:line references
    extract   Pretty-print specific line range from a session file

All output is designed for subagent consumption: file:line references, compact
summaries, no decoration. The session-search agent calls this script; see
.claude/agents/session-search.md for the agent definition.

JSONL format (Claude Code sessions):
    Each line is a JSON object. Key types:
    - user:      User message or tool result
    - assistant: Assistant message (may contain tool_use blocks)
    - system:    System events (subtype=compact_boundary for compaction)
    - progress:  Hook/tool progress events (not conversation content)
    - queue-operation: Session queue events
    - file-history-snapshot: File edit checkpoints
    - ai-title:  Auto-generated session title
    - attachment: File attachments

Session files live at:
    ~/.claude/projects/<project-slug>/<sessionId>.jsonl
Subagent logs at:
    ~/.claude/projects/<project-slug>/<sessionId>/subagents/agent-<id>.jsonl
    with adjacent .meta.json files containing agentType and description.
"""

import argparse
import json
import os
import re
import sys
from pathlib import Path


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

PROJECT_DIR = Path.home() / ".claude" / "projects" / "-workspaces-msc-math"

CONVERSATION_TYPES = {"user", "assistant", "system"}


def iter_records(path: str):
    """Yield (line_number, parsed_dict) for each line in a JSONL file."""
    with open(path) as f:
        for i, line in enumerate(f, 1):
            line = line.strip()
            if not line:
                continue
            try:
                yield i, json.loads(line)
            except json.JSONDecodeError:
                pass


def extract_text(message: dict) -> str:
    """Extract plain text from a message's content array."""
    content = message.get("content", [])
    if isinstance(content, str):
        return content
    parts = []
    for block in content:
        if isinstance(block, str):
            parts.append(block)
        elif isinstance(block, dict):
            if block.get("type") == "text":
                parts.append(block.get("text", ""))
    return "\n".join(parts)


def extract_tool_uses(message: dict) -> list[dict]:
    """Extract tool_use blocks from a message's content array."""
    content = message.get("content", [])
    if isinstance(content, str):
        return []
    tools = []
    for block in content:
        if isinstance(block, dict) and block.get("type") == "tool_use":
            tools.append(block)
    return tools


def summarize_tool_use(tool: dict, max_len: int = 120) -> str:
    """One-line summary of a tool_use block."""
    name = tool.get("name", "?")
    inp = tool.get("input", {})
    if name == "Agent":
        desc = inp.get("description", "")
        stype = inp.get("subagent_type", "")
        return f"Agent({stype}: {desc})"
    if name == "Bash":
        cmd = inp.get("command", "")
        if len(cmd) > max_len:
            cmd = cmd[:max_len] + "..."
        return f"Bash({cmd})"
    if name in ("Read", "Write", "Edit", "Glob", "Grep"):
        path = inp.get("file_path", inp.get("path", inp.get("pattern", "")))
        return f"{name}({path})"
    # Generic
    summary = json.dumps(inp)
    if len(summary) > max_len:
        summary = summary[:max_len] + "..."
    return f"{name}({summary})"


def find_session_file(session_id_or_path: str) -> str:
    """Resolve a session ID or path to an absolute JSONL file path."""
    p = Path(session_id_or_path)
    if p.exists():
        return str(p.resolve())
    # Try as session ID under project dir
    candidate = PROJECT_DIR / f"{session_id_or_path}.jsonl"
    if candidate.exists():
        return str(candidate)
    # Try partial match
    matches = list(PROJECT_DIR.glob(f"{session_id_or_path}*.jsonl"))
    if len(matches) == 1:
        return str(matches[0])
    if len(matches) > 1:
        print(f"Ambiguous session ID, matches: {[m.name for m in matches]}", file=sys.stderr)
        sys.exit(1)
    print(f"Session not found: {session_id_or_path}", file=sys.stderr)
    print(f"Looked in: {PROJECT_DIR}", file=sys.stderr)
    sys.exit(1)


# ---------------------------------------------------------------------------
# Subcommands
# ---------------------------------------------------------------------------

def cmd_list(args):
    """List sessions for this project."""
    if not PROJECT_DIR.exists():
        print(f"Project directory not found: {PROJECT_DIR}", file=sys.stderr)
        sys.exit(1)

    sessions = []
    for f in sorted(PROJECT_DIR.glob("*.jsonl")):
        stat = f.stat()
        sid = f.stem
        title = None
        first_user = None
        n_lines = 0
        has_compaction = False

        with open(f) as fh:
            for line in fh:
                n_lines += 1
                try:
                    obj = json.loads(line)
                except json.JSONDecodeError:
                    continue
                if obj.get("type") == "ai-title" and not title:
                    title = obj.get("aiTitle", "")
                if obj.get("type") == "user" and obj.get("userType") == "external" and not first_user:
                    first_user = extract_text(obj.get("message", {}))[:120]
                if obj.get("subtype") == "compact_boundary":
                    has_compaction = True

        # Check for subagent logs
        subagent_dir = PROJECT_DIR / sid / "subagents"
        n_subagents = len(list(subagent_dir.glob("*.jsonl"))) if subagent_dir.exists() else 0

        sessions.append({
            "id": sid,
            "modified": stat.st_mtime,
            "lines": n_lines,
            "title": title,
            "first_user": first_user,
            "compacted": has_compaction,
            "subagents": n_subagents,
        })

    # Sort by modification time, most recent first
    sessions.sort(key=lambda s: s["modified"], reverse=True)

    limit = args.limit or 20
    for s in sessions[:limit]:
        from datetime import datetime
        ts = datetime.fromtimestamp(s["modified"]).strftime("%Y-%m-%d %H:%M")
        label = s["title"] or (s["first_user"] or "")[:80]
        flags = []
        if s["compacted"]:
            flags.append("compacted")
        if s["subagents"]:
            flags.append(f"{s['subagents']} subagents")
        flag_str = f" [{', '.join(flags)}]" if flags else ""
        print(f"{s['id']}  {ts}  {s['lines']:>5} lines{flag_str}")
        if label:
            print(f"  {label}")


def cmd_structure(args):
    """Show session structure."""
    path = find_session_file(args.session)

    seen_uuids = set()
    compactions = []
    user_msgs = 0
    assistant_msgs = 0
    tool_uses = []
    agent_launches = []
    first_ts = None
    last_ts = None

    for lineno, obj in iter_records(path):
        uuid = obj.get("uuid")
        if uuid:
            if uuid in seen_uuids:
                continue  # deduplicate post-compaction replays
            seen_uuids.add(uuid)

        ts = obj.get("timestamp")
        if ts and not first_ts:
            first_ts = ts
        if ts:
            last_ts = ts

        t = obj.get("type")
        if t == "system" and obj.get("subtype") == "compact_boundary":
            meta = obj.get("compactMetadata", {})
            compactions.append({
                "line": lineno,
                "timestamp": ts,
                "trigger": meta.get("trigger", "?"),
                "pre_tokens": meta.get("preTokens", "?"),
            })
        elif t == "user" and obj.get("userType") == "external" and not obj.get("toolUseResult"):
            user_msgs += 1
        elif t == "assistant":
            assistant_msgs += 1
            for tool in extract_tool_uses(obj.get("message", {})):
                tool_uses.append({"line": lineno, "summary": summarize_tool_use(tool)})
                if tool.get("name") == "Agent":
                    inp = tool.get("input", {})
                    agent_launches.append({
                        "line": lineno,
                        "type": inp.get("subagent_type", "?"),
                        "description": inp.get("description", ""),
                        "background": inp.get("run_in_background", False),
                    })

    # Subagent files
    session_id = Path(path).stem
    subagent_dir = PROJECT_DIR / session_id / "subagents"
    subagent_files = []
    if subagent_dir.exists():
        for meta_f in sorted(subagent_dir.glob("*.meta.json")):
            try:
                meta = json.loads(meta_f.read_text())
                agent_id = meta_f.stem.replace(".meta", "")
                jsonl_f = subagent_dir / f"{agent_id}.jsonl"
                n_lines = sum(1 for _ in open(jsonl_f)) if jsonl_f.exists() else 0
                subagent_files.append({
                    "id": agent_id,
                    "type": meta.get("agentType", "?"),
                    "description": meta.get("description", ""),
                    "lines": n_lines,
                    "path": str(jsonl_f),
                })
            except (json.JSONDecodeError, OSError):
                pass

    # Output
    print(f"Session: {path}")
    print(f"Time: {first_ts} → {last_ts}")
    print(f"User messages: {user_msgs}, Assistant messages: {assistant_msgs}")
    print(f"Unique UUIDs: {len(seen_uuids)}")
    print()

    if compactions:
        print(f"Compactions: {len(compactions)}")
        for c in compactions:
            print(f"  Line {c['line']}: {c['timestamp']} (trigger={c['trigger']}, pre_tokens={c['pre_tokens']})")
        print()

    if agent_launches:
        print(f"Agent launches: {len(agent_launches)}")
        for a in agent_launches:
            bg = " [background]" if a["background"] else ""
            print(f"  Line {a['line']}: {a['type']} — {a['description']}{bg}")
        print()

    if subagent_files:
        print(f"Subagent transcript files: {len(subagent_files)}")
        for s in subagent_files:
            print(f"  {s['path']}  ({s['lines']} lines)")
            print(f"    type={s['type']}, description={s['description']}")
        print()

    if tool_uses:
        print(f"Tool uses: {len(tool_uses)} (showing first 30)")
        for tu in tool_uses[:30]:
            print(f"  {path}:{tu['line']} — {tu['summary']}")


def cmd_search(args):
    """Search for a pattern in conversation messages."""
    path = find_session_file(args.session)
    pattern = re.compile(args.pattern, re.IGNORECASE)

    seen_uuids = set()
    matches = []

    for lineno, obj in iter_records(path):
        uuid = obj.get("uuid")
        if uuid:
            if uuid in seen_uuids:
                continue
            seen_uuids.add(uuid)

        t = obj.get("type")
        if t not in CONVERSATION_TYPES:
            continue

        # Search in message text
        msg = obj.get("message", {})
        text = extract_text(msg)

        # Also search in tool_use summaries
        tool_summaries = []
        for tool in extract_tool_uses(msg):
            tool_summaries.append(summarize_tool_use(tool, max_len=500))
        all_text = text + "\n" + "\n".join(tool_summaries)

        if pattern.search(all_text):
            # Build a concise description of what this line contains
            if t == "user":
                if obj.get("toolUseResult"):
                    desc = f"tool_result"
                else:
                    desc = f"user: {text[:100]}"
            elif t == "assistant":
                tools = extract_tool_uses(msg)
                if tools:
                    desc = f"assistant: {'; '.join(summarize_tool_use(t) for t in tools[:3])}"
                else:
                    desc = f"assistant: {text[:100]}"
            elif t == "system":
                desc = f"system: subtype={obj.get('subtype', '?')}"
            else:
                desc = f"{t}: {text[:100]}"

            matches.append({"line": lineno, "desc": desc})

    # Also search subagent transcripts
    session_id = Path(path).stem
    subagent_dir = PROJECT_DIR / session_id / "subagents"
    subagent_matches = []
    if subagent_dir.exists() and not args.no_subagents:
        for jsonl_f in sorted(subagent_dir.glob("*.jsonl")):
            seen = set()
            for lineno, obj in iter_records(str(jsonl_f)):
                uuid = obj.get("uuid")
                if uuid:
                    if uuid in seen:
                        continue
                    seen.add(uuid)
                t = obj.get("type")
                if t not in CONVERSATION_TYPES:
                    continue
                text = extract_text(obj.get("message", {}))
                if pattern.search(text):
                    subagent_matches.append({
                        "file": str(jsonl_f),
                        "line": lineno,
                        "desc": f"{t}: {text[:100]}",
                    })

    # Output
    limit = args.limit or 50
    print(f"Pattern: {args.pattern}")
    print(f"Session: {path}")
    print(f"Matches in main transcript: {len(matches)}")
    print()

    for m in matches[:limit]:
        print(f"  {path}:{m['line']} — {m['desc']}")

    if subagent_matches:
        print(f"\nMatches in subagent transcripts: {len(subagent_matches)}")
        for m in subagent_matches[:limit]:
            print(f"  {m['file']}:{m['line']} — {m['desc']}")


def cmd_extract(args):
    """Pretty-print specific lines from a session file."""
    path = find_session_file(args.session)
    start = args.start
    end = args.end or start

    for lineno, obj in iter_records(path):
        if lineno < start:
            continue
        if lineno > end:
            break

        t = obj.get("type", "?")
        ts = obj.get("timestamp", "")
        uuid = obj.get("uuid", "")[:8] if obj.get("uuid") else ""

        print(f"--- {path}:{lineno}  type={t}  ts={ts}  uuid={uuid}... ---")

        if t in ("user", "assistant"):
            msg = obj.get("message", {})
            text = extract_text(msg)
            tools = extract_tool_uses(msg)

            if text.strip():
                # Truncate very long text
                if len(text) > 2000:
                    print(text[:2000])
                    print(f"  ... ({len(text)} chars total, truncated)")
                else:
                    print(text)

            for tool in tools:
                print(f"  [tool_use] {summarize_tool_use(tool, max_len=300)}")
                # For Agent calls, show the full prompt
                if tool.get("name") == "Agent":
                    prompt = tool.get("input", {}).get("prompt", "")
                    if len(prompt) > 500:
                        print(f"    prompt: {prompt[:500]}...")
                    else:
                        print(f"    prompt: {prompt}")

            if t == "user" and obj.get("toolUseResult"):
                tr = obj["toolUseResult"]
                print(f"  [tool_result] duration={tr.get('durationMs', '?')}ms")

        elif t == "system":
            subtype = obj.get("subtype", "?")
            content = obj.get("content", "")
            meta = obj.get("compactMetadata", {})
            print(f"  subtype={subtype}, content={content}")
            if meta:
                print(f"  compactMetadata={json.dumps(meta)}")

        else:
            # For other types, dump a compact JSON summary
            keys_to_show = {k: v for k, v in obj.items()
                           if k not in ("parentUuid", "isSidechain", "cwd", "sessionId",
                                       "version", "gitBranch", "slug")}
            summary = json.dumps(keys_to_show)
            if len(summary) > 500:
                summary = summary[:500] + "..."
            print(f"  {summary}")

        print()


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(
        description="Search and inspect Claude Code session JSONL transcripts.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    sub = parser.add_subparsers(dest="command", required=True)

    # list
    p_list = sub.add_parser("list", help="List sessions for this project")
    p_list.add_argument("--limit", type=int, default=20, help="Max sessions to show")

    # structure
    p_struct = sub.add_parser("structure", help="Show session structure")
    p_struct.add_argument("session", help="Session ID or path to .jsonl file")

    # search
    p_search = sub.add_parser("search", help="Search for pattern in messages")
    p_search.add_argument("session", help="Session ID or path to .jsonl file")
    p_search.add_argument("pattern", help="Regex pattern to search for")
    p_search.add_argument("--no-subagents", action="store_true", help="Skip subagent transcripts")
    p_search.add_argument("--limit", type=int, default=50, help="Max matches to show")

    # extract
    p_extract = sub.add_parser("extract", help="Pretty-print specific lines")
    p_extract.add_argument("session", help="Session ID or path to .jsonl file")
    p_extract.add_argument("start", type=int, help="Start line number")
    p_extract.add_argument("end", type=int, nargs="?", help="End line number (default: same as start)")

    args = parser.parse_args()

    if args.command == "list":
        cmd_list(args)
    elif args.command == "structure":
        cmd_structure(args)
    elif args.command == "search":
        cmd_search(args)
    elif args.command == "extract":
        cmd_extract(args)


if __name__ == "__main__":
    main()