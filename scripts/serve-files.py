#!/usr/bin/env python3
"""View local files at /files + their absolute path; Markdown uses Pandoc.

Defaults cover this checkout, /workspaces/msc-math, Codex worktrees/artifacts,
/tmp/msc-math and /data/msc-math. Repeat --root to choose other directories, or --root / for
all files readable by the server user. The listener defaults to loopback.
"""

import argparse
import html
import io
import pathlib
import shutil
import subprocess
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, quote, unquote, urlsplit

STYLE = """<style>
:root { color-scheme: light dark; }
body { max-width:960px; margin:2rem auto; padding:0 1rem; font:16px/1.6 system-ui; }
pre { overflow:auto; padding:1rem; background:light-dark(#f5f5f5,#292a2d); }
img { max-width:100%; } table { border-collapse:collapse; }
td,th { border:1px solid #aaa; padding:.4rem; }
</style>"""
TEXT_SUFFIXES = {
    '.json', '.toml', '.yaml', '.yml', '.rs', '.py', '.sh', '.tex', '.log',
    '.txt', '.csv', '.ini', '.conf', '.js', '.css', '.ts', '.dot',
}
TEXT_NAMES = {'Dockerfile', 'Makefile', '.gitignore', '.git', 'LICENSE'}


def file_url(path):
    """Keep the lexical path so relative links retain their directory context."""
    return '/files' + quote(str(pathlib.Path(path).expanduser().absolute()), safe='/')


class FileViewer(SimpleHTTPRequestHandler):
    roots = ()
    max_text_bytes = 2 * 1024 * 1024

    def end_headers(self):
        self.send_header('Cache-Control', 'no-store')
        self.send_header('X-Content-Type-Options', 'nosniff')
        self.send_header('Vary', 'Accept')
        super().end_headers()

    def translate_path(self, uri):
        route = unquote(urlsplit(uri).path)
        if not route.startswith('/files/') or '\x00' in route:
            raise PermissionError('Use /files followed by an absolute path')
        try:
            path = pathlib.Path(route[len('/files'):]).resolve()
        except (OSError, RuntimeError, ValueError) as exc:
            raise PermissionError('Invalid file path') from exc
        if not any(path == root or root in path.parents for root in self.roots):
            raise PermissionError('Path is outside the configured directories')
        return str(path)

    def guess_type(self, path):
        suffix = pathlib.Path(path).suffix.lower()
        if suffix == '.json':
            return 'application/json'
        if suffix in {'.js', '.css'}:
            return super().guess_type(path)
        if suffix in TEXT_SUFFIXES or suffix in {'.md', '.markdown'}:
            return 'text/plain; charset=utf-8'
        return super().guess_type(path)

    def html_response(self, body):
        self.send_response(200)
        self.send_header('Content-Type', 'text/html; charset=utf-8')
        self.send_header('Content-Length', str(len(body)))
        self.end_headers()
        return io.BytesIO(body)

    def send_head(self):
        route = urlsplit(self.path).path
        if route == '/':
            self.send_response(302)
            self.send_header('Location', '/files/')
            self.send_header('Content-Length', '0')
            self.end_headers()
            return None
        if route in {'/files', '/files/'}:
            links = ''.join(f'<li><a href="{file_url(root)}/">{html.escape(str(root))}</a></li>'
                            for root in self.roots)
            return self.html_response((f'<!doctype html><html><head><meta charset="utf-8">'
                                       f'<title>Workstation files</title>{STYLE}</head>'
                                       f'<body><h1>Workstation files</h1><ul>{links}</ul></body></html>').encode())
        try:
            path = pathlib.Path(self.translate_path(self.path))
            is_file = path.is_file()
        except (PermissionError, OSError) as exc:
            self.send_error(403, str(exc))
            return None
        if not is_file:
            return super().send_head()
        query = parse_qs(urlsplit(self.path).query, keep_blank_values=True)
        if 'raw' in query:
            return super().send_head()
        suffix = path.suffix.lower()
        markdown = suffix in {'.md', '.markdown'}
        text = (suffix in TEXT_SUFFIXES or path.name in TEXT_NAMES) and (
            'text/html' in self.headers.get('Accept', '') or 'view' in query
        )
        if not markdown and not text:
            return super().send_head()
        try:
            if path.stat().st_size > self.max_text_bytes:
                message = '<p>This file exceeds the rendering limit. <a href="?raw=1">Open raw file</a>.</p>'
                return self.html_response(f'<!doctype html><meta charset="utf-8">{STYLE}{message}'.encode())
            if markdown:
                body = subprocess.run(
                    ['pandoc', '--standalone', '--from=gfm-raw_html', '--to=html5',
                     '--mathml', '--metadata', f'title={path.name}'],
                    input=path.read_bytes(), stdout=subprocess.PIPE,
                    stderr=subprocess.PIPE, check=True, timeout=15,
                ).stdout.replace(b'</head>', STYLE.encode() + b'</head>')
            else:
                source = html.escape(path.read_text(encoding='utf-8'))
                body = (f'<!doctype html><html><head><meta charset="utf-8">{STYLE}'
                        f'<title>{html.escape(path.name)}</title></head>'
                        f'<body><pre>{source}</pre></body></html>').encode()
        except UnicodeError:
            return super().send_head()
        except (OSError, subprocess.SubprocessError):
            self.send_error(500, 'Unable to render file')
            return None
        nav = b'<nav><a href="?raw=1">Raw file</a></nav>'
        return self.html_response(body.replace(b'<body>', b'<body>' + nav, 1))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--host', default='127.0.0.1')
    parser.add_argument('--port', type=int, default=8770)
    parser.add_argument('--root', action='append', help='directory to expose; repeatable')
    args = parser.parse_args()
    if not shutil.which('pandoc'):
        parser.error('Pandoc is required for Markdown rendering; see INSTALL.md')
    roots = args.root or [
        str(pathlib.Path(__file__).resolve().parent.parent),
        '/workspaces/msc-math',
        '~/.codex/worktrees', '~/.codex/artifacts', '/tmp/msc-math', '/data/msc-math',
    ]
    FileViewer.roots = tuple(dict.fromkeys(pathlib.Path(root).expanduser().resolve() for root in roots))
    with ThreadingHTTPServer((args.host, args.port), FileViewer) as server:
        print(f'Files: http://{args.host}:{server.server_port}/files/', flush=True)
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            pass


if __name__ == '__main__':
    main()
