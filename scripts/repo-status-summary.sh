#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

STATUS_FILE="${1:-}"

usage() {
  cat <<'EOF'
Usage: scripts/repo-status-summary.sh [tasks/references/repo-status-*.md]

Read-only orientation helper. It compares current HEAD and working tree state
against a dated repo-status reference, reports check-affecting and
orientation-affecting changed paths, repeats the generated-data freshness
caveat and high-risk artifact-refresh areas, and summarizes Git LFS payload
presence.

With no argument, uses the newest tasks/references/repo-status-*.md file by
filename.
EOF
}

case "$STATUS_FILE" in
  -h|--help)
    usage
    exit 0
    ;;
esac

CHECK_PATHS=(
  '*.rs'
  '**/Cargo.toml'
  'Cargo.toml'
  'Cargo.lock'
  'experiments/**/*.jsonl'
  'experiments/**/*.json'
  'experiments/**/*.png'
  'experiments/**/*.svg'
  'experiments/**/*.pdf'
  'experiments/**/*.html'
  'experiments/**/*.txt'
  'experiments/**/*.tex'
  'experiments/**/*.py'
  'thesis/**/*.tex'
  'thesis/bibliography.bib'
  'thesis/.latexmkrc'
  'thesis/*.sh'
  'thesis/*.py'
  'thesis/**/*.png'
  'thesis/**/*.svg'
  'thesis/**/*.pdf'
  'formal/**/*.tex'
  'formal/bibliography.bib'
  'formal/.latexmkrc'
  'tasks/submit-thesis/**/*.pdf'
  '.devcontainer/**'
  'scripts/**'
  ':(exclude)scripts/repo-status-summary.sh'
  ':(exclude)scripts/README.md'
)
ORIENTATION_PATHS=(
  'AGENTS.md'
  '.gitignore'
  '.agents/skills/**'
  '.codex/agents/**'
  'CAPABILITY_CLAIM_MAP.md'
  'tasks/*.md'
  'tasks/**/*.md'
  'research/INDEX.md'
  'experiments/MAP.md'
  'crates/MAP.md'
  'thesis/MAP.md'
  'scripts/README.md'
  'scripts/repo-status-summary.sh'
)

if [[ -z "$STATUS_FILE" ]]; then
  STATUS_FILE="$(
    find tasks/references -maxdepth 1 -type f -name 'repo-status-*.md' -print |
      sort |
      tail -n 1
  )"
fi

if [[ ! -f "$STATUS_FILE" ]]; then
  echo "status file missing: $STATUS_FILE" >&2
  exit 1
fi

VERIFIED_COMMIT="$(
  sed -n 's/^- Commands listed below were run through commit `\([0-9a-f]\+\)`.*/\1/p' "$STATUS_FILE" |
    head -n 1
)"

if [[ -z "$VERIFIED_COMMIT" ]]; then
  echo "could not parse verified commit from $STATUS_FILE" >&2
  exit 1
fi

echo "Repo status summary"
echo
echo "HEAD: $(git rev-parse --short HEAD)"
echo "Verification reference: $STATUS_FILE"
echo "Commands in that reference were run through: $VERIFIED_COMMIT"
echo

echo "Working tree:"
if [[ -n "$(git status --short)" ]]; then
  git status --short
else
  echo "clean"
fi
echo

echo "Changed paths since verified commit:"
if git diff --quiet "$VERIFIED_COMMIT"..HEAD --; then
  echo "none"
else
  git diff --name-only "$VERIFIED_COMMIT"..HEAD --
fi
echo

CHECK_AFFECTING_PATHS="$(
  git diff --name-only "$VERIFIED_COMMIT"..HEAD -- "${CHECK_PATHS[@]}"
)"

UNCOMMITTED_CHECK_AFFECTING_PATHS="$(
  {
    git diff --name-only -- "${CHECK_PATHS[@]}"
    git diff --cached --name-only -- "${CHECK_PATHS[@]}"
    git ls-files --others --exclude-standard -- "${CHECK_PATHS[@]}"
  } | sort -u
)"

ORIENTATION_AFFECTING_PATHS="$(
  git diff --name-only "$VERIFIED_COMMIT"..HEAD -- "${ORIENTATION_PATHS[@]}"
)"

