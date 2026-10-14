from pathlib import Path
import subprocess
import re

p = Path(__file__).resolve().parent
body = subprocess.check_output(
    ['pandoc', str(p / 'README.md'), '--from=markdown', '--to=html5'], text=True
)

# The browser view leads with the incidents. Full introductory reasoning and
# provenance remain available below, instead of blocking the reading path.
intro, rest = body.split('<h2 id="incident-index">', 1)
index, rest = rest.split('<h2 id="incident-details">', 1)
incidents, supporting = rest.split('<h2 id="evidence-and-interpretation-boundaries">', 1)
incidents = incidents.split('</h2>', 1)[1]
timing = dict(re.findall(
    r'href="#incident-(\d+)">.*?</a></td>\s*<td>(.*?)</td>', index, re.S
))

def incident(match):
    number, title, content = match.group(1), match.group(2), match.group(3)
    title = re.sub(r'^Incident \d+\s*', '', title)
    label = timing.get(number, '')
    return (
        f'<details class="incident" id="incident-{number}">'
        f'<summary><span class="number">{number}.</span> {title}</summary>'
        f'<div class="account"><p><strong>Evidence timing:</strong> {label}</p>'
        f'{content}</div></details>'
    )

incidents = re.sub(
    r'<h3 id="incident-(\d+)">(.*?)</h3>(.*?)(?=<h3|\Z)',
    incident, incidents, flags=re.S
)
intro = re.sub(r'<h1.*?</h1>', '', intro, count=1, flags=re.S)
body = (
    '<main><h1>Agent misunderstandings</h1>'
    '<p>40 selected incidents, with the context before the agent’s response, '
    'your correction and the outcome. Each account distinguishes an ordinary '
    'guess from a failure to apply context already supplied.</p>'
    '<p><a href="#interpretation">Read the assessment</a> · '
    '<a href="README.md">Editable source</a></p>'
    + incidents
    + '<details id="interpretation"><summary>Assessment and report context</summary>'
    + intro + '</details>'
    + '<details id="sources"><summary>Evidence limits and source locations</summary>'
    + '<h2>Evidence and interpretation boundaries</h2>' + supporting + '</details>'
    + '</main>'
)
style = '''
html { background: #fff; color: #000; }
body { font: 16px/1.5 Arial, sans-serif; max-width: 62ch;
       margin: 28px auto; padding: 0 20px; }
h1 { font-size: 24px; line-height: 1.25; margin: 0 0 20px; }
h2 { font-size: 18px; line-height: 1.4; margin-top: 28px; }
p { margin: 0 0 16px; }
a { color: #0038a8; text-underline-offset: 2px; }
details { margin: 0; border-bottom: 1px solid #aaa; }
summary { padding: 14px 0; cursor: pointer; font-weight: 400; }
summary .number { font-weight: 700; }
.account { padding: 8px 0 14px; }
details:target { outline: 2px solid #0038a8; outline-offset: 4px; }
code { font-size: 14px; overflow-wrap: anywhere; }
@media(max-width: 600px) { body { margin: 20px auto; padding: 0 16px; } }
'''
script = '''function reveal() {
  const x = document.getElementById(location.hash.slice(1));
  if (x?.tagName === 'DETAILS') { x.open = true; x.scrollIntoView(); }
}
addEventListener('hashchange', reveal); reveal();'''
(p / 'report.html').write_text(
    '<!doctype html><html lang="en"><meta charset="utf-8">'
    '<meta name="viewport" content="width=device-width,initial-scale=1">'
    '<title>Agent misunderstandings</title><style>' + style + '</style>'
    '<body>' + body + '<script>' + script + '</script></body></html>'
)
