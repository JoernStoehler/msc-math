#!/usr/bin/env python3
"""Build disposable correction-enriched reading views from immutable mirrors.

The source mirrors remain the refresher's authority. This script only writes
corpus/views and can be rerun after every refresh.
"""
import argparse
import json
import os
import re
from pathlib import Path
from urllib.parse import urlsplit, urlunsplit

ROOT = Path(__file__).resolve().parents[1]
CORRECTIONS = ROOT / "references/corrections.md"
MANIFEST = ROOT / "corpus/manifest.jsonl"


def canonical_url(url):
    url = url.rstrip(".,;)")
    parts = urlsplit(url)
    path = parts.path.rstrip("/")
    # Markdown mirrors commonly map to the same hosted article's HTML URL.
    if path.lower().endswith(".md"):
        path = path[:-3]
    return urlunsplit((parts.scheme.lower(), parts.netloc.lower(), path, parts.query, ""))


def read_corrections(path=CORRECTIONS):
    """Return numbered correction entries and their linked source URLs."""
    entries = []
    for line in path.read_text(encoding="utf-8").splitlines():
        match = re.match(r"^(\d+)\.\s+(.*)$", line)
        if not match:
            continue
        body = match.group(2)
        # Pull Markdown link destinations first; exclude their duplicate bare URL
        # matches, then capture remaining bare URLs with Markdown punctuation trimmed.
        markdown_urls = re.findall(r"\]\((https?://[^\s)]+)\)", body)
        bare = re.findall(r"https?://[^\s<>]+", body)
        urls = markdown_urls + [u for u in bare if not any(u.startswith(m) for m in markdown_urls)]
        local_paths = re.findall(r"`(corpus/articles/[^`]+)`", body)
        entries.append({"number": int(match.group(1)), "text": body,
                        "urls": [canonical_url(x) for x in urls], "local_paths": local_paths})
    return entries


def load_rows(path=MANIFEST):
    return [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines() if line.strip()]


def build_views(root=ROOT, manifest=MANIFEST, corrections=CORRECTIONS):
    root = Path(root)
    outroot = root / "corpus/views"
    entries = read_corrections(corrections)
    by_url = {}
    for e in entries:
        for url in e["urls"]:
            by_url.setdefault(url, []).append(e)
        for local_path in e["local_paths"]:
            by_url.setdefault("path:" + local_path, []).append(e)
    written = []
    for row in load_rows(manifest):
        source = root / row["path"]
        if not source.is_file():
            continue
        linked = []
        for source_url in (row["url"], row.get("resolved_url")):
            if source_url:
                linked.extend(by_url.get(canonical_url(source_url), []))
        linked.extend(by_url.get("path:" + row["path"], []))
        linked = list({e["number"]: e for e in linked}.values())
        correction_link = os.path.relpath(root / "references/corrections.md", (outroot / Path(row["path"]).relative_to("corpus/articles")).parent)
        front = [f"# Reading view: {row['id']}", "",
                 "> Generated from the normalized source mirror. Immutable response snapshots are retained under `corpus/snapshots/`; this view is disposable and may be regenerated.", "",
                 f"- Source ID: `{row['id']}`", f"- Source URL: {row['url']}",
                 *([f"- Resolved URL: {row['resolved_url']}"] if row.get("resolved_url") else []),
                 f"- Correction links: {len(linked)}", ""]
        if linked:
            front += ["## Curated corrections relevant to this source", ""]
            for e in linked:
                front += [f"### Correction {e['number']}", "", e["text"], "",
                          f"[Open curated correction reference]({correction_link})", ""]
        else:
            front += ["## Curated corrections relevant to this source", "", "No direct source-URL match in the current correction register.", ""]
        front += ["## Source mirror", "", f"<!-- source-id:{row['id']} -->", "", source.read_text(encoding="utf-8").rstrip(), ""]
        target = outroot / Path(row["path"]).relative_to("corpus/articles")
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text("\n".join(front), encoding="utf-8")
        written.append(target)
    # Remove stale generated views when a source is removed from the manifest.
    expected = set(written)
    if outroot.exists():
        for stale in outroot.rglob("*.md"):
            if stale not in expected:
                stale.unlink()
    return written


