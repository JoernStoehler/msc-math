# Figure source

The retained vector PDFs in the parent directory are the paper's selected figures. These Python scripts are their source, copied without changing the drawing code from the research scratch packets:

- `paper_variants.py` produces `affine-outer-body.pdf` and `tent-caps-compact.pdf`.
- `make_figures.py` produces `pfaffian-graph.pdf` and the unprinted `tent-caps.pdf`.
- `draw_covers.py` produces `truncated-covers.pdf`.

Each script writes beside itself. Run it in a scratch copy of this directory and compare the generated figures visually with the retained PDFs. The first two scripts have inline dependencies; the cover script needs NumPy and Matplotlib. The source companion archive includes these scripts and the retained vector PDFs, so the paper's figures can be inspected without the original research scratch directory.
