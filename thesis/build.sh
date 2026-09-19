#!/bin/sh
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
export TEXINPUTS=".:"
export BIBINPUTS=".:"
exec latexmk -norc -pdf -interaction=nonstopmode -halt-on-error -auxdir=build -outdir=build "$@" main.tex
