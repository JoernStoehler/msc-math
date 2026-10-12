# Physical file census

Observed 2026-10-10T16:37:19+00:00 UTC against tracked-path baseline `844fc5be3884c12a93dabd795d0a0d12a92b2815`.

This is a physical manifest for comparison with the semantic inventory. It records tracked paths, current worktree byte sizes and filesystem types; it does not inspect file contents or assess meaning, correctness, quality, duplication or whether data is safe to remove. Paths are enumerated with `git ls-files -z`, so untracked files (including this newly generated manifest and census) are excluded. Sizes and existence are observed from the working tree at the stated time, which may contain edits relative to the baseline commit. Directories are derived from path prefixes; Git does not track empty directories.

The census contains **3,541 tracked paths** in **755 derived directories**, totaling **303,471,625 bytes**. All 3,541 paths exist; 0 are missing. There are 2 symlinks and 41 tracked `Cargo.toml` manifests (including the root manifest and nested experiment packages). The complete manifest, including manifest paths, is [files.json](files.json).

## Counts by top-level path

| Top-level path | Paths |
|---|---:|
| `(root)` | 12 |
| `.agents` | 21 |
| `.claude` | 2 |
| `.codex` | 3 |
| `.vscode` | 2 |
| `LICENSES` | 2 |
| `artifacts` | 1 |
| `crates` | 123 |
| `docs` | 975 |
| `experiments` | 2,212 |
| `formal` | 49 |
| `output` | 2 |
| `papers` | 36 |
| `sandbox` | 3 |
| `scripts` | 26 |
| `submit` | 11 |
| `thesis` | 61 |

## Counts by extension

| Extension | Paths |
|---|---:|
| `.md` | 872 |
| `.json` | 584 |
| `.jsonl` | 503 |
| `.rs` | 437 |
| `.py` | 280 |
| `.png` | 149 |
| `.tex` | 107 |
| `.txt` | 106 |
| `.pdf` | 86 |
| `.tsv` | 69 |
| `.log` | 57 |
| `.csv` | 56 |
| `.toml` | 44 |
| `.sh` | 38 |
| `dotfile` | 26 |
| `.lock` | 16 |
| `.gz` | 13 |
| `.html` | 13 |
| `.svg` | 13 |
| `.time` | 12 |
| `(none)` | 8 |
| `.bib` | 8 |
| `.patch` | 8 |
| `.diff` | 5 |
| `.yaml` | 5 |
| `.js` | 4 |
| `.ndjson` | 4 |
| `.zip` | 3 |
| `.bbl` | 2 |
| `.dot` | 2 |
| `.mjs` | 2 |
| `.npz` | 2 |
| `.stderr` | 2 |
| `.stdout` | 2 |
| `.css` | 1 |
| `.fdb_latexmk` | 1 |
| `.fls` | 1 |

## Physical concentration

`experiments/` contains 2,212 paths and about 220 MB; `docs/` contains 975 paths and about 76 MB. JSONL is the largest extension group by bytes (about 179 MB), followed by PDFs (about 31 MB) and gzip files (about 23 MB). These counts flag where bulk is concentrated; they do not imply redundancy or cleanup value. The largest individual file is `docs/ds-retrospective-revalidation/target-full/results.jsonl.gz` at 10,697,846 bytes.
