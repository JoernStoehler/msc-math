#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

network_checks=true
case "${1:-}" in
  "")
    ;;
  --offline)
    network_checks=false
    ;;
  *)
    printf 'Usage: %s [--offline]\n' "$0" >&2
    exit 2
    ;;
esac

repository="${CODEX_GITHUB_REPOSITORY:-JoernStoehler/msc-math}"
actual_origin="$(git remote get-url origin 2>/dev/null || true)"
identity_ok=false
case "$actual_origin" in
  "https://github.com/${repository}.git"|"git@github.com:${repository}.git")
    identity_ok=true
    ;;
esac

checkout_root="$(git rev-parse --show-toplevel)"
head_sha="$(git rev-parse HEAD)"
branch="$(git symbolic-ref --quiet --short HEAD 2>/dev/null || printf 'DETACHED')"
worktree_clean=false
[[ -z "$(git status --porcelain)" ]] && worktree_clean=true

command_present() {
  command -v "$1" >/dev/null 2>&1
}

gh_installed=false
gh_authenticated=false
github_repository_read=false
if command_present gh; then
  gh_installed=true
  gh auth status >/dev/null 2>&1 && gh_authenticated=true
fi
if [[ "$network_checks" == true && "$gh_authenticated" == true ]]; then
  resolved_repository="$(gh api "repos/${repository}" --jq .full_name 2>/dev/null || true)"
  [[ "$resolved_repository" == "$repository" ]] && github_repository_read=true
fi

git_remote_read=false
if [[ "$network_checks" == true ]]; then
  GIT_TERMINAL_PROMPT=0 git ls-remote --exit-code origin HEAD >/dev/null 2>&1 &&
    git_remote_read=true
fi

rclone_installed=false
rclone_config_present=false
rclone_config_mode_600=false
rclone_remote_configured=false
r2_read_attempted=false
r2_snapshots_read=false
if command_present rclone; then
  rclone_installed=true
  rclone_config_file="$(rclone config file 2>/dev/null | tail -n 1 || true)"
  if [[ -n "$rclone_config_file" && -s "$rclone_config_file" ]]; then
    rclone_config_present=true
    [[ "$(stat -c '%a' "$rclone_config_file" 2>/dev/null || true)" == 600 ]] &&
      rclone_config_mode_600=true
  fi
  rclone listremotes 2>/dev/null | grep -Fxq 'mscmath:' &&
    rclone_remote_configured=true
  if [[ "$network_checks" == true && "$rclone_remote_configured" == true ]]; then
    r2_read_attempted=true
    rclone lsf mscmath:msc-math-artifacts/snapshots --max-depth 1 \
      --s3-acl '' --s3-no-check-bucket >/dev/null 2>&1 &&
      r2_snapshots_read=true
  fi
fi

python_3_12=false
if command_present python3; then
  python3 -c 'import sys; raise SystemExit(sys.version_info[:2] != (3, 12))' &&
    python_3_12=true
fi

uv_installed=false
rust_1_94=false
cargo_installed=false
rustfmt_installed=false
latexmk_installed=false
biber_installed=false
command_present uv && uv_installed=true
command_present cargo && cargo_installed=true
command_present rustfmt && rustfmt_installed=true
command_present latexmk && latexmk_installed=true
command_present biber && biber_installed=true
if command_present rustc; then
  rustc --version 2>/dev/null | grep -Fq 'rustc 1.94.0 ' && rust_1_94=true
fi

setup_secrets_absent=true
if [[ -n "${MSC_MATH_R2_ACCESS_KEY_ID:-}" ||
      -n "${MSC_MATH_R2_SECRET_ACCESS_KEY:-}" ]]; then
  setup_secrets_absent=false
fi

