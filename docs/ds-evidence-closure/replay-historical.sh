#!/bin/sh
# Run from the repository root. Optional argument: output directory.
set -eu
out=${1:-/tmp/ds-historical-association-replay}
cache=${XDG_CACHE_HOME:-$HOME/.cache}/msc-math/artifacts/polytope-invariant-table/c3042846723dbb89db17f2f39d88004d937e1e790949ff6880188fa89fc1900a/files/polytope-table.jsonl
scratch=$(mktemp -d /tmp/ds-historical-replay.XXXXXX)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
mkdir -p "$scratch/methods/_shared" "$scratch/methods/statistical-associations" "$scratch/tables"
cp "$cache" "$scratch/tables/polytope-table.jsonl"
cp experiments/polytope-invariant-table/polytope-provenance-table.jsonl "$scratch/tables/"
printf '%s  %s\n' 607c8731fa03d190d497edc3e8f1b4cca88f7d238260cce527680f568bc33d59 "$scratch/tables/polytope-table.jsonl" 6ff88a5accce9a7ec7e5a494107350b0974b2ce0268ea44caae36a18a7494ef2 "$scratch/tables/polytope-provenance-table.jsonl" | sha256sum -c -
for path in _shared/random_only.py statistical-associations/analyze.py; do
 git show "ea8e2657:experiments/sys-datascience/methods/$path" > "$scratch/methods/$path"
done
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 MKL_NUM_THREADS=1 uv run --script "$scratch/methods/statistical-associations/analyze.py" --tables-dir "$scratch/tables" --out-dir "$out"
