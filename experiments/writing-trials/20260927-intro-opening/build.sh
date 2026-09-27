#!/bin/sh
# usage: sh build.sh <variant-dir>   -> <variant-dir>/opening.pdf
set -eu
cd "$(dirname "$0")"
v="$1"
export TEXINPUTS=".:../../../thesis:"
export BIBINPUTS=".:"
latexmk -norc -pdf -interaction=nonstopmode -halt-on-error -auxdir="$v/build" -outdir="$v/build" -jobname=opening \
  -pdflatex="pdflatex %O '\\def\\variant{$v}\\input{%S}'" wrapper.tex >/dev/null
cp "$v/build/opening.pdf" "$v/opening.pdf"
