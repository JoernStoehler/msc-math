#!/usr/bin/env bash
# Purpose: Generate the repo's experiment JSONL dataflow map as Markdown.
# Context: Parses Cargo bin registrations plus experiment Input/Output headers
#          in Rust binaries and Python analyzers; emits an agent-readable DAG
#          with freshness timestamps for tracked canonical JSONL artifacts.
#          This is a declared entrypoint/header audit, not a full transitive
#          Rust/Python dependency graph or provenance reconstruction.

set -euo pipefail

REPO_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

python3 - "$REPO_ROOT" <<'PY'
import re
import subprocess
import sys
import tomllib
from collections import defaultdict
from datetime import datetime, UTC
from pathlib import Path, PurePosixPath

repo = Path(sys.argv[1]).resolve()
experiments = repo / "experiments"

SECTION_RE = re.compile(r"^(Goal|Input|Output):\s*(.*?)(?=^(?:Goal|Input|Output):|\Z)", re.M | re.S)
JSONL_RE = re.compile(r"(?:experiments/)?[A-Za-z0-9_./{},*<>-]+\.jsonl")


def sanitize_id(prefix: str, value: str) -> str:
    token = re.sub(r"[^A-Za-z0-9]+", "_", value).strip("_")
    return f"{prefix}_{token}" if token else prefix


def mermaid_label(text: str) -> str:
    return text.replace("\\", "\\\\").replace('"', '\\"')


def fmt_timestamp(ts: float | None) -> str:
    if ts is None:
        return "-"
    return datetime.fromtimestamp(ts, UTC).isoformat(timespec="seconds").replace("+00:00", "Z")


def git_last_change(path_str: str) -> str:
    result = subprocess.run(
        ["git", "log", "-1", "--format=%cI", "--", path_str],
        check=False,
        capture_output=True,
        text=True,
        cwd=repo,
    )
    return result.stdout.strip() or "-"


def git_last_change_ts(path_str: str) -> float | None:
    raw = git_last_change(path_str)
    if raw == "-":
        return None
    return datetime.fromisoformat(raw.replace("Z", "+00:00")).timestamp()


def expand_braces(pattern: str) -> list[str]:
    match = re.search(r"\{([^{}]+)\}", pattern)
    if not match:
        return [pattern]
    prefix = pattern[: match.start()]
    suffix = pattern[match.end() :]
    expanded: list[str] = []
    for option in match.group(1).split(","):
        for rest in expand_braces(prefix + option + suffix):
            expanded.append(rest)
    return expanded


def normalize_jsonl(ref: str, base_rel_dir: PurePosixPath) -> list[str]:
    ref = ref.strip("`'\".,) ")
    if not ref:
        return []
    normalized: list[str] = []
    for item in expand_braces(ref):
        if item.startswith("/"):
            continue
        if item.startswith("experiments/"):
            normalized.append(PurePosixPath(item).as_posix())
        else:
            normalized.append((base_rel_dir / item).as_posix())
    return normalized


def extract_doc_sections(path: Path) -> dict[str, list[str]]:
    text = path.read_text(encoding="utf-8", errors="replace")
    if path.suffix == ".rs":
        lines: list[str] = []
        for line in text.splitlines():
            if line.startswith("//!"):
                lines.append(line[3:].lstrip())
            elif lines:
                break
        doc = "\n".join(lines)
    else:
        match = re.search(r'"""(.*?)"""', text, re.S)
        doc = match.group(1) if match else ""

    sections: dict[str, list[str]] = defaultdict(list)
    for name, body in SECTION_RE.findall(doc):
        sections[name].append(body.strip())
    return sections


def extract_jsonls_from_sections(path: Path, section_name: str, base_rel_dir: PurePosixPath) -> list[str]:
    outputs: list[str] = []
    for body in extract_doc_sections(path).get(section_name, []):
        for match in JSONL_RE.findall(body):
            outputs.extend(normalize_jsonl(match, base_rel_dir))
    return sorted(set(outputs))


def extract_rust_jsonls(path: Path, section_name: str, package_base: PurePosixPath) -> list[str]:
    file_base = PurePosixPath(path.relative_to(repo).parent.as_posix())
    outputs: list[str] = []
    for body in extract_doc_sections(path).get(section_name, []):
        for match in JSONL_RE.findall(body):
            base = package_base if "/" in match else file_base
            outputs.extend(normalize_jsonl(match, base))
    return sorted(set(outputs))


rust_bins: list[dict] = []
for cargo_toml in sorted(experiments.glob("**/Cargo.toml")):
    data = tomllib.loads(cargo_toml.read_text(encoding="utf-8"))
    package = data.get("package", {}).get("name")
    for bin_entry in data.get("bin", []):
        rel_path = (cargo_toml.parent / bin_entry["path"]).relative_to(repo)
        rust_bins.append(
            {
                "package": package,
                "name": bin_entry["name"],
                "path": rel_path,
                "base_dir": PurePosixPath(cargo_toml.parent.relative_to(repo).as_posix()),
                "command": f"cargo run -p {package} --release --bin {bin_entry['name']}",
            }
        )

