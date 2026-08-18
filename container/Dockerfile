# syntax=docker/dockerfile:1.7@sha256:a57df69d0ea827fb7266491f2813635de6f17269be881f696fbfdf2d83dda33e
# Environment declaration; design rationale and ownership map: README.md

# Stability: changed only for an intentional Ubuntu base refresh.
# Invalidated by: the pinned base digest.
# Correlation: the digest fixes the minimal OS starting point.
FROM ubuntu:24.04@sha256:561618e2c15bf2397621dd04f96926663a3b5616c189cf7e38db7e82f5c538ea
ARG DEBIAN_FRONTEND=noninteractive
SHELL ["/bin/bash", "-o", "pipefail", "-c"]
ENV LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 TZ=Europe/Berlin

# Stability: ordinary OS/build/diagnostic packages; refreshed intentionally.
# Invalidated by: the base digest or this explicit package set.
# Correlation: small common packages share one apt transaction and cache.
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get update && apt-get install -y --no-install-recommends \
      bash-completion bat bc btop bubblewrap build-essential ca-certificates \
      chktex clang cmake curl dc entr fd-find file fzf gh ghostscript git \
      gnuplot-nox heaptrack htop imagemagick inotify-tools jq lacheck \
      latexmk less libcap-dev libclang-dev librsvg2-bin libseccomp-dev \
      libsqlite3-dev libssl-dev libzstd-dev llvm locales lsof ltrace make \
      netcat-openbsd ninja-build openssh-client openssl pandoc parallel \
      pkg-config poppler-utils procps protobuf-compiler psmisc python3 \
      python3-pip python3-pygments python3-sympy python3-venv qhull-bin \
      ripgrep rsync shellcheck shfmt socat sqlite3 strace sysstat time tmux \
      trash-cli tree universal-ctags unzip util-linux xz-utils zip zsh \
    && locale-gen en_US.UTF-8 \
    && ln -s /usr/bin/fdfind /usr/local/bin/fd \
    && ln -s /usr/bin/batcat /usr/local/bin/bat

# Stability: large thesis toolchain; refreshed intentionally.
# Invalidated by: the base digest or TeX package set.
# Correlation: kept separate so unrelated CLI changes do not rebuild TeX.
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get update && apt-get install -y --no-install-recommends \
      biber latexml texlive-bibtex-extra texlive-extra-utils \
      texlive-fonts-extra texlive-fonts-recommended texlive-latex-extra \
      texlive-latex-recommended texlive-luatex texlive-pictures \
      texlive-science texlive-xetex

# Stability: pinned standalone developer CLIs; independently checksum-verified.
# Invalidated by: these versions/checksums only.
# Correlation: similarly sized archive installs share one cheap layer.
ARG NODE_VERSION=22.22.1
ARG NODE_SHA256=9a6bc82f9b491279147219f6a18add1e18424dce90d41d2a5fcd69d4924ba3aa
ARG UV_VERSION=0.12.2
ARG UV_SHA256=d66e96b5f1ca3b99806eee283a8125d33a0bd669e6e6d9bc4ab7ffda63c41bf4
ARG DUCKDB_VERSION=1.5.4
ARG DUCKDB_SHA256=1f2fa724fb054b3dbe1a9cbd13de5b76997d850e7087ec762ba88db04e0180cf
ARG RCLONE_VERSION=1.75.0
ARG RCLONE_SHA256=aa2804e08f48250e71009c727124b6341cd0288465804a9a09d14663cabafbaa
RUN set -eux; \
    work="$(mktemp -d)"; \
    curl -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64.tar.xz" -o "${work}/node.tar.xz"; \
    echo "${NODE_SHA256}  ${work}/node.tar.xz" | sha256sum -c -; \
    tar -xJf "${work}/node.tar.xz" -C /usr/local --strip-components=1; \
    curl -fsSL "https://github.com/astral-sh/uv/releases/download/${UV_VERSION}/uv-x86_64-unknown-linux-gnu.tar.gz" -o "${work}/uv.tar.gz"; \
    echo "${UV_SHA256}  ${work}/uv.tar.gz" | sha256sum -c -; \
    mkdir "${work}/uv"; tar -xzf "${work}/uv.tar.gz" -C "${work}/uv" --strip-components=1; \
    install -m 0755 "${work}/uv/uv" "${work}/uv/uvx" /usr/local/bin/; \
    curl -fsSL "https://github.com/duckdb/duckdb/releases/download/v${DUCKDB_VERSION}/duckdb_cli-linux-amd64.zip" -o "${work}/duckdb.zip"; \
    echo "${DUCKDB_SHA256}  ${work}/duckdb.zip" | sha256sum -c -; \
    unzip -q "${work}/duckdb.zip" -d "${work}/duckdb"; install -m 0755 "${work}/duckdb/duckdb" /usr/local/bin/; \
    curl -fsSL "https://downloads.rclone.org/v${RCLONE_VERSION}/rclone-v${RCLONE_VERSION}-linux-amd64.zip" -o "${work}/rclone.zip"; \
    echo "${RCLONE_SHA256}  ${work}/rclone.zip" | sha256sum -c -; \
    unzip -q "${work}/rclone.zip" -d "${work}/rclone"; \
    install -m 0755 "${work}/rclone/rclone-v${RCLONE_VERSION}-linux-amd64/rclone" /usr/local/bin/; \
    rm -rf "${work}"; \
    node --version; npm --version; uv --version; duckdb --version; rclone version

