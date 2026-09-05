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
- **Sage is not installed in the current sandbox.** Its recommended route
  below has not yet been tested there.

[Environment details](docs/development-environments.md) records versions,
configuration locations and diagnostic findings. Global skill/docs/memory
distribution is owned by DevOps and paused pending its design discussion with
Jörn; do not independently install another global system from this project.

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

**Recommended:** install Sage in a separate Miniforge/conda-forge environment,
following the [official Sage instructions](https://doc.sagemath.org/html/en/installation/conda.html).
This is upstream-supported; the project also has retained successful
Sage computations (the pentagon certificate records Sage 10.7). Neither fact
establishes that a fresh solve of the commands below works in today's sandbox.

On Linux x86-64, if Miniforge is not already installed:

```bash
df -h "$HOME"
sage_setup_dir=$(mktemp -d)
sage_prefix="$HOME/.local/share/msc-math/miniforge"
curl -fsSL \
  https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-Linux-x86_64.sh \
  -o "$sage_setup_dir/miniforge.sh"
bash "$sage_setup_dir/miniforge.sh" -b -p "$sage_prefix"
```

Use a fresh prefix, or set `sage_prefix` to an existing Miniforge installation
and skip its installer. Then create the environment and check it:

```bash
"$sage_prefix/bin/mamba" create -n sage --channel conda-forge \
  --strict-channel-priority sage
"$sage_prefix/envs/sage/bin/sage" --version
"$sage_prefix/envs/sage/bin/sage" -python -c \
  'from sage.all import QQ, matrix; assert matrix(QQ, [[1,2],[3,4]]).det() == -2; print("Sage arithmetic OK")'
mkdir -p "$HOME/.local/bin"
ln -s "$sage_prefix/envs/sage/bin/sage" "$HOME/.local/bin/sage"
sage --version
```

The symlink command deliberately refuses to overwrite an existing `sage`.
Ensure `~/.local/bin` is on PATH. Expose only `sage`, rather than the entire
Conda environment, so ordinary Python and compiler commands keep their meaning.

Record the installed Sage version with each verification run. The arithmetic
check establishes basic operation; run the relevant certificate to check
project compatibility. No current Sage version pin has been established for
this new environment.

**Does not work here:** the sandbox's apt index offered no `sagemath` candidate
on 2026-09-05. **Not evaluated:** source builds and other distributions; they
have not been ruled out. The sandbox had about 2.2 GB free on its private disk,
so check space before this large installation. An environment installation
has not been attempted there yet.

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
