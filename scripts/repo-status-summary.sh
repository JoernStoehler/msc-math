#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

STATUS_FILE="${1:-}"

usage() {
  cat <<'EOF'
Usage: scripts/repo-status-summary.sh [scripts/repo-status/repo-status-*.md]

Read-only orientation helper. It compares current HEAD and working tree state
against a dated repo-status reference, reports check-affecting and
orientation-affecting changed paths, repeats the generated-data freshness
caveat and high-risk artifact-refresh areas, and lists registered shared
artifacts.

With no argument, uses the newest scripts/repo-status/repo-status-*.md file by
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
  'submit/**/*.pdf'
  'container/**'
  '.codex/config.toml'
  '.codex/base-instructions.md'
  'scripts/**'
  ':(exclude)scripts/repo-status-summary.sh'
  ':(exclude)scripts/README.md'
)
ORIENTATION_PATHS=(
  'AGENTS.md'
  'ARCHITECTURE.md'
  'README.md'
  '.gitignore'
  '.agents/skills/**'
  '.codex/agents/**'
  'docs/README.md'
  'docs/capabilities.md'
  'docs/project-facts.md'
  'docs/project-status.md'
  'docs/reproducibility.md'
  'thesis/central-claim-control.md'
  'experiments/README.md'
  'crates/README.md'
  'thesis/README.md'
  'formal/README.md'
  'scripts/README.md'
  'scripts/repo-status-summary.sh'
)

if [[ -z "$STATUS_FILE" ]]; then
  STATUS_FILE="$(
    find scripts/repo-status -maxdepth 1 -type f -name 'repo-status-*.md' -print |
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
echo "- submit/"
echo

echo "Shared artifacts:"
scripts/artifacts.py list
echo "Materialization is explicit; this status command does not contact R2."
