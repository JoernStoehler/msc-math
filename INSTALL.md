# Environment setup and reproduction

Start here for host or Docker Sandbox setup. Run project commands from the
repository root. The current sandbox is reached with `ssh codex-msc-math.sbx`;
its project mount is `/workspaces/msc-math`. Host-side sandbox lifecycle and
credentials belong to `/workspaces/ESTATE.md`.

## Current state

Checked on 2026-09-05:

- The sandbox has working Astra and native search, Rust, Python/uv, Herdr,
  Micro commenting, LaTeX/Biber and authenticated R2 access.
- Host and sandbox thesis builds passed. The sandbox passed 27 geometry-crate
  tests and downloaded/hash-verified one registered R2 snapshot.
- Sage 10.9 passed exact arithmetic in a fresh SSH shell, the full HKO
  verifier and a 50-case pentagon prefix (not the full pentagon certificate).

[Environment details](docs/development-environments.md) records versions,
configuration locations and diagnostic findings. Global skill/docs/memory
distribution is owned by DevOps. Its agreed sandbox installation is reported
complete; see the environment details for locations and update semantics. Do
not independently install another global system from this project.

## Ordinary language tools

Rust's version and components are in `rust-toolchain.toml`; Cargo dependencies
are in the lockfiles. No special Rust installation method is needed. Ensure
`~/.cargo/bin` is on PATH, including in noninteractive SSH shells.

Use Python 3.12 and uv for ordinary scripts. Run scripts with PEP 723 inline
dependency metadata as `uv run path/to/script.py`; dependencies belong to the
script. Keep Sage's Python separate from this ordinary environment.

## LaTeX and Biber

The following Ubuntu package set is used by `scripts/bootstrap-cloud.sh` and
supports the tested thesis build:

```bash
sudo apt-get update
sudo apt-get install --no-install-recommends \
  biber latexmk texlive-bibtex-extra texlive-latex-extra
./thesis/check-build.sh
```

The output is `thesis/build/main.pdf`. The command builds before checking
the PDF signature, nonempty log, overfull boxes and undefined references.
It does not evaluate the mathematics or writing.

Host and sandbox use different TeX versions. Reusing their generated `build/`
files can cause a Biber control-version mismatch. Preserve any wanted PDF,
move incompatible build outputs aside and rebuild. For an independent build
that leaves the current PDF untouched:

```bash
thesis_check_dir=$(mktemp -d)
(cd thesis && latexmk -g -outdir="$thesis_check_dir" -auxdir="$thesis_check_dir")
```

This last command checks compilation only; inspect the generated log when
the selected layout/reference checks are also required.

## SageMath

**Verified in `codex-msc-math` on 2026-09-05:** Miniforge/conda-forge installed
Sage 10.9 with its own Python 3.13.15, following the
[official Sage route](https://doc.sagemath.org/html/en/installation/conda.html).
Ordinary Python remains 3.12.13. No Conda activation or compiler/Python PATH
changes are needed.

The VM-private disk has only about 2.2 GB free. The tested installation uses
the ignored shared directory `.local-environments/`, consuming about 9.4 GB
including Miniforge, Sage, package caches and setup logs. About 154 GB remained
free on the shared mount after verification. Check current space first.

For a fresh Linux x86-64 installation, allow the package host from the host:

```bash
sbx policy allow network --sandbox codex-msc-math conda.anaconda.org:443
```

Run installation commands inside the sandbox. Use an attached `sbx exec`
for long commands so the sandbox remains running. The current installation
already exists; do not rerun the installer over it.

```bash
df -h / /workspaces/msc-math
sage_setup_dir=/workspaces/msc-math/.local-environments/sage-setup
sage_prefix=/workspaces/msc-math/.local-environments/miniforge
mkdir -p "$sage_setup_dir/tmp"
export TMPDIR="$sage_setup_dir/tmp"
curl -fsSL \
  https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-Linux-x86_64.sh \
  -o "$sage_setup_dir/miniforge.sh"
bash "$sage_setup_dir/miniforge.sh" -b -p "$sage_prefix"
"$sage_prefix/bin/mamba" create -y -n sage --channel conda-forge \
  --strict-channel-priority --ssl-verify /etc/ssl/certs/ca-certificates.crt sage
"$sage_prefix/envs/sage/bin/sage" --version
```

Miniforge's bundled CA trust initially rejected the sandbox proxy certificate.
The explicit system CA bundle fixes this with TLS verification enabled. Without
the network allow rule, the proxy returned HTTP 403, surfaced by mamba as a
ZSTD decompression error. An SSH-only installation also exited 137 without a
diagnostic; the attached `sbx exec` installation succeeded in 184 seconds.

The installed Sage 10.9 launcher does not accept the legacy `-python` option
used by project commands; directly passing the HKO `.py` file also returned
silently without running its main entry point. Dispatch both forms to the
environment's Python. Create executable `/home/agent/.local/bin/sage`
with this content (inspect any existing file before replacing it):

```sh
#!/bin/sh
sage_env=/workspaces/msc-math/.local-environments/miniforge/envs/sage
if [ "${1-}" = -python ]; then
    shift
    exec "$sage_env/bin/python" "$@"
fi
case "${1-}" in
    *.py) exec "$sage_env/bin/python" "$@" ;;
esac
exec "$sage_env/bin/sage" "$@"
```

Ensure `~/.local/bin` is on PATH. This wrapper is already installed in the
current sandbox. In a fresh SSH shell, check:

```bash
sage --version
sage -python -c \
  'from sage.all import QQ, matrix; assert matrix(QQ, [[1,2],[3,4]]).det() == -2; print("Sage arithmetic OK")'
```

Verification at source commit `33f5fe148ae2de5eb2fd5ba70a2a53c06753d6e8` passed:
exact rational and number-field arithmetic; the complete HKO verifier in a
temporary copy of `verify.sage.py` and `witness.json` (4.40 seconds, ranks 25
and 15); and the README's pentagon `--limit 50` command (16.52 seconds wall
time, `LIMITED PREFIX PASSED`). The prefix checks installation compatibility;
it is not a new full pentagon certificate. Canonical packet outputs were not
overwritten. Python, Rust, Micro, Codex and Herdr still resolved at their
previous paths and versions.

