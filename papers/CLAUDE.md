# papers/

Paper and book sources for reading and citation verification.

## Key files

- **`citation-index.md`**: Verified theorem/section numbers for all cited results. **Read this first** before searching books or doing web lookups — most citation questions are already answered here.
- **`.gitignore`**: Unauthorized book PDFs (Higham, GVL) are local-only.

## Directory layout

- `<abbreviationYear>/` — arXiv paper sources (LaTeX). Naming: first letters of author surnames + year. Example: Baracco-Bernardi-Lerario-Mondino 2023 → `bblm2023/`.
- `*.pdf` — standalone paper/book PDFs (not arXiv sources).

## Downloading arXiv papers

```bash
# Download and extract arXiv source
curl -L "https://arxiv.org/e-print/<arxiv-id>" | tar xz -C papers/<abbreviationYear>/
```

## Downloading book/paper PDFs

Some PDFs are committed (freely available); others are gitignored (unauthorized copies for local use only — Jörn verifies at the university library before publication).

### Committed PDFs (freely available)

```bash
cd papers/

# Benzi-Golub-Liesen (2005) — saddle-point survey, author-hosted
curl -L -o BenziGolubLiesen2005.pdf \
  "https://page.math.tu-berlin.de/~liesen/Publicat/BenGolLie05.pdf"

# Cieliebak-Hofer-Latschev-Schlenk (2007) — quantitative symplectic geometry, MSRI
curl -L -o CHLS2007.pdf \
  "https://library.slmath.org/books/Book54/files/01hofer.pdf"
```

### Gitignored PDFs (unauthorized — do NOT commit)

These are excluded in `papers/.gitignore`. They must be re-downloaded after cloning or after worktree cleanup.

```bash
cd papers/

# Golub & Van Loan, "Matrix Computations", 4th ed. (2013)
curl -L -o GVL4_2013.pdf \
  "https://math.ecnu.edu.cn/~jypan/Teaching/books/2013%20Matrix%20Computations%204th.pdf"

# Higham, "Accuracy and Stability of Numerical Algorithms", 2nd ed. (2002)
curl -L -o Higham_2002.pdf \
  "http://ftp.demec.ufpr.br/CFD/bibliografia/Higham_2002_Accuracy%20and%20Stability%20of%20Numerical%20Algorithms.pdf"
```

Failed downloads (2026-04-07):
- **Wedin (1973)**: DTIC blocked the request (`https://apps.dtic.mil/sti/tr/pdf/ADA033735.pdf`). Try DOI: `10.1007/BF01933494` (BIT, Springer paywall).
- **Hofer-Zehnder (1994)**: Springer paywall (`http://link.springer.com/content/pdf/10.1007/978-3-0348-8540-9.pdf`). Returns HTML login page.

## Reading PDFs programmatically

`poppler-utils` is not available in the devcontainer. Use `pymupdf` instead:

```python
import pymupdf  # pip install --break-system-packages pymupdf
doc = pymupdf.open("papers/Higham_2002.pdf")
text = doc[170].get_text()  # page 171 (0-indexed)
```

The Read tool can read PDFs with `pages` parameter if `poppler-utils` is installed, but in this devcontainer it isn't.

## Verifying / updating citation-index.md

When adding new citations or verifying existing ones:

1. **Check `citation-index.md` first** — if the result is already there with a page number, you're done.
2. **For arXiv papers** (in `papers/<name>/`): read the `.tex` source directly with grep. Labels like `\label{formula_theorem}` map to theorem numbers via the shared counter in LaTeX.
3. **For book PDFs**: use pymupdf to search for theorem text:
   ```python
   import pymupdf
   doc = pymupdf.open("papers/GVL4_2013.pdf")
   for i in range(len(doc)):
       text = doc[i].get_text()
       if 'Corollary 8.6.2' in text:
           print(f"Page {i+1}: {text[text.find('Corollary 8.6.2'):text.find('Corollary 8.6.2')+300]}")
   ```
4. **Update `citation-index.md`** with: result name, location (§/Thm/Cor number), page number, statement snippet.
5. **Watch for shared counters**: HK2017 numbers Theorems, Corollaries, and Remarks in one sequence (so "1.4" is a Remark, not a Theorem). Always verify the environment type.

## Bib entries

Add to `thesis/bibliography.bib`. Verify author names against the actual paper — never from memory. Agent-produced entries should be marked with a `% [TODO: JÖRN - verify]` comment until checked.
