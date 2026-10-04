"""HTTP regression checks for the shared-state browser surface."""

import importlib.util
import json
import shutil
import subprocess
import sys
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

    def test_listener_starts_to_explain_an_invalid_initial_state(self):
        (dashboard.REPO / dashboard.STATE).write_text("{")
        code = """import importlib.util, sys
from pathlib import Path
spec = importlib.util.spec_from_file_location('dashboard', sys.argv[1])
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
module.REPO = Path(sys.argv[2])
sys.argv = ['dashboard', '--host', '127.0.0.1', '--port', '0']
module.main()
"""
        process = subprocess.Popen(
            [sys.executable, "-u", "-c", code, str(Path(dashboard.__file__)), str(dashboard.REPO)],
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
        )
        try:
            import select
            self.assertTrue(select.select([process.stdout], [], [], 3)[0], "No listener startup report")
            line = process.stdout.readline().strip()
            self.assertTrue(line.startswith("Dashboard: http://127.0.0.1:"), line)
            base = line.removeprefix("Dashboard: ")
            with urlopen(base + "/docs/dashboard/index.html", timeout=3) as response:
                self.assertEqual(response.status, 200)
            with self.assertRaises(HTTPError) as error:
                urlopen(base + "/" + dashboard.STATE, timeout=3)
            self.assertEqual(error.exception.code, 503)
        finally:
            process.terminate()
            process.communicate(timeout=3)

    def test_invalid_state_fails_instead_of_serving_success(self):
        (dashboard.REPO / dashboard.STATE).write_text("{")
        with self.request("/" + dashboard.STATE) as response:
            self.assertEqual(response.status, 503)
        with self.request("/docs/dashboard/index.html") as response:
            self.assertEqual(response.status, 200)  # UI can explain the failure.

    def test_source_change_and_missing_source_are_distinct(self):
        baseline_path = dashboard.REPO / dashboard.BASELINE
        baseline = json.loads(baseline_path.read_text())
        baseline["reconciled"] = True
        baseline["sha256"] = {
            path: dashboard.hashlib.sha256(dashboard.read_file(path)).hexdigest()
            for path in baseline["sha256"]
        }
        baseline_path.write_text(json.dumps(baseline))
        with self.request("/_dashboard/source-status") as response:
            self.assertTrue(json.load(response)["reconciled"])
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
        for path in ("/INSTALL.md", "/AGENTS.md", "/AGENTS.md.commentary.md", "/docs/coordination/otel-design.md", "/docs/coordination/graphs/tasks.svg", "/docs/coordination/skill-migration-plan.md"):
            with self.request(path) as response:
                self.assertEqual(response.status, 200)
        for path in ("/README.md", "/../AGENTS.md", "/%2e%2e/AGENTS.md", "/docs/", "/.git/config"):
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

    def test_completion_cannot_hide_an_unfinished_prerequisite(self):
        state = self.state()
        goal = next(task for task in state["tasks"] if task["id"] == "workflow-done")
        goal["status"] = "done"
        goal["owner"] = None
        (dashboard.REPO / dashboard.STATE).write_text(json.dumps(state))
        with self.request("/" + dashboard.STATE) as response:
            self.assertEqual(response.status, 503)

    def test_completed_assignment_releases_owner(self):
        state = self.state()
        task = next(task for task in state["tasks"] if task["status"] == "done")
        task["owner"] = "leftover-owner"
        (dashboard.REPO / dashboard.STATE).write_text(json.dumps(state))
        with self.request("/" + dashboard.STATE) as response:
            self.assertEqual(response.status, 503)

    def test_dependency_cycle_is_rejected(self):
        state = self.state()
        task = state["tasks"][0]
        task["depends_on"] = [task["id"]]
        (dashboard.REPO / dashboard.STATE).write_text(json.dumps(state))
        with self.request("/" + dashboard.STATE) as response:
            self.assertEqual(response.status, 503)


if __name__ == "__main__":
    unittest.main()