python_scripts: list[dict] = []
for analyze_py in sorted(experiments.glob("**/analyze.py")):
    python_scripts.append(
        {
            "path": analyze_py.relative_to(repo),
            "command": f"uv run {analyze_py.relative_to(repo).as_posix()}",
        }
    )

data_nodes: set[str] = set()
data_producers: dict[str, list[Path]] = defaultdict(list)
data_input_dependencies: dict[str, set[str]] = defaultdict(set)
data_consumers: dict[str, list[Path]] = defaultdict(list)
producer_kinds: dict[str, str] = {}
producer_ios: dict[str, dict[str, list[str] | str]] = {}
unconnected_rust: list[str] = []
unconnected_py: list[str] = []

for binary in rust_bins:
    rel = binary["path"].as_posix()
    inputs = extract_rust_jsonls(repo / rel, "Input", binary["base_dir"])
    outputs = extract_rust_jsonls(repo / rel, "Output", binary["base_dir"])
    producer_kinds[binary["command"]] = "rust"
    producer_ios[binary["command"]] = {"path": rel, "inputs": inputs, "outputs": outputs}
    if not inputs and not outputs:
        unconnected_rust.append(binary["command"])
    for path in inputs:
        data_nodes.add(path)
        data_consumers[path].append(repo / rel)
    for path in outputs:
        data_nodes.add(path)
        data_producers[path].append(repo / rel)
        for input_path in inputs:
            if input_path not in outputs:
                data_input_dependencies[path].add(input_path)

for script in python_scripts:
    rel = script["path"].as_posix()
    base_dir = PurePosixPath(script["path"].parent.as_posix())
    inputs = extract_jsonls_from_sections(repo / rel, "Input", base_dir)
    outputs = extract_jsonls_from_sections(repo / rel, "Output", base_dir)
    producer_kinds[script["command"]] = "python"
    producer_ios[script["command"]] = {"path": rel, "inputs": inputs, "outputs": outputs}
    if not inputs and not outputs:
        unconnected_py.append(script["command"])
    for path in inputs:
        data_nodes.add(path)
        data_consumers[path].append(repo / rel)
    for path in outputs:
        data_nodes.add(path)
        data_producers[path].append(repo / rel)
        for input_path in inputs:
            if input_path not in outputs:
                data_input_dependencies[path].add(input_path)

tracked_jsonls = subprocess.run(
    ["git", "ls-files", "--", "experiments/**/*.jsonl"],
    check=True,
    capture_output=True,
    text=True,
    cwd=repo,
).stdout.splitlines()
tracked_jsonls_set = {path for path in tracked_jsonls if path}
data_nodes.update(tracked_jsonls_set)


def is_pattern(path: str) -> bool:
    return any(ch in path for ch in "*{}<>")


def is_smoke(path: str) -> bool:
    pure = PurePosixPath(path)
    return pure.name.startswith("smoke") or "/data/smoke" in path or "/smoke-" in path


def join_paths(paths: list[str]) -> str:
    if not paths:
        return "-"
    return "<br>".join(f"`{path}`" for path in paths)


canonical_tracked = sorted(path for path in tracked_jsonls_set if not is_smoke(path))
tracked_smoke = sorted(path for path in tracked_jsonls_set if is_smoke(path))
pattern_paths = sorted(path for path in data_nodes if is_pattern(path))
orphan_tracked = sorted(
    path
    for path in canonical_tracked
    if path not in data_producers and path not in data_consumers
)
canonical_freshness_targets = [path for path in canonical_tracked if path not in orphan_tracked]


def freshness_row(path: str) -> dict[str, str]:
    abs_path = repo / path
    exists = abs_path.exists()
    data_fs = abs_path.stat().st_mtime if exists else None
    producer_fs_times = [
        producer.stat().st_mtime
        for producer in data_producers.get(path, [])
        if producer.exists()
    ]
    producer_git_times = [
        git_last_change_ts(str(producer.relative_to(repo)))
        for producer in data_producers.get(path, [])
        if producer.exists()
    ]
    producer_git_times = [ts for ts in producer_git_times if ts is not None]
    newest_producer_fs = max(producer_fs_times) if producer_fs_times else None
    newest_producer_git = max(producer_git_times) if producer_git_times else None
    input_paths = sorted(data_input_dependencies.get(path, set()))
    input_fs_times = [
        (repo / input_path).stat().st_mtime
        for input_path in input_paths
        if (repo / input_path).exists()
    ]
    input_git_times = [
        git_last_change_ts(input_path)
        for input_path in input_paths
    ]
    input_git_times = [ts for ts in input_git_times if ts is not None]
    newest_input_fs = max(input_fs_times) if input_fs_times else None
    newest_input_git = max(input_git_times) if input_git_times else None

    if not exists:
        status = "missing"
    elif newest_producer_fs is None:
        status = "no-producer-header"
    else:
        stale_from_source = newest_producer_fs is not None and newest_producer_fs > data_fs
        stale_from_input = newest_input_fs is not None and newest_input_fs > data_fs
        if stale_from_source and stale_from_input:
            status = "source-and-input-newer-than-data"
        elif stale_from_source:
            status = "source-newer-than-data"
        elif stale_from_input:
            status = "input-newer-than-data"
        else:
            status = "ok"

    return {
        "path": path,
        "exists": "yes" if exists else "no",
        "data_fs": fmt_timestamp(data_fs),
        "data_git": git_last_change(path),
        "producer_fs": fmt_timestamp(newest_producer_fs),
        "producer_git": fmt_timestamp(newest_producer_git),
        "input_fs": fmt_timestamp(newest_input_fs),
        "input_git": fmt_timestamp(newest_input_git),
        "status": status,
    }