UNCOMMITTED_ORIENTATION_AFFECTING_PATHS="$(
  {
    git diff --name-only -- "${ORIENTATION_PATHS[@]}"
    git diff --cached --name-only -- "${ORIENTATION_PATHS[@]}"
    git ls-files --others --exclude-standard -- "${ORIENTATION_PATHS[@]}"
  } | sort -u
)"

echo "Check-affecting changed paths:"
if [[ -n "$CHECK_AFFECTING_PATHS" ]]; then
  printf '%s\n' "$CHECK_AFFECTING_PATHS"
else
  echo "none"
fi
echo

echo "Uncommitted check-affecting paths:"
if [[ -n "$UNCOMMITTED_CHECK_AFFECTING_PATHS" ]]; then
  printf '%s\n' "$UNCOMMITTED_CHECK_AFFECTING_PATHS"
else
  echo "none"
fi
echo

echo "Orientation-affecting changed paths:"
if [[ -n "$ORIENTATION_AFFECTING_PATHS" ]]; then
  printf '%s\n' "$ORIENTATION_AFFECTING_PATHS"
else
  echo "none"
fi
echo

echo "Uncommitted orientation-affecting paths:"
if [[ -n "$UNCOMMITTED_ORIENTATION_AFFECTING_PATHS" ]]; then
  printf '%s\n' "$UNCOMMITTED_ORIENTATION_AFFECTING_PATHS"
else
  echo "none"
fi
echo

echo "Refresh guidance:"
if [[ -n "$UNCOMMITTED_CHECK_AFFECTING_PATHS" ]]; then
  echo "Working tree has uncommitted check-affecting paths. Inspect them before relying on old results."
elif [[ -n "$(git status --short)" ]]; then
  echo "Working tree is not clean, but no uncommitted check-affecting path is detected by this helper."
  echo "For stronger claims, inspect git status and read $STATUS_FILE."
elif [[ -z "$CHECK_AFFECTING_PATHS" ]]; then
  echo "No code, data, build-contract, thesis, formal, or other script path changed since the referenced checks."
  if [[ -n "$ORIENTATION_AFFECTING_PATHS" ]]; then
    echo "Task, map, or helper guidance changed since then; read the listed orientation-affecting paths for current next-work routing."
  fi
  echo "For stronger claims, read $STATUS_FILE and rerun affected checks."
else
  echo "Potentially check-affecting paths changed since the referenced checks."
  echo "Read $STATUS_FILE, inspect the changed paths above, and rerun affected checks before relying on old results."
fi
echo
echo "Data freshness:"
echo "The referenced checks did not refresh tracked datasets, figures, or generated experiment reports."
echo "Use the Artifact-Refresh Boundary section in $STATUS_FILE before treating generated artifacts as fresh."
echo "High-risk refresh areas:"
echo "- experiments/verification/"
echo "- experiments/hko-local-maximum/"
echo "- experiments/sys-landscape/"
echo "- experiments/dev-quadratic-program/numerics-audit/"
echo "- experiments/combinatorial-cells/"
echo "- experiments/crosspolytope/ and experiments/visualization/"
echo "- tasks/submit-thesis/"
echo

echo "LFS payloads:"
if git lfs version >/dev/null 2>&1; then
  LFS_LIST="$(git lfs ls-files)"
  if [[ -z "$LFS_LIST" ]]; then
    echo "no LFS-tracked files reported"
  else
    LFS_TOTAL="$(printf '%s\n' "$LFS_LIST" | awk 'NF {count++} END {print count + 0}')"
    LFS_PRESENT="$(printf '%s\n' "$LFS_LIST" | awk '$2 == "*" {count++} END {print count + 0}')"
    LFS_MISSING="$(printf '%s\n' "$LFS_LIST" | awk '$2 == "-" {count++} END {print count + 0}')"
    echo "$LFS_PRESENT present, $LFS_MISSING missing, $LFS_TOTAL tracked"
    if [[ "$LFS_MISSING" != "0" ]]; then
      printf '%s\n' "$LFS_LIST" |
        awk '$2 == "-" {for (i = 3; i <= NF; i++) printf "%s%s", $i, (i == NF ? ORS : OFS)}'
      echo "Run targeted git lfs pull only for files needed by the task."
    fi
  fi
else
  echo "git lfs is not available in this environment"
fi
