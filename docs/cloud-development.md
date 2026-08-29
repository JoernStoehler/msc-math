# Codex Cloud Development

Codex Cloud checks out the selected Git commit and then runs the setup script
configured for its environment. Select Python `3.12` and Rust `1.94.0` through
the environment's supported package-version settings, then configure the
`msc-math` cloud environment to run this repository setup command:

```bash
scripts/bootstrap-cloud.sh
```

Configure its maintenance command separately:

```bash
scripts/maintain-cloud.sh
```

The universal image supplies Python, `uv`, Rust, and ordinary build tools. The
setup script verifies the selected runtimes, installs only the normal LaTeX and
artifact dependencies missing from that image, configures R2, and fetches the
locked Cargo dependency graph. The maintenance script refreshes that
repository-dependent Cargo cache after a cached environment checks out a newer
revision; it does not need setup-only secrets or reinstall stable system
packages.

Python scripts keep their dependencies in PEP 723 metadata and are resolved by
`uv` when run. The cloud setup deliberately does not launch Docker or an
expensive scientific producer.

Add these two values as Codex Cloud **secrets**:

```text
MSC_MATH_R2_ACCESS_KEY_ID
MSC_MATH_R2_SECRET_ACCESS_KEY
```

Codex Cloud secrets are available during setup but removed before the agent
phase. The setup script therefore creates a private mode-`0600` rclone config
inside the ephemeral cloud container and verifies access to the artifact
prefix. It never writes credentials into the repository. See the
[official cloud environment documentation](https://learn.chatgpt.com/docs/environments/cloud-environment)
for the setup, caching, environment-variable, and secret lifecycle.

After setup, ordinary source work needs no external data. Materialize a named
dataset only when a task consumes it:

```bash
scripts/artifacts.py list
scripts/artifacts.py materialize polytope-datasets
```

The local data and cache layout across development environments is not yet
settled. Until it is, do not infer a shared mount or persistence guarantee from
an environment-local path.
