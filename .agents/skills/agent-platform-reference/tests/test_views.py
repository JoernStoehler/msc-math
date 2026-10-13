import importlib.util
import json
from pathlib import Path
import re

import pytest

SCRIPT = Path(__file__).resolve().parents[1] / "scripts/build_views.py"
spec = importlib.util.spec_from_file_location("build_views", SCRIPT)
build_views = importlib.util.module_from_spec(spec)
spec.loader.exec_module(build_views)


def fixture(tmp_path):
    mirror = tmp_path / "corpus/articles/vendor/page.md"
    mirror.parent.mkdir(parents=True)
    mirror.write_text("# Original\n\n| Name | Value |\n|---|---|\n| A | B |\n\n[Related](https://example.test/related)\n\n```python\nprint('ok')\n```\n")
    manifest = tmp_path / "corpus/manifest.jsonl"
    manifest.parent.mkdir(parents=True, exist_ok=True)
    manifest.write_text(json.dumps({"id": "vendor-page", "url": "https://example.test/page", "path": "corpus/articles/vendor/page.md"}) + "\n")
    corrections = tmp_path / "references/corrections.md"
    corrections.parent.mkdir(parents=True)
    corrections.write_text("# Corrections\n\n1. **Correction:** Avoid this claim. Source: https://example.test/page\n")
    return mirror, manifest, corrections


def test_changed_mirror_regenerates_view_and_preserves_correction(tmp_path):
    mirror, manifest, corrections = fixture(tmp_path)
    build_views.build_views(tmp_path, manifest, corrections)
    view = tmp_path / "corpus/views/vendor/page.md"
    first = view.read_text()
    assert "Avoid this claim" in first
    mirror.write_text("# Updated source\n\nChanged upstream text.\n")
    build_views.build_views(tmp_path, manifest, corrections)
    second = view.read_text()
    assert "Updated source" in second and "Changed upstream text" in second
    assert "Avoid this claim" in second
    assert "Immutable response snapshots" in second


def test_no_correction_match_still_generates_mirror_view(tmp_path):
    mirror, manifest, corrections = fixture(tmp_path)
    corrections.write_text("# Corrections\n\n1. Claim without a linked source.\n")
    build_views.build_views(tmp_path, manifest, corrections)
    view = (tmp_path / "corpus/views/vendor/page.md").read_text()
    assert "No direct source-URL match" in view
    assert "| Name | Value |" in view
    assert "[Related](https://example.test/related)" in view
    assert "```python\nprint('ok')\n```" in view


def test_canonical_url_strips_fragment_and_trailing_punctuation():
    assert build_views.canonical_url("https://Example.test/path/#section,") == "https://example.test/path"
    assert build_views.canonical_url("https://Example.test/path.md") == "https://example.test/path"


def test_evaluation_is_deterministic_and_discloses_proxy(tmp_path):
    mirror, manifest, corrections = fixture(tmp_path)
    build_views.build_views(tmp_path, manifest, corrections)
    result = build_views.evaluate(tmp_path, manifest, corrections)
    assert "not an LLM/model" in result["method"]
    counts = [v["factual_term_query_count"] for v in result["systems"].values()]
    assert len(set(counts)) == 1  # same source body, therefore equal lexical coverage
    assert result["source_context_correction_visibility"]["C_generated_views_plus_references"]["linked_source_correction_pairs_visible"] == 1


def test_actual_correction_register_matches_and_generated_links_resolve():
    repo = Path(__file__).resolve().parents[1]
    entries = build_views.read_corrections(repo / "references/corrections.md")
    assert len(entries) >= 10
    # Match corrections using either their primary/resolved source URL or the
    # local mirror paths explicitly curated in the register.
    rows = build_views.load_rows(repo / "corpus/manifest.jsonl")
    pairs = []
    for entry in entries:
        for row in rows:
            urls = {build_views.canonical_url(row.get("resolved_url") or row["url"]), build_views.canonical_url(row["url"])}
            if urls.intersection(entry["urls"]) or row["path"] in entry["local_paths"]:
                pairs.append((row, entry))
    assert len(pairs) > 0
    build_views.build_views(repo)
    views = list((repo / "corpus/views").rglob("*.md"))
    linked_views = 0
    for view in views:
        content = view.read_text(encoding="utf-8")
        if "[Open curated correction reference]" not in content:
            continue
        linked_views += 1
        rel = re.search(r"\[Open curated correction reference\]\(([^)]+)\)", content).group(1)
        assert (view.parent / rel).resolve().is_file(), (view, rel)
    assert linked_views > 0

