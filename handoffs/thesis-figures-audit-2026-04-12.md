# Handoff: Thesis figure consistency audit — 2026-04-12

## Scope

Addresses the `[open]` TASKS.md item "Thesis figure consistency check" (Thesis section). DoD requires a file with three sections:

- (a) Broken `\includegraphics` references (source file:line → missing asset path)
- (b) Stale `thesis/assets/` entries (asset path → crates/ source path → staleness reason)
- (c) Advisory caveat about thesis restructuring

Read-only audit. No file under `thesis/assets/` or `thesis/*.tex` is modified.

## Method

1. Enumerated thesis TeX files: `find thesis -name '*.tex'` → 16 files.
2. Searched all of them for `\includegraphics`: `grep -rn '\\includegraphics' thesis/` → **0 matches**.
3. Checked for `thesis/assets/`: `ls thesis/assets/` → "No such file or directory". The directory does not exist on `main` as of `hygiene-audit` branch point (commit `9d29951a`).
4. No sync script or `crates/ → thesis/assets/` copy registry was found (cross-checked against `CLAUDE.md`, `thesis/README*`, `scripts/`).

## (a) Broken `\includegraphics` references

**None.** Zero `\includegraphics` calls exist in `thesis/**/*.tex`. The grep command

```
grep -rn '\\includegraphics' thesis/
```

produced no output. There is nothing to validate.

## (b) Stale assets

**None.** `thesis/assets/` does not exist in the current worktree. There are no asset files to compare against `crates/**` sources, and no mtime or content-hash check is possible.

## (c) Advisory caveat

This audit is **degenerate but not vacuous**. It establishes a verifiable baseline that, as of 2026-04-12, the thesis contains zero figures and zero assets. Three facts constrain how to read that baseline:

1. **Thesis content is pre-assembly.** Per `TASKS.md`, "Experiment writeup drafts", "Experiments chapter", "Introduction", "Conclusion", and "Tube rotation formula implementation" are all `[blocked]` in the Thesis section. The `.tex` files present today (`tube-algorithm.tex`, `appendix-numerical.tex`, `simple-minimizer-existence.tex`, `general-case-algorithm*.tex`, `pruned-general-case-algorithm.tex`, `lagrangian-product-algorithm*.tex`, `clarkedual-action-principle.tex`, `algorithms.tex`, `proofs.tex`, `basic-definitions.tex`, `appendix-notation.tex`, `experiments.tex`, `preamble.tex`, `main.tex`) are algorithm and definition scaffolding. Figure-bearing content has not been written yet.

2. **Restructuring is Jörn-gated.** `TASKS.md` "Thesis restructuring" is `[Jörn]`. Current `.tex` content may be substantially rewritten or relocated before any figures are added. A figure-consistency pass run against today's files risks being invalidated the moment restructuring lands. The TASKS.md item itself notes: "Conditional: only makes sense if current thesis .tex content is kept rather than rewritten."

3. **No sync infrastructure exists.** `CLAUDE.md` states the convention: "`thesis/` copies figures and tables from `crates/` into `thesis/assets/` instead of linking. Never modify `thesis/` content from experiment code." There is no automated copier, no `thesis/assets/MANIFEST`, and no per-asset source annotation. When figures begin to land, staleness detection will require one of:

   - A convention that each `thesis/assets/<fig>.png` has a sibling `thesis/assets/<fig>.source` file recording `<crates/path> <sha256>` so a future audit can cross-walk directly.
   - A script under `scripts/` (e.g. `scripts/thesis-assets-check.sh`) that takes a copy-list and diffs each pair.
   - A `thesis/assets/README.md` manifest.

   This audit does not prescribe which option to adopt — that is a `[Jörn]` decision. It does recommend that the decision be made *before* the first figure is added, not after, so the first copy already records its provenance.

## Recommendation to Jörn

- Mark the `[open]` audit task as `[done] [2026-04-12]` with a one-line note "no figures yet; rerun after experiment writeups and thesis restructuring".
- Decide the asset-provenance convention (manifest file vs `.source` sidecar vs script) before the first figure lands. Add the chosen convention to `CLAUDE.md` so the next figure-consistency audit has an authoritative target.
- Revisit this audit after the `[blocked]` experiment-writeup and experiments-chapter items unblock.

## Ground truth commands (for reproducibility)

```
find thesis -name '*.tex' | wc -l            # 16
grep -rn '\\includegraphics' thesis/         # (no output)
ls thesis/assets/ 2>&1                        # No such file or directory
```

Run at worktree `hygiene-audit` on commit base `9d29951a` (branch point from `main`).
