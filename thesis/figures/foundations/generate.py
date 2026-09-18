# /// script
# requires-python = ">=3.12"
# dependencies = ["matplotlib"]
# ///

"""Generate thesis-native explanatory figures for the early theory chapters.

The figures are explanatory only: they do not provide proof or empirical
evidence.  Run from the repository root with

    uv run --script thesis/figures/foundations/generate.py

Outputs are vector PDFs beside this producer.
"""

from __future__ import annotations

import math
from pathlib import Path

import matplotlib.pyplot as plt
from matplotlib.patches import FancyArrowPatch, FancyBboxPatch, Polygon


OUT = Path(__file__).resolve().parent
TEXT_WIDTH = 5.4

BLUE = "#255f8f"
ORANGE = "#b05a2a"
GREEN = "#4f7d55"
RED = "#9b4141"
PURPLE = "#6f5792"
INK = "#202020"
MID = "#6b6b6b"
LIGHT = "#e8e8e8"


def setup() -> None:
    plt.rcParams.update(
        {
            "font.family": "serif",
            "font.serif": ["Computer Modern Roman", "CMU Serif", "DejaVu Serif"],
            "mathtext.fontset": "cm",
            "font.size": 9.5,
            "axes.titlesize": 10.5,
            "figure.dpi": 160,
            "savefig.bbox": "tight",
            "savefig.pad_inches": 0.03,
            "pdf.fonttype": 42,
        }
    )


def save(fig: plt.Figure, name: str) -> None:
    fig.savefig(
        OUT / name,
        format="pdf",
        bbox_inches="tight",
        pad_inches=0.03,
        metadata={
            "Creator": "thesis/figures/foundations/generate.py",
            "CreationDate": None,
        },
    )
    plt.close(fig)


def characteristic_figure() -> None:
    fig, ax = plt.subplots(figsize=(TEXT_WIDTH, 2.25))
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 4.1)
    ax.axis("off")

    plane = [(1.0, 0.75), (6.7, 0.75), (8.9, 2.25), (3.2, 2.25)]
    ax.add_patch(
        Polygon(plane, closed=True, facecolor="#edf2f5", edgecolor=MID, lw=1.1)
    )
    x = (4.9, 1.5)
    ax.plot(*x, "o", color=INK, ms=4.5, zorder=5)
    ax.text(x[0] - 0.18, x[1] - 0.38, r"$x$", ha="right", va="top")
    ax.text(7.05, 0.88, r"$T_x\partial K$", color=MID)

    # The normal and characteristic arrows are a dimension-reduced schematic.
    ax.add_patch(
        FancyArrowPatch(
            x, (6.15, 3.45), arrowstyle="-|>", mutation_scale=12, lw=1.8, color=ORANGE
        )
    )
    ax.text(6.15, 3.58, r"$\nabla H_K(x)$", color=ORANGE, ha="center")
    ax.add_patch(
        FancyArrowPatch(
            x, (2.25, 1.02), arrowstyle="-|>", mutation_scale=12, lw=2.2, color=BLUE
        )
    )
    ax.text(2.35, 0.34, r"$R_x=J_0\nabla H_K(x)$", color=BLUE, ha="center", va="top")
    ax.plot([1.8, 7.7], [0.93, 1.97], color=BLUE, lw=0.8, alpha=0.35)
    ax.text(
        7.05, 1.85, r"$\ell_x=\ker(\omega_0|_{T_x\partial K})$", color=BLUE, ha="center"
    )

    ax.text(
        9.65,
        3.25,
        r"$-J_0R_x=\nabla H_K(x)$" + "\n" + r"$\lambda_0(R_x)=1$",
        ha="right",
        va="top",
        bbox={"boxstyle": "round,pad=0.35", "fc": "white", "ec": "#b8b8b8", "lw": 0.8},
    )
    ax.text(
        9.65,
        0.52,
        r"$A(\gamma)=\int_0^T\lambda_0(\dot\gamma)\,dt=T$",
        ha="right",
        va="bottom",
        color=INK,
        bbox={"boxstyle": "round,pad=0.3", "fc": "#f7f7f7", "ec": "#b8b8b8", "lw": 0.8},
    )
    save(fig, "characteristic-normalization.pdf")