freshness_rows = [freshness_row(path) for path in canonical_freshness_targets]

print("# DATAFLOW")
print()
print("Generated by `scripts/dataflow.sh`.")
print()
print("This is a declared entrypoint/header audit of experiment JSONL producers and consumers in the current worktree, not a full transitive source provenance graph.")
print()
print("This inventory treats tracked canonical JSONL outputs as stale until rerun.")
print()
print("Timestamp audit semantics:")
print("- `fs mtime` is the current file timestamp in the worktree and approximates last generation for regenerated artifacts.")
print("- `git last change` is the latest committed timestamp for that path and approximates the last committed content change.")
print("- `newest input` is the newest tracked canonical JSONL consumed by the producing command, so `input-newer-than-data` flags wrong DAG time direction.")
print()
print("## Canonical Producer Inventory")
print()
for command in sorted(producer_ios):
    inputs = [
        path
        for path in producer_ios[command]["inputs"]  # type: ignore[index]
        if path in tracked_jsonls_set and not is_smoke(path)
    ]
    outputs = [
        path
        for path in producer_ios[command]["outputs"]  # type: ignore[index]
        if path in tracked_jsonls_set and not is_smoke(path)
    ]
    if not inputs and not outputs:
        continue
    print(f"### `{command}`")
    print()
    print(f"- Kind: `{producer_kinds[command]}`")
    print(f"- Source: `{producer_ios[command]['path']}`")
    print(f"- Inputs: {join_paths(inputs)}")
    print(f"- Outputs: {join_paths(outputs)}")
    print()

print("## Canonical Timestamp Audit")
print()
print("| JSONL | exists | fs mtime (UTC) | git last change (UTC) | newest producer fs mtime (UTC) | newest producer git change (UTC) | newest input fs mtime (UTC) | newest input git change (UTC) | status |")
print("|---|---|---|---|---|---|---|---|---|")
for row in freshness_rows:
    print(
        f"| `{row['path']}` | {row['exists']} | {row['data_fs']} | {row['data_git']} | "
        f"{row['producer_fs']} | {row['producer_git']} | {row['input_fs']} | {row['input_git']} | {row['status']} |"
    )

real_problems = [
    row for row in freshness_rows if row["status"] not in {"ok", "no-producer-header"}
]
missing_headers = [row for row in freshness_rows if row["status"] == "no-producer-header"]
if real_problems:
    print()
    print("## Freshness Problems")
    print()
    for row in real_problems:
        print(
            f"- `{row['path']}`: status `{row['status']}`, data fs `{row['data_fs']}`, "
            f"producer fs `{row['producer_fs']}`, producer git `{row['producer_git']}`, "
            f"input fs `{row['input_fs']}`, input git `{row['input_git']}`"
        )

if missing_headers:
    print()
    print("## Missing Producer Headers")
    print()
    print("These tracked JSONL files exist and were refreshed in this session, but the")
    print("current experiment headers do not declare a producer command for them, so the")
    print("generator cannot place them fully inside the DAG freshness audit.")
    print()
    for row in missing_headers:
        print(
            f"- `{row['path']}`: status `{row['status']}`, data fs `{row['data_fs']}`, "
            f"producer fs `{row['producer_fs']}`, producer git `{row['producer_git']}`, "
            f"input fs `{row['input_fs']}`, input git `{row['input_git']}`"
        )

if tracked_smoke or pattern_paths:
    print()
    print("## Non-Canonical JSONL Paths")
    print()
    if tracked_smoke:
        print("### Tracked Smoke Outputs")
        print()
        for path in tracked_smoke:
            print(f"- `{path}`")
        print()
    if orphan_tracked:
        print("### Detached Tracked JSONL Paths")
        print()
        print("Tracked JSONL with no declared producer and no declared consumer in")
        print("experiment headers. These are repo-history leftovers or manual mirrors,")
        print("not freshness targets for the current canonical DAG.")
        print()
        for path in orphan_tracked:
            print(f"- `{path}`")
        print()
    if pattern_paths:
        print("### Header Patterns / Untracked Paths")
        print()
        for path in pattern_paths:
            print(f"- `{path}`")

if unconnected_rust or unconnected_py:
    print()
    print("## No JSONL Edges Detected")
    print()
    for command in sorted(unconnected_rust):
        print(f"- `{command}`")
    for command in sorted(unconnected_py):
        print(f"- `{command}`")
PY
