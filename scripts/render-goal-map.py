#!/usr/bin/env python3
"""Render the provisional conceptual map; current.json still owns assignments."""
import html
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / "docs/coordination/goal-map.json"
OUT = ROOT / "docs/coordination/graphs"


def main():
    model = json.loads(SOURCE.read_text())
    nodes = {node["id"]: node for node in model["nodes"]}
    if len(nodes) != len(model["nodes"]):
        raise ValueError("Duplicate node IDs")
    q = lambda value: json.dumps(value, ensure_ascii=False)
    styles = {
        "target": ("doubleoctagon", "#e8efe7", "#47624c"),
        "requirement": ("box", "#edf2f8", "#52718b"),
        "question": ("hexagon", "#fff3ce", "#a27b29"),
        "criterion": ("box", "#faf1e7", "#94704e"),
        "candidate": ("box", "#f1eef6", "#7d6a91"),
    }
    lines = ["// Generated from goal-map.json; not an assignment graph.",
             "digraph goals {",
             'graph [rankdir=BT, bgcolor="transparent", nodesep=0.35, ranksep=0.6, pad=0.2];',
             'node [fontname="sans-serif", fontsize=13, style="rounded,filled", margin="0.13,0.1"];',
             'edge [fontname="sans-serif", fontsize=10, arrowsize=0.65];']
    for node in nodes.values():
        shape, fill, border = styles[node["kind"]]
        lines.append(f'{q(node["id"])} [id={q("goal-"+node["id"])}, label={q(node["label"])}, shape={shape}, fillcolor={q(fill)}, color={q(border)}, tooltip={q(node["status"])}];')
    edge_styles = {"necessary": ("solid", "#344f66"),
                   "candidate": ("dashed", "#8a7c98"),
                   "definition": ("dotted", "#a27b29")}
    for i, edge in enumerate(model["edges"]):
        if edge["from"] not in nodes or edge["to"] not in nodes:
            raise ValueError("Unknown edge endpoint")
        style, color = edge_styles[edge["kind"]]
        lines.append(f'{q(edge["from"])} -> {q(edge["to"])} [id={q("edge-"+str(i))}, style={style}, color={q(color)}, label={q(edge["label"])}];')
    lines.append("}")
    OUT.mkdir(exist_ok=True)
    dot = "\n".join(lines) + "\n"
    (OUT / "goal-map.dot").write_text(dot)
    svg = subprocess.run(["dot", "-Tsvg"], input=dot, text=True, capture_output=True, check=True).stdout
    (OUT / "goal-map.svg").write_text(svg)
    inline = svg[svg.index("<svg"):]
    # Source text enters the browser as textContent; escape '<' in JSON too.
    data = json.dumps(model, ensure_ascii=False).replace("<", "\\u003c")
    page = '''<!doctype html><html lang="en"><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>msc-math goal map</title>
<style>
*{box-sizing:border-box}body{margin:0;background:#faf9f6;color:#25312d;font:16px/1.5 system-ui,sans-serif}
header{padding:24px 28px 12px;max-width:1200px}h1{font-size:25px;margin:0 0 8px}p{margin:8px 0}
.layout{display:grid;grid-template-columns:minmax(0,1fr) 340px;gap:18px;padding:12px 24px 24px}
.map{min-width:0;overflow:auto;background:white;border:1px solid #d9ded8;border-radius:10px;max-height:80vh;padding:12px}
.map svg{width:auto;height:auto;min-width:1497px;max-width:none}.node{cursor:pointer}.node:focus polygon,.node:focus path{stroke:#131e19;stroke-width:3}
aside{background:#fff;border:1px solid #d9ded8;border-radius:10px;padding:18px;align-self:start;overflow-wrap:anywhere}
h2{font-size:18px;margin:0 0 8px}.badge{font-weight:600;color:#805e24;font-size:14px}.legend{font-size:14px}
button{font:inherit;padding:5px 10px;border:1px solid #aab4ac;border-radius:6px;background:white;cursor:pointer}
li{margin-bottom:5px}.subdued{opacity:.12}@media(max-width:850px){.layout{grid-template-columns:1fr;padding:10px}header{padding:18px}.map{max-height:65vh}aside{width:100%}}
</style><header><h1>TITLE</h1><p>STATUS</p>
<p class="legend"><b>Solid:</b> X is necessary for Y under the stated definition (Y implies X).<br>
<b>Dashed:</b> X might help reach Y; necessity and efficacy are unestablished.<br>
<b>Dotted:</b> an unresolved choice changes the endpoint's meaning.</p>
<p>FRICTION</p><button id="filter" aria-pressed="false">Emphasize necessity claims</button>
<p>Click a node for its status, interpretation and source pointers. This is a goal model, not a launch plan.</p></header>
<main class="layout"><section class="map" aria-label="Provisional goal graph">SVG</section>
<aside aria-live="polite"><h2 id="name"></h2><p class="badge" id="status"></p><p id="detail"></p><b>Source pointers</b><ul id="sources"></ul></aside></main>
<script>const model=DATA;const byId=Object.fromEntries(model.nodes.map(n=>[n.id,n]));
function select(id){const n=byId[id];document.getElementById('name').textContent=n.label.replaceAll('\\n',' ');document.getElementById('status').textContent=n.status;document.getElementById('detail').textContent=n.detail;const list=document.getElementById('sources');list.replaceChildren(...n.sources.map(s=>{const li=document.createElement('li');li.textContent=s;return li;}));}
for(const n of model.nodes){const el=document.getElementById('goal-'+n.id);el.setAttribute('tabindex','0');el.setAttribute('role','button');el.setAttribute('aria-label',n.label.replaceAll('\\n',' '));el.addEventListener('click',()=>select(n.id));el.addEventListener('keydown',e=>{if(e.key==='Enter'||e.key===' '){e.preventDefault();select(n.id);}});}
let filtered=false;document.getElementById('filter').addEventListener('click',e=>{filtered=!filtered;e.target.setAttribute('aria-pressed',String(filtered));e.target.textContent=filtered?'Show all candidate paths':'Emphasize necessity claims';model.edges.forEach((edge,i)=>document.getElementById('edge-'+i).classList.toggle('subdued',filtered&&edge.kind!=='necessary'));});select('criterion');</script></html>'''
    # Substitute only marked template slots, before adding model data/SVG.
    page = page.replace("TITLE", html.escape(model["title"]))
    page = page.replace("STATUS", html.escape(model["status"]))
    page = page.replace("FRICTION", html.escape(model["friction"]))
    page = page.replace("SVG", inline).replace("DATA", data)
    (OUT / "goal-map.html").write_text(page)
    print("Rendered 12-node provisional map as DOT, SVG and interactive HTML")


if __name__ == "__main__":
    main()
