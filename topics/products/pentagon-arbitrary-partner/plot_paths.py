"""Render retained path measurements; no new geometry or capacity calls."""
import json
from pathlib import Path
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

root=Path(__file__).parent
summary=json.loads((root/'symmetrization-v1-summary.json').read_text())
rows=[json.loads(line) for line in (root/'symmetrization-v1.jsonl').read_text().splitlines()]
fig,axes=plt.subplots(1,2,figsize=(10,3.8),layout='constrained')
labels={'disk_8':'disk hull (4 sides)','gaussian_8':'Gaussian hull (6 sides)',
        'halfplanes_5':'irregular pentagon','cover_3':'translated-template hull','regular_3_0.1700':'regular triangle'}
for p in summary['paths']:
    selected=sorted([r for r in rows if r['parent_id']==p['body_id']], key=lambda r:r['source']['t'])
    t=[r['source']['t'] for r in selected]; y=[r['scalars']['capacity_over_A2'] for r in selected]
    color=axes[0].plot(t,y,label=labels[p['name']])[0].get_color()
    axes[0].scatter([p['threshold']],[1],s=30,color=color,zorder=3)
    axes[1].plot(t,[r['scalars']['sys']/p['grid'][0]['sys'] for r in selected],color=color)
axes[0].set_ylabel('Capacity / fixed two-bounce ceiling')
axes[0].set_title('An asymmetric obstruction disappears')
axes[1].set_ylabel('Systolic ratio / its initial value')
axes[1].set_title('Area cost changes the resulting tradeoff')
for ax in axes:
    ax.set_xlabel('t in (1 − t)K + t(−K)'); ax.grid(alpha=.2); ax.axhline(1,color='black',linewidth=.6)
axes[0].legend(fontsize=8,loc='lower right')
fig.savefig(root/'symmetrization-paths.svg')
fig.savefig(root/'symmetrization-paths.png',dpi=150)
