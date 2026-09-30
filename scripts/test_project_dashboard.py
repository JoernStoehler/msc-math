"""HTTP regression checks for the shared-state browser surface."""

import importlib.util
import json
import shutil
import tempfile
import threading
import unittest
from http.server import ThreadingHTTPServer
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen

spec = importlib.util.spec_from_file_location(
    "dashboard", Path(__file__).with_name("serve-project-dashboard.py")
)
dashboard = importlib.util.module_from_spec(spec)
spec.loader.exec_module(dashboard)


class QuietHandler(dashboard.DashboardHandler):
    def log_message(self, *_args):
        pass


class DashboardHTTPTests(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(prefix="msc-dashboard-", dir="/tmp")
        self.original_root = dashboard.REPO
        fixture = Path(self.scratch.name)
        for name in dashboard.FILES:
            destination = fixture / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(self.original_root / name, destination)
        dashboard.REPO = fixture
        self.server = ThreadingHTTPServer(("127.0.0.1", 0), QuietHandler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()
        self.base = f"http://127.0.0.1:{self.server.server_port}"

    def tearDown(self):
        self.server.shutdown()
        self.server.server_close()
        self.thread.join()
        dashboard.REPO = self.original_root
        self.scratch.cleanup()

    def request(self, path, method="GET"):
        try:
            return urlopen(Request(self.base + path, method=method), timeout=3)
        except HTTPError as error:
            return error

    def state(self):
        with self.request("/" + dashboard.STATE) as response:
            self.assertEqual(response.status, 200)
            self.assertEqual(response.headers["Cache-Control"], "no-store")
            return json.load(response)

    def test_reports_change_without_server_restart(self):
        state = self.state()
        state["summary"] = "Second reported outcome"
        (dashboard.REPO / dashboard.STATE).write_text(json.dumps(state))
        self.assertEqual(self.state()["summary"], "Second reported outcome")

    def test_invalid_state_fails_instead_of_serving_success(self):
        (dashboard.REPO / dashboard.STATE).write_text("{")
        with self.request("/" + dashboard.STATE) as response:
            self.assertEqual(response.status, 503)
        with self.request("/docs/dashboard/index.html") as response:
            self.assertEqual(response.status, 200)  # UI can explain the failure.

    def test_source_change_and_missing_source_are_distinct(self):
        path = "thesis/chapters/07-hko.tex"
        with (dashboard.REPO / path).open("a") as output:
            output.write("\n% changed fixture\n")
        with self.request("/_dashboard/source-status") as response:
            status = json.load(response)
        self.assertTrue(status["known"])
        self.assertIn(path, status["changed"])
        self.assertFalse(status["reconciled"])
        (dashboard.REPO / path).unlink()
        with self.request("/_dashboard/source-status") as response:
            self.assertFalse(json.load(response)["known"])

    def test_bookmarks_head_and_bounded_read_only_serving(self):
        with self.request("/docs/project-now.html") as response:
            self.assertTrue(response.url.endswith("/docs/dashboard/thesis.html"))
        with self.request("/docs/dashboard/index.html", "HEAD") as response:
            self.assertEqual(response.status, 200)
            self.assertGreater(int(response.headers["Content-Length"]), 0)
            self.assertEqual(response.read(), b"")
        for path in ("/AGENTS.md", "/../AGENTS.md", "/%2e%2e/AGENTS.md", "/docs/", "/.git/config"):
            with self.request(path) as response:
                self.assertEqual(response.status, 404)
        with self.request("/" + dashboard.STATE, "POST") as response:
            self.assertEqual(response.status, 405)
        # An allowlisted name must not expose a symlink outside the checkout.
        path = dashboard.REPO / "docs/README.md"
        path.unlink()
        path.symlink_to(self.original_root / "README.md")
        with self.request("/docs/README.md") as response:
            self.assertEqual(response.status, 404)

    def test_dependency_cycle_is_rejected(self):
        state = self.state()
        task = state["tasks"][0]
        task["depends_on"] = [task["id"]]
        (dashboard.REPO / dashboard.STATE).write_text(json.dumps(state))
        with self.request("/" + dashboard.STATE) as response:
            self.assertEqual(response.status, 503)


if __name__ == "__main__":
    unittest.main()
