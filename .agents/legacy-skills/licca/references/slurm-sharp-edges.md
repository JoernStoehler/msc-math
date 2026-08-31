# LICCA Slurm Sharp Edges

- LICCA can execute a spool copy under `/var/spool/slurmd/`; do not derive
  repository outputs from `BASH_SOURCE[0]` unless execution of the repo copy is
  verified. Prefer `SLURM_SUBMIT_DIR`, or `cd "$HOME/msc-math"` when that fixed
  checkout is intended.
- `--mem=0` can request all node memory and make small smoke jobs pend with
  `QOSGrpMemLimit`. Use bounded memory unless full-node memory is justified.
- Commas delimit `sbatch --export` entries. Export comma-containing values in
  the shell first, then pass only the variable name:

  ```bash
  export DATASCIENCE_PRODUCERS='random,random-product'
  sbatch --export=ALL,DATASCIENCE_PRODUCERS ...
  ```

- A displayed `squeue TIME` such as `0:16` is Slurm formatting, not necessarily
  minutes. Confirm elapsed time with `sacct`.

Bounded monitoring examples:

```bash
sacct -j <jobid> --format=JobID,JobName%24,Partition,State,Elapsed,AllocCPUS,CPUTime,MaxRSS,ExitCode
tail -n 80 logs/*.out
wc -l path/to/shards/*.jsonl
```
