import hashlib
from pathlib import Path
import sys
from types import SimpleNamespace
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'scripts'))
import refresh
from validate import check
import pytest


class DummyResponse:
    def __init__(self,body,status=200,content_type='text/html',url='https://help.openai.com/en/articles/1',headers=None):
        self.body=body.encode('utf-8'); self.status_code=status; self.url=url
        self.headers={'Content-Type':content_type,**(headers or {})}; self.encoding='utf-8'
    def iter_content(self,chunk_size=65536): yield self.body
    def raise_for_status(self):
        if self.status_code>=400: raise RuntimeError(f'HTTP {self.status_code}')


class DummySession:
    def __init__(self,response): self.response=response; self.headers=[]
    def get(self,url,headers,timeout,allow_redirects,stream):
        self.headers.append(headers)
        return self.response


def record():
    return dict(id='test-source',url='https://help.openai.com/en/articles/1',format='html',path='corpus/articles/openai/chatgpt/test-source.md',status='pending',etag=None,last_modified=None,sha256=None,bytes=None)


def test_html_removes_nav_preserves_code_and_heading():
    h='<html><body><nav>DONOTKEEP</nav><main><h1>Title</h1><p>'+('important documented instructions '*8)+'</p><pre><code>git clone --depth 1</code></pre></main></body></html>'
    v=refresh.to_markdown(h,'html','https://help.openai.com/en/articles/1')
    assert 'DONOTKEEP' not in v and '# Title' in v and 'git clone --depth 1' in v


def test_markdown_not_damaged():
    v=refresh.to_markdown('# Title\n\n'+('text '*30)+'\n```rust\nfn x() {}\n```','markdown','https://code.claude.com/docs/en/foo.md')
    assert '```rust\nfn x() {}\n```' in v


def test_invalid_url():
    for url in ['http://help.openai.com/x','https://example.com/','https://help.openai.com.evil.org/x','file:///etc/passwd']:
        with pytest.raises(ValueError): refresh.validate_url(url)


def test_refresh_success_then_304(tmp_path,monkeypatch):
    monkeypatch.setattr(refresh,'ROOT',tmp_path)
    body='<main><h1>Page</h1><p>'+('Long factual text. '*15)+'</p></main>'
    resp=DummyResponse(body,headers={'ETag':'"hash123"'})
    r,status=refresh.fetch_one(record(),DummySession(resp))
    assert status=='changed' and r['status']=='ok'
    path=tmp_path/r['path']; assert path.exists()
    orig=path.read_bytes()
    r2,status2=refresh.fetch_one(r,DummySession(DummyResponse('',status=304)))
    assert status2=='unchanged-304' and path.read_bytes()==orig


def test_corrupt_or_short_response_keeps_prior_file(tmp_path,monkeypatch):
    monkeypatch.setattr(refresh,'ROOT',tmp_path)
    r=record(); path=tmp_path/r['path']; path.parent.mkdir(parents=True)
    path.write_text('# Verified doc\n'+'really useful\n'*15)
    r.update(status='ok',sha256=hashlib.sha256(path.read_bytes()).hexdigest())
    r2,status=refresh.fetch_one(r,DummySession(DummyResponse('<html><body>nope</body></html>')))
    assert status=='error' and path.read_text().startswith('# Verified doc')


def test_discovery_parses_md_links_preserves_existing(monkeypatch):
    txt='- [Skill](https://code.claude.com/docs/en/skills.md)\n- [New](https://code.claude.com/docs/en/feature-new.md)'
    monkeypatch.setattr(refresh,'get_text',lambda url:(None,txt))
    originals=[dict(record(),id='claude-code-skills',url='https://code.claude.com/docs/en/skills.md')]
    names=refresh.discover_claude(originals,100)
    assert names==['claude-code-feature-new'] and len(originals)==2


def test_validation_of_package():
    assert check()==[]