# Stability: large mathematical environment, changed only by its explicit lock.
# Invalidated by: Miniforge pin or Sage lock.
# Correlation: isolated from every unrelated language toolchain.
ARG MINIFORGE_VERSION=25.11.0-1
ARG MINIFORGE_SHA256=be1bad9d4e67a8753eb76fb4940e9a08036786675c7adf060627e55791bf110d
ENV MINIFORGE_DIR=/opt/miniforge \
    PATH=/opt/miniforge/envs/sage/bin:/opt/miniforge/bin:${PATH}
COPY container/locks/sage-10.9-python-3.12-linux-64.explicit /opt/locks/sage.explicit
RUN set -eux; \
    installer=/tmp/miniforge.sh; \
    curl -fsSL "https://github.com/conda-forge/miniforge/releases/download/${MINIFORGE_VERSION}/Miniforge3-${MINIFORGE_VERSION}-Linux-x86_64.sh" -o "${installer}"; \
    echo "${MINIFORGE_SHA256}  ${installer}" | sha256sum -c -; \
    bash "${installer}" -b -p "${MINIFORGE_DIR}"; rm "${installer}"; \
    "${MINIFORGE_DIR}/bin/mamba" create -y -n sage --file /opt/locks/sage.explicit; \
    "${MINIFORGE_DIR}/bin/conda" clean -afy; \
    "${MINIFORGE_DIR}/envs/sage/bin/sage" --version

# Sage stays explicitly invocable, but its environment must not contaminate
# later compiler/tool builds or ordinary runtime commands.
ENV PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

# Stability: follows the repository Rust pin and explicit Cargo tool versions.
# Invalidated by: rust-toolchain.toml or Rust/Cargo-tool pins.
# Correlation: Rust components and their Cargo tools share one toolchain.
ARG RUSTUP_VERSION=1.28.2
ARG RUSTUP_SHA256=20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da0cdd43c
ENV RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo
COPY rust-toolchain.toml /opt/locks/rust-toolchain.toml
RUN set -eux; \
    grep -q 'channel = "1.94.0"' /opt/locks/rust-toolchain.toml; \
    curl -fsSL "https://static.rust-lang.org/rustup/archive/${RUSTUP_VERSION}/x86_64-unknown-linux-gnu/rustup-init" -o /tmp/rustup-init; \
    echo "${RUSTUP_SHA256}  /tmp/rustup-init" | sha256sum -c -; \
    chmod 0755 /tmp/rustup-init; \
    /tmp/rustup-init -y --no-modify-path --profile minimal --default-toolchain 1.94.0; \
    rm /tmp/rustup-init; \
    /opt/cargo/bin/rustup component add clippy rustfmt rust-analyzer llvm-tools-preview; \
    /opt/cargo/bin/cargo install cargo-llvm-cov --version 0.8.6 --locked; \
    /opt/cargo/bin/cargo install flamegraph --version 0.6.12 --locked; \
    /opt/cargo/bin/rustc --version

