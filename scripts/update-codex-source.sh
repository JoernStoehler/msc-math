#!/usr/bin/env bash
set -euo pipefail

# Source navigation only: no build, installation, config edit or daemon restart.
codex_source_dir=${CODEX_SOURCE_DIR:-${XDG_CACHE_HOME:-$HOME/.cache}/codex-source}
codex_source_remote=https://github.com/openai/codex.git

case ${1:-} in
  '')
    codex_source_version=$(codex --version)
    codex_source_version=${codex_source_version#codex-cli }
    if [[ ! $codex_source_version =~ ^[0-9]+\.[0-9]+\.[0-9]+([-.][a-zA-Z0-9.-]+)?$ ]]; then
      printf 'Cannot select a release tag from codex --version: %s\n' "$codex_source_version" >&2
      exit 1
    fi
    codex_source_ref=rust-v$codex_source_version
    ;;
  --main) codex_source_ref=main ;;
  -h|--help)
    printf '%s\n' 'Usage: bash scripts/update-codex-source.sh [--main]' \
      'Default: refresh source at the installed CLI release tag.' \
      '--main: inspect upstream development; this does not match the installed CLI.' \
      'CODEX_SOURCE_DIR overrides ~/.cache/codex-source (or $XDG_CACHE_HOME/codex-source).'
    exit 0
    ;;
  *) printf 'Unknown option: %s\n' "$1" >&2; exit 2 ;;
esac
if (( $# > 1 )); then
  printf 'Expected at most one option.\n' >&2
  exit 2
fi

if [[ -e $codex_source_dir ]]; then
  if [[ ! -d $codex_source_dir/.git ]]; then
    printf 'Refusing existing non-checkout path: %s\n' "$codex_source_dir" >&2
    exit 1
  fi
  if [[ $(git -C "$codex_source_dir" remote get-url origin) != "$codex_source_remote" ]]; then
    printf 'Refusing checkout with a different origin: %s\n' "$codex_source_dir" >&2
    exit 1
  fi
  if [[ -n $(git -C "$codex_source_dir" status --porcelain) ]]; then
    printf 'Refusing to refresh a dirty checkout: %s\n' "$codex_source_dir" >&2
    exit 1
  fi
  git -C "$codex_source_dir" fetch --depth 1 origin "$codex_source_ref"
  git -C "$codex_source_dir" checkout --detach FETCH_HEAD
else
  mkdir -p "$(dirname "$codex_source_dir")"
  git clone --depth 1 --branch "$codex_source_ref" "$codex_source_remote" "$codex_source_dir"
fi

printf 'Source: %s\nRequested ref: %s\nCommit: ' "$codex_source_dir" "$codex_source_ref"
git -C "$codex_source_dir" rev-parse HEAD
