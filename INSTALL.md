# Project toolchain setup

This file owns project installation commands. For entering the existing host
or sandbox, credentials, verified versions and remaining environment gaps, see
[development environments](docs/development-environments.md). Global skills,
documentation and memory distribution belongs to DevOps and is currently paused
pending its design discussion with Jörn.

## Rust, Python and thesis tools

Rust follows `rust-toolchain.toml`, with dependencies locked by `Cargo.lock`.
With rustup installed, run from the repository root:

```bash
rustup show
cargo fetch --locked
cargo test -p euclidean-polytopes --lib
```

Use Python 3.12 and uv for the ordinary Python scripts. Scripts with inline
PEP 723 dependency metadata run with `uv run path/to/script.py`; the script
owns its dependencies. Sage has its own Python environment below.

The Ubuntu package set used by the project's cloud bootstrap is:

```bash
sudo apt-get update
sudo apt-get install --no-install-recommends \
  biber latexmk rclone texlive-bibtex-extra texlive-latex-extra
```

Then `./thesis/check-build.sh` builds the thesis and checks selected PDF/log
conditions. [Artifact setup](docs/artifacts.md) owns private R2 configuration.
Do not run the cloud bootstrap on an existing local environment just to install
these packages: it also configures credentials and checks Cloud-specific inputs.

## SageMath

Use Miniforge/conda-forge, following the
[official Sage installation instructions](https://doc.sagemath.org/html/en/installation/conda.html).
The recipe below adapts the former `container/Dockerfile` at `25d60692^`:
Miniforge 25.11.0-1, Sage 10.9 and Python 3.12. Its installer checksum was
confirmed against the [release checksum](https://github.com/conda-forge/miniforge/releases/download/25.11.0-1/Miniforge3-25.11.0-1-Linux-x86_64.sh.sha256)
on 2026-09-05. These commands target Linux x86-64.

This is a documented installation route, not a completed installation in the
current sandbox. That sandbox had only about 2.2 GB free on its private disk;
check available space before downloading and installing the environment.
Use a fresh installation prefix, or reuse an existing Miniforge installation
and start at the environment-creation command.

```bash
df -h "$HOME"
sage_setup_dir=$(mktemp -d)
sage_prefix="$HOME/.local/share/msc-math/miniforge"
curl -fsSL \
  https://github.com/conda-forge/miniforge/releases/download/25.11.0-1/Miniforge3-25.11.0-1-Linux-x86_64.sh \
  -o "$sage_setup_dir/miniforge.sh"
printf '%s  %s\n' \
  be1bad9d4e67a8753eb76fb4940e9a08036786675c7adf060627e55791bf110d \
  "$sage_setup_dir/miniforge.sh" | sha256sum -c -
# Continue only if the checksum passed.
bash "$sage_setup_dir/miniforge.sh" -b -p "$sage_prefix"
"$sage_prefix/bin/mamba" create -n sage --channel conda-forge \
  --strict-channel-priority sage=10.9 python=3.12
"$sage_prefix/envs/sage/bin/sage" --version
"$sage_prefix/envs/sage/bin/sage" -python -c \
  'from sage.all import QQ, matrix; assert matrix(QQ, [[1,2],[3,4]]).det() == -2; print("Sage arithmetic OK")'
```

For the normal `sage` command, expose only its executable (if that path does not
already exist), rather than adding the entire Conda environment to PATH:

```bash
mkdir -p "$HOME/.local/bin"
ln -s "$sage_prefix/envs/sage/bin/sage" "$HOME/.local/bin/sage"
sage --version
```

`~/.local/bin` is already on PATH in the current sandbox. Keeping Conda's Python,
compilers and shared tools off general PATH preserves ordinary project commands;
the old Dockerfile made the same separation explicitly.

The arithmetic check verifies installation only. The actual certificate commands
and output contracts belong to the [HKO packet](experiments/hko-local-maximum/theorem/README.md)
and [pentagon packet](experiments/regular-products/pentagon-rotation-formula-proof/README.md).
The retained pentagon output records Sage 10.7; a new environment does not certify
that old run or establish compatibility until the relevant verification is run.

### Alternatives and historical recovery

- `apt install sagemath` is not a working route with the current sandbox's apt
  index: it offered no candidate on 2026-09-05. This is environment-specific.
- The former explicit Sage 10.9 lock was for Linux x86-64-v3. It is available
  through `git show b46f3302^:container/locks/sage-10.9-python-3.12-linux-64.explicit`.
  Use it only when reproducing that environment on a compatible CPU. The recipe
  above resolves dependencies for the installing machine; it does not promise
  the old lock's exact package set.
- Source builds and other distributions have not been ruled out. Miniforge is
  the documented default because it is upstream-supported and was used here.

Inspect the former Dockerfile with `git show 25d60692^:container/Dockerfile`.
The earlier Sage 10.7 setup is in `git show 8c2aa080:.devcontainer/Dockerfile`.