def intersect(a: tuple[float, float], b: tuple[float, float]) -> tuple[float, float]:
    det = a[0] * b[1] - a[1] * b[0]
    return ((b[1] - a[1]) / det, (a[0] - b[0]) / det)


def facet_polarity_figure() -> None:
    angles = [0, 58, 121, 187, 244, 311]
    radii = [1.00, 0.86, 1.08, 0.92, 0.82, 1.10]
    rows = [
        (r * math.cos(math.radians(theta)), r * math.sin(math.radians(theta)))
        for theta, r in zip(angles, radii, strict=True)
    ]
    vertices = [intersect(rows[i], rows[(i + 1) % len(rows)]) for i in range(len(rows))]
    hi = 1
    assert all(
        row[0] * vertex[0] + row[1] * vertex[1] <= 1.0 + 1e-10
        for vertex in vertices
        for row in rows
    )
    assert all(
        abs(rows[hi][0] * vertex[0] + rows[hi][1] * vertex[1] - 1.0) < 1e-10
        for vertex in (vertices[(hi - 1) % len(vertices)], vertices[hi])
    )

    fig, (ax_k, ax_p) = plt.subplots(1, 2, figsize=(TEXT_WIDTH, 2.45))
    for ax in (ax_k, ax_p):
        ax.set_aspect("equal")
        ax.axis("off")

    ax_k.add_patch(Polygon(vertices, closed=True, fc="#f2f2f2", ec=INK, lw=1.2))
    facet = [vertices[(hi - 1) % len(vertices)], vertices[hi]]
    ax_k.plot(
        [p[0] for p in facet],
        [p[1] for p in facet],
        color=ORANGE,
        lw=3.2,
        solid_capstyle="round",
    )
    a = rows[hi]
    norm_a = math.hypot(*a)
    n = (a[0] / norm_a, a[1] / norm_a)
    h = 1.0 / norm_a
    foot = (h * n[0], h * n[1])
    ax_k.add_patch(
        FancyArrowPatch(
            (0, 0), foot, arrowstyle="-|>", mutation_scale=10, lw=1.5, color=ORANGE
        )
    )
    ax_k.text(0.47 * foot[0] - 0.08, 0.47 * foot[1], r"$h_i$", color=ORANGE, ha="right")
    ax_k.add_patch(
        FancyArrowPatch(
            foot,
            (foot[0] + 0.6 * n[0], foot[1] + 0.6 * n[1]),
            arrowstyle="-|>",
            mutation_scale=10,
            lw=1.4,
            color=BLUE,
        )
    )
    ax_k.text(
        foot[0] + 0.42 * n[0] + 0.08,
        foot[1] + 0.42 * n[1],
        r"$n_i$",
        color=BLUE,
        ha="left",
    )
    ax_k.plot(0, 0, "o", color=INK, ms=3)
    mid_f = ((facet[0][0] + facet[1][0]) / 2, (facet[0][1] + facet[1][1]) / 2)
    ax_k.text(
        mid_f[0] - 0.18, mid_f[1] - 0.10, r"$F_i$", color=ORANGE, ha="right", va="top"
    )
    ax_k.set_xlim(-1.55, 1.55)
    ax_k.set_ylim(-1.45, 1.55)
    ax_k.set_title(r"facet of $K$")

    ax_p.add_patch(Polygon(rows, closed=True, fc="#edf2f5", ec=INK, lw=1.2))
    ax_p.scatter([p[0] for p in rows], [p[1] for p in rows], s=18, color=INK, zorder=3)
    ax_p.scatter([a[0]], [a[1]], s=55, color=ORANGE, zorder=4)
    ax_p.add_patch(
        FancyArrowPatch(
            (0, 0), a, arrowstyle="-|>", mutation_scale=10, lw=1.6, color=ORANGE
        )
    )
    ax_p.plot(0, 0, "o", color=INK, ms=3)
    ax_p.text(a[0] + 0.05, a[1] + 0.11, r"$a_i=n_i/h_i$", color=ORANGE, ha="left")
    ax_p.text(
        0.0, -1.22, r"$K^\circ=\operatorname{conv}\{a_1,\ldots,a_F\}$", ha="center"
    )
    ax_p.set_xlim(-1.55, 1.55)
    ax_p.set_ylim(-1.45, 1.55)
    ax_p.set_title(r"polar vertex")
    fig.subplots_adjust(wspace=0.16)
    save(fig, "facet-polarity.pdf")


