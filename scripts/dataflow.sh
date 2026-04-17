#!/usr/bin/env bash
# Purpose: Audit declared experiment artifact ownership and freshness.
# Context: Parses Cargo binary entrypoints plus experiment Python scripts for
#          `Input Artifacts:` / `Output Artifacts:` declarations, then reports
#          file-granular producers, consumers, Git state, and simple timestamp
#          freshness risks for declared artifacts under `experiments/`.

set -euo pipefail

REPO_ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
OUTPUT_PATH="$REPO_ROOT/DATAFLOW.md"
FORMAT="markdown"

usage() {
  cat <<'EOF'
Usage: bash scripts/dataflow.sh [--format markdown|mermaid]

Formats:
  markdown  Default. Tabular audit.
  mermaid   Markdown document with a fenced Mermaid graph plus audit sections.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --format)
      FORMAT="${2:-}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

case "$FORMAT" in
  markdown|mermaid) ;;
  *)
    echo "Unsupported format: $FORMAT" >&2
    usage >&2
    exit 2
    ;;
esac

UPDATE_CMD=("bash" "scripts/dataflow.sh")
if [[ "$FORMAT" != "markdown" ]]; then
  UPDATE_CMD+=("--format" "$FORMAT")
fi
UPDATE_CMD_STR="$(printf '%q ' "${UPDATE_CMD[@]}")"
UPDATE_CMD_STR="${UPDATE_CMD_STR% }"

TMP_FILE="$(mktemp)"
python3 - "$REPO_ROOT" "$FORMAT" "$UPDATE_CMD_STR" <<'PY' > "$TMP_FILE"
import fnmatch
import re
import subprocess
import sys
import tomllib
from collections import defaultdict
from datetime import UTC, datetime
from pathlib import Path, PurePosixPath

repo = Path(sys.argv[1]).resolve()
output_format = sys.argv[2]
update_cmd = sys.argv[3]
experiments = repo / "experiments"

ARTIFACT_EXTENSIONS = (".jsonl", ".png", ".csv")
ARTIFACT_SUFFIX_RE = "|".join(ext[1:] for ext in ARTIFACT_EXTENSIONS)
SECTION_RE = re.compile(
    r"^(Goal|Input Artifacts|Output Artifacts):\s*(.*?)(?=^(?:Goal|Input Artifacts|Output Artifacts):|\Z)",
    re.M | re.S,
)
ARTIFACT_RE = re.compile(
    rf"(?:experiments/)?[A-Za-z0-9_./{{}},*?\[\]<>-]+\.(?:{ARTIFACT_SUFFIX_RE})"
)
GLOB_CHARS = set("*?[]{}")
TS_EPS = 1e-9


def run(cmd: list[str], *, check: bool = True) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        cmd,
        cwd=repo,
        check=check,
        capture_output=True,
        text=True,
    )


def fmt_timestamp(ts: float | None) -> str:
    if ts is None:
        return "-"
    return datetime.fromtimestamp(ts, UTC).isoformat(timespec="seconds").replace("+00:00", "Z")


def sanitize_id(prefix: str, value: str) -> str:
    token = re.sub(r"[^A-Za-z0-9]+", "_", value).strip("_")
    return f"{prefix}_{token}" if token else prefix


def mermaid_text(text: str) -> str:
    return text.replace('"', "'")


def git_last_change(path_str: str) -> str:
    result = run(["git", "log", "-1", "--format=%cI", "--", path_str], check=False)
    return result.stdout.strip() or "-"


def git_last_change_ts(path_str: str) -> float | None:
    raw = git_last_change(path_str)
    if raw == "-":
        return None
    return datetime.fromisoformat(raw.replace("Z", "+00:00")).timestamp()


def path_exists(rel_path: str) -> bool:
    return (repo / rel_path).exists()


def path_mtime(rel_path: str) -> float | None:
    path = repo / rel_path
    return path.stat().st_mtime if path.exists() else None


def is_ignored(rel_path: str) -> bool:
    result = run(["git", "check-ignore", "-q", "--", rel_path], check=False)
    return result.returncode == 0


def is_pattern(path_str: str) -> bool:
    return any(ch in GLOB_CHARS for ch in path_str)


