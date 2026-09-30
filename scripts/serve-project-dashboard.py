#!/usr/bin/env python3
"""Serve Jörn's work/resource view at http://127.0.0.1:8765/.

Run from the repository root. --check validates coordination and source contracts.
Use --host and --port to configure the listener; the default is local only.
"""

import argparse
import hashlib
import json
import mimetypes
from datetime import datetime
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlsplit

REPO = Path(__file__).resolve().parent.parent
STATE = "docs/coordination/current.json"
BASELINE = "docs/dashboard/thesis-sources.json"
# Explicitly named files only: no directory handler, path expansion or write API.
FILES = {
    "AGENTS.md", "AGENTS.md.commentary.md",
    "submit/archive-closure-checklist.md",
    "docs/reproducibility.md",
    "crates/README.md",
    "thesis/README.md",
    "ARCHITECTURE.md",
    STATE, BASELINE,
    "docs/dashboard/index.html", "docs/dashboard/thesis.html",
    "docs/dashboard/README.md", "docs/coordination/README.md",
    "docs/coordination/otel-design.md",
    "docs/coordination/skill-migration-plan.md",
    "docs/coordination/knowledge-evaluation-plan.md",
    "scripts/coordination-load.py", "scripts/test_coordination_load.py",
    "docs/coordination/graphs/tasks.dot",
    "docs/coordination/graphs/tasks.svg",
    "docs/coordination/handoff.md", "docs/coordination/thesis-work.md",
    "docs/project-facts.md", "docs/README.md", "docs/knowledge/README.md",
    "docs/development-environments.md",
    "docs/knowledge/hko-author-context.md", "docs/knowledge/optimizer-review-route.md",
    "docs/knowledge/optional-working-card.md", "docs/resume/thesis-resume.pdf",
    "docs/history/coordination-surface-migration/resume-20260930.md",
    "docs/consolidation/2026-09-24/claim-disposition.md",
    "docs/history/memories/planning-context.md", "thesis/main.tex",
    "docs/history/process-audit/REPORT.md",
    "thesis/chapters/07-hko.tex",
    "experiments/hko-local-maximum/non-hko/README.md",
    "experiments/hko-local-maximum/non-hko/PROOF.md",
}
REDIRECTS = {
    "/": "/docs/dashboard/index.html",
    "/docs/project-dashboard.html": "/docs/dashboard/index.html",
    "/docs/project-now.html": "/docs/dashboard/thesis.html",
    "/RESUME.md": "/docs/coordination/handoff.md",
    "/docs/WORK_REMAINING.md": "/docs/coordination/thesis-work.md",
}


def read_file(relative_path):
    path = REPO / relative_path
    if not path.resolve().is_relative_to(REPO) or not path.is_file():
        raise OSError(f"Missing or out-of-repository file: {relative_path}")
    return path.read_bytes()


def timestamp(value):
    parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None:
        raise ValueError("Observation timestamps must include a timezone")
    return parsed


