#!/usr/bin/env bash
# Benchmark three ways to acquire the SAME pinned Codex release. Needs network.
# Times include checkout/extraction; Git network bytes require separate instrumentation.
set -euo pipefail
TAG="${1:-rust-v0.162.1}"
case "$TAG" in *[!a-zA-Z0-9._-]*|'') echo 'Unsafe ref' >&2; exit 2;; esac
WORK="${2:-$(mktemp -d)}"
mkdir -p "$WORK"
for cmd in git curl tar python3 du; do command -v "$cmd" >/dev/null || { echo "Missing $cmd" >&2; exit 2; }; done
for name in codex.tar.gz archive-tree shallow sparse; do
 if [[ -e "$WORK/$name" ]]; then echo "Path exists (refusing to overwrite): $WORK/$name" >&2; exit 2; fi
done
LOG="$WORK/results.tsv"
printf 'method\tseconds\ton_disk_bytes\tgit_bytes\tarchive_compressed_bytes\tstatus\n' > "$LOG"
size_bytes(){ [[ -e "$1" ]] && du -sk "$1" | awk '{printf "%.0f", $1*1024}' || echo 0; }
get_archive(){
  curl -fL --retry 2 --progress-bar "https://github.com/openai/codex/archive/refs/tags/$TAG.tar.gz" -o "$WORK/codex.tar.gz"
  mkdir "$WORK/archive-tree"
  tar -xzf "$WORK/codex.tar.gz" -C "$WORK/archive-tree" --strip-components=1
}
get_shallow(){
  git clone --progress --depth 1 --single-branch --no-tags --branch "$TAG" https://github.com/openai/codex.git "$WORK/shallow"
}
get_sparse(){
  git clone --progress --depth 1 --single-branch --no-tags --filter=blob:none --sparse --branch "$TAG" https://github.com/openai/codex.git "$WORK/sparse"
  (cd "$WORK/sparse" && git sparse-checkout set docs codex-rs/core)
}
run_case(){
  local name="$1" fn="$2" start elapsed status path git_bytes archive_bytes disk
  start=$(python3 -c 'import time; print(time.monotonic())')
  if "$fn"; then status=ok; else status=failed; fi
  elapsed=$(python3 - "$start" <<'PY'
import sys,time
print(round(time.monotonic()-float(sys.argv[1]),3))
PY
)
  path="$WORK/$name"
  [[ "$name" == archive ]] && path="$WORK/archive-tree"
  archive_bytes=0
  [[ "$name" == archive ]] && archive_bytes=$(size_bytes "$WORK/codex.tar.gz")
  git_bytes=$(size_bytes "$path/.git")
  disk=$(size_bytes "$path")
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$name" "$elapsed" "$disk" "$git_bytes" "$archive_bytes" "$status" | tee -a "$LOG"
}
run_case archive get_archive
run_case shallow get_shallow
run_case sparse get_sparse
printf '\nMeasurements: %s\n' "$LOG"
printf 'Caution: du is filesystem allocation, not exact transfer bytes; for network transfer use independent monitoring.\n'
