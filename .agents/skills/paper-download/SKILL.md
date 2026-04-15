---
name: paper-download
description: "Download and manage paper or book source files under `papers/`. Use when asked to fetch arXiv sources, add a paper PDF, hydrate local-only PDFs, or update download instructions. Do not use this for ordinary paper reading or citation verification unless source acquisition is part of the task."
---

# Paper Download

This skill is for source acquisition, not for reading papers. For citation checks, start from `papers/citation-index.md` and the files already present under `papers/`.

## Layout

- `papers/<abbreviationYear>/`: extracted arXiv source trees. Name by author initials plus year, for example `bblm2023`.
- `papers/*.pdf`: standalone paper or book PDFs.
- `papers/.gitignore`: local-only PDFs that must not be committed.

## ArXiv Sources

Prefer arXiv source when available because agents can grep LaTeX directly.

```bash
mkdir -p papers/<abbreviationYear>
curl -L "https://arxiv.org/e-print/<arxiv-id>" | tar xz -C papers/<abbreviationYear>/
```

After extraction, inspect the top-level files and keep the original source layout unless there is a concrete reason to normalize it.

## PDFs

Before committing a PDF, check whether it is freely available for redistribution. If that is unclear, put the file name in `papers/.gitignore` and leave the PDF local-only.

Known freely available PDFs:

```bash
cd papers/

curl -L -o BenziGolubLiesen2005.pdf \
  "https://page.math.tu-berlin.de/~liesen/Publicat/BenGolLie05.pdf"

curl -L -o CHLS2007.pdf \
  "https://library.slmath.org/books/Book54/files/01hofer.pdf"
```

Known local-only PDFs:

```bash
cd papers/

curl -L -o GVL4_2013.pdf \
  "https://math.ecnu.edu.cn/~jypan/Teaching/books/2013%20Matrix%20Computations%204th.pdf"

curl -L -o Higham_2002.pdf \
  "http://ftp.demec.ufpr.br/CFD/bibliografia/Higham_2002_Accuracy%20and%20Stability%20of%20Numerical%20Algorithms.pdf"
```

Known blocked downloads from 2026-04-07:
- Wedin 1973: DTIC blocked `https://apps.dtic.mil/sti/tr/pdf/ADA033735.pdf`; try DOI `10.1007/BF01933494`.
- Hofer-Zehnder 1994: Springer paywall at `http://link.springer.com/content/pdf/10.1007/978-3-0348-8540-9.pdf`.

## After Adding Sources

- Update `papers/citation-index.md` only when the task includes citation verification.
- Add bibliography entries to `thesis/bibliography.bib` only when requested or needed by the current thesis edit.
- Mark agent-produced bibliography entries with the repo's thesis verification marker until checked by the user.
