# Datascience Batch Datasets

This directory holds shared table snapshots for method waves.

Each batch directory should contain:

- `dataset/polytope-table.jsonl`
- `dataset/observation-table.jsonl`
- `FINGERPRINT.md`

Batch datasets are method inputs. They should be read-only for executor agents.
Method-specific outputs belong under `../methods/<slug>/`.

Use `/tmp` only for scratch. Do not cite a private `/tmp` dataset as method
source truth unless it has first been reproduced into a batch directory.
