#!/usr/bin/env bash
# Download Claude Code system prompts for Jörn to review.
# Usage: bash scripts/download-system-prompt.sh [output-dir]
# Default output: /tmp/cc-system-prompts/

set -euo pipefail

REPO="https://raw.githubusercontent.com/Piebald-AI/claude-code-system-prompts/main"
OUT="${1:-/tmp/cc-system-prompts}"
mkdir -p "$OUT"

echo "Downloading system prompt index..."
curl -sL "$REPO/README.md" -o "$OUT/README.md"

echo "Downloading system prompt files (includes tool descriptions)..."
# All prompts + tool descriptions live in system-prompts/
curl -sL "https://api.github.com/repos/Piebald-AI/claude-code-system-prompts/contents/system-prompts" \
  | python3 -c "import json,sys; [print(f['name']) for f in json.load(sys.stdin) if f['name'].endswith('.md')]" \
  | while read -r f; do
      curl -sL "$REPO/system-prompts/$f" -o "$OUT/$f"
    done

COUNT=$(ls "$OUT"/*.md 2>/dev/null | grep -v README | wc -l)
echo ""
echo "Done. $COUNT files in: $OUT"
echo ""
echo "Key files for agent-centric work:"
echo "  system-prompt-system-section.md"
echo "  system-prompt-writing-subagent-prompts.md"
echo "  system-prompt-subagent-delegation-examples.md"
echo "  system-prompt-executing-actions-with-care.md"
echo "  system-prompt-fork-usage-guidelines.md"
echo "  tool-description-agent-*.md"
echo ""
echo "To check for changes: git diff $OUT/"
