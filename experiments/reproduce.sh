#!/usr/bin/env bash
#
# reproduce.sh — Full pipeline from zero data to compiled thesis.
#
# PURPOSE: This script documents every command needed to reproduce all
# datasets, figures, tables, and the final thesis PDF from source code alone.
# It is the single source of truth for the experiment pipeline.
#
# This script is NOT expected to be run end-to-end in practice (some steps
# take hours, and the visualization screenshots are manual). Its purpose is
# to be *runnable* and to *document* the full pipeline, so that:
#   - A reader can trace how any figure or table was produced
#   - An agent can update it when adding/removing experiments
#   - Reproducibility is verifiable by running individual steps
#
# Pipeline: Rust binaries → .jsonl data → Python scripts → figures/tables → LaTeX
#
# Prerequisites:
#   - Rust toolchain (cargo)
#   - Python 3.11+ with numpy, matplotlib, scikit-learn
#   - TeX Live (latexmk, pdflatex, biber)
#
# All paths are relative to the repo root.

set -euo pipefail
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

# ── Step 0: Build ────────────────────────────────────────────────────────────
# Build the library crate and all experiment binaries.

(cd crates && cargo build --release)
(cd experiments && cargo build --release)

# ── Step 1: Generate datasets (.jsonl) ───────────────────────────────────────
# Each binary writes data to its own experiment folder.
# Order does not matter — experiments are independent.

# Ablation study dataset (54 polytopes × 4 variants = 216 entries)
experiments/target/release/ablation
# → experiments/ablation/ablation.jsonl

# Correctness verification dataset (47 polytopes, 71 capacity values)
experiments/target/release/correctness
# → experiments/correctness/correctness.jsonl

# Benchmark timing dataset (~85 polytopes)
experiments/target/release/benchmark
# → experiments/benchmark/benchmark.jsonl

# Rejection sampling acceptance rates (1000 attempts × 18 configs)
experiments/target/release/acceptance_sweep
# → experiments/rejection-sampling/acceptance.jsonl

# Random polytope systolic ratio sweep (F=5..12)
experiments/target/release/random_sweep
# → experiments/random-sweep/random-sweep.jsonl

# Random Lagrangian product sweep (bucketed by polygon pair)
experiments/target/release/random_product_sweep
# → experiments/random-product-sweep/random-product-sweep.jsonl

# Pentagon perturbation study (100 perturbed samples)
experiments/target/release/pentagon_perturb
# → experiments/pentagon-perturb/pentagon-perturb.jsonl

# Lagrangian product systematic sweep (5×5 grid + polygon pairs up to 6-gons)
# NOTE: This is the slowest step — may take tens of minutes.
experiments/target/release/lagrangian_sweep
# → experiments/lagrangian-products/lagrangian-products-5x5.jsonl
# → experiments/lagrangian-products/lagrangian-products-*-6deg.jsonl (9 files)

# ── Step 2: Generate figures and tables ──────────────────────────────────────
# Python scripts read .jsonl from their experiment folder, write figures/tables
# to the same folder.

# Ablation study: agreement check + timing comparison figure
python3 experiments/ablation/ablation.py
# → experiments/ablation/ablation_timing.png

# Benchmark: timing model fit + unified figure
python3 experiments/benchmark/benchmark.py
# → experiments/benchmark/benchmark_timing.png
# → experiments/benchmark/profiling/timing_model.json

# Random sweep: systolic ratio vs facet count
python3 experiments/random-sweep/random_sweep.py
# → experiments/random-sweep/random_sweep_sys_vs_f.png

# Random product sweep: systolic ratio vs polygon pair
python3 experiments/random-product-sweep/random_product_sweep.py
# → experiments/random-product-sweep/random_product_sweep_sys_vs_pair.png

# Pentagon perturbations: histogram + stats table + PCA table
python3 experiments/pentagon-perturb/pentagon_perturb.py
# → experiments/pentagon-perturb/pentagon_perturb_sys_hist.png
# → experiments/pentagon-perturb/pentagon_perturb_stats.tex
# → experiments/pentagon-perturb/pentagon_perturb_pca.tex

# Lagrangian products: 5×5 grid plot + polygon pairs plot
python3 experiments/lagrangian-products/lagrangian_products.py
# → experiments/lagrangian-products/lagrangian_products_5x5.png
# → experiments/lagrangian-products/lagrangian_products_polygon_pairs.png

# ── Step 3: Visualization screenshots ────────────────────────────────────────
# Automated via Playwright (headless Chrome). Requires: npm install playwright.
# The script starts a local server, loads each polytope, and takes screenshots.

(cd experiments/visualization/viz && npx serve -l 8080 &)
sleep 2  # wait for server
(cd experiments/visualization/viz && node screenshot-figures.mjs)
kill %1 2>/dev/null  # stop server
# → experiments/visualization/viz-hypercube-edges.png
# → experiments/visualization/viz-hypercube-ridges.png
# → experiments/visualization/viz-hypercube-traj.png
# → experiments/visualization/viz-simplex-traj.png
# → experiments/visualization/viz-hko-pentagon-edges.png
# → experiments/visualization/viz-hko-pentagon-traj.png
# → experiments/visualization/viz-lagrangian-tri-product-traj.png

# ── Step 4: Compile thesis ───────────────────────────────────────────────────

(cd thesis && latexmk -pdf main.tex)
# → thesis/build/main.pdf

echo "Done. Thesis PDF: thesis/build/main.pdf"
