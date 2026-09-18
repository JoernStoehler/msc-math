# Frozen baseline recovery evidence

This packet records the recovery of the rejected 86-page baseline. It is not the current build route. Build the selected manuscript with `sh thesis/build.sh`.

The original recovery programs are Git-only because their root-relative paths refer to the original layout. Inspect or replay them in a disposable export of **`3b180fb2d27f0ecad44008e607ab835691730573`**:

```sh
replay_dir=$(mktemp -d)
git archive 3b180fb2d27f0ecad44008e607ab835691730573 | tar -x -C "$replay_dir"
(cd "$replay_dir" && python3 docs/source-recovery/verify.py)
# Remove the disposable export after inspecting its report.
```

The export command and input closure were checked during consolidation: all 59 manifest input hashes match that commit, and the three programs parse. The expensive 86-page build/render comparison was **not rerun**. Its original report remains [verification.json](verification.json). Replay needs the TeX/Biber, bubblewrap and Poppler dependencies documented in the original README at that commit.

`recover.py` and `preserve_review_evidence.py` are one-time migration programs, not regeneration commands. They additionally depend on old `.git/codex/` inputs; a Git export alone does not supply those. Their historical availability does not establish that they can now run. Use retained recovered artifacts instead.

[dependency-manifest.json](dependency-manifest.json) records exact inputs and path substitutions; `evidence/` retains original build records/frozen PDF and `support/` retains generator/provenance material. [Research dependencies](research-dependencies.md) records research inputs outside PDF closure. Current missing-cache limitations are in [external dependencies](../../resume/external-dependencies.md).