def read_state():
    state = json.loads(read_file(STATE))
    if state["schema_version"] != 1:
        raise ValueError("Unsupported coordination schema")
    timestamp(state["updated_at"])
    for key in ("scope", "summary"):
        if not isinstance(state[key], str) or not state[key]:
            raise ValueError(f"Missing {key}")
    tasks = state["tasks"]
    ids = [task["id"] for task in tasks]
    if len(set(ids)) != len(ids):
        raise ValueError("Duplicate task IDs")
    dependencies = {}
    for task in tasks:
        if task["status"] not in {"ready", "running", "blocked", "done", "parked", "dropped"}:
            raise ValueError(f"Unknown status: {task['id']}")
        if task["status"] == "running" and not task["owner"]:
            raise ValueError(f"Running task has no owner: {task['id']}")
        if task["status"] == "blocked" and not task["blocker"]:
            raise ValueError(f"Blocked task has no reason: {task['id']}")
        if task["status"] == "done" and task["owner"]:
            raise ValueError(f"Completed task still has an owner: {task['id']}")
        if task.get("kind", "task") not in {"task", "milestone", "crux"}:
            raise ValueError(f"Unknown task kind: {task['id']}")
        if task.get("selection", "selected") not in {"selected", "unselected", "parked"}:
            raise ValueError(f"Unknown task selection: {task['id']}")
        for key in ("title", "outcome"):
            if not isinstance(task[key], str) or not task[key]:
                raise ValueError(f"Missing {key}: {task['id']}")
        timestamp(task["updated_at"])
        dependencies[task["id"]] = task["depends_on"]
        if any(dep not in ids for dep in task["depends_on"]):
            raise ValueError(f"Unknown dependency: {task['id']}")
    by_id = {task["id"]: task for task in tasks}
    for task in tasks:
        if task["status"] == "done" and any(by_id[dep]["status"] != "done" for dep in task["depends_on"]):
            raise ValueError(f"Completed task has an unfinished prerequisite: {task['id']}")
    def visit(node, active, visited):
        if node in active:
            raise ValueError(f"Dependency cycle: {node}")
        if node not in visited:
            for dependency in dependencies[node]:
                visit(dependency, active | {node}, visited)
            visited.add(node)
    visited = set()
    for node in ids:
        visit(node, set(), visited)
    for decision in state["decisions"]:
        if decision["status"] not in {"open", "parked", "resolved"}:
            raise ValueError(f"Unknown decision status: {decision['id']}")
        for key in ("question", "recommendation", "consequence"):
            if not isinstance(decision[key], str) or not decision[key]:
                raise ValueError(f"Missing decision {key}")
    paths = [link["path"] for link in state["links"]]
    for record in [*tasks, *state["decisions"]]:
        paths.extend(record["sources"])
    for path in paths:
        if path not in FILES:
            raise ValueError(f"Source needs an explicit serving entry: {path}")
        read_file(path)
    for observation in state["resources"]:
        if observation["observed_at"] is not None:
            timestamp(observation["observed_at"])
        if not observation["basis"]:
            raise ValueError("Resource observation needs evidence/scope")
    for change in state["changes"]:
        timestamp(change["at"])
    return state


def source_status():
    try:
        baseline = json.loads(read_file(BASELINE))
        changed = []
        for path, digest in baseline["sha256"].items():
            if path not in FILES:
                raise ValueError(f"Baseline source not allowlisted: {path}")
            if hashlib.sha256(read_file(path)).hexdigest() != digest:
                changed.append(path)
        return {"known": True, "changed": changed,
                "reconciled": baseline["reconciled"], "note": baseline["note"]}
    except (OSError, ValueError, KeyError, TypeError):
        return {"known": False, "reconciled": False}


class DashboardHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self._serve(True)

    def do_HEAD(self):
        self._serve(False)

    def _reply(self, code, body, content_type, send_body):
        self.send_response(code)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.send_header("X-Content-Type-Options", "nosniff")
        self.end_headers()
        if send_body:
            self.wfile.write(body)

    def _serve(self, send_body):
        request_path = urlsplit(self.path).path
        if request_path in REDIRECTS:
            self.send_response(302)
            self.send_header("Location", REDIRECTS[request_path])
            self.send_header("Content-Length", "0")
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            return
        if request_path == "/_dashboard/source-status":
            self._reply(200, json.dumps(source_status()).encode(), "application/json", send_body)
            return
        relative = request_path.removeprefix("/")
        if relative not in FILES:
            self._reply(404, b"Not found\n", "text/plain", send_body)
            return
        try:
            if relative == STATE:
                body = json.dumps(read_state(), ensure_ascii=False).encode()
            else:
                body = read_file(relative)
        except (OSError, ValueError, KeyError, TypeError) as error:
            code = 503 if relative == STATE else 404
            self.log_error("Cannot serve %s: %s", relative, error)
            self._reply(code, b"Source unavailable or invalid\n", "text/plain", send_body)
            return
        content_type = mimetypes.guess_type(relative)[0] or "text/plain"
        if relative.endswith((".md", ".tex")):
            content_type = "text/plain"
        if content_type.startswith("text/") or relative.endswith(".json"):
            content_type += "; charset=utf-8"
        self._reply(200, body, content_type, send_body)

    def do_POST(self):
        self._reply(405, b"Method not allowed\n", "text/plain", True)

    do_PUT = do_DELETE = do_PATCH = do_POST


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=8765)
    parser.add_argument("--check", action="store_true", help="validate without starting a listener")
    args = parser.parse_args()
    if args.check:
        read_state()
        status = source_status()
        if not status["known"]:
            parser.error("Scientific source baseline is unreadable")
        print("PASS: coordination state, dependencies, served links and scientific baseline readable")
        print(f"Scientific reconciliation: {status['reconciled']}; changed sources: {status['changed']}")
        return
    with ThreadingHTTPServer((args.host, args.port), DashboardHandler) as server:
        print(f"Dashboard: http://{args.host}:{server.server_port}/", flush=True)
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            pass


if __name__ == "__main__":
    main()
