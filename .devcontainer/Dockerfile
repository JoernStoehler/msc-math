FROM mcr.microsoft.com/devcontainers/base:ubuntu

ENV DEBIAN_FRONTEND=noninteractive

SHELL ["/bin/bash", "-lc"]

# -----------------------------------------------------------------------------
# Local devcontainer image for msc-math (Jörn's Ubuntu desktop)
#
# Design goals:
# - Keep the image simple and close to standard devcontainer patterns (KISS).
# - Install heavyweight, rarely changing dependencies in earlier layers so they
#   are cached across rebuilds.
# - Install more frequently tweaked tools (Node/npm, uv) after base deps.
# - Install language toolchains (Rust) once for the dev user here rather
#   than in postCreate, so they are baked into the image.
# -----------------------------------------------------------------------------

# Base system dependencies (rarely change; heavy but shared by all tools).
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    git \
    git-lfs \
    build-essential \
    pkg-config \
    libssl-dev \
    libclang-dev \
    python3 \
    python3-venv \
    python3-pip \
    python3-sympy \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# TeX toolchain for PDF and HTML builds (~2GB installed).
# TexLive 2023 from Ubuntu apt repositories.
RUN apt-get update && apt-get install -y --no-install-recommends \
    texlive-latex-recommended \
    texlive-latex-extra \
    texlive-fonts-recommended \
    texlive-fonts-extra \
    texlive-bibtex-extra \
    texlive-pictures \
    texlive-science \
    texlive-xetex \
    texlive-luatex \
    texlive-extra-utils \
    latexmk \
    biber \
    chktex \
    lacheck \
    ghostscript \
    imagemagick \
    python3-pygments \
    latexml \
    && rm -rf /var/lib/apt/lists/*

# Common CLI tools used by agents (cheap to adjust; comes after base deps)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ripgrep \
    jq \
    gh \
    universal-ctags \
    tree \
    inotify-tools \
    entr \
    qhull-bin \
    tmux \
    trash-cli \
    && rm -rf /var/lib/apt/lists/*

# VS Code tunnel CLI (bake into image to avoid slow postCreate download)
RUN set -eux; \
    tmpdir="$(mktemp -d)"; \
    curl -fsSL "https://update.code.visualstudio.com/latest/cli-linux-x64/stable" -o "$tmpdir/vscode-cli.tar.gz"; \
    tar -xzf "$tmpdir/vscode-cli.tar.gz" -C "$tmpdir"; \
    install -m 0755 "$tmpdir/code" /usr/local/bin/code-tunnel; \
    rm -rf "$tmpdir"

# Node.js 22.x (npm included) for JS tooling.
# We keep this in the image so that npm is always available for postCreate
# steps without re-downloading Node on each run.
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get update \
    && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/* \
    && node -v && npm -v

# Install uv (Python packaging/runner) globally. Pin for reproducibility.
# The installer appends "/bin" to UV_INSTALL_DIR, so point it at /usr/local to land in /usr/local/bin.
ENV UV_INSTALL_DIR=/usr/local
ENV UV_VERSION=0.5.14
RUN curl -LsSf https://astral.sh/uv/${UV_VERSION}/install.sh | sh

# Install SageMath via the officially documented conda-forge route.
# Ubuntu 24.04 does not currently expose a usable `sagemath` apt candidate in
# this image, so we bake a Miniforge-based Sage environment into the image and
# expose `sage` / `mamba` / `conda` via `/usr/local/bin`.
ENV MINIFORGE_DIR=/opt/miniforge
RUN set -eux; \
    installer="/tmp/Miniforge3-$(uname)-$(uname -m).sh"; \
    curl -L "https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-$(uname)-$(uname -m).sh" -o "$installer"; \
    bash "$installer" -b -p "$MINIFORGE_DIR"; \
    rm -f "$installer"; \
    "$MINIFORGE_DIR/bin/mamba" create -y -n sage sage; \
    ln -sf "$MINIFORGE_DIR/bin/mamba" /usr/local/bin/mamba; \
    ln -sf "$MINIFORGE_DIR/bin/conda" /usr/local/bin/conda; \
    ln -sf "$MINIFORGE_DIR/envs/sage/bin/sage" /usr/local/bin/sage; \
    "$MINIFORGE_DIR/envs/sage/bin/sage" --version; \
    "$MINIFORGE_DIR/bin/conda" clean -afy

# Switch to dev user for language toolchains.
# Toolchains live in the dev user's HOME so that cargo/elan behave normally.
USER vscode
ENV HOME=/home/vscode
ENV PATH="$HOME/.cargo/bin:$HOME/.local/bin:/usr/local:$PATH"

# Install Rust (rustup + basic components) in this devcontainer image.
# We install rustfmt/clippy here so they're available without additional setup.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable \
    && "$HOME/.cargo/bin/rustup" component add rustfmt clippy rust-analyzer || true

# Cargo tools: nextest (per-test timing/parallel), cargo-watch (auto-rerun)
RUN curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C "$HOME/.cargo/bin" \
    && "$HOME/.cargo/bin/cargo" install cargo-watch

WORKDIR /workspaces/msc-math
