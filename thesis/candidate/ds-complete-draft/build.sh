#!/bin/sh
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
export TEXINPUTS=.:ds-complete-draft:
export BIBINPUTS=.:ds-complete-draft:
exec latexmk -norc -pdf -interaction=nonstopmode -halt-on-error -outdir=ds-complete-draft/build ds-complete-draft/preview.tex
