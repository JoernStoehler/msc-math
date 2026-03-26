---
name: download-paper
description: Download an arXiv paper into papers/ and add a bib entry. Load when Jörn asks to fetch a paper or when a citation needs its source.
disable-model-invocation: true
---

# Download Paper Workflow

## 1. Identify the paper

Get the arXiv ID (e.g. `2303.13348`) from Jörn or from a citation.

## 2. Create directory

Convention: `papers/<abbreviationYear>/` where abbreviation is first letters of author surnames + publication year.

Example: Baracco-Bernardi-Lerario-Mondino 2023 → `papers/bblm2023/`

## 3. Download source

```bash
mkdir -p papers/<name>/
cd papers/<name>/
curl -L "https://arxiv.org/e-print/<arxiv-id>" -o source.tar.gz
tar xzf source.tar.gz
rm source.tar.gz
```

If the source is a single .tex file rather than a tarball, `curl` it directly.

## 4. Add bibliography entry

Add to `thesis/bibliography.bib`. Get the entry from the arXiv abstract page or from the paper's .bib file if included in the source. Verify author names and title against the actual paper — never from memory.

## 5. Verify

- `ls papers/<name>/` shows .tex source files
- `grep '<CITEKEY>' thesis/bibliography.bib` finds the entry
- Report to Jörn: paper name, directory, cite key