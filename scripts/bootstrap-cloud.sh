#!/usr/bin/env bash
set -euo pipefail

readonly RCLONE_VERSION=1.75.0
readonly RCLONE_SHA256=aa2804e08f48250e71009c727124b6341cd0288465804a9a09d14663cabafbaa
readonly UV_VERSION=0.12.2
readonly UV_SHA256=d66e96b5f1ca3b99806eee283a8125d33a0bd669e6e6d9bc4ab7ffda63c41bf4
readonly R2_ENDPOINT=https://ef19d5c4c89e0b61a5a1560041679e2d.r2.cloudflarestorage.com

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

install_system_packages() {
  sudo apt-get update
  sudo apt-get install -y --no-install-recommends \
    biber build-essential ca-certificates curl jq latexmk libssl-dev \
    pkg-config poppler-utils python3 python3-venv qhull-bin ripgrep rsync \
    shellcheck unzip \
    texlive-bibtex-extra texlive-extra-utils texlive-fonts-extra \
    texlive-fonts-recommended \
    texlive-latex-extra texlive-latex-recommended texlive-luatex \
    texlive-pictures texlive-science texlive-xetex
}

install_standalone_tools() (
  local work
  work="$(mktemp -d)"
  trap 'rm -rf -- "${work}"' EXIT
  install -d "${HOME}/.local/bin"

  curl -fsSL \
    "https://downloads.rclone.org/v${RCLONE_VERSION}/rclone-v${RCLONE_VERSION}-linux-amd64.zip" \
    -o "${work}/rclone.zip"
  echo "${RCLONE_SHA256}  ${work}/rclone.zip" | sha256sum -c -
  unzip -q "${work}/rclone.zip" -d "${work}/rclone"
  install -m 0755 \
    "${work}/rclone/rclone-v${RCLONE_VERSION}-linux-amd64/rclone" \
    "${HOME}/.local/bin/rclone"

  curl -fsSL \
    "https://github.com/astral-sh/uv/releases/download/${UV_VERSION}/uv-x86_64-unknown-linux-gnu.tar.gz" \
    -o "${work}/uv.tar.gz"
  echo "${UV_SHA256}  ${work}/uv.tar.gz" | sha256sum -c -
  mkdir "${work}/uv"
  tar -xzf "${work}/uv.tar.gz" -C "${work}/uv" --strip-components=1
  install -m 0755 "${work}/uv/uv" "${work}/uv/uvx" "${HOME}/.local/bin/"
)

configure_r2() {
  [[ -n "${MSC_MATH_R2_ACCESS_KEY_ID:-}" ]] ||
    die 'Codex secret MSC_MATH_R2_ACCESS_KEY_ID is required'
  [[ -n "${MSC_MATH_R2_SECRET_ACCESS_KEY:-}" ]] ||
    die 'Codex secret MSC_MATH_R2_SECRET_ACCESS_KEY is required'

  install -d -m 0700 "${HOME}/.config/rclone"
  RCLONE_CONFIG="${HOME}/.config/rclone/rclone.conf" \
    "${HOME}/.local/bin/rclone" config create mscmath s3 \
      provider Cloudflare \
      access_key_id "${MSC_MATH_R2_ACCESS_KEY_ID}" \
      secret_access_key "${MSC_MATH_R2_SECRET_ACCESS_KEY}" \
      endpoint "${R2_ENDPOINT}" \
      acl '' \
      no_check_bucket true >/dev/null
  chmod 0600 "${HOME}/.config/rclone/rclone.conf"
  unset MSC_MATH_R2_ACCESS_KEY_ID MSC_MATH_R2_SECRET_ACCESS_KEY
  "${HOME}/.local/bin/rclone" lsf \
    mscmath:msc-math-artifacts/snapshots --max-depth 1 \
    --s3-acl '' --s3-no-check-bucket >/dev/null
}

configure_rust() {
  command -v rustup >/dev/null || die 'Codex universal image must provide rustup'
  rustup toolchain install 1.94.0 --profile minimal \
    --component clippy,rustfmt,rust-analyzer
  cargo fetch --locked
}

main() {
  local path_line
  [[ "$(uname -m)" == x86_64 ]] || die 'msc-math cloud setup currently requires x86_64'
  install_system_packages
  install_standalone_tools
  export PATH="${HOME}/.local/bin:${PATH}"
  # The literal shell expression belongs in future interactive shells.
  # shellcheck disable=SC2016
  path_line='export PATH="$HOME/.local/bin:$PATH"'
  grep -Fqx "${path_line}" "${HOME}/.bashrc" 2>/dev/null ||
    printf '%s\n' "${path_line}" >>"${HOME}/.bashrc"
  configure_r2
  configure_rust
  rclone version
  uv --version
  rustc --version
  latexmk --version >/dev/null
}

main "$@"
