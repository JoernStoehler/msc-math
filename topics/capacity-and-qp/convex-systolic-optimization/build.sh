#!/bin/sh
set -eu
cd "$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
latexmk -norc -pdf -interaction=nonstopmode -halt-on-error -outdir=build main.tex
cp build/main.pdf paper.pdf
