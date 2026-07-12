#!/usr/bin/env python3
"""Derive privacy-preserving symbolic process relations from process-events v2."""

from __future__ import annotations
import argparse
import hashlib
import json
import os
import tempfile
from collections import Counter, defaultdict
from datetime import datetime
from pathlib import Path
from typing import Any

SCHEMA = "ai-use-process-relations-v1"
EVENT_SCHEMA = "ai-use-process-events-v2"
TYPES = {
    "explicit_session_lineage",
    "worktree_created",
    "worktree_removed",
    "worktree_used_by_session",
    "cleanup_ownership_transfer",
    "branch_created_then_merged",
    "file_transferred",
    "file_deleted",
    "prompt_reuse_candidate",
}


def digest(x: Any) -> str:
    return (
        "sha256:"
        + hashlib.sha256(
            json.dumps(
                x, sort_keys=True, separators=(",", ":"), ensure_ascii=True
            ).encode()
        ).hexdigest()
    )


def file_hash(p: Path) -> str:
    return "sha256:" + hashlib.sha256(p.read_bytes()).hexdigest()


def private_write(path: Path, text: str) -> None:
    """Atomically replace path with a file that was private from creation."""
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(prefix=f".{path.name}.", dir=path.parent)
    try:
        os.fchmod(fd, 0o600)
        with os.fdopen(fd, "w", encoding="utf-8") as handle:
            fd = -1
            handle.write(text)
            handle.flush()
            os.fsync(handle.fileno())
        os.replace(temporary, path)
        os.chmod(path, 0o600)
    finally:
        if fd >= 0:
            os.close(fd)
        try:
            os.unlink(temporary)
        except FileNotFoundError:
            pass


def when(x: Any) -> datetime | None:
    if not isinstance(x, str):
        return None
    try:
        d = datetime.fromisoformat(x.replace("Z", "+00:00"))
        return d if d.tzinfo else None
    except ValueError:
        return None


def authoritative(e):
    return e["evidence_status"].get("command_outcome") == "succeeded"


def before(a, b):
    x, y = when(a.get("timestamp")), when(b.get("timestamp"))
    return x is not None and y is not None and x < y


def evidence_status(row, nested=False, action=None):
    out = row.get("outcome") if isinstance(row.get("outcome"), dict) else {}
    status = {"issued": True, "command_outcome": "unknown"}
    implies = bool((action or {}).get("tool_success_implies_action_success"))
    if nested:
        if out.get("wrapper_status") == "completed" or out.get("status") in {
            "succeeded",
            "failed",
        }:
            status["enclosing_wrapper_completed"] = True
        if out.get("status") in {"succeeded", "failed"}:
            status["enclosing_tool_outcome"] = out["status"]
    elif out.get("status") in {"succeeded", "failed"} and implies:
        status["command_outcome"] = out["status"]
    elif out.get("status") in {"succeeded", "failed"}:
        status["enclosing_tool_outcome"] = out["status"]
    elif out.get("wrapper_status") == "completed":
        status["enclosing_wrapper_completed"] = True
    return status


def action_events(rows):
    result = []
    for row in rows:
        if row.get("record_type") != "tool_call":
            continue
        packs = [(row.get("command_id"), row.get("actions", []), False)]
        packs += [
            (n.get("command_id"), n.get("actions", []), True)
            for n in row.get("nested_commands", [])
            if isinstance(n, dict)
        ]
        for cmd, actions, nested in packs:
            for i, a in enumerate(actions if isinstance(actions, list) else []):
                if not isinstance(a, dict):
                    continue
                identity = a.get("action_event_id") or digest(
                    {
                        "source_log_id": row.get("source_log_id"),
                        "event_ordinal": row.get("event_ordinal"),
                        "call_id": row.get("call_id"),
                        "command_id": cmd,
                        "index": a.get("action_index", i),
                    }
                )
                result.append(
                    {
                        "event_id": identity,
                        "source_event_id": row.get("event_id"),
                        "source_log_id": row.get("source_log_id"),
                        "session_id": row.get("session_id"),
                        "timestamp": row.get("timestamp"),
                        "event_ordinal": row.get("event_ordinal"),
                        "action": a,
                        "evidence_status": evidence_status(row, nested, a),
                    }
                )
    return result


