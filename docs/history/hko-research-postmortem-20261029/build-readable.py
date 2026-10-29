#!/usr/bin/env python3
"""Build the self-contained historical report; no scientific producer runs."""
from __future__ import annotations

import base64
import html
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]


def run(*args: str) -> str:
    return subprocess.check_output(args, cwd=HERE, text=True)


def main() -> None:
    subprocess.run(['dot', '-Tsvg', 'dependency.dot', '-o', 'dependency.svg'], cwd=HERE, check=True)
    markup = run(
        'pandoc', 'REPORT.md', '--from=markdown+tex_math_single_backslash+tex_math_dollars',
        '--to=html5', '--standalone', '--embed-resources', '--mathml',
        '--toc', '--toc-depth=2', '--template=report-template.html',
        '--metadata=title:How the exact HKO improvement was obtained',
    )
    for source, ident, title in [
        ('cost-evidence.md', 'cost-evidence-appendix', 'Detailed timing and cost evidence'),
        ('causal-evidence.md', 'causal-evidence-appendix', 'Causal evidence and historical source locators'),
    ]:
        body = run('pandoc', source, '--from=markdown+tex_math_single_backslash+tex_math_dollars', '--to=html5', '--mathml')
        appendix = f'<details class="evidence" id="{ident}"><summary>{html.escape(title)}</summary><div>{body}</div></details>'
        markup = markup.replace('</main>', appendix + '\n</main>')

    sources: dict[str, tuple[str, str]] = {}

    def replace_link(match: re.Match[str]) -> str:
        target, label = html.unescape(match.group(1)), match.group(2)
        if target.startswith(('#', 'http:', 'https:', 'mailto:', 'data:')):
            return match.group(0)
        if target in ('cost-evidence.md', 'causal-evidence.md'):
            ident = 'cost-evidence-appendix' if target.startswith('cost-') else 'causal-evidence-appendix'
            return f'<a href="#{ident}">{label}</a>'
        if target in ('usage-projection.json', 'cost-extract.py'):
            kind = 'application/json' if target.endswith('.json') else 'text/plain'
            data = base64.b64encode((HERE / target).read_bytes()).decode('ascii')
            return f'<a download="{target}" href="data:{kind};base64,{data}">{label}</a>'
        path = Path(target) if target.startswith('/') else (HERE / target).resolve()
        try:
            locator = path.relative_to(REPO).as_posix()
        except ValueError:
            locator = str(path)
        if locator not in sources:
            sources[locator] = (f'checkout-source-{len(sources) + 1}', re.sub('<[^>]+>', '', label))
        ident, _ = sources[locator]
        return f'<a href="#{ident}" title="Checkout source: {html.escape(locator, quote=True)}">{label}</a>'

    markup = re.sub(r'<a\s+href="([^"]+)"[^>]*>(.*?)</a>', replace_link, markup, flags=re.S)
    catalogue = '<section id="checkout-source-files"><h2>Referenced checkout source files</h2><p>The scientific source files below remain in the repository; their paths are locators, not embedded proof copies. The detailed cost and causal evidence above is embedded in this reading copy.</p><ol class="source-list">'
    for locator, (ident, label) in sources.items():
        catalogue += f'<li id="{ident}"><span>{html.escape(html.unescape(label))}</span><br><code>{html.escape(locator)}</code></li>'
    catalogue += '</ol></section>'
    markup = markup.replace('</main>', catalogue + '\n</main>')
    markup = re.sub(r'(<table(?:\s[^>]*)?>)', r'<div class="table-wrap">\1', markup)
    markup = markup.replace('</table>', '</table></div>')
    markup = markup.replace('</details></nav>', '<ul class="evidence-links"><li><a href="#cost-evidence-appendix">Detailed cost evidence</a></li><li><a href="#causal-evidence-appendix">Causal source locators</a></li><li><a href="#checkout-source-files">Referenced source files</a></li></ul></details></nav>')
    markup = markup.replace('</body>', '''<script>
function revealTarget() {
  const id = decodeURIComponent(location.hash.slice(1));
  if (!id) return;
  const target = document.getElementById(id);
  if (!target) return;
  for (let node = target; node; node = node.parentElement) {
    if (node.tagName === 'DETAILS') node.open = true;
  }
  requestAnimationFrame(() => target.scrollIntoView());
}
addEventListener('hashchange', revealTarget);
addEventListener('load', () => {
  if (innerWidth < 1000) document.getElementById('report-contents').open = false;
  revealTarget();
});
</script></body>''')
    (HERE / 'report.html').write_text(markup)
    print('Built self-contained report.html with MathML, static SVG, embedded evidence, and source locators.')


if __name__ == '__main__':
    main()