def word_closure_figure() -> None:
    fig, (ax_d, ax_k) = plt.subplots(1, 2, figsize=(TEXT_WIDTH, 2.62))
    colors = [BLUE, ORANGE, GREEN, PURPLE]
    labels = [r"$\tau_1R_r$", r"$\tau_2R_t$", r"$\tau_3R_\ell$", r"$\tau_4R_b$"]
    # The second panel is exactly this displacement polygon translated by
    # b=(1,-1); equal axis spans make that relation visible without rescaling.
    points = [(0, 0), (0, 2), (-2, 2), (-2, 0), (0, 0)]
    assert points[-1] == points[0]
    for i in range(4):
        ax_d.add_patch(
            FancyArrowPatch(
                points[i],
                points[i + 1],
                arrowstyle="-|>",
                mutation_scale=10,
                lw=2.4,
                color=colors[i],
            )
        )
        mx = (points[i][0] + points[i + 1][0]) / 2
        my = (points[i][1] + points[i + 1][1]) / 2
        offsets = [(0.16, 0), (0, 0.15), (-0.14, 0), (0, -0.18)]
        ax_d.text(
            mx + offsets[i][0],
            my + offsets[i][1],
            labels[i],
            color=colors[i],
            ha="center",
            va="center",
        )
    ax_d.plot(0, 0, "o", color=INK, ms=4)
    ax_d.text(0.05, -0.07, r"$0$", ha="left", va="top")
    ax_d.text(-1.0, -0.48, r"$\sum_k\tau_kR_{\sigma_k}=0$", ha="center")
    ax_d.set_xlim(-2.35, 0.65)
    ax_d.set_ylim(-0.65, 2.35)
    ax_d.set_aspect("equal")
    ax_d.axis("off")
    ax_d.set_title("closure in displacement space")

    square = [(-1, -1), (1, -1), (1, 1), (-1, 1)]
    ax_k.add_patch(Polygon(square, closed=True, fc="#f3f3f3", ec=INK, lw=1.1))
    path = [(1, -1), (1, 1), (-1, 1), (-1, -1), (1, -1)]
    assert path == [(x + 1, y - 1) for x, y in points]
    for i in range(4):
        ax_k.add_patch(
            FancyArrowPatch(
                path[i],
                path[i + 1],
                arrowstyle="-|>",
                mutation_scale=10,
                lw=2.8,
                color=colors[i],
            )
        )
    ax_k.plot(1, -1, "o", color=INK, ms=4)
    ax_k.text(0.88, -1.15, r"$b$", ha="right", va="top")
    ax_k.text(1.12, 0, r"$F_r$", color=BLUE, va="center")
    ax_k.text(0, 1.12, r"$F_t$", color=ORANGE, ha="center")
    ax_k.text(-1.12, 0, r"$F_\ell$", color=GREEN, ha="right", va="center")
    ax_k.text(0, -1.12, r"$F_b$", color=PURPLE, ha="center", va="top")
    ax_k.set_xlim(-1.5, 1.5)
    ax_k.set_ylim(-1.5, 1.5)
    ax_k.set_aspect("equal")
    ax_k.axis("off")
    ax_k.set_title("one feasible translated path")

    fig.subplots_adjust(bottom=0.08, wspace=0.20)
    save(fig, "word-closure-basepoint.pdf")


def box(ax, xy, width, height, title, body, color) -> None:
    patch = FancyBboxPatch(
        xy,
        width,
        height,
        boxstyle="round,pad=0.03,rounding_size=0.06",
        fc="white",
        ec=color,
        lw=1.5,
    )
    ax.add_patch(patch)
    ax.text(
        xy[0] + width / 2,
        xy[1] + height * 0.68,
        title,
        ha="center",
        va="center",
        color=color,
        fontsize=9.0,
    )
    ax.text(
        xy[0] + width / 2,
        xy[1] + height * 0.31,
        body,
        ha="center",
        va="center",
        fontsize=8.0,
    )