def rel(kind, ev, fields, confidence="high", eligible=True, reason=None):
    refs = [
        {
            k: e.get(k)
            for k in (
                "event_id",
                "source_event_id",
                "source_log_id",
                "timestamp",
                "evidence_status",
            )
            if e.get(k) is not None
        }
        for e in ev
    ]
    core = {
        "relation_type": kind,
        **fields,
        "evidence": refs,
        "confidence": confidence,
        "eligible": eligible,
    }
    if reason:
        core["exclusion_reason"] = reason
    return {"record_type": "candidate_relation", "relation_id": digest(core), **core}


def direct_relation(e, kind):
    a = e["action"]
    fields = {k: v for k, v in a.items() if k.endswith("_id") or k.endswith("_ids")}
    fields["session_id"] = e["session_id"]
    ok = authoritative(e)
    reason = (
        None
        if ok
        else (
            "command_failed"
            if e["evidence_status"]["command_outcome"] == "failed"
            else "command_outcome_unknown"
        )
    )
    # File operations are structural candidates even when successful, never asserted handoffs.
    return rel(kind, [e], fields, a.get("confidence", "medium"), ok, reason)


def prompt_reuse(
    rows,
    min_overlap=3,
    min_similarity=0.5,
    max_shingle_df=20,
    min_token_ratio=0.5,
    exact_frequency_ceiling=1000,
    ambiguity_group_cap=200,
    stats=None,
):
    stats = stats if stats is not None else Counter()
    messages = [
        x
        for x in rows
        if x.get("record_type") == "message_fingerprint"
        and (
            (
                x.get("role") in {"agent", "assistant"}
                and x.get("delivery_kind") == "agent_output"
            )
            or (
                x.get("role") == "user"
                and x.get("message_origin") == "human_user_candidate"
            )
        )
    ]
    groups = {}
    for m in messages:
        shingles = tuple(sorted(set(m.get("winnowed_fingerprint_ids") or [])))
        role = "agent" if m["role"] in {"agent", "assistant"} else "user"
        key = (role, m.get("normalized_text_id"), shingles, m.get("token_count"))
        groups.setdefault(
            key,
            {
                "role": role,
                "normalized_text_id": key[1],
                "shingles": set(shingles),
                "token_count": key[3],
                "members": [],
            },
        )["members"].append(m)
    sources = [g for g in groups.values() if g["role"] == "agent"]
    targets = [g for g in groups.values() if g["role"] == "user"]
    exact, posting, df = defaultdict(list), defaultdict(list), Counter()
    for i, g in enumerate(sources):
        if g["normalized_text_id"]:
            exact[g["normalized_text_id"]].append(i)
        for shingle in g["shingles"]:
            # Session prevalence, not grouped-document count: broadcasts of an
            # identical template must still make its shingles non-distinctive.
            df[shingle] += len({m.get("session_id") for m in g["members"]})
    for i, g in enumerate(sources):
        for shingle in g["shingles"]:
            if df[shingle] <= max_shingle_df:
                posting[shingle].append(i)
    stats["high_prevalence_shingles_suppressed"] += sum(
        count > max_shingle_df for count in df.values()
    )
    result = []
    for target in targets:
        overlap_counts = Counter()
        for shingle in target["shingles"]:
            if df[shingle] <= max_shingle_df:
                overlap_counts.update(posting.get(shingle, []))
        candidates = set(exact.get(target["normalized_text_id"], [])) | {
            i for i, n in overlap_counts.items() if n >= min_overlap
        }
        for i in candidates:
            source = sources[i]
            is_exact = bool(
                target["normalized_text_id"]
                and target["normalized_text_id"] == source["normalized_text_id"]
            )
            overlap = len(source["shingles"] & target["shingles"])
            union = len(source["shingles"] | target["shingles"])
            similarity = 1.0 if is_exact else (overlap / union if union else 0.0)
            st, tt = source["token_count"], target["token_count"]
            ratio = (
                min(st, tt) / max(st, tt)
                if isinstance(st, int) and isinstance(tt, int) and max(st, tt)
                else None
            )
            if not is_exact and (
                overlap < min_overlap
                or similarity < min_similarity
                or ratio is None
                or ratio < min_token_ratio
            ):
                continue
            frequency = len(source["members"]) + len(target["members"])
            if is_exact and frequency > exact_frequency_ceiling:
                stats["exact_frequency_ceiling_suppressed"] += 1
                continue
            # Cap before membership checks: runtime is bounded independently of
            # broadcast multiplicity and no source×target pairs are materialized.
            src_all = sorted(
                source["members"],
                key=lambda m: (m.get("timestamp") or "", m.get("message_id") or ""),
            )
            tgt_all = sorted(
                target["members"],
                key=lambda m: (m.get("timestamp") or "", m.get("message_id") or ""),
            )
            truncated = (
                len(src_all) > ambiguity_group_cap or len(tgt_all) > ambiguity_group_cap
            )
            src = src_all[:ambiguity_group_cap]
            tgt = tgt_all[:ambiguity_group_cap]
            if truncated:
                stats["ambiguity_groups_truncated"] += 1
            # Sweep chronology with inclusion/exclusion counts. This computes
            # member sets in O((S+T) log(S+T)), without materializing pairs.
            src = [m for m in src if when(m.get("timestamp")) is not None]
            tgt = [m for m in tgt if when(m.get("timestamp")) is not None]
            prior_total = 0
            by_session = Counter()
            by_log = Counter()
            by_pair = Counter()
            si = 0
            tmembers = {}
            for tm in tgt:
                while si < len(src) and before(src[si], tm):
                    sm = src[si]
                    pair = (sm.get("session_id"), sm.get("source_log_id"))
                    prior_total += 1
                    by_session[pair[0]] += 1
                    by_log[pair[1]] += 1
                    by_pair[pair] += 1
                    si += 1
                pair = (tm.get("session_id"), tm.get("source_log_id"))
                if (
                    prior_total - by_session[pair[0]] - by_log[pair[1]] + by_pair[pair]
                    > 0
                ):
                    tmembers[tm.get("message_id")] = tm
            if not tmembers:
                continue
            valid_targets = sorted(
                tmembers.values(), key=lambda m: when(m.get("timestamp")), reverse=True
            )
            later_total = 0
            by_session = Counter()
            by_log = Counter()
            by_pair = Counter()
            ti = 0
            smembers = {}
            for sm in sorted(src, key=lambda m: when(m.get("timestamp")), reverse=True):
                while ti < len(valid_targets) and before(sm, valid_targets[ti]):
                    tm = valid_targets[ti]
                    pair = (tm.get("session_id"), tm.get("source_log_id"))
                    later_total += 1
                    by_session[pair[0]] += 1
                    by_log[pair[1]] += 1
                    by_pair[pair] += 1
                    ti += 1
                pair = (sm.get("session_id"), sm.get("source_log_id"))
                if (
                    later_total - by_session[pair[0]] - by_log[pair[1]] + by_pair[pair]
                    > 0
                ):
                    smembers[sm.get("message_id")] = sm
            if not smembers:
                continue
            ev = []
            for m in [*smembers.values(), *tmembers.values()]:
                ev.append(
                    {
                        "event_id": m.get("message_id")
                        or digest(
                            {
                                "source_log_id": m.get("source_log_id"),
                                "event_ordinal": m.get("event_ordinal"),
                            }
                        ),
                        "source_log_id": m.get("source_log_id"),
                        "timestamp": m.get("timestamp"),
                        "evidence_status": {
                            "issued": True,
                            "command_outcome": "unknown",
                        },
                    }
                )
            fields = {
                "reuse_label": "exact" if is_exact else "edited",
                "normalized_text_id": target["normalized_text_id"]
                if is_exact
                else None,
                "source_message_ids": sorted(smembers),
                "target_message_ids": sorted(tmembers),
                "source_session_ids": sorted(
                    {m.get("session_id") for m in smembers.values()}
                ),
                "target_session_ids": sorted(
                    {m.get("session_id") for m in tmembers.values()}
                ),
                "ambiguity_group_id": digest(
                    {"source": sorted(smembers), "target": sorted(tmembers)}
                ),
                "source_occurrence_count": len(source["members"]),
                "target_occurrence_count": len(target["members"]),
                "ambiguity_group_truncated": truncated,
                "exact_common_broadcast": is_exact and frequency > max_shingle_df,
                "overlap_count": overlap,
                "source_shingle_count": len(source["shingles"]),
                "target_shingle_count": len(target["shingles"]),
                "jaccard_similarity": round(similarity, 6),
                "token_count_ratio": round(ratio, 6) if ratio is not None else None,
            }
            result.append(
                rel(
                    "prompt_reuse_candidate",
                    ev,
                    fields,
                    "high" if is_exact else "medium",
                    False,
                    "pending_review",
                )
            )
    return result


