#!/bin/sh
# Run here. Rebuilds only the postmortem's static reading surface.
set -eu
cd "$(dirname "$0")"
python3 build-readable.py
