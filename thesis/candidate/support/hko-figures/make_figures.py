import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D  # noqa: F401

OUT = "/workspaces/msc-math/.git/codex/hko-figures"
plt.rcParams.update({
    "font.size": 14, "axes.titlesize": 15, "axes.labelsize": 14,
    "xtick.labelsize": 12, "ytick.labelsize": 12,
    "legend.fontsize": 12, "mathtext.fontset": "dejavusans",
})

def save(fig, stem):
    fig.savefig(f"{OUT}/{stem}.pdf", bbox_inches="tight")
    fig.savefig(f"{OUT}/{stem}.png", dpi=180, bbox_inches="tight")
    plt.close(fig)

def fig_one_two():
    fig = plt.figure(figsize=(11, 4.5))
    ax = fig.add_subplot(1, 2, 1)
    x = np.linspace(-1.25, 1.25, 500)
    ax.plot(x, 1+x, lw=2, color="#2563a6", label=r"$1+x$")
    ax.plot(x, 1-x, lw=2, color="#d26a2e", label=r"$1-x$")
    ax.plot(x, np.minimum(1+x, 1-x), lw=3, color="#1b1b1b", label=r"$f(x)=1-|x|$")
    ax.scatter([0], [1], color="#1b1b1b", zorder=5)
    ax.annotate(r"touching point", (0, 1), xytext=(.28, 1.28), arrowprops={"arrowstyle":"->"})
    ax.set(xlabel=r"transverse direction $x$", ylabel=r"upper bound", title="One-dimensional mechanism")
    ax.set_xlim(-1.25, 1.25); ax.set_ylim(-.35, 2.35); ax.grid(alpha=.2)
    ax.legend(frameon=False, loc="upper left")

    ax = fig.add_subplot(1, 2, 2)
    x = np.linspace(-1.15, 1.15, 90); y = np.linspace(-1.15, 1.15, 90)
    X, Y = np.meshgrid(x, y)
    planes = [1+X, 1-X/2+np.sqrt(3)*Y/2, 1-X/2-np.sqrt(3)*Y/2]
    Z = np.minimum.reduce(planes)
    levels = np.linspace(-.2, 1, 13)
    ax.contourf(X, Y, Z, levels=levels, cmap="viridis", alpha=.88)
    ax.contour(X, Y, Z, levels=[1], colors="black", linewidths=2)
    # show the three planar gradients in the transverse plane
    grads = [(1,0,r"$r_1=(1,0)$"),(-.5,np.sqrt(3)/2,r"$r_2=(-1/2,\sqrt{3}/2)$"),(-.5,-np.sqrt(3)/2,r"$r_3=(-1/2,-\sqrt{3}/2)$")]
    label_pos = [(0.78, .08), (-.82, .78), (-.82, -.74)]
    for gx, gy, lab in grads:
        ax.arrow(0, 0, gx*.58, gy*.58, color="#b22222", width=.012, head_width=.08, length_includes_head=True)
        ax.text(*label_pos.pop(0), lab, color="#8b1a1a", ha="center", va="center")
    ax.scatter([0], [0], color="black", s=20, zorder=5)
    ax.set(xlabel="$x$", ylabel="$y$", title="Three bounds in a transverse plane")
    ax.set_xlim(-1.15, 1.15); ax.set_ylim(-1.15, 1.15); ax.set_aspect("equal")
    ax.grid(alpha=.2)
    cbar = fig.colorbar(ax.collections[0], ax=ax, fraction=.046, pad=.04)
    cbar.set_label("lower-envelope upper bound")
    fig.tight_layout()
    save(fig, "upper-bound-mechanisms")

def fig_symmetry():
    fig = plt.figure(figsize=(7, 5.4))
    ax = fig.add_subplot(111, projection="3d", computed_zorder=False)
    u = np.linspace(-1.15, 1.15, 100); v = np.linspace(-1.15, 1.15, 100)
    U, V = np.meshgrid(u, v)
    Z = 1 - np.abs(U)
    ax.plot_surface(U, V, Z, cmap="plasma", alpha=.70, linewidth=0, antialiased=True, zorder=1)
    vs = np.linspace(-1.05, 1.05, 250)
    ax.plot(np.zeros_like(vs), vs, np.ones_like(vs), color="black", lw=2.5, label="symmetry ridge", zorder=10)
    us = np.linspace(-1.05,1.05,250)
    slice_z = 1 - np.abs(us)
    ax.plot(us, np.zeros_like(us), slice_z, color="#16803c", lw=3, label="transverse slice $v=0$", zorder=11)
    ax.scatter([0], [0], [1], color="black", s=22)
    ax.set(xlabel=r"transverse coordinate $u$", ylabel=r"symmetry coordinate $v$", zlabel="upper bound", title="Symmetry extension of a strict transverse maximum")
    ax.set_zlim(-.35, 1.65); ax.view_init(elev=27, azim=-58)
    ax.legend(frameon=False, loc="upper left", bbox_to_anchor=(0.01, 0.99))
    fig.tight_layout()
    save(fig, "symmetry-extension")

if __name__ == "__main__":
    fig_one_two(); fig_symmetry()
