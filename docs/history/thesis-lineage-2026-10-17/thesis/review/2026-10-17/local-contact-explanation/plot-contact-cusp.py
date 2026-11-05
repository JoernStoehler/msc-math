"""Plot the exact first-order model in contact-stability/REPORT.md, not data."""
from pathlib import Path
import math
import sys
out=Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=True)
c=2*math.sqrt(5)/5
# Horizontal scale is the parameter of delta a0=a0, delta a1=-a1.
def xy(t,d): return (350+9000*t,300-4500*d)
def line(t0,d0,t1,d1,**attrs):
 x0,y0=xy(t0,d0);x1,y1=xy(t1,d1)
 a=' '.join(f'{k.replace("_","-")}="{v}"' for k,v in attrs.items())
 return f'<line x1="{x0:g}" y1="{y0:g}" x2="{x1:g}" y2="{y1:g}" {a}/>'
a=['<svg xmlns="http://www.w3.org/2000/svg" width="700" height="510" viewBox="0 0 700 510" role="img" aria-labelledby="title desc">','<title id="title">Competing contact branches create a cusp</title>','<desc id="desc">The linearized logarithmic loss is the maximum of two lines, with slopes plus and minus two square root five over five. It increases for either sign of this shape change. This is a first-order model, not measured ratios.</desc>','<rect width="700" height="510" fill="white"/>','<g font-family="sans-serif" fill="#18212b">','<text x="350" y="31" text-anchor="middle" font-size="21">Competing contacts create a cusp</text>','<text x="350" y="58" text-anchor="middle" font-size="14">First-order logarithmic loss along one explicit shape change</text>']
a+=[line(-.031,0,.031,0,stroke='#707b86'),line(0,-.027,0,.042,stroke='#707b86')]
for t in [-.03,-.02,-.01,.01,.02,.03]:
 x,y=xy(t,0);a+=[f'<text x="{x:g}" y="{y+22:g}" text-anchor="middle" font-size="12">{t:.2f}</text>']
for d in [-.02,.02,.04]:
 x,y=xy(0,d);a+=[f'<text x="{x-10:g}" y="{y+4:g}" text-anchor="end" font-size="12">{d:.2f}</text>']
a += [line(-.03,-c*.03,.03,c*.03,stroke='#3b82a0',stroke_width=2,stroke_dasharray='7 5'),line(-.03,c*.03,.03,-c*.03,stroke='#c87824',stroke_width=2,stroke_dasharray='7 5'),line(-.03,c*.03,0,0,stroke='#702e91',stroke_width=4),line(0,0,.03,c*.03,stroke='#702e91',stroke_width=4)]
a += ['<text x="425" y="146" font-size="16" fill="#702e91">D(tδ) = (2√5/5)|t|</text>','<text x="640" y="288" font-size="14">t</text>','<text x="40" y="445" font-size="14">Shape change: δa₀ = a₀, δa₁ = −a₁; all other normal changes zero.</text>','<text x="40" y="472" font-size="14">log(ρH / sys(z₀ + tδ)) = D(tδ) + O(t²).</text>','<text x="40" y="495" font-size="12" fill="#56616e">Dashed: two active linear forms. Purple: their maximum. Model only; no fitted values.</text>','</g></svg>']
(out/'contact-cusp.svg').write_text('\n'.join(a)+'\n')
