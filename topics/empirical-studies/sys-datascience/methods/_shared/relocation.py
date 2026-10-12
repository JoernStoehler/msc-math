"""Resolve frozen repository path labels without rewriting their identity bytes."""

from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[5]
MOVED_PACKETS = ("sys-datascience", "sys-landscape", "polytope-datasets", "polytope-invariant-table")


def resolve_repo_path(label: str | Path, root: Path = REPO_ROOT) -> Path:
    path = Path(label)
    if path.is_absolute():
        try:
            path = path.relative_to(root)
        except ValueError:
            return path
    for name in MOVED_PACKETS:
        old = Path("experiments") / name
        if path == old or old in path.parents:
            return root / "topics/empirical-studies" / name / path.relative_to(old)
    return root / path
