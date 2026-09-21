# Candidate only. Do not build until this file and the host profile have been
# reviewed. The build must not COPY source, credentials, host paths, or session
# data into the image.

FROM docker.io/docker/sandbox-templates:codex-docker

USER root

ENV DEBIAN_FRONTEND=noninteractive

# Project-facing build and document tooling. Keep the package list here in sync
# with INSTALL.md and the relevant local READMEs; do not treat this as a full
# reproduction environment until Sage's separate environment is specified.
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        biber \
        build-essential \
        ca-certificates \
        curl \
        git \
        latexmk \
        pkg-config \
        texlive-bibtex-extra \
        texlive-latex-extra \
    && rm -rf /var/lib/apt/lists/*

USER agent

ENV PATH=/home/agent/.cargo/bin:/home/agent/.local/bin:$PATH

# Rust is pinned to the repository's rust-toolchain.toml version rather than
# inheriting an arbitrary compiler from the base image. The exact installer
# source and cache strategy remain review points before the first build.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
        | sh -s -- -y --default-toolchain none --profile minimal \
    && rustup toolchain install 1.94.0 --profile minimal \
    && rustup component add clippy rustfmt rust-analyzer --toolchain 1.94.0 \
    && curl -LsSf https://astral.sh/uv/install.sh | sh

# Deliberately not copied from .local-environments/: Sage/Miniforge needs an
# explicit version and installation/cache policy. Deliberately not updating the
# Codex CLI here yet: the template's version is not an accepted contract, and
# the update/pinning choice should be made before spending time on a build.
