#!/usr/bin/env bash
# Submit the LICCA fixed-F ascent pipeline with Slurm dependencies.
#
# Run on a LICCA login node from:
#   cd "$HOME/msc-math/experiments/sys-datascience/produce"
#   ./submit-licca-ascent-pipeline.sh
#
# This submits production general/product ascent arrays and then submits the
# merge job after both arrays finish successfully. The merge job writes review
# targets. Table build is intentionally not chained here because table builds
# consume canonical producer filenames, and promotion of merged review targets is
# still a human/agent review decision.

set -euo pipefail

mkdir -p logs

general_jid="$(sbatch --parsable licca-ascent-general-production.slurm.sh)"
product_jid="$(sbatch --parsable licca-ascent-product-production.slurm.sh)"
merge_jid="$(sbatch --parsable \
  --export=ALL,MERGE_ARGS="--require-cache --fresh-fixed-f" \
  --dependency=afterok:${general_jid}:${product_jid} \
  licca-merge-ascent-shards.slurm.sh)"

cat <<EOF
Submitted LICCA datascience ascent pipeline.

1. general ascent array: $general_jid
2. product ascent array: $product_jid
3. merge after both arrays succeed: $merge_jid

Monitor:
  squeue -u "$USER"

After merge review accepts the merged files, promote review targets to canonical
producer filenames, then submit:
  sbatch ../licca-build-dataset.slurm.sh
EOF
