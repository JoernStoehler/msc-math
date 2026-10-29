# /// script
# dependencies = ["matplotlib>=3.8", "networkx>=3.2", "numpy>=1.26"]
# ///
"""Verified explanatory figures for the simplex Pfaffian graph and endpoint caps."""
from pathlib import Path
from itertools import combinations
import math
import numpy as np
import networkx as nx
import matplotlib.pyplot as plt
from matplotlib.patches import Polygon, FancyArrowPatch

OUT = Path(__file__).resolve().parent
plt.rcParams.update({
    'font.family': 'DejaVu Serif', 'font.size': 10.5,
    'mathtext.fontset': 'dejavuserif', 'axes.linewidth': .8,
    'pdf.fonttype': 42, 'savefig.facecolor': 'white',
})
ACCENT = '#bd5c36'
DARK = '#24292b'
LIGHT = '#a9b3b8'

# Exact signed lift. Each vertex denotes epsilon * (f_i wedge f_j).
f = np.array([[1,0,0,0], [0,1,0,0], [0,0,1,0], [0,0,0,1], [1,1,1,1]], dtype=int)
pairs = list(combinations(range(5), 2))
verts = [(p, sign) for p in pairs for sign in (+1, -1)]
def det4(rows):
    a = np.array(rows, dtype=int)
    # The determinant is integral; floating evaluation is exact after rounding here.
    d = float(np.linalg.det(a))
    assert abs(d - round(d)) < 1e-8
    return round(d)
def B(v, w):
    p, sg = v; q, tg = w
    return sg*tg*det4([f[p[0]], f[p[1]], f[q[0]], f[q[1]]])
G = nx.Graph()
G.add_nodes_from(verts)
G.add_edges_from((v,w) for v,w in combinations(verts,2) if B(v,w)==1)
assert G.number_of_nodes()==20 and G.number_of_edges()==30
assert set(dict(G.degree()).values())=={3}
assert nx.is_connected(G)
assert sum(nx.triangles(G).values())==0
assert all(len(set(G[a]) & set(G[b])) <= 1 for a,b in combinations(G,2))
assert nx.is_isomorphic(G, nx.dodecahedral_graph())
center = ((2,3), +1)  # *(f_3 wedge f_4) = e_12, using zero-based indices.
leaves = sorted(G[center])
assert len(leaves)==3 and all(not G.has_edge(a,b) for a,b in combinations(leaves,2))
print('Graph: 20 vertices, 30 edges, cubic, triangle-free, C4-free, dodecahedral.')
print('Center:', center, 'positive neighbors:', leaves)

# Tutte embedding with a pentagonal outer face gives a readable planar Schlegel view.
planar, embedding = nx.check_planarity(G)
assert planar
faces = []
seen = set()
for a,b in embedding.edges():
    if (a,b) not in seen:
        face = embedding.traverse_face(a,b)
        seen.update((face[i],face[(i+1)%len(face)]) for i in range(len(face)))
        faces.append(face)
assert len(faces)==12 and all(len(face)==5 for face in faces)
distance = nx.single_source_shortest_path_length(G, center)
outer = max(faces, key=lambda face: (min(distance[v] for v in face), sum(distance[v] for v in face)))
inner = [v for v in verts if v not in outer]
outer_pos = {v: np.array([math.cos(2*math.pi*i/5 + math.pi/2),
                          math.sin(2*math.pi*i/5 + math.pi/2)]) for i,v in enumerate(outer)}
M = np.zeros((len(inner),len(inner)))
R = np.zeros((len(inner),2))
index = {v:i for i,v in enumerate(inner)}
for v in inner:
    i=index[v]; M[i,i]=G.degree[v]
    for w in G[v]:
        if w in index: M[i,index[w]]-=1
        else: R[i]+=outer_pos[w]
sol=np.linalg.solve(M,R)
pos={**outer_pos, **{v:sol[index[v]] for v in inner}}
xy = np.array([pos[v] for v in verts])
assert np.isfinite(xy).all()
fig, ax = plt.subplots(figsize=(7.3, 5.75))
fig.subplots_adjust(left=.075, right=.985, bottom=.19, top=.91)
for a,b in G.edges:
    ax.plot([pos[a][0],pos[b][0]], [pos[a][1],pos[b][1]], color=LIGHT,
            lw=1.15, zorder=1, solid_capstyle='round')
for a,b in [(center,l) for l in leaves]:
    ax.plot([pos[a][0],pos[b][0]], [pos[a][1],pos[b][1]], color=ACCENT,
            lw=2.8, zorder=2, solid_capstyle='round')
others = [v for v in verts if v not in [center,*leaves]]
ax.scatter([pos[v][0] for v in others], [pos[v][1] for v in others], s=35,
           color='white', edgecolor=DARK, linewidth=.9, zorder=3)
