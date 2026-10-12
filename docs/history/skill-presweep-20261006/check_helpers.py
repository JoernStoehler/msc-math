#!/usr/bin/env python3
"""Offline checks: no model, login, credentials, service or site requests."""
import contextlib
import io
import json
import os
from pathlib import Path
import runpy
import subprocess
import tempfile
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[3]
SKILLS = ROOT / '.agents/skills'

with tempfile.TemporaryDirectory() as tmp:
    base = Path(tmp)
    bins = base / 'bin'; bins.mkdir()
    scratch = base / 'scratch'; scratch.mkdir()
    def stub(name, body):
        p = bins / name; p.write_text('#!/usr/bin/python3\n' + body); p.chmod(0o755)
    stub('codex', 'print("Logged in (stub)")\n')
    stub('timeout', 'import json, os, sys\nfrom pathlib import Path\nPath(os.environ["CAPTURE"]).write_text(json.dumps(sys.argv[1:]))\nsys.exit(int(os.environ.get("MOCK_EXIT", "0")))\n')
    env = dict(os.environ, PATH=str(bins) + ':' + os.environ['PATH'], TMPDIR=str(scratch),
               CAPTURE=str(base / 'args.json'), LOG=str(base / 'run.jsonl'))
    wrapper = SKILLS / 'codex-cli-claude-cloud/scripts/codex-run.sh'
    for value in ['0', '00', '000', '-1', 'x', '1.5', '']:
        r = subprocess.run(['sh', str(wrapper), '-t', value, 'hello'], env=env, capture_output=True)
        assert r.returncode == 2, (value, r.returncode)
    for value in ['1', '001', '600']:
        r = subprocess.run(['sh', str(wrapper), '-t', value, '-w', 'hello'], env=env, capture_output=True)
        assert r.returncode == 0, r.stderr
        args = json.loads((base/'args.json').read_text())
        assert 'web_search="live"' in args and args[1] == value, args
        assert not list(scratch.iterdir())
    r = subprocess.run(['sh', str(wrapper), '-t', '1', 'hello'], env=dict(env, MOCK_EXIT='124'), capture_output=True)
    assert r.returncode == 124 and not list(scratch.iterdir())
    r = subprocess.run(['sh', str(wrapper), '@' + str(base/'missing')], env=env, capture_output=True)
    assert r.returncode == 2 and not list(scratch.iterdir())
    print('PASS wrapper: positive/invalid timeouts, live-search argv, exit propagation, temporary cleanup')

    # Run actual bootstrap against a temporary output mount and existing Codex stub.
    cloud = base/'outputs'; cloud.mkdir()
    bootstrap = (SKILLS/'codex-cli-claude-cloud/scripts/bootstrap.sh').read_text().replace('/mnt/attach/outputs', str(cloud))
    for kind in ['absent', 'empty', 'populated', 'file', 'symlink']:
        home = base/kind; home.mkdir()
        sessions = home/'sessions'
        if kind in ['empty', 'populated']: sessions.mkdir()
        if kind == 'populated': (sessions/'retained').write_text('preserve')
        if kind == 'file': sessions.write_text('preserve')
        if kind == 'symlink': sessions.symlink_to(base/'missing-target')
        r = subprocess.run(['sh'], input=bootstrap, text=True, env=dict(env, CODEX_HOME=str(home)), capture_output=True)
        assert r.returncode == 0, r.stderr
        if kind in ['absent', 'empty']: assert sessions.is_symlink()
        if kind == 'populated': assert (sessions/'retained').read_text() == 'preserve'
        if kind == 'file': assert sessions.read_text() == 'preserve' and not sessions.is_symlink()
        if kind == 'symlink': assert os.readlink(sessions) == str(base/'missing-target')
    print('PASS bootstrap: absent/empty directory linking; populated directory, regular file and symlink preservation')

    # Copy only package mechanics; fake host launch/share calls. PATH has no Pandoc.
    review = base/'review'; review.mkdir()
    (review/'review-open').write_text((SKILLS/'review-files/scripts/review-open').read_text())
    stub('systemd-run', 'import os\nfrom pathlib import Path\nPath(os.environ["OPENED"]).write_text("opened")\n')
    for tool in ['python3', 'curl', 'realpath', 'dirname']:
        import shutil
        (bins/tool).symlink_to(shutil.which(tool))
    for name in ['review-server','review-share']:
        p=review/name;p.write_text('#!/bin/sh\nprintf "http://127.0.0.1:1234\\n"\n');p.chmod(0o755)
    pdf = base/'test.pdf';pdf.write_bytes(b'%PDF-1.4 fixture')
    md = base/'test.md';md.write_text('# fixture')
    review_env=dict(env, PATH=str(bins), OPENED=str(base/'opened'))
    for flags in [[], ['--print-only']]:
        r=subprocess.run(['/bin/bash',str(review/'review-open'),'--local',*flags,str(pdf)],env=review_env,capture_output=True)
        assert r.returncode==0 and b'http://127.0.0.1:1234' in r.stdout, r.stderr
        assert not (base/'opened').exists()
    r=subprocess.run(['/bin/bash',str(review/'review-open'),'--local',str(md)],env=review_env,capture_output=True)
    assert r.returncode==127 and b'pandoc' in r.stderr
    r=subprocess.run(['/bin/bash',str(review/'review-open'),'--sandbox',str(pdf)],env=dict(review_env, REVIEW_FILES_PUBLIC_BASE_URL='invalid'),capture_output=True)
    assert r.returncode==1 and b'docs/development-environments.md' in r.stderr
    print('PASS review delivery: PDF without Pandoc; URL-only default; Markdown dependency; missing sandbox setup route')

    # Exercise the unmodified annotation fetcher with synthetic response pages.
    fakehome=base/'fakehome'; cred=fakehome/'.config/hypothesis/api-token';cred.parent.mkdir(parents=True);cred.write_text('synthetic-token')
    for count in [0,199,200,201]:
        rows=[{'id':f'{i:05d}'} for i in range(count)]
        pages=[{'rows':rows[i:i+200]} for i in range(0,count,200)]
        if count%200==0:pages.append({'rows':[]})
        queries=[]
        class Opener:
            def open(self, req, timeout):
                queries.append(req.full_url)
                return io.StringIO(json.dumps(pages.pop(0)))
        out=base/f'feedback-{count}.json'
        with patch('pathlib.Path.home',return_value=fakehome), patch('urllib.request.build_opener',return_value=Opener()),patch('sys.argv',['fetch','--uri','https://example.invalid/document','--output',str(out)]),contextlib.redirect_stdout(io.StringIO()):
            runpy.run_path(str(SKILLS/'human-prose-review/scripts/fetch_review.py'),run_name='__main__')
        assert len(json.loads(out.read_text())['annotations'])==count
        assert all('user=acct%3AJoernStoehler%40hypothes.is' in q and 'uri=https%3A%2F%2Fexample.invalid%2Fdocument' in q for q in queries)
    pages=[{'rows':[{'id':f'{i:05d}'} for i in range(200)]},{'rows':[{'id':'00000'}]}]
    out=base/'repeated.json'
    with patch('pathlib.Path.home',return_value=fakehome),patch('urllib.request.build_opener',return_value=Opener()),patch('sys.argv',['fetch','--uri','https://example.invalid/document','--output',str(out)]),contextlib.redirect_stdout(io.StringIO()):
        try:runpy.run_path(str(SKILLS/'human-prose-review/scripts/fetch_review.py'),run_name='__main__')
        except SystemExit as e:assert 'Repeated annotation' in str(e)
        else:raise AssertionError('repeated ID accepted')
    assert not out.exists()
    print('PASS annotation mechanics: 0/199/200/201 rows, scoped query, duplicate-ID refusal without snapshot')
