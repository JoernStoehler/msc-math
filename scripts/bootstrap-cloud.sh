#!/usr/bin/env bash
set -euo pipefail

readonly R2_ENDPOINT=https://ef19d5c4c89e0b61a5a1560041679e2d.r2.cloudflarestorage.com

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

preflight() {
  [[ "$(uname -m)" == x86_64 ]] ||
    die 'msc-math cloud setup currently requires x86_64'
  [[ -n "${MSC_MATH_R2_ACCESS_KEY_ID:-}" ]] ||
    die 'Codex secret MSC_MATH_R2_ACCESS_KEY_ID is required'
  [[ -n "${MSC_MATH_R2_SECRET_ACCESS_KEY:-}" ]] ||
    die 'Codex secret MSC_MATH_R2_SECRET_ACCESS_KEY is required'
}

verify_universal_runtimes() {
  command -v uv >/dev/null ||
    die 'Codex universal image must provide uv'
  command -v rustup >/dev/null ||
    die 'Codex universal image must provide rustup'
  python3 -c 'import sys; raise SystemExit(sys.version_info[:2] != (3, 12))' ||
    die 'select Python 3.12 in the Codex environment'
  rustc --version | grep -Fq 'rustc 1.94.0 ' ||
    die 'select Rust 1.94.0 in the Codex environment'
  rustfmt --version >/dev/null
  cargo clippy --version >/dev/null
  rust-analyzer --version >/dev/null
}

install_system_packages() {
  sudo apt-get update
  sudo env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    biber latexmk rclone texlive-bibtex-extra texlive-latex-extra
}

configure_r2() {
  install -d -m 0700 "${HOME}/.config/rclone"
  RCLONE_CONFIG="${HOME}/.config/rclone/rclone.conf" \
    rclone config create mscmath s3 \
      provider Cloudflare \
      access_key_id "${MSC_MATH_R2_ACCESS_KEY_ID}" \
      secret_access_key "${MSC_MATH_R2_SECRET_ACCESS_KEY}" \
      endpoint "${R2_ENDPOINT}" \
      acl '' \
      no_check_bucket true >/dev/null
  chmod 0600 "${HOME}/.config/rclone/rclone.conf"
  unset MSC_MATH_R2_ACCESS_KEY_ID MSC_MATH_R2_SECRET_ACCESS_KEY
  rclone lsf mscmath:msc-math-artifacts/snapshots --max-depth 1 \
    --s3-acl '' --s3-no-check-bucket >/dev/null
}

main() {
  preflight
  verify_universal_runtimes
  install_system_packages
  configure_r2
  cargo fetch --locked
  rclone version
  uv --version
  rustc --version
  latexmk --version >/dev/null
}

main "$@"