def dedupe(seq):
    seen = set()
    out = []
    for item in seq:
        if item not in seen:
            seen.add(item)
            out.append(item)
    return out


def expand_braces(pattern: str) -> list[str]:
    match = re.search(r"\{([^{}]+)\}", pattern)
    if not match:
        return [pattern]
    prefix = pattern[: match.start()]
    suffix = pattern[match.end() :]
    expanded: list[str] = []
    for option in match.group(1).split(","):
        expanded.extend(expand_braces(prefix + option + suffix))
    return expanded


def extract_doc_text(path: Path) -> str:
    text = path.read_text(encoding="utf-8", errors="replace")
    if path.suffix == ".rs":
        lines: list[str] = []
        started = False
        for line in text.splitlines():
            if not started:
                if line.startswith("//!"):
                    lines.append(line[3:].lstrip())
                    started = True
                elif line.startswith("#![") or not line.strip():
                    continue
                else:
                    break
            else:
                if line.startswith("//!"):
                    lines.append(line[3:].lstrip())
                elif not line.strip():
                    lines.append("")
                else:
                    break
        return "\n".join(lines)

    match = re.search(r'([\'"]{3})(.*?)\1', text, re.S)
    return match.group(2) if match else ""


def parse_sections(path: Path) -> dict[str, list[str]]:
    sections: dict[str, list[str]] = defaultdict(list)
    for name, body in SECTION_RE.findall(extract_doc_text(path)):
        sections[name].append(body.strip())
    return sections


def normalize_ref(
    ref: str,
    *,
    file_base: PurePosixPath,
    package_base: PurePosixPath | None,
    source_kind: str,
) -> list[str]:
    ref = ref.strip("`'\".,;:) ")
    if not ref or ref.lower().startswith("none"):
        return []
    normalized: list[str] = []
    for item in expand_braces(ref):
        if item.startswith("/"):
            continue
        if item.startswith("experiments/"):
            normalized.append(PurePosixPath(item).as_posix())
        elif source_kind == "rust" and package_base is not None and "/" in item:
            normalized.append((package_base / item).as_posix())
        else:
            normalized.append((file_base / item).as_posix())
    return normalized


def extract_declared_artifacts(
    path: Path,
    *,
    source_kind: str,
    package_base: PurePosixPath | None,
) -> dict[str, list[str]]:
    file_base = PurePosixPath(path.relative_to(repo).parent.as_posix())
    sections = parse_sections(path)
    parsed: dict[str, list[str]] = {}
    for field in ("Input Artifacts", "Output Artifacts"):
        refs: list[str] = []
        for body in sections.get(field, []):
            for match in ARTIFACT_RE.findall(body):
                refs.extend(
                    normalize_ref(
                        match,
                        file_base=file_base,
                        package_base=package_base,
                        source_kind=source_kind,
                    )
                )
        parsed[field] = dedupe(refs)
    parsed["__missing__"] = [
        field for field in ("Input Artifacts", "Output Artifacts") if field not in sections
    ]
    return parsed


def collect_sources() -> list[dict]:
    sources: list[dict] = []
    for cargo_toml in sorted(experiments.glob("**/Cargo.toml")):
        data = tomllib.loads(cargo_toml.read_text(encoding="utf-8"))
        package = data.get("package", {}).get("name")
        package_base = PurePosixPath(cargo_toml.parent.relative_to(repo).as_posix())
        for bin_entry in data.get("bin", []):
            rel_path = (cargo_toml.parent / bin_entry["path"]).relative_to(repo)
            sources.append(
                {
                    "kind": "rust",
                    "path": rel_path.as_posix(),
                    "package_base": package_base,
                    "display": f"{rel_path.as_posix()} (cargo bin {bin_entry['name']})",
                    "command": f"cargo run -p {package} --release --bin {bin_entry['name']}",
                }
            )
    for py_path in sorted(experiments.glob("**/*.py")):
        rel_path = py_path.relative_to(repo).as_posix()
        sources.append(
            {
                "kind": "python",
                "path": rel_path,
                "package_base": None,
                "display": f"{rel_path} (python)",
                "command": f"uv run {rel_path}",
            }
        )
    return sources


