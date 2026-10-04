"""HTTP contracts for absolute-path file viewing and dashboard compatibility."""

import hashlib
import importlib.util
import json
import pathlib
import shutil
import subprocess
import tempfile
import threading
import unittest
from urllib.error import HTTPError
from urllib.request import Request, urlopen

spec = importlib.util.spec_from_file_location('viewer', pathlib.Path(__file__).with_name('serve-files.py'))
viewer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(viewer)


class FileViewerTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.scratch = tempfile.TemporaryDirectory(prefix='msc-files-')
        cls.root = pathlib.Path(cls.scratch.name)
        cls.checkouts = [cls.root / 'main', cls.root / '.codex/worktrees/example']
        for number, root in enumerate(cls.checkouts):
            (root / 'docs/dashboard').mkdir(parents=True)
            (root / 'note ü.md').write_text(f'# Checkout {number}\n\n[Config](config.toml)\n\n**Proof note**\n')
            (root / 'config.toml').write_text('title = "<script>literal config</script>"\n')
            (root / 'state.json').write_text(json.dumps({'checkout': number}))
            (root / 'dashboard.html').write_text('<!doctype html><script>fetch("state.json")</script>')
            (root / 'style.css').write_text('body { color: black; }')
            (root / 'sample.pdf').write_bytes(b'%PDF-1.4\nfixture\n')
        class QuietHandler(viewer.FileViewer):
            roots = (cls.root,)
            max_text_bytes = 2048
            def log_message(self, *_args):
                pass
        cls.server = viewer.ThreadingHTTPServer(('127.0.0.1', 0), QuietHandler)
        cls.thread = threading.Thread(target=cls.server.serve_forever, daemon=True)
        cls.thread.start()
        cls.base = f'http://127.0.0.1:{cls.server.server_port}'

    @classmethod
    def tearDownClass(cls):
        cls.server.shutdown()
        cls.server.server_close()
        cls.thread.join()
        cls.scratch.cleanup()

    def request(self, path, accept='text/html', query='', method='GET'):
        request = Request(self.base + viewer.file_url(path) + query,
                          headers={'Accept': accept}, method=method)
        try:
            with urlopen(request, timeout=20) as response:
                return response.status, response.headers, response.read()
        except HTTPError as response:
            return response.code, response.headers, response.read()

    @unittest.skipUnless(shutil.which('pandoc'), 'Pandoc is required')
    def test_markdown_and_same_named_files_in_distinct_checkouts(self):
        for number, root in enumerate(self.checkouts):
            status, headers, body = self.request(root / 'note ü.md')
            self.assertEqual(status, 200)
            self.assertIn('text/html', headers['Content-Type'])
            self.assertIn(f'Checkout {number}'.encode(), body)
            self.assertIn(b'<strong>Proof note</strong>', body)
            self.assertIn(b'href="config.toml"', body)
            _, _, raw = self.request(root / 'note ü.md', query='?raw=1')
            self.assertEqual(raw, (root / 'note ü.md').read_bytes())

    def test_config_display_and_raw_content(self):
        path = self.checkouts[0] / 'config.toml'
        _, _, body = self.request(path)
        self.assertIn(b'&lt;script&gt;literal config&lt;/script&gt;', body)
        self.assertNotIn(b'<script>literal config</script>', body)
        _, _, raw = self.request(path, query='?raw=1')
        self.assertEqual(raw, path.read_bytes())

    def test_json_navigation_and_dashboard_fetch_use_the_same_url(self):
        for number, root in enumerate(self.checkouts):
            _, headers, body = self.request(root / 'state.json')
            self.assertIn('text/html', headers['Content-Type'])
            self.assertIn(b'&quot;checkout&quot;', body)
            _, headers, body = self.request(root / 'state.json', accept='*/*')
            self.assertEqual(headers['Content-Type'], 'application/json')
            self.assertEqual(json.loads(body), {'checkout': number})
            self.assertEqual(headers['Vary'], 'Accept')

    def test_html_pdf_css_and_javascript_keep_their_types_and_bytes(self):
        root = self.checkouts[0]
        (root / 'source-status.js').write_text('window.test = true;')
        for name, mime in [('dashboard.html', 'text/html'), ('sample.pdf', 'application/pdf'),
                           ('style.css', 'text/css'), ('source-status.js', 'javascript')]:
            _, headers, body = self.request(root / name, accept='*/*')
            self.assertIn(mime, headers['Content-Type'])
            self.assertEqual(body, (root / name).read_bytes())
            self.assertEqual(headers['Cache-Control'], 'no-store')
        status, headers, body = self.request(root / 'sample.pdf', method='HEAD')
        self.assertEqual(status, 200)
        self.assertEqual(body, b'')
        self.assertEqual(int(headers['Content-Length']), (root / 'sample.pdf').stat().st_size)

    def test_directory_links_keep_the_absolute_path_and_escape_names(self):
        status, _, body = self.request(self.checkouts[0])
        self.assertEqual(status, 200)
        self.assertIn(b'note%20%C3%BC.md', body)
        with urlopen(self.base + '/files/') as response:
            self.assertIn(viewer.file_url(self.root).encode(), response.read())

    def test_outside_root_symlinks_and_write_requests_are_rejected(self):
        outside = self.root.parent / 'not-exposed'
        link = self.checkouts[0] / 'outside.txt'
        link.symlink_to(outside)
        self.assertEqual(self.request(outside)[0], 403)
        self.assertEqual(self.request(link)[0], 403)
        path = self.checkouts[0] / 'config.toml'
        self.assertEqual(self.request(path, method='POST')[0], 501)
        self.assertEqual(self.request(self.checkouts[0] / 'missing')[0], 404)

    def test_large_files_offer_raw_access_without_rendering(self):
        path = self.checkouts[0] / 'large.md'
        path.write_text('# Large\n' + 'x' * 3000)
        _, _, body = self.request(path)
        self.assertIn(b'rendering limit', body)
        _, _, raw = self.request(path, query='?raw=1')
        self.assertEqual(raw, path.read_bytes())

    @unittest.skipUnless(shutil.which('node'), 'Node verifies browser source-check logic')
    def test_file_based_source_check_detects_changes_and_unavailable_checks(self):
        root = self.checkouts[0]
        source = root / 'claim.md'
        source.write_text('# Original claim\n')
        baseline = root / 'docs/dashboard/thesis-sources.json'
        baseline.write_text(json.dumps({'reconciled': False, 'note': 'Dated baseline',
                                       'sha256': {'claim.md': hashlib.sha256(source.read_bytes()).hexdigest()}}))
        page_url = self.base + viewer.file_url(root / 'docs/dashboard/index.html')
        script_path = pathlib.Path(__file__).parent.parent / 'docs/dashboard/source-status.js'
        program = """
const fs = require('node:fs'), vm = require('node:vm');
const location = new URL(process.argv[1]);
const context = {window: {}, location, URL, crypto: require('node:crypto').webcrypto,
  fetch: (url, options) => fetch(new URL(url, location), options)};
vm.createContext(context);
vm.runInContext(fs.readFileSync(process.argv[2], 'utf8'), context);
(async () => {
  const first = await context.window.dashboardSourceStatus();
  fs.appendFileSync(process.argv[3], 'Changed claim\\n');
  const changed = await context.window.dashboardSourceStatus();
  context.crypto = undefined;
  const unavailable = await context.window.dashboardSourceStatus();
  console.log(JSON.stringify({first, changed, unavailable}));
})().catch(e => {console.error(e); process.exit(1)});
"""
        result = subprocess.run(['node', '-e', program, page_url, str(script_path), str(source)],
                                check=True, capture_output=True, text=True, timeout=20)
        result = json.loads(result.stdout)
        self.assertTrue(result['first']['known'])
        self.assertFalse(result['first']['reconciled'])
        self.assertEqual(result['first']['changed'], [])
        self.assertEqual(result['changed']['changed'], ['claim.md'])
        self.assertFalse(result['unavailable']['known'])
        self.assertFalse(result['unavailable']['reconciled'])


if __name__ == '__main__':
    unittest.main()
