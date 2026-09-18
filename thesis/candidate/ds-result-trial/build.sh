#!/bin/sh
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
export TEXINPUTS=.:ds-result-trial:
export BIBINPUTS=.:ds-complete-draft:
exec latexmk -norc -pdf -interaction=nonstopmode -halt-on-error -outdir=ds-result-trial/build ds-result-trial/preview.tex