def derive(rows, selected=None, reuse_config=None):
    wanted = selected or TYPES
    out = []
    for row in rows:
        if row.get("record_type") == "lineage" and "explicit_session_lineage" in wanted:
            e = {
                "event_id": row.get("event_id") or digest(row),
                "source_log_id": row.get("source_log_id"),
                "timestamp": row.get("timestamp"),
                "evidence_status": {"issued": True, "command_outcome": "unknown"},
            }
            out.append(
                rel(
                    "explicit_session_lineage",
                    [e],
                    {
                        "child_session_id": row.get("session_id"),
                        "parent_session_id": row.get("parent_session_id"),
                    },
                    "high",
                    True,
                )
            )
    events = action_events(rows)
    mapping = {
        "git_worktree_add": "worktree_created",
        "git_worktree_remove": "worktree_removed",
        "file_transfer": "file_transferred",
        "file_delete": "file_deleted",
    }
    for e in events:
        k = mapping.get(e["action"].get("action"))
        if k in wanted:
            out.append(direct_relation(e, k))

    # A missing repo id deliberately forms a separate key; it cannot silently join a known repo.
    keyed = defaultdict(list)
    for e in events:
        a = e["action"]
        if a.get("action") in {"git_worktree_add", "git_worktree_remove"} and a.get(
            "worktree_path_id"
        ):
            keyed[(a.get("repo_path_id"), a["worktree_path_id"])].append(e)
    episodes = []
    for key, es in keyed.items():
        # Equal/unknown timestamps cannot define state transitions.
        es.sort(
            key=lambda e: (
                when(e["timestamp"]) is None,
                when(e["timestamp"])
                or datetime.max.replace(tzinfo=__import__("datetime").timezone.utc),
                0 if e["action"]["action"] == "git_worktree_add" else 1,
                e["event_id"],
            )
        )
        active = None
        number = 0
        for e in es:
            op = e["action"]["action"]
            if op == "git_worktree_add" and authoritative(e):
                if active is not None:
                    active["ambiguous"] = True
                number += 1
                active = {
                    "key": key,
                    "number": number,
                    "create": e,
                    "remove": None,
                    "ambiguous": when(e["timestamp"]) is None,
                }
                episodes.append(active)
            elif op == "git_worktree_remove":
                if not authoritative(e):
                    # Attach attempt only if there is one unambiguously active episode.
                    if active:
                        active.setdefault("remove_attempts", []).append(e)
                elif active and before(active["create"], e):
                    active["remove"] = e
                    active = None
                elif active:
                    active["ambiguous"] = True
                    active.setdefault("remove_attempts", []).append(e)
        # A second, non-mutating reconstruction makes wrapper-only evidence
        # inspectable without allowing it into authoritative lifecycle counts.
        wrappers = [
            e
            for e in es
            if e["evidence_status"].get("enclosing_wrapper_completed")
            and e["evidence_status"]["command_outcome"] == "unknown"
        ]
        active = None
        number = 0
        for e in wrappers:
            op = e["action"]["action"]
            if op == "git_worktree_add":
                if active is not None:
                    active["ambiguous"] = True
                number += 1
                active = {
                    "key": key,
                    "number": "wrapper-" + str(number),
                    "create": e,
                    "remove": None,
                    "ambiguous": when(e["timestamp"]) is None,
                    "transition_evidence": "wrapper_completed_unverified",
                }
                episodes.append(active)
            elif op == "git_worktree_remove" and active:
                if before(active["create"], e):
                    active["remove"] = e
                    active = None
                else:
                    active["ambiguous"] = True
                    active.setdefault("remove_attempts", []).append(e)
    # Usage must be strictly within an episode, and is one row per session and episode.
    if "worktree_used_by_session" in wanted:
        for ep in episodes:
            uses = []
            for e in events:
                if e["action"].get("repo_path_id") != ep["key"][1]:
                    continue
                if not before(ep["create"], e):
                    continue
                if ep["remove"] and not before(e, ep["remove"]):
                    continue
                uses.append(e)
            for session, group in __import__("itertools").groupby(
                sorted(
                    uses,
                    key=lambda x: (
                        x["session_id"],
                        x["timestamp"] or "",
                        x["event_id"],
                    ),
                ),
                lambda x: x["session_id"],
            ):
                group = list(group)
                ambiguous = ep["ambiguous"] or any(
                    when(x["timestamp"]) is None for x in group
                )
                transition = ep.get("transition_evidence", "authoritative_success")
                eligible = not ambiguous and transition == "authoritative_success"
                out.append(
                    rel(
                        "worktree_used_by_session",
                        [ep["create"], *group],
                        {
                            "repo_path_id": ep["key"][0],
                            "worktree_path_id": ep["key"][1],
                            "episode": ep["number"],
                            "session_id": session,
                            "creator_session_id": ep["create"]["session_id"],
                            "use_event_count": len(group),
                            "transition_evidence": transition,
                        },
                        "medium" if ambiguous or not eligible else "high",
                        eligible,
                        "ambiguous_chronology"
                        if ambiguous
                        else ("command_outcome_unknown" if not eligible else None),
                    )
                )
    if "cleanup_ownership_transfer" in wanted:
        for ep in episodes:
            remove = ep["remove"]
            if remove:
                cls = (
                    "self"
                    if remove["session_id"] == ep["create"]["session_id"]
                    else "other"
                )
            elif ep.get("remove_attempts"):
                cls = (
                    "failed-attempt"
                    if any(
                        x["evidence_status"]["command_outcome"] == "failed"
                        for x in ep["remove_attempts"]
                    )
                    else "ambiguous"
                )
            else:
                cls = "unresolved"
            ambiguous = ep["ambiguous"] or cls == "ambiguous"
            ev = [ep["create"]] + (
                [remove] if remove else ep.get("remove_attempts", [])
            )
            transition = ep.get("transition_evidence", "authoritative_success")
            eligible = (
                bool(remove) and not ambiguous and transition == "authoritative_success"
            )
            reason = (
                "ambiguous_chronology"
                if ambiguous
                else (
                    "command_outcome_unknown"
                    if transition != "authoritative_success"
                    else (
                        "removal_failed"
                        if cls == "failed-attempt"
                        else ("no_removal_observed" if cls == "unresolved" else None)
                    )
                )
            )
            out.append(
                rel(
                    "cleanup_ownership_transfer",
                    ev,
                    {
                        "repo_path_id": ep["key"][0],
                        "worktree_path_id": ep["key"][1],
                        "episode": ep["number"],
                        "creator_session_id": ep["create"]["session_id"],
                        "remover_session_id": remove["session_id"] if remove else None,
                        "cleanup_class": cls,
                        "transition_evidence": transition,
                    },
                    "medium" if ambiguous or not eligible else "high",
                    eligible,
                    reason,
                )
            )

    if "branch_created_then_merged" in wanted:
        creates = defaultdict(list)
        for e in events:
            a = e["action"]
            if (
                a.get("action") == "git_worktree_add"
                and authoritative(e)
                and a.get("branch_ref_id")
            ):
                creates[(a.get("repo_path_id"), a["branch_ref_id"])].append(e)
        for e in events:
            a = e["action"]
            if a.get("action") != "git_merge" or not authoritative(e):
                continue
            for ref in a.get("git_ref_ids", []):
                prior = [
                    c
                    for c in creates.get((a.get("repo_path_id"), ref), [])
                    if before(c, e)
                ]
                if not prior:
                    continue
                latest = max(when(c["timestamp"]) for c in prior)
                candidates = [c for c in prior if when(c["timestamp"]) == latest]
                ambiguous = len(candidates) != 1
                fields = {
                    "repo_path_id": a.get("repo_path_id"),
                    "branch_ref_id": ref,
                    "merge_session_id": e["session_id"],
                }
                if ambiguous:
                    fields["candidate_creator_session_ids"] = sorted(
                        {c["session_id"] for c in candidates}
                    )
                else:
                    fields["creator_session_id"] = candidates[0]["session_id"]
                out.append(
                    rel(
                        "branch_created_then_merged",
                        [*candidates, e],
                        fields,
                        "medium" if ambiguous else "high",
                        not ambiguous,
                        "branch_reuse_ambiguous" if ambiguous else None,
                    )
                )
        # Nested functions.exec commands have only enclosing-wrapper evidence.
        # Pair the latest strictly-prior create without promoting it to a merge.
        wrapper_creates = defaultdict(list)
        for e in events:
            a = e["action"]
            if (
                a.get("action") == "git_worktree_add"
                and e["evidence_status"].get("enclosing_wrapper_completed")
                and a.get("branch_ref_id")
            ):
                wrapper_creates[(a.get("repo_path_id"), a["branch_ref_id"])].append(e)
        for e in events:
            a = e["action"]
            if a.get("action") != "git_merge" or not e["evidence_status"].get(
                "enclosing_wrapper_completed"
            ):
                continue
            for ref in a.get("git_ref_ids", []):
                prior = [
                    c
                    for c in wrapper_creates.get((a.get("repo_path_id"), ref), [])
                    if before(c, e)
                ]
                if not prior:
                    continue
                latest = max(when(c["timestamp"]) for c in prior)
                candidates = [c for c in prior if when(c["timestamp"]) == latest]
                fields = {
                    "repo_path_id": a.get("repo_path_id"),
                    "branch_ref_id": ref,
                    "merge_session_id": e["session_id"],
                    "transition_evidence": "wrapper_completed_unverified",
                }
                if len(candidates) == 1:
                    fields["creator_session_id"] = candidates[0]["session_id"]
                else:
                    fields["candidate_creator_session_ids"] = sorted(
                        {c["session_id"] for c in candidates}
                    )
                out.append(
                    rel(
                        "branch_created_then_merged",
                        [*candidates, e],
                        fields,
                        "medium",
                        False,
                        "command_outcome_unknown",
                    )
                )
    if "prompt_reuse_candidate" in wanted:
        out.extend(prompt_reuse(rows, **(reuse_config or {})))
    # IDs are content-derived; collapse exact duplicates.
    return sorted(
        {x["relation_id"]: x for x in out}.values(),
        key=lambda x: (x["relation_type"], x["relation_id"]),
    )