def evaluate(root=ROOT, manifest=MANIFEST, corrections=CORRECTIONS):
    """Deterministic factual-term presence proxy; deliberately not a model evaluation."""
    root = Path(root)
    rows = load_rows(manifest)
    entries = read_corrections(corrections)
    refs = "\n".join(p.read_text(encoding="utf-8", errors="ignore") for p in (root / "references").glob("*.md"))
    corpus_paths = list((root / "corpus/articles").rglob("*.md"))
    view_paths = list((root / "corpus/views").rglob("*.md"))
    def blob(paths): return "\n".join(p.read_text(encoding="utf-8", errors="ignore") for p in paths).lower()
    source_blob, view_blob = blob(corpus_paths), blob(view_paths)
    # Fixed topical phrases, independent of source IDs and URLs. A models all
    # material consolidated beneath references; B keeps article mirrors separate;
    # C indexes generated views. The same mirror bodies occur in all three.
    queries = ["project instructions", "custom gpts", "agents.md", "cowork", "skills", "custom instructions"]
    a, b, c = refs.lower() + source_blob, refs.lower() + source_blob, refs.lower() + view_blob
    available = {r["id"]: r for r in rows if (root / r["path"]).is_file()}
    source_queries = list(available.items())
    correction_queries = entries
    def report(text):
        hits = [q for q in queries if q in text]
        return {"factual_term_queries_hit": hits, "factual_term_query_count": len(hits), "factual_term_queries_total": len(queries)}
    # Contextual visibility is measured in the retrieved source-specific file,
    # rather than the aggregate corpus where corrections are trivially present.
    linked_pairs = []
    for e in entries:
        for r in rows:
            urls = {canonical_url(r.get("resolved_url") or r["url"]), canonical_url(r["url"])}
            if ((urls & set(e["urls"])) or r["path"] in e["local_paths"]) and (root / r["path"]).is_file():
                linked_pairs.append((r, e))
    def contextual(paths_by_id):
        matched = 0
        for r, e in linked_pairs:
            p = paths_by_id.get(r["id"])
            if p and e["text"].lower() in p.read_text(encoding="utf-8", errors="ignore").lower():
                matched += 1
        return {"linked_source_correction_pairs_visible": matched, "linked_source_correction_pairs_total": len(linked_pairs)}
    articles_by_id = {r["id"]: root / r["path"] for r in available.values()}
    views_by_id = {r["id"]: root / "corpus/views" / Path(r["path"]).relative_to("corpus/articles") for r in available.values()}
    return {"method": "deterministic presence of fixed topical phrases in indexed text; this is not an LLM/model retrieval evaluation. A and B contain identical source bodies under different layouts; C wraps those same bodies in generated views.",
            "systems": {"A_all_material_consolidated_under_references": report(a), "B_corpus_plus_references": report(b),
                        "C_generated_views_plus_references": report(c)},
            "source_context_correction_visibility": {"A_references_only": {"linked_source_correction_pairs_visible": 0, "linked_source_correction_pairs_total": len(linked_pairs)},
                                                       "B_articles_plus_references": contextual(articles_by_id),
                                                       "C_generated_views_plus_references": contextual(views_by_id)}}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--evaluate", action="store_true", help="print deterministic retrieval comparison as JSON")
    args = parser.parse_args()
    build_views()
    if args.evaluate:
        print(json.dumps(evaluate(), indent=2, sort_keys=True))
    else:
        print(f"Built {len(list((ROOT/'corpus/views').rglob('*.md')))} reading views")


if __name__ == "__main__":
    main()
