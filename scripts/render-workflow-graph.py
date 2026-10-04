#!/usr/bin/env python3
"""Project current.json into a compact DOT/SVG view; --check detects drift."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import textwrap
import xml.etree.ElementTree as ET


REPO = Path(__file__).resolve().parents[1]
COLORS = {
    "done": ("#e7f1e8", "#6f8d73"),
    "running": ("#e5effa", "#3675ac"),
    "ready": ("#fff7dc", "#aa8a31"),
    "blocked": ("#fce9e5", "#ba6251"),
    "parked": ("#f1f0ed", "#93918c"),
    "dropped": ("#f1f0ed", "#93918c"),
}


def quote(value):
    return json.dumps(value, ensure_ascii=False)


def projection(state, digest):
    tasks = state["tasks"]
    by_id = {task["id"]: task for task in tasks}
    if len(by_id) != len(tasks):
        raise ValueError("Duplicate task IDs")
    if "msc-math-done" not in by_id:
        raise ValueError("Missing msc-math-done milestone")
    visited = set()

    def visit(task_id, active):
        if task_id in active:
            raise ValueError(f"Dependency cycle: {task_id}")
        if task_id in visited:
            return
        for dependency in by_id[task_id]["depends_on"]:
            if dependency not in by_id:
                raise ValueError(f"Unknown prerequisite: {dependency}")
            visit(dependency, active | {task_id})
        visited.add(task_id)

    for task in tasks:
        visit(task["id"], set())
        if task["status"] not in COLORS:
            raise ValueError(f"Unknown status: {task['id']}")
        if task.get("kind", "task") not in {"task", "milestone", "crux"}:
            raise ValueError(f"Unknown kind: {task['id']}")
        if task.get("selection", "selected") not in {"selected", "unselected", "parked"}:
            raise ValueError(f"Unknown selection: {task['id']}")
        if task["status"] == "blocked" and not task.get("blocker"):
            raise ValueError(f"Missing blocker: {task['id']}")
        if task["status"] == "done" and task.get("owner"):
            raise ValueError(f"Completed assignment still owned: {task['id']}")
        if task["status"] == "done" and any(by_id[dep]["status"] != "done" for dep in task["depends_on"]):
            raise ValueError(f"Completed task has an unfinished prerequisite: {task['id']}")

    lines = [
        "// Generated from docs/coordination/current.json; do not edit.",
        f"// coordination-sha256: {digest}",
        "digraph project {",
        '  graph [rankdir=BT, newrank=true, bgcolor="white", fontname="sans-serif",',
        '    fontsize=11, nodesep=0.18, ranksep=0.34, pad=0.15,',
        '    label="msc-math · prerequisites → dependent\\nREADY candidates need selection · thesis STOPPED · semantic dashboard owns the detailed view", labelloc=b];',
        '  node [shape=box, style="rounded,filled", fontname="sans-serif",',
        '    fontsize=12, margin="0.08,0.06", width=1.4];',
        '  edge [color="#78818a", arrowsize=0.6, penwidth=1.0];',
    ]
    def details(task):
        result = task["outcome"]
        for field in ("blocker", "plan_gap", "owner", "project", "authorization", "next_step"):
            if task.get(field):
                result += f"\n{field}: {task[field]}"
        return result + "\nSources: " + ", ".join(task["sources"])

    aliases = {}
    completed, parked = [], []
    candidates = {}
    for task in tasks:
        task_id = task["id"]
        if task.get("selection", "selected") != "selected":
            if task.get("parallel_ready"):
                project = task.get("project", "other")
                candidates.setdefault(project, []).append(task)
                aliases[task_id] = "__candidates_" + project
                continue
            parked.append(task)
            aliases[task_id] = "__parked"
            continue
        if task["status"] == "done":
            completed.append(task)
            aliases[task_id] = "__completed"
            continue
        aliases[task_id] = task_id
        kind = task.get("kind", "task")
        shape = {"milestone": "doubleoctagon", "crux": "hexagon"}.get(kind, "box")
        fill, border = COLORS[task["status"]]
        name = "\n".join(textwrap.fill(part, width=19) for part in task.get("graph_label", task["title"]).splitlines())
        node_label = f"{name}\n{task['status'].upper()}"
        attrs = f"label={quote(node_label)}, shape={shape}, fillcolor={quote(fill)}, color={quote(border)}, tooltip={quote(details(task))}"
        if kind == "milestone":
            attrs += ", penwidth=2, fontsize=14"
        lines.append(f"  {quote(task_id)} [{attrs}];")

    summaries = [
        ("__completed", "DONE · owners released", completed, "#e7f1e8"),
        ("__parked", "UNSELECTED / PARKED · no execution", parked, "#f1f0ed"),
    ]
    summaries.extend(("__candidates_" + project, project.upper() + " · READY IF SELECTED", items, "#fff7dc") for project, items in candidates.items())
    for node_id, heading, items, fill in summaries:
        if not items:
            continue
        labels = [heading]
        tooltips = []
        for task in items:
            name = task.get("graph_label", task["title"]).replace("\n", " ")
            labels.append(name + (f" · {task['status'].upper()}" if node_id == "__parked" else ""))
            tooltips.append(task["title"] + "\n" + details(task))
        label = "\n".join(labels)
        tooltip = "\n\n".join(tooltips)
        lines.append(f"  {quote(node_id)} [label={quote(label)}, fontsize=11, fillcolor={quote(fill)}, color=\"#93918c\", tooltip={quote(tooltip)}];")
    edges = set()
    for task in tasks:
        for dependency in task["depends_on"]:
            start, end = aliases[dependency], aliases[task["id"]]
            if start == end or (start, end) in edges:
                continue
            edges.add((start, end))
            style = ' [style=dashed, color="#a9a9a5"]' if end == "__parked" else ""
            lines.append(f"  {quote(start)} -> {quote(end)}{style};")
    # Below all selected work: visible parked routes never become prerequisites.
    if parked and completed and ("__completed", "__parked") not in edges:
        lines.append('  "__parked" -> "__completed" [style=invis];')
    elif parked and completed:
        # The non-blocking proposal link is presented with no rank constraint;
        # a separate invisible edge places the parked shelf below foundations.
        lines = [line.replace('"__completed" -> "__parked" [style=dashed,', '"__completed" -> "__parked" [constraint=false, style=dashed,') for line in lines]
        lines.append('  "__parked" -> "__completed" [style=invis];')
    lines.extend(['  { rank=max; "msc-math-done"; }', "}"])
    return "\n".join(lines) + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--state", type=Path, default=REPO / "docs/coordination/current.json")
    parser.add_argument("--output-dir", type=Path, default=REPO / "docs/coordination/graphs")
    parser.add_argument("--check", action="store_true", help="Check DOT projection and SVG source hash without rerendering")
    args = parser.parse_args()
    try:
        raw = args.state.read_bytes()
        digest = hashlib.sha256(raw).hexdigest()
        dot = projection(json.loads(raw), digest)
        dot_path, svg_path = (args.output_dir / name for name in ("tasks.dot", "tasks.svg"))
        marker = f"<!-- coordination-sha256: {digest} -->"
        if args.check:
            if dot_path.read_text() != dot:
                raise ValueError("DOT is stale; rerun scripts/render-workflow-graph.py")
            svg = svg_path.read_text()
            if marker not in svg:
                raise ValueError("SVG source hash is stale; rerun scripts/render-workflow-graph.py")
            if ET.fromstring(svg).tag != "{http://www.w3.org/2000/svg}svg":
                raise ValueError("Rendered file is not SVG")
            print("Project DOT projection and SVG source hash match current.json")
            return
        svg = subprocess.run(["dot", "-Tsvg"], input=dot, text=True, capture_output=True, check=True).stdout
        svg = svg.replace("?>", "?>\n" + marker, 1)
        ET.fromstring(svg)
        args.output_dir.mkdir(parents=True, exist_ok=True)
        dot_path.write_text(dot)
        svg_path.write_text(svg)
        print(f"Projected {len(json.loads(raw)['tasks'])} task records from current.json")
    except (OSError, ValueError, KeyError, subprocess.CalledProcessError, ET.ParseError) as error:
        parser.exit(1, f"Project graph: {error}\n")


if __name__ == "__main__":
    main()