tracked_paths = {
    path
    for path in run(["git", "ls-files", "--", "experiments"]).stdout.splitlines()
    if path.endswith(ARTIFACT_EXTENSIONS)
}
present_paths = {
    path.relative_to(repo).as_posix()
    for path in experiments.rglob("*")
    if path.is_file() and path.suffix in ARTIFACT_EXTENSIONS
}
artifact_paths = set(tracked_paths) | set(present_paths)

sources = collect_sources()
missing_header_fields: list[tuple[str, list[str]]] = []
unmatched_declared_patterns: list[tuple[str, str, str]] = []
artifact_producers: dict[str, list[str]] = defaultdict(list)
artifact_consumers: dict[str, list[str]] = defaultdict(list)
source_inputs: dict[str, list[str]] = {}
source_outputs: dict[str, list[str]] = {}
source_displays: dict[str, str] = {}

for source in sources:
    rel_path = source["path"]
    source_displays[rel_path] = source["display"]
    parsed = extract_declared_artifacts(
        repo / rel_path,
        source_kind=source["kind"],
        package_base=source["package_base"],
    )
    if parsed["__missing__"]:
        missing_header_fields.append((rel_path, parsed["__missing__"]))

    resolved_inputs: list[str] = []
    resolved_outputs: list[str] = []

    for field_name, target in (
        ("Input Artifacts", resolved_inputs),
        ("Output Artifacts", resolved_outputs),
    ):
        for ref in parsed[field_name]:
            if is_pattern(ref):
                matches = sorted(path for path in artifact_paths if fnmatch.fnmatchcase(path, ref))
                if matches:
                    target.extend(matches)
                else:
                    unmatched_declared_patterns.append((rel_path, field_name, ref))
            else:
                artifact_paths.add(ref)
                target.append(ref)

    source_inputs[rel_path] = dedupe(resolved_inputs)
    source_outputs[rel_path] = dedupe(resolved_outputs)

    for artifact in source_inputs[rel_path]:
        artifact_consumers[artifact].append(rel_path)
    for artifact in source_outputs[rel_path]:
        artifact_producers[artifact].append(rel_path)

artifact_paths = sorted(artifact_paths)

freshness_flags: dict[str, list[str]] = defaultdict(list)
for source_path, outputs in source_outputs.items():
    source_m = path_mtime(source_path)
    source_git = git_last_change_ts(source_path)
    input_m = max(
        (ts for ts in (path_mtime(path) for path in source_inputs[source_path]) if ts is not None),
        default=None,
    )
    input_git = max(
        (ts for ts in (git_last_change_ts(path) for path in source_inputs[source_path]) if ts is not None),
        default=None,
    )
    for artifact in outputs:
        art_m = path_mtime(artifact)
        art_git = git_last_change_ts(artifact)
        if art_m is None:
            freshness_flags[artifact].append("missing")
            continue
        if source_m is not None and source_m > art_m + TS_EPS:
            freshness_flags[artifact].append("source-mtime-newer")
        if input_m is not None and input_m > art_m + TS_EPS:
            freshness_flags[artifact].append("input-mtime-newer")
        if source_git is not None and art_git is not None and source_git > art_git + TS_EPS:
            freshness_flags[artifact].append("source-git-newer")
        if input_git is not None and art_git is not None and input_git > art_git + TS_EPS:
            freshness_flags[artifact].append("input-git-newer")


def join_display(paths: list[str]) -> str:
    if not paths:
        return "-"
    return "<br>".join(f"`{source_displays[path]}`" for path in sorted(paths))


def join_flags(flags: list[str]) -> str:
    flags = dedupe(flags)
    return ", ".join(flags) if flags else "ok"


def format_summary_lines() -> list[str]:
    return [
        f"- Sources scanned: `{len(sources)}`",
        f"- Artifact files seen or declared: `{len(artifact_paths)}`",
        f"- Missing declaration fields: `{len(missing_header_fields)}`",
        f"- Tracked artifacts without a producer: `{len(tracked_without_producer)}`",
        f"- Tracked artifacts without a consumer: `{len(tracked_without_consumer)}`",
        f"- Artifacts with multiple producers: `{len(multiple_producers)}`",
        f"- Artifacts with freshness-risk flags: `{len(freshness_risks)}`",
        f"- Unmatched declared patterns: `{len(unmatched_declared_patterns)}`",
    ]


