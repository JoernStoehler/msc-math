#!/usr/bin/env bash
# Submit the LICCA fixed-F ascent smoke pipeline with Slurm dependencies.
#
# Run on a LICCA login node from:
#   cd "$HOME/msc-math/experiments/sys-landscape/datascience/produce"
#   ./submit-licca-ascent-smoke-pipeline.sh
#
# This is production-shaped smoke: it submits the smoke general/product ascent
# array jobs, then submits the same merge Slurm job after both smoke arrays
# finish successfully. It checks dependency-chain mechanics and shard merge
# behavior before production submission.

set -euo pipefail

mkdir -p logs

general_jid="$(sbatch --parsable licca-ascent-general-smoke.slurm.sh)"
product_jid="$(sbatch --parsable licca-ascent-product-smoke.slurm.sh)"
merge_jid="$(sbatch --parsable --dependency=afterok:${general_jid}:${product_jid} \
  licca-merge-ascent-shards.slurm.sh)"

cat <<EOF
Submitted LICCA datascience ascent smoke pipeline.

1. general smoke ascent array: $general_jid
2. product smoke ascent array: $product_jid
3. merge after both smoke arrays succeed: $merge_jid

Monitor:
  squeue -u "$USER"

Inspect after completion:
  sacct -j $general_jid,$product_jid,$merge_jid --format=JobID%20,JobName%24,Partition,State,Elapsed,AllocCPUS,CPUTime,MaxRSS,ExitCode
  tail -n 120 logs/ds-smoke-general-${general_jid}_0.out
  tail -n 120 logs/ds-smoke-product-${product_jid}_0.out
  tail -n 160 logs/ds-merge-ascent-${merge_jid}.out
EOF
