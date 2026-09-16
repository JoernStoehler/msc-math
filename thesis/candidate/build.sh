#!/bin/sh
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
# Use only this recovered tree plus the installed TeX distribution.
# Ignore parent/user latexmk configuration and inherited project search paths.
export TEXINPUTS=".:legacy:"
export BIBINPUTS=".:legacy:"
exec latexmk -norc -pdf -interaction=nonstopmode -halt-on-error \
  -auxdir=build -outdir=build main.tex
