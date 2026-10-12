# Final retention and reproduction inventory

Prepared after complete scientific acceptance, 2026-10-05 01:05 UTC.
No producer rerun, artifact move or upload was performed for this inventory.
The root owns later large-generated-snapshot policy R2; this group makes no
storage-policy change.

## Essential evidence versus optional workload caches

The essential new exact certificates are `formulas.json`, `clipped.json`,
`feasibility-events.json`, `open-proof.json`, `closed-proof.json`, `equality.json`
and `physical-control.json`. Together they retain signed derivation, complete
parameter incidences, all open/closed sign predicates, equality zeros and
physical capacity/area controls. The accepted joint denominator input and its
preflight catalogue remain checksummed inherited inputs. Source files and
read-only reviewers belong in Git; mathematical proof notes retain the
inherited capacity, canonicalization/vertex and primary zero-edge implications.

`closure-census.json` and `sturm-tests.json` are validation/coverage summaries.
`factor-workload.json` and `factor-relevant.json` are exploratory workload
caches, **not inputs required by the final independent reviewers**. Their raw
factor counts were not used as evidence of positivity. No retained file was
removed or relocated.

## Observed files

Byte sizes and full SHA-256 hashes are retained in [retention.json](retention.json).
The table gives exact byte sizes and shortened display hashes; the JSON owns
the full hashes. Inherited paths resolve relative to this directory.

| File | Role | Bytes | Snapshot SHA-256 prefix |
| --- | --- | ---: | --- |
| `formulas.json` | essential certificate | 4,222,714 | `b97085e52cb38aae` |
| `clipped.json` | essential certificate | 7,114,245 | `f4bc75f2c5c34f07` |
| `feasibility-events.json` | essential certificate | 211,028 | `c0f2d997d09f4a38` |
| `open-proof.json` | essential certificate | 5,628,791 | `25fe7d84621a640e` |
| `closed-proof.json` | essential certificate | 9,383,770 | `992ed90533b2c5e0` |
| `equality.json` | essential certificate | 1,601,214 | `b84baa18f2597a79` |
| `physical-control.json` | essential certificate | 32,309 | `83300e6f5cd57a9a` |
| `closure-census.json` | validation/coverage summary | 9,955 | `c46b2446ca456178` |
| `sturm-tests.json` | validation/coverage summary | 723 | `843ef7622620f557` |
| `factor-workload.json` | workload cache, not needed by final reviewers | 1,102,592 | `5ec5de4b4a0fe451` |
| `factor-relevant.json` | workload cache, not needed by final reviewers | 264,241 | `835f95a8f5a91ea8` |
| `../joint-denominators/joint-denominators.json` | checksummed inherited input | 17,667,310 | `602a61a96420a9f7` |
| `../preflight/active-events.json` | checksummed inherited input | 11,824,885 | `a5f9c24f565611da` |
| `../joint-denominators/extract.py` | source dependency | 25,190 | `628464638b765136` |
| `../joint-denominators/review_joint.py` | source dependency | 21,009 | `f1352dd9ed4c6870` |
| `../preflight/active_events.py` | source dependency | 10,351 | `77ba0f3db5d56e85` |
| `../../hko-angle-nonconformal/review_candidates.py` | source dependency | 10,193 | `540df82132e46025` |

## Snapshot hashes and mathematical sections

Full snapshot hashes identify exactly the independently reviewed observed
files. Producer timing fields make whole JSON byte hashes nondeterministic on
regeneration; absolute observational paths and upstream snapshot hashes can
also change. A failed full byte-hash comparison alone does not show a changed
mathematical certificate.

The inventory separately hashes a canonical mathematical-section projection:
recursively remove the exact keys `input`, `source`, `source_catalogue`,
`field_engine`, `elapsed_seconds`, `seconds`, `screen_seconds`, `total_seconds`,
and keys ending `_sha256`; preserve every other field, order arrays, sort
object keys, and encode compact UTF-8 JSON with `ensure_ascii=False`. These
math-section hashes distinguish content from the timing/provenance wrapper;
they are retained alongside full snapshot hashes, **not replacements for
source/input integrity checks**. Explicit polynomial coefficients, signed
multipliers, roots/isolating intervals, domain incidences and physical values
remain in the projection. Different valid isolating/refinement policies or
code versions may change mathematical encoding without changing the theorem;
independent replay checks mathematical equivalence.

`python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/class-proof/retention.py`
recomputes the inventory to stdout only and writes no files.

## Exact regeneration and write effects

Run from the repository root with Python 3. Producers use only the standard
library except the optional factor helper, whose inline dependency is SymPy.
The essential producer sequence is:

```sh
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/class-proof/derive.py
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/class-proof/clip.py
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/class-proof/clip.py --events-only
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/class-proof/prove_open.py
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/class-proof/prove_closed.py
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/class-proof/equality.py
python -B docs/empirical-viterbo-design/night-20261005/orientation-wide/class-proof/physical_control.py
```

Each command reads its checked-in peer sources and retained upstream inputs
and replaces only its owned sibling output: respectively `formulas.json`,
`clipped.json`, `feasibility-events.json`, `open-proof.json`, `closed-proof.json`,
`equality.json` and `physical-control.json`. These regeneration commands are
documented, not rerun for this retention request. Snapshot dependency hashes
change naturally after timing-bearing upstream regeneration. Peer input
resolution uses `__file__`-relative locations; absolute input metadata records
the observed workstation location and is not a required destination elsewhere.

`python -B .../sturm.py` writes only sibling `sturm-tests.json`.
Optional `uv run .../factor_workload.py` and
`uv run .../factor_workload.py --relevant` write only the respective factor
workload caches. The exact complete commands and summary-generation details
are owned by [extraction.md](extraction.md). All `review_*.py` scripts are
read-only; [review.md](review.md) gives their exact replay sequence and the
accepted input boundaries. No manuscript, dashboard, registry or pre-existing
retained evidence is overwritten by these class-proof producers.
