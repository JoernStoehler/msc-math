#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

STATUS_FILE="tasks/references/repo-status-smoke-and-core-2026-05-31.md"

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
  git diff --name-only "$VERIFIED_COMMIT"..HEAD -- \
    '*.rs' \
    '**/Cargo.toml' \
    'Cargo.toml' \
    'Cargo.lock' \
    'experiments/**/*.jsonl' \
    'experiments/**/*.png' \
    'experiments/**/*.tex' \
    'thesis/**/*.tex' \
    'thesis/bibliography.bib' \
    'formal/**/*.tex' \
    'formal/bibliography.bib' \
    '.devcontainer/**' \
    'scripts/**' \
    ':(exclude)scripts/repo-status-summary.sh'
)"

echo "Check-affecting changed paths:"
if [[ -n "$CHECK_AFFECTING_PATHS" ]]; then
  printf '%s\n' "$CHECK_AFFECTING_PATHS"
else
  echo "none"
fi
echo

echo "Refresh guidance:"
if [[ -n "$(git status --short)" ]]; then
  echo "Working tree is not clean. Inspect uncommitted changes before relying on old results."
elif [[ -z "$CHECK_AFFECTING_PATHS" ]]; then
  echo "No code, data, build-contract, thesis, formal, or other script path changed since the referenced checks."
  echo "For stronger claims, read $STATUS_FILE and rerun affected checks."
else
  echo "Potentially check-affecting paths changed since the referenced checks."
  echo "Read $STATUS_FILE, inspect the changed paths above, and rerun affected checks before relying on old results."
fi
echo
echo "Data freshness:"
echo "The referenced checks did not refresh tracked datasets, figures, or generated experiment reports."
echo "Use the Artifact-Refresh Boundary section in $STATUS_FILE before treating generated artifacts as fresh."
