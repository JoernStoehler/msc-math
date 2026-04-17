# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib"]
# ///

"""
Shared figure configuration for experiment plots.

Goal: All figures render at thesis text width (5.4") with readable fonts
      matching the 12pt Computer Modern thesis style.
Input Artifacts: None
Output Artifacts: None

Usage:
    from figure_config import setup, FIGSIZE_SINGLE, FIGSIZE_DUAL, FIGSIZE_WIDE
    setup()
    fig, ax = plt.subplots(figsize=FIGSIZE_SINGLE)
"""

import matplotlib.pyplot as plt

# \textwidth for \documentclass[a4paper,12pt]{article} = 390.0pt / 72.27 pt/in = 5.394"
TEXT_WIDTH = 5.4  # inches

# Standard figure sizes
FIGSIZE_SINGLE = (TEXT_WIDTH, 3.5)       # single plot
FIGSIZE_DUAL = (TEXT_WIDTH, 3.0)         # 1x2 subplots
FIGSIZE_TRIPLE = (TEXT_WIDTH, 3.0)       # 1x3 subplots
FIGSIZE_WIDE = (TEXT_WIDTH, 2.5)         # wide multi-panel (e.g. 2x5)
FIGSIZE_SQUARE = (TEXT_WIDTH, TEXT_WIDTH) # square plot (e.g. 2D scatter)

# Font sizes — tuned for readability at TEXT_WIDTH
FONT_SIZE = 10        # axis labels, legend
FONT_SIZE_SMALL = 8   # tick labels, annotations
FONT_SIZE_TITLE = 11  # subplot titles
FONT_SIZE_SUPTITLE = 12  # figure-level suptitle

# Plot elements
MARKER_SIZE = 4
LINE_WIDTH = 1.5
SCATTER_SIZE = 18  # scatter plot marker area (points^2)

DPI = 150


def setup():
    """Configure matplotlib for thesis-quality figures."""
    plt.rcParams.update({
        # Use TeX-compatible fonts (Computer Modern via mathtext)
        "font.family": "serif",
        "font.serif": ["Computer Modern Roman", "CMU Serif", "DejaVu Serif"],
        "mathtext.fontset": "cm",

        # Font sizes
        "font.size": FONT_SIZE,
        "axes.titlesize": FONT_SIZE_TITLE,
        "axes.labelsize": FONT_SIZE,
        "xtick.labelsize": FONT_SIZE_SMALL,
        "ytick.labelsize": FONT_SIZE_SMALL,
        "legend.fontsize": FONT_SIZE_SMALL,
        "figure.titlesize": FONT_SIZE_SUPTITLE,

        # Line/marker defaults
        "lines.linewidth": LINE_WIDTH,
        "lines.markersize": MARKER_SIZE,

        # Figure defaults
        "figure.dpi": DPI,
        "savefig.dpi": DPI,
        "savefig.bbox": "tight",

        # Clean style
        "axes.grid": True,
        "grid.alpha": 0.3,
    })