export HEALTH_NETWORK_CHECKS="$network_checks"
export HEALTH_REPOSITORY="$repository"
export HEALTH_CHECKOUT_ROOT="$checkout_root"
export HEALTH_HEAD_SHA="$head_sha"
export HEALTH_BRANCH="$branch"
export HEALTH_WORKTREE_CLEAN="$worktree_clean"
export HEALTH_IDENTITY_OK="$identity_ok"
export HEALTH_GH_INSTALLED="$gh_installed"
export HEALTH_GH_AUTHENTICATED="$gh_authenticated"
export HEALTH_GITHUB_REPOSITORY_READ="$github_repository_read"
export HEALTH_GIT_REMOTE_READ="$git_remote_read"
export HEALTH_RCLONE_INSTALLED="$rclone_installed"
export HEALTH_RCLONE_CONFIG_PRESENT="$rclone_config_present"
export HEALTH_RCLONE_CONFIG_MODE_600="$rclone_config_mode_600"
export HEALTH_RCLONE_REMOTE_CONFIGURED="$rclone_remote_configured"
export HEALTH_R2_READ_ATTEMPTED="$r2_read_attempted"
export HEALTH_R2_SNAPSHOTS_READ="$r2_snapshots_read"
export HEALTH_PYTHON_3_12="$python_3_12"
export HEALTH_UV_INSTALLED="$uv_installed"
export HEALTH_RUST_1_94="$rust_1_94"
export HEALTH_CARGO_INSTALLED="$cargo_installed"
export HEALTH_RUSTFMT_INSTALLED="$rustfmt_installed"
export HEALTH_LATEXMK_INSTALLED="$latexmk_installed"
export HEALTH_BIBER_INSTALLED="$biber_installed"
export HEALTH_SETUP_SECRETS_ABSENT="$setup_secrets_absent"

python3 - <<'PY'
import json
import os


def boolean(name: str) -> bool:
    return os.environ[name] == "true"


print(json.dumps({
    "schema_version": 1,
    "network_checks": boolean("HEALTH_NETWORK_CHECKS"),
    "repository": {
        "expected": os.environ["HEALTH_REPOSITORY"],
        "checkout_root": os.environ["HEALTH_CHECKOUT_ROOT"],
        "identity_ok": boolean("HEALTH_IDENTITY_OK"),
        "head_sha": os.environ["HEALTH_HEAD_SHA"],
        "branch": os.environ["HEALTH_BRANCH"],
        "worktree_clean": boolean("HEALTH_WORKTREE_CLEAN"),
    },
    "github": {
        "client_installed": boolean("HEALTH_GH_INSTALLED"),
        "authenticated": boolean("HEALTH_GH_AUTHENTICATED"),
        "repository_read_ok": boolean("HEALTH_GITHUB_REPOSITORY_READ"),
        "remote_read_ok": boolean("HEALTH_GIT_REMOTE_READ"),
        "mutation_proven": False,
    },
    "r2": {
        "client_installed": boolean("HEALTH_RCLONE_INSTALLED"),
        "config_present": boolean("HEALTH_RCLONE_CONFIG_PRESENT"),
        "config_mode_600": boolean("HEALTH_RCLONE_CONFIG_MODE_600"),
        "remote_configured": boolean("HEALTH_RCLONE_REMOTE_CONFIGURED"),
        "read_attempted": boolean("HEALTH_R2_READ_ATTEMPTED"),
        "snapshots_read_ok": boolean("HEALTH_R2_SNAPSHOTS_READ"),
        "mutation_proven": False,
    },
    "runtime": {
        "python_3_12": boolean("HEALTH_PYTHON_3_12"),
        "uv": boolean("HEALTH_UV_INSTALLED"),
        "rust_1_94": boolean("HEALTH_RUST_1_94"),
        "cargo": boolean("HEALTH_CARGO_INSTALLED"),
        "rustfmt": boolean("HEALTH_RUSTFMT_INSTALLED"),
        "latexmk": boolean("HEALTH_LATEXMK_INSTALLED"),
        "biber": boolean("HEALTH_BIBER_INSTALLED"),
    },
    "secret_lifecycle": {
        "setup_secrets_absent_from_agent": boolean("HEALTH_SETUP_SECRETS_ABSENT"),
    },
    "actor_verification_required": [
        "tool_inventory",
        "connector_and_mcp_exposure",
        "mutation_provenance",
        "fresh_process_persistence",
        "fresh_environment_persistence",
    ],
}, sort_keys=True))
PY

