# Environment setup and reproduction

Start here for host setup and reproduction. Run project commands from the
repository root. As of 2026-09-19, ordinary work resumes from this host
checkout. The retained `codex-msc-math` sandbox is stopped while its private
state is wound down; its Codex rollouts have been imported into the host
session store. Do not start or recreate it merely to resume project work.
The sandbox references below are retained for historical recovery, not as the
current setup path. Host-side sandbox lifecycle and
credentials belong to the host's `~/.dotfiles/memories/host-estate.md`
(`/home/joern/.dotfiles/memories/host-estate.md`).

## Minimal workflow

A checkout contains the complete selected thesis source and its figure inputs.
It does not need Sage or external datasets to build the PDF:

```bash
sh thesis/build.sh
bash thesis/check-build.sh
```

The PDF is `thesis/build/main.pdf`. The second command forces a rebuild and
checks the PDF signature, overfull boxes and undefined references; it does not
validate the mathematics or writing. See the LaTeX dependencies below.

For reusable Rust code, use the pinned toolchain and lockfile:

```bash
cargo fmt --all -- --check
cargo check --workspace --locked
```

These commands cover the root workspace. Focused runtime checks and their
scientific scope are mapped in [docs/algorithm-testing.md](docs/algorithm-testing.md).
Standalone experiment method crates have their own manifests and commands.

[Environment details](docs/development-environments.md) records current
workstation/cloud roles and dated setup observations. Host credentials,
editors, browser transport and agent sessions are not prerequisites for the
PDF or Rust build. The old sandbox shell/editor configuration remains in Git
history; do not apply it to a workstation installation.

## Ordinary language tools

Rust's version and components are in `rust-toolchain.toml`; Cargo dependencies
are in the lockfiles. No special Rust installation method is needed. Ensure
`~/.cargo/bin` is on PATH, including in noninteractive SSH shells.

Use Python 3.12 and uv for ordinary scripts. Run scripts with PEP 723 inline
dependency metadata as `uv run path/to/script.py`; dependencies belong to the
script. Keep Sage's Python separate from this ordinary environment.

## LaTeX and Biber

On Ubuntu, the thesis build needs the following package set. It is also used
by `scripts/bootstrap-cloud.sh`; that cloud bootstrap additionally requires
its private R2 credentials and is not a prerequisite for a local PDF build:

```bash
sudo apt-get update
sudo apt-get install --no-install-recommends \
  biber latexmk lmodern texlive-bibtex-extra texlive-latex-extra
bash thesis/check-build.sh
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
sh thesis/build.sh -g -outdir="$thesis_check_dir" -auxdir="$thesis_check_dir"
```

This last command checks compilation only; inspect the generated log when
the selected layout/reference checks are also required.

## SageMath

Sage is needed for the original HKO and pentagon certificate verifiers, not
for building the thesis or running the Rust crates. Install a separate Sage
Python environment using the [Sage installation instructions](https://doc.sagemath.org/html/en/installation/).
The project previously verified Sage 10.9; record the version actually used.
Do not replace ordinary Python or put Sage's libraries into the ordinary uv
environment. The historical Conda solve was not pinned; a fresh solve may
select different versions and still needs the packet's compatibility checks.

For a Conda environment named `sage`, a basic arithmetic check is:

```bash
conda run --name sage python -c \
  'from sage.all import QQ, matrix; assert matrix(QQ, [[1,2],[3,4]]).det() == -2; print("Sage arithmetic OK")'
```

The packet READMEs use `conda run --name sage python`. If Sage is installed
elsewhere, substitute the Python executable from that environment. The
previously tested Conda launcher did not support `sage -python`; calling its
Python directly avoids relying on that option. A basic arithmetic check is
only an installation check. The theorem packets own their complete checks,
input/output paths, and trust boundaries.

The former sandbox used an absolute-prefix installation under
`/workspaces/msc-math/.local-environments/miniforge/envs/sage/`.
Its existence is not a portable installation promise. Historical provisioning
and troubleshooting commands are preserved in local Git history at commit
`52b1526c8ff5613a400f2f4028b4290f35841b21`
(`git show 52b1526c:INSTALL.md`). Do not start a retired sandbox
or overwrite an existing environment merely to run a verifier.

## R2 data

Install `rclone` (on Ubuntu: `sudo apt-get install rclone`). Configure its
`mscmath` S3 remote for Cloudflare R2, bucket `msc-math-artifacts`, using
private credentials. Reading snapshots needs object-read access; publishing
also needs write access. These credentials are not distributed with the
repository, so an unauthenticated clone cannot download the registered data.

Use `rclone config`, or supply these variables privately to the process:

```text
RCLONE_CONFIG_MSCMATH_TYPE=s3
RCLONE_CONFIG_MSCMATH_PROVIDER=Cloudflare
RCLONE_CONFIG_MSCMATH_ACCESS_KEY_ID=<R2 access key id>
RCLONE_CONFIG_MSCMATH_SECRET_ACCESS_KEY=<R2 secret access key>
RCLONE_CONFIG_MSCMATH_ENDPOINT=https://ef19d5c4c89e0b61a5a1560041679e2d.r2.cloudflarestorage.com
```

Keep credentials out of Git. If storing them in
`~/.config/rclone/rclone.conf`, restrict that file to mode `0600`.
The helper supplies R2's empty ACL and skips
account-level bucket checks. Discover and download only the artifact needed:

```bash
python3 scripts/artifacts.py list
python3 scripts/artifacts.py materialize combinatorial-cell-widths --no-link
```

The second command downloads and verifies one registered snapshot. It contacts
R2 even when a local cache exists. Omit `--no-link` when the producer needs
the repository links. Links point into an environment-specific cache and may
not resolve in a different execution environment.

[Artifact contracts](docs/artifacts.md) explain cache placement, link handling
and publication. For Codex Cloud use the configured setup/maintenance scripts
described in [environment details](docs/development-environments.md), not the
ephemeral-variable recipe after Cloud setup has finished.

## Reproduce a result

Setup checks establish tools, not scientific results. Use the producer or
verifier that owns the requested result:

- [HKO certificate](experiments/hko-local-maximum/theorem/README.md) (Sage).
- [Second ten-facet local maximum](experiments/hko-local-maximum/non-hko/README.md)
  (ordinary Python/SymPy; includes an independent checker).
- [Pentagon certificate](experiments/regular-products/pentagon-rotation-formula-proof/README.md);
  run without Python optimization, because its checks use assertions.
- [Data science](experiments/sys-datascience/README.md) and the named method's
  producer. Historical ratios, current capacity reevaluation and the one
  unresolved geometry have distinct evidence contracts.
- [Thesis reproduction and archive](docs/reproducibility.md) for the result
  inventory, final PDF and release bundle.

Record the source commit, actual tool versions, command and inputs with a run.
Use the producer's comparison contract: timing-bearing logs and outputs from
different environments are not automatically byte-identical. A fresh setup
does not by itself reproduce every retained result.
