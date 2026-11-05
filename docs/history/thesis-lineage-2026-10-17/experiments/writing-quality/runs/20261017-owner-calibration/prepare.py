"""Freeze the current hard prose slice; does not edit the manuscript."""
from pathlib import Path
from hashlib import sha256
import argparse, json, re, subprocess

p=argparse.ArgumentParser();p.add_argument('--repo',type=Path,default=Path.cwd());p.add_argument('--output',type=Path,required=True);a=p.parse_args()
repo=a.repo.resolve();out=a.output;out.mkdir(parents=True,exist_ok=True)
src=repo/'thesis/chapters/08-data-science.tex';pdf=repo/'thesis/review/kai-packet/thesis-for-kai.pdf'
s=src.read_text();heading=r'\subsection{Coarse selection succeeds; stronger selection need not}'
body=s.split(heading,1)[1].split('For generic ten-facet bodies,',1)[0].strip()
assert body.startswith('Selecting products with small $R$')
assert body.endswith('does not reliably order the ratios within it.')
(out/'source-excerpt.tex').write_text(body+'\n')
md=r'''# Coarse selection succeeds; stronger selection need not

**Current thesis excerpt · about two minutes.** Context from the preceding subsection: a ridge is a two-dimensional face, $A_\omega(F)$ is its absolute symplectic area, and $R(K)=\sum_F A_\omega(F)/\sqrt{\operatorname{vol}_4(K)}$. All ratios reported here are numerical. The excerpt below is unchanged prose from the current candidate.

---

'''+body+'''

---

[Full PDF, physical page 59](../../../../thesis/review/kai-packet/thesis-for-kai.pdf#page=59) · [Evidence and prose success criteria](../../OWNER-PLAN.md).

This short check concerns how these results are explained. It does not accept the chapter's scope or the rest of the thesis.

[review: /root/message_reviewer]
'''
(out/'current-selection.md').write_text(md)
# Compare each prose part with the actual fixed PDF, independently of Markdown rendering.
page=subprocess.check_output(['pdftotext','-f','59','-l','59','-layout',str(pdf),'-'],text=True)
(out/'pdf-page59.txt').write_text(page)
parts=re.split(r'\\\[.*?\\\]',body,flags=re.S)
def norm(t):
 t=t.replace('ﬁ','fi').replace('ﬂ','fl').replace('−','-')
 t=re.sub(r'-\s*\n\s*','',t)
 return re.sub(r'\s+','',t.replace('$',''))
assert all(norm(part) in norm(page) for part in parts), 'Prose does not match fixed candidate PDF p59'
assert all(v in page for v in ['0.626','0.316','0.034','0.032','+0.301'])
receipt={'source':'thesis/chapters/08-data-science.tex','source_sha256':sha256(src.read_bytes()).hexdigest(),'pdf':'thesis/review/kai-packet/thesis-for-kai.pdf','pdf_sha256':sha256(pdf.read_bytes()).hexdigest(),'physical_page':59,'excerpt_words':len(body.split()),'excerpt_sha256':sha256((body+'\n').encode()).hexdigest(),'pdf_prose_match':'PASS: both complete prose parts around the displayed equation match fixed PDF after whitespace, line hyphenation and math-marker normalization','transformation':'Subsection heading and labels omitted; body unchanged; TeX rendered by Pandoc','limits':'No new human verdict; source/prose matching does not check the displayed equation or visual layout.'}
(out/'verification.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt,indent=2))