def arrow(ax, start, end, label, label_xy=None) -> None:
    ax.add_patch(
        FancyArrowPatch(
            start, end, arrowstyle="-|>", mutation_scale=10, lw=1.15, color=MID
        )
    )
    if label_xy is None:
        label_xy = ((start[0] + end[0]) / 2, (start[1] + end[1]) / 2 + 0.13)
    ax.text(*label_xy, label, ha="center", va="center", color=MID, fontsize=8.2)


def simple_pipeline_figure() -> None:
    fig, ax = plt.subplots(figsize=(TEXT_WIDTH, 3.0))
    ax.set_xlim(0, 10)
    ax.set_ylim(0, 5.8)
    ax.axis("off")
    w, h = 2.62, 1.2
    x = [0.15, 3.69, 7.23]
    y_top, y_bot = 3.85, 1.22

    box(ax, (x[0], y_top), w, h, r"minimum orbit $\gamma$", r"$A=I_K=T$", BLUE)
    box(
        ax,
        (x[1], y_top),
        w,
        h,
        "piecewise-affine\n" + r"dual $z_{1,n}$",
        r"$A(z_{1,n})\to T$",
        BLUE,
    )
    box(ax, (x[2], y_top), w, h, r"pure dual $z_{2,n}$", r"$A\nearrow,\ I_K=T$", ORANGE)
    box(
        ax,
        (x[2], y_bot),
        w,
        h,
        r"simple dual $z_{3,n}$",
        r"each $R_i$ once; $A\nearrow$",
        ORANGE,
    )
    box(
        ax,
        (x[1], y_bot),
        w,
        h,
        r"feasible dual $z_{4,n}$",
        r"$A=I_K=T^2/A(z_{3,n})$",
        GREEN,
    )
    box(
        ax,
        (x[0], y_bot),
        w,
        h,
        "limit +\nreconstruct",
        r"simple $\gamma_*$ on $\partial K$",
        GREEN,
    )

    arrow(
        ax,
        (x[0] + w, y_top + h / 2),
        (x[1], y_top + h / 2),
        "approximate",
        ((x[0] + w + x[1]) / 2, 5.52),
    )
    arrow(
        ax,
        (x[1] + w, y_top + h / 2),
        (x[2], y_top + h / 2),
        "split mixed\nvelocities",
        ((x[1] + w + x[2]) / 2, 5.52),
    )
    arrow(
        ax,
        (x[2] + w / 2, y_top),
        (x[2] + w / 2, y_bot + h),
        "merge repeats",
        (x[2] + w / 2 + 0.70, (y_top + y_bot + h) / 2),
    )
    arrow(
        ax,
        (x[2], y_bot + h / 2),
        (x[1] + w, y_bot + h / 2),
        "rescale\n" + r"$\beta_n=T/A(z_{3,n})$",
        ((x[2] + x[1] + w) / 2, 2.72),
    )
    arrow(
        ax,
        (x[1], y_bot + h / 2),
        (x[0] + w, y_bot + h / 2),
        "compactness",
        ((x[1] + x[0] + w) / 2, 2.72),
    )

    ax.text(
        5.0,
        0.35,
        r"dual minimality: $A(z_{1,n})\leq A(z_{3,n})\leq T$"
        + "\n"
        + r"hence $A(z_{3,n})\to T$ and $\beta_n\to1$",
        ha="center",
        va="center",
        fontsize=8.6,
        bbox={
            "boxstyle": "round,pad=0.27",
            "fc": "#f4f4f4",
            "ec": "#bdbdbd",
            "lw": 0.7,
        },
    )
    save(fig, "simple-minimizer-pipeline.pdf")


def main() -> None:
    setup()
    characteristic_figure()
    facet_polarity_figure()
    word_closure_figure()
    simple_pipeline_figure()


if __name__ == "__main__":
    main()