def render_list_section(title: str, rows: list[str]) -> None:
    if not rows:
        return
    print(f"## {title}")
    print()
    for row in rows:
        print(f"- {row}")
    print()


def render_problem_sections() -> None:
    render_list_section(
        "Missing Declaration Fields",
        [f"`{path}`: missing {', '.join(fields)}" for path, fields in missing_header_fields],
    )
    render_list_section(
        "Tracked Artifacts Without Producer",
        [f"`{path}`" for path in tracked_without_producer],
    )
    render_list_section(
        "Tracked Artifacts Without Consumer",
        [f"`{path}`" for path in tracked_without_consumer],
    )
    render_list_section(
        "Artifacts With Multiple Producers",
        [
            f"`{path}`: {join_display(sorted(set(artifact_producers[path])))}"
            for path in multiple_producers
        ],
    )
    render_list_section(
        "Freshness Risks",
        [f"`{path}`: {join_flags(freshness_flags[path])}" for path in freshness_risks],
    )
    render_list_section(
        "Unmatched Declared Patterns",
        [
            f"`{source}` `{field}` -> `{pattern}`"
            for source, field, pattern in unmatched_declared_patterns
        ],
    )


def artifact_class(artifact: str) -> str:
    if artifact in multiple_producers:
        return "artifact_multi"
    if artifact in tracked_without_producer:
        return "artifact_no_producer"
    if freshness_flags.get(artifact):
        return "artifact_freshness"
    if artifact in tracked_without_consumer:
        return "artifact_no_consumer"
    if artifact in tracked_paths:
        return "artifact_tracked"
    return "artifact_untracked"


def artifact_label(artifact: str) -> str:
    tags: list[str] = []
    if artifact in tracked_paths:
        tags.append("tracked")
    if is_ignored(artifact):
        tags.append("ignored")
    if artifact in tracked_without_producer:
        tags.append("no producer")
    if artifact in tracked_without_consumer:
        tags.append("no consumer")
    if artifact in multiple_producers:
        tags.append("multi producer")
    if freshness_flags.get(artifact):
        tags.append(join_flags(freshness_flags[artifact]))
    suffix = f"\\n[{'; '.join(tags)}]" if tags else ""
    return mermaid_text(f"{artifact}{suffix}")


def source_label(source_path: str) -> str:
    source = next(item for item in sources if item["path"] == source_path)
    return mermaid_text(f"{source_path}\\n{source['command']}")


def render_mermaid() -> None:
    print(f"Regenerate with `{update_cmd}`.")
    print()
    print("# Experiment Artifact Dataflow Audit")
    print()
    print(f"- Generated at: `{datetime.now(UTC).isoformat(timespec='seconds').replace('+00:00', 'Z')}`")
    print("- Format: `mermaid`")
    print(
        "- Scope: declared `Input Artifacts:` / `Output Artifacts:` on Cargo binary entrypoints "
        "and experiment Python scripts"
    )
    print(f"- Artifact suffixes: `{', '.join(ARTIFACT_EXTENSIONS)}`")
    print()
    print("## Summary")
    print()
    for line in format_summary_lines():
        print(line)
    print()
    print("## Graph")
    print()
    print("```mermaid")
    print("flowchart LR")

    for source in sorted(sources, key=lambda item: item["path"]):
        src_id = sanitize_id("src", source["path"])
        print(f'    {src_id}["{source_label(source["path"])}"]')
    for artifact in artifact_paths:
        art_id = sanitize_id("art", artifact)
        print(f'    {art_id}["{artifact_label(artifact)}"]')

    for source in sorted(sources, key=lambda item: item["path"]):
        src_id = sanitize_id("src", source["path"])
        for artifact in source_outputs[source["path"]]:
            print(f"    {src_id} --> {sanitize_id('art', artifact)}")
        for artifact in source_inputs[source["path"]]:
            print(f"    {sanitize_id('art', artifact)} --> {src_id}")

    print()
    print("    classDef source fill:#e8f1ff,stroke:#3766b1,stroke-width:1px;")
    print("    classDef artifact_tracked fill:#eef7e8,stroke:#4d7a3a,stroke-width:1px;")
    print("    classDef artifact_untracked fill:#f7f7f7,stroke:#777,stroke-width:1px;")
    print("    classDef artifact_no_producer fill:#ffe7e7,stroke:#b23b3b,stroke-width:2px;")
    print("    classDef artifact_no_consumer fill:#fff3d9,stroke:#b38728,stroke-width:1px;")
    print("    classDef artifact_multi fill:#ffe7ff,stroke:#9a3aa5,stroke-width:2px;")
    print("    classDef artifact_freshness fill:#eaf4ff,stroke:#2c82c9,stroke-width:2px;")

    if sources:
        print(
            "    class "
            + ",".join(sanitize_id("src", source["path"]) for source in sorted(sources, key=lambda item: item["path"]))
            + " source;"
        )

    by_class: dict[str, list[str]] = defaultdict(list)
    for artifact in artifact_paths:
        by_class[artifact_class(artifact)].append(sanitize_id("art", artifact))
    for class_name, ids in sorted(by_class.items()):
        if ids:
            print(f"    class {','.join(ids)} {class_name};")

    print("```")
    print()
    render_problem_sections()


