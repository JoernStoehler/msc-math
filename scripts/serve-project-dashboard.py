#!/usr/bin/env python3
"""Serve the project dashboards and their cited sources from a fixed local URL.

From the repository root, run: python3 scripts/serve-project-dashboard.py
Then open http://127.0.0.1:8765/docs/project-dashboard.html.
Use --host and --port to change the listening address and port.
"""

import argparse
import hashlib
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlsplit


REPO = Path(__file__).resolve().parent.parent
# Manual review baseline: refresh these hashes when reconciling the dashboard to changed sources.
SOURCE_BASELINE_SHA256 = {
    "RESUME.md": "6f35ab53c2c6c437bf1c89754d6b2a7953115665ca4b8100409cc0b52c2ea231",
    "docs/project-facts.md": "ac9f2b73f9183ca1fef29a1d7672c524700e49e68e74896ed87e12eba5334033",
    "docs/WORK_REMAINING.md": "e6bcc2f9cab88b2381c276c9249df70d1be6916d7998260507af71fc1a1deff8",
    "docs/consolidation/2026-09-24/claim-disposition.md": "3e048444faff47fed46cab8b266676eb4100cfed2be95c90e755cc7334b765a9",
    "docs/history/memories/planning-context.md": "306fff369f7e2eea4008ce888ed2c3a8484d8f94362105c994915717361cbcef",
    "thesis/main.tex": "fede5525da8703d64ad391c3862b40e8a95ebb4781a8d53d9321639d93525225",
    "thesis/chapters/07-hko.tex": "5deb679f1f5410e04169dc302ce1870789daf80b075d9460bd02b8679cd6d08d",
    "experiments/hko-local-maximum/non-hko/README.md": "3bde2732bb108117b3580b8b913506a651dc6c24aa38a7ea0617f99f968b5aae",
    "experiments/hko-local-maximum/non-hko/PROOF.md": "51a8dab531e4dbc87149c979ee2d1c7a1b6b1448cbda9a49fbe46dc02713a8b3",
}
ALLOWED = {
    "/docs/project-dashboard.html": ("docs/project-dashboard.html", "text/html; charset=utf-8"),
    "/docs/project-now.html": ("docs/project-now.html", "text/html; charset=utf-8"),
    "/RESUME.md": ("RESUME.md", "text/plain; charset=utf-8"),
    "/docs/project-facts.md": ("docs/project-facts.md", "text/plain; charset=utf-8"),
    "/docs/WORK_REMAINING.md": ("docs/WORK_REMAINING.md", "text/plain; charset=utf-8"),
    "/docs/consolidation/2026-09-24/claim-disposition.md": (
        "docs/consolidation/2026-09-24/claim-disposition.md", "text/plain; charset=utf-8"
    ),
    "/docs/history/memories/planning-context.md": (
        "docs/history/memories/planning-context.md", "text/plain; charset=utf-8"
    ),
    "/thesis/main.tex": ("thesis/main.tex", "text/plain; charset=utf-8"),
    "/thesis/chapters/07-hko.tex": ("thesis/chapters/07-hko.tex", "text/plain; charset=utf-8"),
    "/experiments/hko-local-maximum/non-hko/README.md": (
        "experiments/hko-local-maximum/non-hko/README.md", "text/plain; charset=utf-8"
    ),
    "/experiments/hko-local-maximum/non-hko/PROOF.md": (
        "experiments/hko-local-maximum/non-hko/PROOF.md", "text/plain; charset=utf-8"
    ),
}


class DashboardHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self._serve(send_body=True)

    def do_HEAD(self):
        self._serve(send_body=False)

    def _serve(self, send_body):
        request_path = urlsplit(self.path).path
        if request_path == "/_dashboard/source-status":
            self._source_status(send_body)
            return
        item = ALLOWED.get(request_path)
        if item is None:
            self._error(404, "Not found", send_body)
            return

        relative_path, content_type = item
        path = REPO / relative_path
        # Keep the allowlist bounded to repository files even if one is replaced by a symlink.
        if not path.resolve().is_relative_to(REPO) or not path.is_file():
            self._error(404, "Not found", send_body)
            return
        try:
            body = path.read_bytes()  # Fresh copy on every request.
        except OSError:
            self._error(404, "Not found", send_body)
            return

        self.send_response(200)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        if send_body:
            self.wfile.write(body)

    def _source_status(self, send_body):
        try:
            changed = []
            for relative_path, expected_digest in SOURCE_BASELINE_SHA256.items():
                path = REPO / relative_path
                if not path.resolve().is_relative_to(REPO) or not path.is_file():
                    raise OSError(f"Cannot read cited source: {relative_path}")
                digest = hashlib.sha256(path.read_bytes()).hexdigest()
                if digest != expected_digest:
                    changed.append(relative_path)
            body = json.dumps({"known": True, "changed": changed}).encode()
        except OSError:
            body = json.dumps({"known": False}).encode()

        self.send_response(200)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        if send_body:
            self.wfile.write(body)

    def _error(self, status, message, send_body=True):
        body = f"{message}\n".encode()
        self.send_response(status)
        self.send_header("Content-Type", "text/plain; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        if send_body:
            self.wfile.write(body)

    def do_POST(self):
        self._error(405, "Method not allowed")

    do_PUT = do_DELETE = do_PATCH = do_POST


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--host", default="127.0.0.1", help="listen address (default: 127.0.0.1)")
    parser.add_argument("--port", type=int, default=8765, help="listen port (default: 8765)")
    args = parser.parse_args()
    with ThreadingHTTPServer((args.host, args.port), DashboardHandler) as server:
        print(
            f"Serving dashboard at http://{args.host}:{server.server_port}/docs/project-dashboard.html",
            flush=True,
        )
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            pass


if __name__ == "__main__":
    main()
