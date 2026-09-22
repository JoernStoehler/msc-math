# syntax=docker/dockerfile:1

# Candidate image for the msc-math sbx profile.
#
# This intentionally contains project tooling, not the repository itself. The
# repository and its host-side state are supplied by the sbx mounts. Codex is
# updated explicitly after sandbox creation because its release cadence is much
# faster than this image should be rebuilt.

FROM docker.io/docker/sandbox-templates:codex

# Make failures in the installer pipelines fail the image build.
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

USER root

ARG DEBIAN_FRONTEND=noninteractive

# Project-facing system tools:
# - build-essential/pkg-config: native dependencies for Rust crates
# - latexmk/biber/texlive-*: thesis and formal-document builds
# - libgraph-easy-perl: provides the `graph-easy` command used by view-graph.sh
# - python3: fallback interpreter; ordinary scripts still use uv metadata
# - jq/rg/graphviz/zip/unzip/rsync: repository and agent workflow utilities
# - openssh-client: useful for documented sandbox/remote workflows
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        biber \
        build-essential \
        ca-certificates \
        curl \
        git \
        graphviz \
        jq \
        latexmk \
        less \
        libgraph-easy-perl \
        openssh-client \
        pkg-config \
        python3 \
        ripgrep \
        rsync \
        texlive-bibtex-extra \
        texlive-latex-extra \
        unzip \
        zip \
    && rm -rf /var/lib/apt/lists/*

USER agent

ARG MINIFORGE_VERSION=26.7.2-0
ARG MINIFORGE_SHA256=281b0ac7d550802efc81af633225a5e6116d29ae72f3ab4eae7168c3931a4c05

# Keep user-installed toolchains in the image, while leaving build caches to
# the mounted /tmp/msc-math/cache/vm tree configured by sbxenv.yaml.
ENV PATH=/home/agent/.local/bin:/home/agent/.cargo/bin:/home/agent/.local/share/miniforge/bin:$PATH \
    UV_CACHE_DIR=/tmp/msc-math/cache/vm/uv \
    SAGE_PREFIX=/home/agent/.local/share/miniforge/envs/sage \
    SAGE_PYTHON=/home/agent/.local/share/miniforge/envs/sage/bin/python

# Sage is a separately managed conda environment in the project. Install a
# checksummed Miniforge release and pin the Sage version verified by the repo's
# installation notes. Keeping it in the image removes the old 9.4-GB host
# environment from the runtime contract without pretending it is disposable
# scratch state.
RUN miniforge_installer="/tmp/Miniforge3-${MINIFORGE_VERSION}-Linux-x86_64.sh" \
    && curl --proto '=https' --tlsv1.2 --fail --silent --show-error --location \
        "https://github.com/conda-forge/miniforge/releases/download/${MINIFORGE_VERSION}/Miniforge3-${MINIFORGE_VERSION}-Linux-x86_64.sh" \
        --output "${miniforge_installer}" \
    && echo "${MINIFORGE_SHA256}  ${miniforge_installer}" | sha256sum --check --status \
    && bash "${miniforge_installer}" -b -p /home/agent/.local/share/miniforge \
    && rm -f "${miniforge_installer}"

ARG SAGE_VERSION=10.9

# Keep the conda package cache outside the image layer. If only the Sage
# version or this solve command changes, unchanged packages are reusable by the
# next build even though the environment layer itself must be recreated.
RUN --mount=type=cache,id=msc-math-miniforge-pkgs,target=/home/agent/.local/share/miniforge/pkgs,uid=1000,gid=1000,sharing=locked \
    /home/agent/.local/share/miniforge/bin/mamba create \
        --yes \
        --channel conda-forge \
        --strict-channel-priority \
        --ssl-verify /etc/ssl/certs/ca-certificates.crt \
        --name sage \
        "sage=${SAGE_VERSION}"

# Rustup itself is deliberately not pinned. Keep this bootstrap separate from
# the toolchain layer so changing the repository's compiler version does not
# redownload/reinstall rustup itself.
RUN curl --proto '=https' --tlsv1.2 --fail --silent --show-error https://sh.rustup.rs \
        | sh -s -- -y --default-toolchain none --profile minimal

ARG RUST_TOOLCHAIN=1.94.0

# Install the repository's pinned Rust toolchain. Rustup's download cache is
# persistent across builds but is not copied into the final image.
RUN --mount=type=cache,id=msc-math-rustup-downloads,target=/home/agent/.rustup/downloads,uid=1000,gid=1000,sharing=locked \
    rustup toolchain install "${RUST_TOOLCHAIN}" --profile minimal \
    && rustup component add clippy rustfmt rust-analyzer --toolchain "${RUST_TOOLCHAIN}" \
    && rustup default "${RUST_TOOLCHAIN}"

# Keep uv in its own final tool layer: changing this installer should not
# invalidate Rust or Sage.
RUN curl --proto '=https' --tlsv1.2 --fail --silent --show-error https://astral.sh/uv/install.sh \
        | sh

# Expose the actual Sage launcher for `.sage` files and version checks. Python
# files that import Sage use the explicit $SAGE_PYTHON interpreter instead of
# redefining the meaning of the `sage` command.
RUN ln -s /home/agent/.local/share/miniforge/envs/sage/bin/sage \
    /home/agent/.local/bin/sage
