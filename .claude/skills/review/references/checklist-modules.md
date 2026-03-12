# Review Checklist: Module Sanity (Phase 0)

Run BEFORE phase 1/2 reviews. If builds or tests are broken, style and content reviews are wasted work.

## 1. Library Builds and Passes Tests

```bash
cd crates/ && cargo build 2>&1
cd crates/ && cargo clippy --lib -- -D warnings 2>&1
timeout 5m cargo test --lib 2>&1 | tee /tmp/test-output.txt
```
Report: compilation errors, clippy warnings, test failures.

## 2. Experiments Build

```bash
cd experiments/ && cargo build --release 2>&1
```
Report: compilation errors. Note which binaries failed.

## 3. Experiment Directory Structure

For each experiment directory in `experiments/<name>/`:
- Has `<name>.rs` (Rust binary source)?
- Has entry in `experiments/Cargo.toml` as `[[bin]]`?
- Has `README.md`?
- If has `.py` script: has matching `.png` output committed?
- If has `.rs` binary: has matching `.jsonl` data committed?

## 4. reproduce.sh Consistency

Read `experiments/reproduce.sh`. For each step:
- Does the referenced binary/script exist?
- Does the referenced data file exist?
- Are there experiments NOT mentioned in reproduce.sh?

## 5. Data Freshness

For experiments with committed `.jsonl` data:
```bash
git log -1 --format=%H -- experiments/<name>/<name>.rs
git log -1 --format=%H -- experiments/<name>/<name>.jsonl
```
If the binary was modified more recently than the data, flag as potentially stale.

## 6. Run Experiment Binaries

Run each binary with `timeout 3m`. Report PASS/CRASH/TIMEOUT for each. For crashes, include the last 5 lines of stderr.

## 7. Commit Checklist

- [ ] All tests pass (`cargo test`)
- [ ] Zero clippy warnings (`cargo clippy`)
- [ ] Working tree clean (no uncommitted changes relevant to the branch)
