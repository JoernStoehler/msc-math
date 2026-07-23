# Source papers

This directory caches external source material and paper-specific checking
notes. Cached paper sources are immutable inputs: add a project note or formal
derivation elsewhere rather than editing a paper to match the project.

The files here establish what a cited source says. They do not establish that a
project theorem is proved, an implementation is correct, or wording is active
in the thesis. Follow paper-derived claims into `formal/`, `crates/`,
`experiments/`, and `thesis/` as appropriate.

## Physical inventory

This table covers every current immediate source directory and project note.

| Path | Contents |
| --- | --- |
| `hk2017/` | LaTeX source and figures for Haim-Kislev, *On the symplectic size of convex polytopes* |
| `ch2021/` | split and complete LaTeX source plus figures for Chaidez--Hutchings, *Computing Reeb dynamics on 4d convex polytopes* |
| `hko2024/` | LaTeX source and figures for Haim-Kislev--Ostrover, *A Counterexample to Viterbo's Conjecture* |
| `citation-index.md` | checked theorem/section numbering and source-availability notes |
| `matlab-extraction.md` | project analysis of the HK2017 reference MATLAB implementation |
| `sign_convention_verification.md` | project check of the coordinate and symplectic sign conventions used around the HK2017 route |

The last two Markdown files are project analyses, not part of the cached
papers. Check their reasoning and the named original source before relying on a
claim.

## Local-only references

`papers/.gitignore` names PDFs or directories that may exist in a local
checkout but are not redistributable repository sources. Their absence from
Git is intentional. `citation-index.md` records the known source availability
and published-numbering checks.

When citing a numbered result, verify its numbering in the published version;
cached LaTeX and the published version can differ.
