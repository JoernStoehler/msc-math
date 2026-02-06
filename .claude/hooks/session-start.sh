#!/bin/bash
set -euo pipefail

# Only run in remote (Claude Code on the web) environments
if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

# Install GitHub CLI if not already present
if ! command -v gh &>/dev/null; then
  GH_VERSION="2.86.0"
  curl -fsSL "https://github.com/cli/cli/releases/download/v${GH_VERSION}/gh_${GH_VERSION}_linux_amd64.tar.gz" -o /tmp/gh.tar.gz
  tar -xzf /tmp/gh.tar.gz -C /tmp
  cp "/tmp/gh_${GH_VERSION}_linux_amd64/bin/gh" /usr/local/bin/gh
  rm -rf /tmp/gh.tar.gz "/tmp/gh_${GH_VERSION}_linux_amd64"
fi

# Verify gh is available
gh --version
