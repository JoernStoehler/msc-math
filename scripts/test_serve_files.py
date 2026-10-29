"""HTTP contracts for absolute-path file viewing and dashboard compatibility."""

import hashlib
import gzip
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


class WorkbenchArtifactTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.scratch = tempfile.TemporaryDirectory(prefix='workbench-assets-')
        cls.root = pathlib.Path(cls.scratch.name)
        cls.builds = tuple(cls.root / name for name in ('resources', 'docs'))
        for build in (*cls.builds, cls.root / 'other'):
            (build / 'assets' / 'chunks').mkdir(parents=True)
            (build / 'index.html').write_text('<!doctype html><title>Built page</title>' * 100)
            (build / 'data.json').write_text('{"current":1}')
            (build / 'assets/app-AbCd1234.js').write_text('export const value = 1;\n' * 2000)
            (build / 'assets/chunks/framework.AbCd1234.js').write_text('export const framework = 1;\n' * 1000)
            (build / 'assets/page.md.AbCd1234.lean.js').write_text('export const page = 1;\n' * 1000)
            (build / 'assets/style-AbCd1234.css').write_text('body { color: black; }\n' * 1000)
            (build / 'assets/font.AbCd1234.woff2').write_bytes(b'font fixture')
            (build / 'assets/data-AbCd1234.json').write_text('{"dynamic":true}')
            (build / 'assets/loader.js').write_text('export const loader = 1;')
            (build / 'app-AbCd1234.js').write_text('export const outsideAssets = 1;')
        class QuietHandler(viewer.FileViewer):
            roots = (cls.root,)
            optimized_artifact_roots = cls.builds
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
        viewer.artifact_bytes.cache_clear()

    def request(self, path, headers=None, method='GET', query=''):
        request = Request(self.base + viewer.file_url(path) + query,
                          headers={'Accept': '*/*', **(headers or {})}, method=method)
        try:
            with urlopen(request, timeout=10) as response:
                return response.status, response.headers, response.read()
        except HTTPError as response:
            return response.code, response.headers, response.read()

    def test_gzip_bytes_types_and_immutable_scope(self):
        for build in self.builds:
            for name in ('assets/app-AbCd1234.js', 'assets/chunks/framework.AbCd1234.js',
                         'assets/page.md.AbCd1234.lean.js', 'assets/style-AbCd1234.css'):
                path = build / name
                status, headers, compressed = self.request(path, {'Accept-Encoding': 'gzip'})
                self.assertEqual(status, 200)
                self.assertEqual(headers['Content-Encoding'], 'gzip')
                self.assertEqual(gzip.decompress(compressed), path.read_bytes())
                self.assertLess(len(compressed), path.stat().st_size / 10)
                self.assertEqual(int(headers['Content-Length']), len(compressed))
                self.assertEqual(headers['Cache-Control'], 'private, max-age=31536000, immutable')
                self.assertEqual(headers['Vary'], 'Accept, Accept-Encoding')
            _, headers, body = self.request(build / 'assets/font.AbCd1234.woff2')
            self.assertIn('immutable', headers['Cache-Control'])
            self.assertNotIn('Content-Encoding', headers)
            self.assertEqual(body, b'font fixture')

    def test_html_data_and_unhashed_or_nonasset_files_do_not_become_immutable(self):
        for name in ('index.html', 'data.json', 'assets/data-AbCd1234.json',
                     'assets/loader.js', 'app-AbCd1234.js'):
            path = self.builds[0] / name
            status, headers, body = self.request(path, {'Accept-Encoding': 'gzip'})
            self.assertEqual(status, 200)
            self.assertEqual(headers['Cache-Control'], 'no-store')
            if path.suffix == '.json':
                self.assertNotIn('Content-Encoding', headers)
                self.assertEqual(body, path.read_bytes())
        status, headers, body = self.request(self.builds[0] / '', {'Accept-Encoding': 'gzip'}, query='/')
        self.assertEqual(status, 200)
        self.assertEqual(headers['Cache-Control'], 'no-store')
        self.assertEqual(gzip.decompress(body), (self.builds[0] / 'index.html').read_bytes())

    def test_conditional_requests_and_head_are_representation_aware(self):
        path = self.builds[0] / 'assets/app-AbCd1234.js'
        _, plain_headers, plain = self.request(path)
        _, headers, body = self.request(path, {'Accept-Encoding': 'gzip'})
        self.assertNotEqual(plain_headers['ETag'], headers['ETag'])
        status, head, empty = self.request(path, {'Accept-Encoding': 'gzip'}, method='HEAD')
        self.assertEqual(status, 200)
        self.assertEqual(empty, b'')
        self.assertEqual(head['ETag'], headers['ETag'])
        self.assertEqual(int(head['Content-Length']), len(body))
        status, cached, empty = self.request(path, {'Accept-Encoding': 'gzip', 'If-None-Match': headers['ETag']})
        self.assertEqual(status, 304)
        self.assertEqual(empty, b'')
        self.assertIn('immutable', cached['Cache-Control'])
        self.assertEqual(cached['Vary'], 'Accept, Accept-Encoding')
        status, _, body = self.request(path, {'If-None-Match': headers['ETag']})
        self.assertEqual(status, 200)
        self.assertEqual(body, plain)
        status, _, _ = self.request(path, {'If-Modified-Since': plain_headers['Last-Modified']})
        self.assertEqual(status, 304)

    def test_encoding_exclusions_and_identity_fallback(self):
        path = self.builds[0] / 'assets/app-AbCd1234.js'
        for encoding in ('gzip;q=0, *;q=1', 'br', 'gzip;q=0.1, identity;q=1'):
            _, headers, body = self.request(path, {'Accept-Encoding': encoding})
            self.assertNotIn('Content-Encoding', headers)
            self.assertEqual(body, path.read_bytes())
        status, _, _ = self.request(path, {'Accept-Encoding': 'gzip;q=0, identity;q=0'})
        self.assertEqual(status, 406)
        status, headers, body = self.request(path, {'Accept-Encoding': '*;q=0, gzip;q=1'})
        self.assertEqual(status, 200)
        self.assertEqual(gzip.decompress(body), path.read_bytes())

    def test_rendered_source_and_other_directories_preserve_existing_policy(self):
        path = self.builds[0] / 'assets/loader.js'
        _, headers, body = self.request(path, {'Accept': 'text/html', 'Accept-Encoding': 'gzip'})
        self.assertIn('text/html', headers['Content-Type'])
        self.assertIn(b'<pre>', body)
        self.assertEqual(headers['Cache-Control'], 'no-store')
        self.assertEqual(headers['Vary'], 'Accept')
        self.assertNotIn('Content-Encoding', headers)
        outside = self.root / 'other/assets/app-AbCd1234.js'
        _, headers, body = self.request(outside, {'Accept-Encoding': 'gzip'})
        self.assertEqual(headers['Cache-Control'], 'no-store')
        self.assertNotIn('Content-Encoding', headers)
        self.assertEqual(body, outside.read_bytes())
        link = self.builds[0] / 'assets/link-AbCd1234.js'
        link.symlink_to(outside)
        _, headers, body = self.request(link, {'Accept-Encoding': 'gzip'})
        self.assertEqual(headers['Cache-Control'], 'no-store')
        self.assertNotIn('Content-Encoding', headers)
        self.assertEqual(body, outside.read_bytes())
        self.assertEqual(self.request(self.root.parent / 'not-exposed.js')[0], 403)

    def test_same_build_symlink_aliases_do_not_receive_immutable_caching(self):
        target = self.builds[0] / 'assets/app-AbCd1234.js'
        aliases = (self.builds[0] / 'assets/current.js',
                   self.builds[0] / 'assets/chunks/app-AbCd1234.js')
        for alias in aliases:
            alias.symlink_to(target)
            status, headers, body = self.request(alias, {'Accept-Encoding': 'gzip'})
            self.assertEqual(status, 200)
            self.assertEqual(headers['Cache-Control'], 'no-store')
            self.assertEqual(gzip.decompress(body), target.read_bytes())
        _, canonical_headers, _ = self.request(target)
        self.assertIn('immutable', canonical_headers['Cache-Control'])

    def test_republished_html_invalidates_bounded_representation_cache(self):
        path = self.builds[1] / 'index.html'
        _, before, old = self.request(path, {'Accept-Encoding': 'gzip'})
        original = path.read_text()
        path.write_text(original.replace('Built page', 'Other page'))
        _, after, new = self.request(path, {'Accept-Encoding': 'gzip'})
        self.assertNotEqual(before['ETag'], after['ETag'])
        self.assertNotEqual(old, new)
        self.assertEqual(gzip.decompress(new), path.read_bytes())
        self.assertEqual(after['Cache-Control'], 'no-store')


if __name__ == '__main__':
    unittest.main()
