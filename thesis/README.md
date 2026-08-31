# Thesis

`main.tex` defines the active publication surface. Files under `legacy/` are
source material only and are not part of the thesis unless deliberately copied
into active text.

## Active structure

The numbered TeX files loaded by `main.tex` are the active thesis. Former
writing companions and cross-cutting plans are quarantined under
`legacy/non-current-planning-companions/`; use them only as fallible recovery
material, not as current thesis state or instructions.

In publication prose, use “exact” only when it distinguishes arithmetic or
representation from numerical approximation, or when it is part of a standard
term such as “exact form.” Mathematics is otherwise understood literally:
do not use “exact” as a synonym for proved, rigorous, complete, or fully
specified.

## Supporting files

- `DEVELOPMENT.md`: minimal maintainer orientation.
- `preamble.tex`: thesis-local LaTeX setup and definitions.
- `bibliography.bib`: active bibliography.
- `figures/`: thesis-owned final assets and their producers.
- `working/`: thesis-owned candidate assets. Some are active TeX inputs; check
  the active TeX references rather than inferring use from the directory name.

## Build

```bash
cd thesis
latexmk
./check-build.sh
```

The build checks compilation and selected structural conditions. It does not
establish proof correctness, source adequacy, or Jörn/Kai acceptance.

## Legacy

`legacy/README.md` explains the retained historical source. Search `legacy/`
only when an active companion or concrete missing argument points there.
