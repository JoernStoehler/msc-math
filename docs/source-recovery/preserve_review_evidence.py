#!/usr/bin/env python3
"""Copy the bounded review/feedback record unchanged, with a checksum map."""
import hashlib
import json
import shutil
from pathlib import Path

root = Path(__file__).resolve().parents[2]
destination = root / "docs/review-evidence"
pairs = []
calibration = root / ".git/codex/review-calibration"
for source in sorted(calibration.rglob("*")):
    if source.is_file():
        relative = source.relative_to(calibration)
        if relative.parts[0] == "pro-review-2026-09-15":
            target = destination / relative
        else:
            target = destination / "calibration" / relative
        pairs.append((source, target))
blind = root / ".git/codex/blind-self-review-2026-09-15"
for source in sorted(blind.iterdir()):
    if source.is_file():
        pairs.append((source, destination / "blind-self-review-2026-09-15" / source.name))
pairs += [
    (root / ".git/codex/ds-first-wave/writing/jorn-reading-feedback-2026-09-14.md",
     destination / "human-feedback/ds-reading-2026-09-14.md"),
    (root / ".git/codex/thesis-review-1800/README.md",
     destination / "frozen-1800-context.md"),
]
records = []
for source, target in pairs:
    target.parent.mkdir(parents=True, exist_ok=True)
    if target.exists() and target.read_bytes() != source.read_bytes():
        raise RuntimeError(f"Refusing to overwrite changed evidence: {target}")
    shutil.copy2(source, target)
    digest = hashlib.sha256(source.read_bytes()).hexdigest()
    assert hashlib.sha256(target.read_bytes()).hexdigest() == digest
    records.append({"old": source.relative_to(root).as_posix(),
                    "new": target.relative_to(root).as_posix(),
                    "sha256": digest, "bytes": source.stat().st_size})
(destination / "manifest.json").write_text(json.dumps(records, indent=2) + "\n")
print(json.dumps({"files": len(records), "bytes": sum(r["bytes"] for r in records)}))