ax.scatter([pos[v][0] for v in leaves], [pos[v][1] for v in leaves], s=105,
           color='white', edgecolor=ACCENT, linewidth=2, zorder=4)
ax.scatter([pos[center][0]], [pos[center][1]], s=135,
           color=ACCENT, edgecolor=ACCENT, zorder=5)
ax.set_aspect('equal'); ax.axis('off')
ax.set_title('Positive Pfaffian interactions', loc='left', fontsize=14, color=DARK, pad=12)
fig.text(.075,.105, r'$G:\quad vw\in E\ \Longleftrightarrow\ B(v,w)=+1.\qquad'
                  r'\lambda_v=\frac{1}{2},\quad \sum_{w\in N(v)}\lambda_w=\frac{1}{2}.$',
         fontsize=11.2, color=DARK)
fig.text(.075,.055, 'Highlighted star: every center–leaf edge contributes; leaf–leaf terms vanish.',
         fontsize=9.2, color='#505b60')
for ext in ('pdf','png'):
    fig.savefig(OUT/f'pfaffian-graph.{ext}', dpi=220)
plt.close(fig)

# Endpoint caps. The source polygon has vertices (1,0),(s,1),(-1,0),(-s,-1).
# At u=x^2 its exact vertical bounds are A_s(u) and B_s(u) below.
def upper(u,s):
    r=np.sqrt(u)
    if s==1:
        return (u+r)/2
    return np.where(u <= s*s, (u+r)/(1+s), (r-u)/(1-s))
def lower(u,s):
    return (u-np.sqrt(u))/(1+s)
fig, axes = plt.subplots(3, 2, figsize=(8.0,8.15), gridspec_kw={'width_ratios':[1,1.34]})
fig.subplots_adjust(left=.10,right=.97,top=.91,bottom=.11,hspace=.31,wspace=.25)
fig.suptitle(r'Single-tent sources and their squared endpoint caps', x=.1, ha='left',
             fontsize=14, color=DARK)
for row,s in enumerate((0,.5,1)):
    src, cap=axes[row]
    polygon=np.array([[1,0],[s,1],[-1,0],[-s,-1]])
    src.add_patch(Polygon(polygon, closed=True, facecolor=ACCENT, alpha=.12,
                          edgecolor=ACCENT, linewidth=1.8))
    src.plot(polygon[:,0],polygon[:,1],'o',color=ACCENT,ms=3)
    src.axhline(0,color=LIGHT,lw=.65);src.axvline(0,color=LIGHT,lw=.65)
    src.set(xlim=(-1.12,1.12),ylim=(-1.12,1.12),xticks=[-1,0,1],yticks=[-1,0,1])
    src.set_aspect('equal');src.set_xlabel('$x$');src.set_ylabel('$y$',rotation=0,labelpad=9)
    src.text(.03,.92,rf'$K_{{{s:g}}}$',transform=src.transAxes,va='top',fontsize=11,
             bbox=dict(facecolor='white',edgecolor='none',alpha=.75,pad=1))
    u=np.linspace(0,1,700); hi=upper(u,s); lo=lower(u,s)
    assert np.all(hi>=lo-1e-12)
    cap.fill_between(u,lo,hi,color=ACCENT,alpha=.12)
    cap.plot(u,hi,color=ACCENT,lw=1.8)
    cap.plot(u,lo,color=DARK,lw=1.15)
    if 0<s<1:
        cap.plot([s*s],[upper(np.array([s*s]),s)[0]],'o',color=ACCENT,ms=3.6)
        cap.axvline(s*s,color=LIGHT,lw=.7,ls='--')
    cap.axhline(0,color=LIGHT,lw=.65)
    cap.set(xlim=(-.045,1.045),ylim=(-.54,1.09 if s==1 else .78),xticks=[0,.25,.5,.75,1],yticks=[-.5,0,.5,1] if s==1 else [-.5,0,.5])
    cap.set_xlabel('$u=x^2$');cap.set_ylabel('$v=xy$',rotation=0,labelpad=13)
    cap.text(.03,.92,rf'$C_{{{s:g}}}$',transform=cap.transAxes,va='top',fontsize=11,
             bbox=dict(facecolor='white',edgecolor='none',alpha=.75,pad=1))
    for a in (src,cap):
        a.spines[['top','right']].set_visible(False)
        a.tick_params(labelsize=8.5,length=2.5,color=LIGHT)
fig.text(.10,.045, r'$C_s=\{(x^2,xy):(x,y)\in K_s\}$;  shaded regions show geometry only.',
         fontsize=10.3,color=DARK)
for ext in ('pdf','png'):
    fig.savefig(OUT/f'tent-caps.{ext}',dpi=220)
plt.close(fig)
print('Caps: source vertices and exact piecewise boundary functions plotted for s=0, 1/2, 1.')
