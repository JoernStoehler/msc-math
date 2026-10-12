#!/bin/sh
# Bounded one-shot Codex run for Claude VMs. Prints Codex's final message on stdout;
# full event log goes to $LOG (default: ./codex-run.<pid>.jsonl, stderr in $LOG.err). Exit: codex's, 124 = timeout.
# Usage: codex-run.sh [-t SECONDS] [-C DIR] [-w] [-m MODEL] [-e EFFORT] PROMPT|@promptfile
#   -t  timeout seconds >0 (default 300)   -C  working dir (default .)   -w  live web-search tool
#   -m  model                              -e  model_reasoning_effort (low|medium|high)
# SANDBOX=read-only|workspace-write|danger-full-access (default workspace-write; verified working in this VM 2026-09-29).
# Never reads or prints credentials. Requires codex on PATH and logged in (bootstrap.sh / SKILL.md).
T=300; D=.; W=; M=; E=
while getopts t:C:wm:e: o; do case $o in t) T=$OPTARG;; C) D=$OPTARG;; w) W=1;; m) M=$OPTARG;; e) E=$OPTARG;; *) exit 2;; esac; done
shift $((OPTIND-1))
[ $# -eq 1 ] || { sed -n 2,8p "$0"; exit 2; }
case $T in ''|*[!0-9]*) echo "codex-run: -t must be a positive integer" >&2; exit 2;; esac
case $T in *[1-9]*) ;; *) echo "codex-run: -t must be a positive integer" >&2; exit 2;; esac
command -v codex >/dev/null || { echo "codex missing: run scripts/bootstrap.sh" >&2; exit 3; }
PF=; OUT=
cleanup() { [ -z "$PF" ] || rm -f -- "$PF"; [ -z "$OUT" ] || rm -f -- "$OUT"; }
trap cleanup EXIT
trap 'exit 129' HUP
trap 'exit 130' INT
trap 'exit 143' TERM
PF=$(mktemp) || exit 1
OUT=$(mktemp) || exit 1
case $1 in @*) cat -- "${1#@}" >"$PF" || { echo "codex-run: cannot read prompt file" >&2; exit 2; };; *) printf '%s' "$1" >"$PF";; esac
LOG=${LOG:-./codex-run.$$.jsonl}
set -- --sandbox "${SANDBOX:-workspace-write}" --skip-git-repo-check -C "$D" --json -o "$OUT"
[ -n "$W" ] && set -- "$@" -c 'web_search="live"'
[ -n "$M" ] && set -- "$@" -m "$M"
[ -n "$E" ] && set -- "$@" -c "model_reasoning_effort=\"$E\""
# prompt via stdin ("-"): no argv size limit, no option injection
timeout --kill-after=10 "$T" codex exec "$@" - <"$PF" >"$LOG" 2>"$LOG.err"; rc=$?
cat "$OUT"
[ $rc -eq 0 ] || echo "codex-run: exit $rc (124=timeout); see $LOG $LOG.err" >&2
exit $rc
