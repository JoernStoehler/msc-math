# AGENTS.md

Paper and book sources for reading and citation verification.

## Key files

- **`citation-index.md`**: Verified theorem and section numbers for cited results. Read this first before searching books or doing web lookups.
- **`.gitignore`**: Unauthorized book PDFs are local-only.

## Directory layout

- `<abbreviationYear>/`: arXiv paper sources. Naming: first letters of author surnames plus year.
- `*.pdf`: standalone paper and book PDFs.

## Downloading arXiv papers

```bash
curl -L "https://arxiv.org/e-print/<arxiv-id>" | tar xz -C papers/<abbreviationYear>/
```

## Downloading book and paper PDFs

Some PDFs are committed because they are freely available. Others are gitignored local-only copies and must not be committed.

### Committed PDFs

```bash
cd papers/

curl -L -o BenziGolubLiesen2005.pdf \
  "https://page.math.tu-berlin.de/~liesen/Publicat/BenGolLie05.pdf"

curl -L -o CHLS2007.pdf \
  "https://library.slmath.org/books/Book54/files/01hofer.pdf"
```

### Gitignored PDFs

```bash
cd papers/

curl -L -o GVL4_2013.pdf \
  "https://math.ecnu.edu.cn/~jypan/Teaching/books/2013%20Matrix%20Computations%204th.pdf"

curl -L -o Higham_2002.pdf \
  "http://ftp.demec.ufpr.br/CFD/bibliografia/Higham_2002_Accuracy%20and%20Stability%20of%20Numerical%20Algorithms.pdf"
```

Failed downloads (2026-04-07):
- **Wedin (1973)**: DTIC blocked the request (`https://apps.dtic.mil/sti/tr/pdf/ADA033735.pdf`). Try DOI `10.1007/BF01933494`.
- **Hofer-Zehnder (1994)**: Springer paywall (`http://link.springer.com/content/pdf/10.1007/978-3-0348-8540-9.pdf`).

## Reading PDFs programmatically

`poppler-utils` is not available in the devcontainer. Use `pymupdf` instead:

```python
import pymupdf

doc = pymupdf.open("papers/Higham_2002.pdf")
text = doc[170].get_text()
```

## Verifying and updating citation-index.md

1. Check `citation-index.md` first.
2. For arXiv sources in `papers/<name>/`, read the LaTeX source directly and resolve labels there.
3. For PDFs, use `pymupdf` to search theorem text.
4. Update `citation-index.md` with result name, exact location, page number, and a short statement snippet.
5. Watch for shared counters across theorem-like environments.

## Bib entries

Add entries to `thesis/bibliography.bib`. Verify author names against the paper itself, not memory. Agent-produced entries should be marked with `% [TODO: JÖRN - verify]` until checked.
