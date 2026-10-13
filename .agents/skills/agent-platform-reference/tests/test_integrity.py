"""Integrity and documented-contract checks; these do not probe live products."""
import hashlib
import json
from pathlib import Path
import sys

import pytest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
import refresh
from validate import check


class DummyResponse:
    def __init__(self, body="", status=200, content_type="text/html", url="https://help.openai.com/en/articles/1", headers=None):
        self.body = body.encode("utf-8")
        self.status_code = status
        self.url = url
        self.headers = {"Content-Type": content_type, **(headers or {})}

    def iter_content(self, chunk_size=65536):
        yield self.body

    def raise_for_status(self):
        if self.status_code >= 400:
            raise RuntimeError(f"HTTP {self.status_code}")

    def close(self):
        pass


class DummySession:
    def __init__(self, response):
        self.response = response
        self.calls = []

    def get(self, url, **kwargs):
        self.calls.append(url)
        return self.response


def source_record(path="corpus/articles/openai/chatgpt/test-source.md"):
    return {
        "id": "test-source", "url": "https://help.openai.com/en/articles/1",
        "format": "html", "path": path, "status": "pending", "etag": None,
        "last_modified": None, "sha256": None, "bytes": None,
    }


def test_raw_snapshot_hash_corruption_is_reported(tmp_path):
    # Minimal complete package: validator should independently verify both the
    # normalized mirror and its retained raw response snapshot.
    (tmp_path / "SKILL.md").write_text("---\nname: agent-platform-reference\ndescription: valid\n---\n" + "x" * 120)
    refs = tmp_path / "references"
    refs.mkdir()
    (refs / "base.md").write_text("# Reference\n" + "x" * 120)
    raw = b"raw upstream response payload"
    raw_hash = hashlib.sha256(raw).hexdigest()
    raw_path = tmp_path / "corpus/snapshots" / raw_hash
    raw_path.parent.mkdir(parents=True)
    raw_path.write_bytes(raw)
    mirror = tmp_path / "corpus/articles/openai/chatgpt/source.md"
    mirror.parent.mkdir(parents=True)
    mirror.write_text("# Source\n" + "verified normalized content " * 5)
    row = source_record("corpus/articles/openai/chatgpt/source.md")
    row.update(status="ok", sha256=hashlib.sha256(mirror.read_bytes()).hexdigest(),
               raw_path=str(raw_path.relative_to(tmp_path)), raw_sha256=raw_hash,
               raw_bytes=len(raw))
    manifest = tmp_path / "corpus/manifest.jsonl"
    manifest.write_text(json.dumps(row) + "\n")
    assert check(tmp_path) == []
    raw_path.write_bytes(b"tampered response payload")
    errors = check(tmp_path)
    assert any("raw" in error.lower() and "hash" in error.lower() for error in errors), errors


def test_successful_changed_fetch_preserves_curated_correction_bytes(tmp_path, monkeypatch):
    monkeypatch.setattr(refresh, "ROOT", tmp_path)
    rec = source_record()
    target = tmp_path / rec["path"]
    target.parent.mkdir(parents=True)
    target.write_text("# Old mirror\n" + "old documented content " * 8)
    rec.update(status="ok", sha256=hashlib.sha256(target.read_bytes()).hexdigest())
    corrections = tmp_path / "references/corrections.md"
    corrections.parent.mkdir(parents=True)
    curated = b"# Curated corrections\r\nKeep this hand-edited claim exactly.\r\n"
    corrections.write_bytes(curated)
    html = "<main><h1>Changed</h1><p>" + "new upstream factual text " * 10 + "</p></main>"
    session = DummySession(DummyResponse(html))
    updated, outcome = refresh.fetch_one(rec, session)
    assert outcome == "changed" and updated["status"] == "ok"
    assert "Changed" in target.read_text()
    assert corrections.read_bytes() == curated


def test_html_conversion_preserves_table_code_fence_and_absolute_link():
    html = """<html><body><main><h1>Reference</h1>
    <p>""" + "Useful factual context. " * 8 + """</p>
    <table><tr><th>Key</th><th>Value</th></tr><tr><td>A</td><td>B</td></tr></table>
    <pre><code class="language-python">def run():\n    return 1</code></pre>
    <a href="/docs/next">Next</a></main></body></html>"""
    text = refresh.to_markdown(html, "html", "https://help.openai.com/en/articles/current")
    assert "| Key | Value |" in text and "| A | B |" in text
    assert "```\ndef run():\n    return 1\n```" in text
    assert "[Next](https://help.openai.com/docs/next)" in text


def test_corrupt_local_copy_rejects_304_and_marks_refresh_error(tmp_path, monkeypatch):
    monkeypatch.setattr(refresh, "ROOT", tmp_path)
    rec = source_record()
    target = tmp_path / rec["path"]
    target.parent.mkdir(parents=True)
    target.write_text("# Expected mirror\n" + "valid content " * 10)
    rec.update(status="ok", sha256=hashlib.sha256(target.read_bytes()).hexdigest())
    target.write_text("# Tampered mirror\n" + "corrupt content " * 10)
    response = DummyResponse("", status=304)
    updated, outcome = refresh.fetch_one(rec, DummySession(response))
    assert outcome == "error"
    assert updated["status"] == "error" and updated["refresh_status"] == "error"
    assert "corrupt local copy" in updated["error"]


def test_unsafe_redirect_host_is_rejected_before_second_request():
    response = DummyResponse(status=302, headers={"Location": "https://example.evil.invalid/steal"})
    session = DummySession(response)
    with pytest.raises(ValueError, match="not allowlisted"):
        refresh.get_text("https://help.openai.com/en/articles/1", session=session)
    assert session.calls == ["https://help.openai.com/en/articles/1"]


def test_local_source_documents_project_precedence_and_no_settings_api():
    # Assertions pin documented source statements only; they do not establish
    # live ChatGPT behavior or API capability beyond the cited documentation.
    projects = (ROOT / "corpus/articles/openai/chatgpt/chatgpt-projects.md").read_text().lower()
    custom_instructions = (ROOT / "corpus/articles/openai/chatgpt/chatgpt-custom-instructions.md").read_text().lower()
    assert "project instructions apply only within that project and override your global custom instructions" in projects
    assert "there will be no api for custom instructions" in custom_instructions