# Stability: Python tool graph changes only with its hash lock.
# Invalidated by: pre-commit lock files.
# Correlation: isolated from repository Python environments.
COPY container/locks/pre-commit.txt /opt/locks/pre-commit.txt
RUN set -eux; \
    uv venv --python /usr/bin/python3 /opt/pre-commit; \
    uv pip sync --python /opt/pre-commit/bin/python --require-hashes /opt/locks/pre-commit.txt; \
    /opt/pre-commit/bin/pre-commit --version

# Stability: small development-runtime privilege and executable boundary.
# Invalidated by: this explicit package set and exposed-tool list.
# Correlation: intentionally late so these additions do not rebuild TeX or Sage.
RUN --mount=type=cache,target=/var/cache/apt,sharing=locked \
    --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get update && apt-get install -y --no-install-recommends \
      hyperfine sudo valgrind \
    && ln -s /opt/miniforge/envs/sage/bin/sage /usr/local/bin/sage \
    && ln -s /opt/miniforge/bin/conda /usr/local/bin/conda \
    && ln -s /opt/miniforge/bin/mamba /usr/local/bin/mamba \
    && ln -s /opt/pre-commit/bin/pre-commit /usr/local/bin/pre-commit

# Stability: host identity and small system conventions.
# Invalidated by: UID/GID or copied tmux configuration.
# Correlation: kept after heavyweight toolchains because machine identity varies.
ARG DEVELOPER_UID=1000
ARG DEVELOPER_GID=1000
RUN set -eux; \
    existing_user="$(getent passwd "${DEVELOPER_UID}" | cut -d: -f1 || true)"; \
    test -z "${existing_user}" || userdel -r "${existing_user}" || userdel "${existing_user}"; \
    existing_group="$(getent group "${DEVELOPER_GID}" | cut -d: -f1 || true)"; \
    test -z "${existing_group}" || groupdel "${existing_group}"; \
    groupadd --gid "${DEVELOPER_GID}" developer; \
    useradd --uid "${DEVELOPER_UID}" --gid "${DEVELOPER_GID}" --create-home --shell /bin/bash developer; \
    echo 'developer ALL=(ALL:ALL) NOPASSWD: ALL' > /etc/sudoers.d/developer; \
    chmod 0440 /etc/sudoers.d/developer; \
    visudo --check --file=/etc/sudoers.d/developer; \
    install -d -o developer -g developer -m 0700 /home/developer/.codex; \
    install -d -o developer -g developer -m 0700 /home/developer/.config/gh; \
    install -d -o developer -g developer -m 0755 \
      /home/developer/.cache /home/developer/.cargo \
      /home/developer/.local /home/developer/.local/bin
COPY --chmod=0644 container/tmux.conf /etc/tmux.conf
RUN git config --system init.defaultBranch main \
    && git config --system core.autocrlf false \
    && git config --system credential.https://github.com.helper '!gh auth git-credential'

# Stability: pinned agent/app-server protocol; intentionally cheap to update.
# Invalidated by: CODEX_VERSION only.
ARG CODEX_VERSION=0.147.0
RUN --mount=type=cache,target=/root/.npm,sharing=locked \
    npm install --global "@openai/codex@${CODEX_VERSION}" \
    && test "$(codex --version)" = "codex-cli ${CODEX_VERSION}"

# Stability: metadata-only; changes on every source revision.
# Invalidated by: explicit revision argument.
# Correlation: last so it cannot invalidate heavyweight layers.
ARG WORKSPACE_REVISION=uncommitted
LABEL org.opencontainers.image.title="msc-math development workspace" \
      org.opencontainers.image.revision="${WORKSPACE_REVISION}" \
      io.joern.msc-math.developer-uid="${DEVELOPER_UID}" \
      io.joern.msc-math.developer-gid="${DEVELOPER_GID}"

ENV HOME=/home/developer \
    USER=developer \
    LOGNAME=developer \
    CODEX_HOME=/home/developer/.codex \
    RUSTUP_HOME=/opt/rustup \
    CARGO_HOME=/home/developer/.cargo \
    UV_CACHE_DIR=/home/developer/.cache/uv \
    NPM_CONFIG_CACHE=/home/developer/.cache/npm \
    NPM_CONFIG_PREFIX=/home/developer/.local \
    PATH=/home/developer/.local/bin:/home/developer/.cargo/bin:/opt/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin

USER developer
WORKDIR /workspaces/msc-math
CMD ["sleep", "infinity"]