def render_markdown() -> None:
    print(f"Regenerate with `{update_cmd}`.")
    print()
    print("# Experiment Artifact Dataflow Audit")
    print()
    print(f"- Generated at: `{datetime.now(UTC).isoformat(timespec='seconds').replace('+00:00', 'Z')}`")
    print(
        "- Scope: declared `Input Artifacts:` / `Output Artifacts:` on Cargo binary entrypoints "
        "and experiment Python scripts"
    )
    print(f"- Artifact suffixes: `{', '.join(ARTIFACT_EXTENSIONS)}`")
    print()
    print("## Summary")
    print()
    for line in format_summary_lines():
        print(line)
    print()
    render_problem_sections()
    print("## Artifacts")
    print()
    print("| Artifact | Present | Tracked | Ignored | Mtime | Git Last Change | Producer | Consumers | Freshness |")
    print("| --- | --- | --- | --- | --- | --- | --- | --- | --- |")
    for artifact in artifact_paths:
        present = "yes" if path_exists(artifact) else "no"
        tracked = "yes" if artifact in tracked_paths else "no"
        ignored = "yes" if is_ignored(artifact) else "no"
        print(
            "| "
            + " | ".join(
                [
                    f"`{artifact}`",
                    present,
                    tracked,
                    ignored,
                    fmt_timestamp(path_mtime(artifact)),
                    git_last_change(artifact),
                    join_display(sorted(set(artifact_producers.get(artifact, [])))),
                    join_display(sorted(set(artifact_consumers.get(artifact, [])))),
                    join_flags(freshness_flags[artifact]),
                ]
            )
            + " |"
        )

    print()
    print("## Sources")
    print()
    print("| Source | Command | Input Artifacts | Output Artifacts |")
    print("| --- | --- | --- | --- |")
    for source in sorted(sources, key=lambda item: item["path"]):
        src = source["path"]
        in_paths = source_inputs[src]
        out_paths = source_outputs[src]
        print(
            "| "
            + " | ".join(
                [
                    f"`{source['display']}`",
                    f"`{source['command']}`",
                    "<br>".join(f"`{path}`" for path in in_paths) or "-",
                    "<br>".join(f"`{path}`" for path in out_paths) or "-",
                ]
            )
            + " |"
        )

tracked_without_producer = [
    path for path in artifact_paths if path in tracked_paths and not artifact_producers.get(path)
]
tracked_without_consumer = [
    path for path in artifact_paths if path in tracked_paths and not artifact_consumers.get(path)
]
multiple_producers = [
    path for path in artifact_paths if len(set(artifact_producers.get(path, []))) > 1
]
freshness_risks = [path for path in artifact_paths if freshness_flags.get(path)]

if output_format == "mermaid":
    render_mermaid()
else:
    render_markdown()
PY

mv "$TMP_FILE" "$OUTPUT_PATH"
echo "Wrote $OUTPUT_PATH"