Record the installed Sage version with each verification run. The arithmetic
check establishes basic operation; run the relevant certificate to check
project compatibility. The successful solve was unpinned; future solves can
select other versions. Local installation and verification logs, the temporary
HKO packet, and an explicit package export are retained in
`.local-environments/sage-setup/`.

This is disposable environment data, not research source or a Git backup.
It survives sandbox removal because it is on the host mount, but contains
absolute prefixes and sandbox Linux binaries: do not assume it is relocatable
or usable on the host. The wrapper and network policy are sandbox state and
must be restored separately after recreation. Reinstall at a new path rather
than moving the prefix. Deleting this directory removes the local Sage setup,
caches and logs; it does not remove tracked research results.

**Does not work here:** the sandbox's apt index offered no `sagemath` candidate
on 2026-09-05. **Not evaluated:** source builds and other distributions; they
have not been ruled out.

## R2 data

Install `rclone` (on Ubuntu: `sudo apt-get install rclone`). Configure its
`mscmath` S3 remote for Cloudflare R2, bucket `msc-math-artifacts`, using
private credentials. Reading snapshots needs object-read access; publishing
also needs write access. The current sandbox already has this configuration.

Use `rclone config`, or supply these variables privately to the process:

```text
RCLONE_CONFIG_MSCMATH_TYPE=s3
RCLONE_CONFIG_MSCMATH_PROVIDER=Cloudflare
RCLONE_CONFIG_MSCMATH_ACCESS_KEY_ID=<R2 access key id>
RCLONE_CONFIG_MSCMATH_SECRET_ACCESS_KEY=<R2 secret access key>
RCLONE_CONFIG_MSCMATH_ENDPOINT=https://ef19d5c4c89e0b61a5a1560041679e2d.r2.cloudflarestorage.com
```

Keep credentials out of Git. The helper supplies R2's empty ACL and skips
account-level bucket checks. Discover and download only the artifact needed:

```bash
python3 scripts/artifacts.py list
python3 scripts/artifacts.py materialize combinatorial-cell-widths --no-link
```

The second command is known to work in the sandbox: it downloaded 11 MB and
verified the registered file hashes. Omit `--no-link` when the producer needs
the repository links. Links can point into an environment-specific cache;
host-created links need not resolve inside the sandbox.

[Artifact contracts](docs/artifacts.md) explain cache placement, link handling
and publication. For Codex Cloud use the configured setup/maintenance scripts
described in [environment details](docs/development-environments.md), not the
ephemeral-variable recipe after Cloud setup has finished.

## Micro comments

Micro is installed, but Ctrl+K comments are custom configuration. The known
working host configuration is `~/.config/micro/`: `init.lua`, `bindings.json`,
`settings.json`, `syntax/markdown.yaml` and
`colorschemes/herdr-one-light.micro`. Copy those files, preserving their
relative paths, into the target user's Micro configuration when provisioning
the same interface; compare existing customizations before replacing them.

The current sandbox already has these files. Verify in a disposable Markdown
file: Ctrl+K, type a comment, Ctrl+S, then read the saved file. Expected text
is `{>>comment<<}`. This exact interaction passed over SSH.

## Reproduce a result

Setup checks establish tools, not scientific results. Use the producer or
verifier that owns the requested result:

- [HKO certificate](experiments/hko-local-maximum/theorem/README.md).
- [Pentagon certificate](experiments/regular-products/pentagon-rotation-formula-proof/README.md);
  run without Python optimization, because its checks use assertions.
- [Data science](experiments/sys-datascience/README.md) and the named method's
  producer; overall data-science scope remains incomplete.
- [Thesis reproduction and archive](docs/reproducibility.md) for the result
  inventory, final PDF and release bundle.

Record the source commit, actual tool versions, command and inputs with a run.
Use the producer's comparison contract: timing-bearing logs and outputs from
different environments are not automatically byte-identical. A fresh setup
does not by itself reproduce every retained result.
