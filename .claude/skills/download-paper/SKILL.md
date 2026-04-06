---
name: download-paper
description: Download an arXiv paper into papers/ and add a bib entry. Load when Jörn asks to fetch a paper or when a citation needs its source.
---

# Download Paper

**Naming:** `papers/<abbreviationYear>/` — first letters of author surnames + year. Example: Baracco-Bernardi-Lerario-Mondino 2023 → `papers/bblm2023/`

**Source:** `curl -L "https://arxiv.org/e-print/<arxiv-id>"` — untar if tarball, direct download if single .tex.

**Bib entry:** Add to `thesis/bibliography.bib`. Get from arXiv abstract page or paper's own .bib. **Verify author names and title against the actual paper — never from memory.**

**Report:** paper name, directory path, cite key.
