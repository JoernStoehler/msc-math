#!/usr/bin/env python3
"""Freeze and serve PDF, HTML or Markdown with Hypothesis, on the host tailnet."""
import argparse
import hashlib
import json
import mimetypes
import os
from pathlib import Path
import secrets
import shutil
import socket
import subprocess
import sys
import tarfile
import tempfile
import time
import urllib.request
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import quote, unquote, urlsplit

VIEWER_REV = '87d50c92347d3d3b8034a18cc10282af1189931b'
VIEWER_SHA = '992d6d7af4a22a9e991ba412c92d80f07c0714546858a9de78ed09c44c882f47'
EMBED = '<script src="https://hypothes.is/embed.js" async></script>'
STYLE = '''<style>body{max-width:46rem;margin:3rem auto;padding:0 2rem 4rem;
font:18px/1.65 Georgia,serif}math[display="block"]{overflow-x:auto}</style>'''


def viewer_assets():
    cache = Path(os.environ.get('XDG_CACHE_HOME', Path.home() / '.cache')) / 'human-prose-review'
    target = cache / VIEWER_REV
    if target.is_dir():
        return target / 'viewer'
    cache.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(dir=cache) as scratch:
        archive = Path(scratch) / 'viewer.tar.gz'
        urllib.request.urlretrieve(
            f'https://codeload.github.com/hypothesis/pdf.js-hypothes.is/tar.gz/{VIEWER_REV}', archive)
        if hashlib.sha256(archive.read_bytes()).hexdigest() != VIEWER_SHA:
            raise SystemExit('PDF viewer download checksum mismatch.')
        with tarfile.open(archive) as bundle:
            bundle.extractall(scratch, filter='data')
        tree = Path(scratch) / ('pdf.js-hypothes.is-' + VIEWER_REV)
        try:
            tree.rename(target)
        except OSError:
            if not target.is_dir():
                raise
    return target / 'viewer'


def serve(manifest_path):
    m = json.loads(manifest_path.read_text())
    prefix = '/' + m['link_token']
    folder = manifest_path.parent
    allowed = {prefix + '/artifact': folder / m['served_file']}
    if m.get('viewer_assets'):
        root = Path(m['viewer_assets'])
        allowed.update({prefix + '/viewer/' + p.relative_to(root).as_posix(): p
                        for p in root.rglob('*') if p.is_file() and p.suffix != '.pdf'})

    class Handler(BaseHTTPRequestHandler):
        def do_GET(self):
            path = unquote(urlsplit(self.path).path)
            file = allowed.get(path)
            if file is None:
                self.send_error(404)
                return
            body = file.read_bytes()
            self.send_response(200)
            self.send_header('Content-Type', mimetypes.guess_type(str(file))[0] or 'application/octet-stream')
            self.send_header('Content-Length', str(len(body)))
            self.send_header('Cache-Control', 'no-store')
            self.send_header('X-Content-Type-Options', 'nosniff')
            self.end_headers()
            self.wfile.write(body)

        def log_message(self, *_):
            pass

    ThreadingHTTPServer((m['bind'], m['port']), Handler).serve_forever()


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('artifact', type=Path, nargs='?')
    p.add_argument('--output', type=Path, help='New retained review directory; must not exist')
    p.add_argument('--serve', type=Path, help=argparse.SUPPRESS)
    a = p.parse_args()
    if a.serve:
        serve(a.serve)
        return
    if not a.artifact or not a.output:
        p.error('artifact and --output are required')
    source = a.artifact.resolve(strict=True)
    suffix = source.suffix.lower()
    if suffix not in {'.md', '.markdown', '.html', '.htm', '.pdf'}:
        p.error('Expected PDF, standalone HTML or Markdown; render other formats first.')
    for tool in ['tailscale', 'systemd-run'] + (['pandoc'] if suffix in {'.md', '.markdown'} else []):
        if not shutil.which(tool):
            p.error(f'{tool} is required on the host; do not send Jörn a localhost link.')
    bind = subprocess.check_output(['tailscale', 'ip', '-4'], text=True).strip().splitlines()[0]
    socket.inet_aton(bind)
    assets = viewer_assets() if suffix == '.pdf' else None
    out = a.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    original = out / ('source' + suffix)
    shutil.copyfile(source, original)
    if suffix == '.pdf':
        served = original
    else:
        if suffix in {'.md', '.markdown'}:
            page = subprocess.check_output(['pandoc', '--standalone',
                '--from=markdown+tex_math_single_backslash', '--to=html5', '--mathml',
                '--embed-resources', '--resource-path=' + str(source.parent),
                '--metadata=title:Prose review', str(original)], text=True)
            page = page.replace('</head>', STYLE + '</head>')
        else:
            page = original.read_text()
        if 'https://hypothes.is/embed.js' not in page:
            if '</head>' in page:
                page = page.replace('</head>', EMBED + '</head>', 1)
            else:
                page = EMBED + page
        served = out / 'review.html'
        served.write_text(page, encoding='utf-8')
    with socket.socket() as sock:
        sock.bind((bind, 0))
        port = sock.getsockname()[1]
    token = secrets.token_urlsafe(18)
    base = f'http://{bind}:{port}/{token}'
    uri = base + '/artifact'
    url = base + '/viewer/web/viewer.html?file=' + quote(uri, safe='') if assets else uri
    unit = 'human-prose-review-' + secrets.token_hex(6)
    manifest = {'source_path': str(source), 'source_file': original.name,
        'source_sha256': hashlib.sha256(original.read_bytes()).hexdigest(),
        'served_file': served.name, 'served_sha256': hashlib.sha256(served.read_bytes()).hexdigest(),
        'document_uri': uri, 'review_url': url, 'viewer_assets': str(assets) if assets else None,
        'viewer_revision': VIEWER_REV if assets else None, 'bind': bind, 'port': port,
        'link_token': token, 'service_unit': unit, 'lifetime_hours': 8}
    mp = out / 'review.json'
    mp.write_text(json.dumps(manifest, indent=2) + '\n')
    subprocess.run(['systemd-run', '--user', '--quiet', '--collect', '--unit=' + unit,
        '--property=Type=exec', '--property=RuntimeMaxSec=8h', '--', sys.executable,
        str(Path(__file__).resolve()), '--serve', str(mp)], check=True)
    local = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    for _ in range(50):
        try:
            with local.open(uri, timeout=1) as response:
                response.read(1)
            break
        except OSError:
            time.sleep(.1)
    else:
        subprocess.run(['systemctl', '--user', 'stop', unit], check=False)
        raise SystemExit('Review server did not start; review files remain in ' + str(out))
    print(url)
    print('Manifest: ' + str(mp))


if __name__ == '__main__':
    main()
