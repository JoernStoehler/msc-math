# Boosting and Nearest-Neighbor Supervised Alternatives

Status: needs current retained-dataset rerun.

This report path is current as a status marker, not as evidence. The previous
report used the old pre-LICCA `282`-row dataset and was removed from current
source truth.

Run:

```bash
uv run --script experiments/sys-landscape/datascience/methods/supervised-alternatives/analyze.py --dataset-dir experiments/sys-landscape/datascience/dataset --permutations 20
```

Current method-table role:

- cheap supervised alternatives for regression/classification checks;
- evaluates whether standard alternatives change the table-column regression or
  endpoint-vs-random classification story;
- current thesis use is pending rerun or explicit abandonment from current data.