def read_jsonl(p):
    rows = []
    with p.open(encoding="utf-8") as f:
        for n, line in enumerate(f, 1):
            if line.strip():
                x = json.loads(line)
                if not isinstance(x, dict):
                    raise ValueError(f"line {n}: expected object")
                rows.append(x)
    return rows


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--input", required=True, type=Path)
    p.add_argument("--input-manifest", required=True, type=Path)
    p.add_argument("--output", required=True, type=Path)
    p.add_argument("--manifest", type=Path)
    p.add_argument("--relation-type", action="append", choices=sorted(TYPES))
    p.add_argument("--reuse-min-overlap", type=int, default=3)
    p.add_argument("--reuse-min-similarity", type=float, default=0.5)
    p.add_argument("--reuse-max-shingle-df", type=int, default=20)
    p.add_argument("--reuse-min-token-ratio", type=float, default=0.5)
    p.add_argument("--reuse-exact-frequency-ceiling", type=int, default=1000)
    p.add_argument("--reuse-ambiguity-group-cap", type=int, default=200)
    a = p.parse_args()
    source = json.loads(a.input_manifest.read_text())
    if source.get("schema") != EVENT_SCHEMA:
        p.error(f"input manifest schema must be {EVENT_SCHEMA}")
    if source.get("output_hash") != file_hash(a.input):
        p.error("input manifest output_hash does not match input")
    if (
        a.reuse_min_overlap < 1
        or not 0 <= a.reuse_min_similarity <= 1
        or a.reuse_max_shingle_df < 1
        or not 0 <= a.reuse_min_token_ratio <= 1
        or a.reuse_exact_frequency_ceiling < 1
        or a.reuse_ambiguity_group_cap < 1
    ):
        p.error("invalid prompt reuse threshold")
    rows = read_jsonl(a.input)
    selected = set(a.relation_type) if a.relation_type else None
    reuse_stats = Counter()
    reuse_config = {
        "min_overlap": a.reuse_min_overlap,
        "min_similarity": a.reuse_min_similarity,
        "max_shingle_df": a.reuse_max_shingle_df,
        "min_token_ratio": a.reuse_min_token_ratio,
        "exact_frequency_ceiling": a.reuse_exact_frequency_ceiling,
        "ambiguity_group_cap": a.reuse_ambiguity_group_cap,
        "stats": reuse_stats,
    }
    relations = derive(rows, selected, reuse_config)
    data = "".join(json.dumps(x, sort_keys=True) + "\n" for x in relations)
    private_write(a.output, data)
    propagated = {
        k: source.get(k)
        for k in (
            "window",
            "key_fingerprint",
            "input_inventory",
            "script_hash",
            "coverage",
        )
    }
    manifest = {
        "schema": SCHEMA,
        "source_event_schema": EVENT_SCHEMA,
        "source": propagated,
        "input_hash": file_hash(a.input),
        "input_manifest_hash": file_hash(a.input_manifest),
        "output_hash": "sha256:" + hashlib.sha256(data.encode()).hexdigest(),
        "script_hash": file_hash(Path(__file__)),
        "relation_types": sorted(selected or TYPES),
        "prompt_reuse_config": {k: v for k, v in reuse_config.items() if k != "stats"},
        "prompt_reuse_suppressions": dict(sorted(reuse_stats.items())),
        "rows": len(relations),
        "by_relation_type": dict(
            sorted(Counter(x["relation_type"] for x in relations).items())
        ),
        "by_cleanup_class": dict(
            sorted(
                Counter(
                    x.get("cleanup_class") for x in relations if x.get("cleanup_class")
                ).items()
            )
        ),
        "by_transition_evidence": dict(
            sorted(
                Counter(
                    x.get("transition_evidence")
                    for x in relations
                    if x.get("transition_evidence")
                ).items()
            )
        ),
        "eligible": dict(
            sorted(Counter(str(x["eligible"]).lower() for x in relations).items())
        ),
        "eligible_cleanup": dict(
            sorted(
                Counter(
                    str(x["eligible"]).lower()
                    for x in relations
                    if x["relation_type"] == "cleanup_ownership_transfer"
                ).items()
            )
        ),
    }
    private_write(
        a.manifest or a.output.with_suffix(a.output.suffix + ".manifest.json"),
        json.dumps(manifest, indent=2, sort_keys=True) + "\n",
    )


if __name__ == "__main__":
    main()
