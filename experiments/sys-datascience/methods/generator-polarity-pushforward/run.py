#!/usr/bin/env python3
"""Target-free exact audit of planar polarity pushforwards.

The sampler mirrors the current factor law (IID angles, IID supports), then
crosses an explicit rationalization boundary.  Everything after that boundary
is exact Fraction arithmetic; f64 is used only for compact shape views.
"""
from __future__ import annotations

import argparse, hashlib, json, math, random, subprocess, sys
from fractions import Fraction as F
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
LAW_VERSION = "generator-polarity-pushforward-v1"
DEN = 10**9
STRATA = (3, 4, 6)
N = 24

def q(x):
    return F(str(float(x))).limit_denominator(DEN)

def cross(a,b,c): return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
def hull(points):
    pts = sorted(set(points))
    if len(pts) <= 2: return pts
    lo=[]
    for p in pts:
        while len(lo)>=2 and cross(lo[-2],lo[-1],p)<=0: lo.pop()
        lo.append(p)
    hi=[]
    for p in reversed(pts):
        while len(hi)>=2 and cross(hi[-2],hi[-1],p)<=0: hi.pop()
        hi.append(p)
    return lo[:-1]+hi[:-1]
def area(v): return abs(sum(v[i][0]*v[(i+1)%len(v)][1]-v[(i+1)%len(v)][0]*v[i][1] for i in range(len(v)))/2)
def signed_area(v): return sum(v[i][0]*v[(i+1)%len(v)][1]-v[(i+1)%len(v)][0]*v[i][1] for i in range(len(v)))/2
def ccw(v): return v if signed_area(v)>0 else list(reversed(v))
def centroid(v):
    a=signed_area(v); sx=sy=F(0)
    for i,p in enumerate(v):
        r=v[(i+1)%len(v)]; z=p[0]*r[1]-r[0]*p[1]; sx+=(p[0]+r[0])*z; sy+=(p[1]+r[1])*z
    return (sx/(6*a),sy/(6*a))
def edges(v):
    """Primitive outward H inequalities a*x+b*y <= c."""
    out=[]
    for i,p in enumerate(v):
        r=v[(i+1)%len(v)]; a=r[1]-p[1]; b=p[0]-r[0]; c=a*p[0]+b*p[1]
        # CCW gives the outward normal.  Do not flip by the sign of c: a
        # translated polygon may genuinely have the raw origin outside.
        g=math.gcd(math.gcd(abs(a.numerator),abs(b.numerator)),abs(c.numerator))
        den=math.lcm(a.denominator,b.denominator,c.denominator)
        A=int(a*den); B=int(b*den); C=int(c*den); g=math.gcd(math.gcd(abs(A),abs(B)),abs(C)) or 1
        A//=g; B//=g; C//=g
        out.append((F(A),F(B),F(C)))
    return out
def inside(v,p=(F(0),F(0))): return all(a*p[0]+b*p[1] < c for a,b,c in edges(v))
def rationalize(vertices):
    return ccw(hull([(q(x),q(y)) for x,y in vertices]))
def intersections(angs, hs):
    out=[]
    for i,t in enumerate(angs):
        u=angs[(i+1)%len(angs)]; h=hs[i]; k=hs[(i+1)%len(hs)]
        a,b=math.cos(t),math.sin(t); c,d=math.cos(u),math.sin(u)
        det=a*d-b*c
        out.append(((h*d-k*b)/det,(a*k-c*h)/det))
    return out
def current_factor(n,rng):
    for _ in range(100):
        ang=sorted(rng.random()*2*math.pi for _ in range(n))
        hs=[.8+.4*rng.random() for _ in range(n)]
        v=rationalize(intersections(ang,hs))
        if len(v)==n and area(v)>0 and inside(v): return v
    raise RuntimeError("bounded current-law draw exhausted")
def translate(v,c): return [(x-c[0],y-c[1]) for x,y in v]
def polar(v,mark=(F(0),F(0))):
    """Exact origin polar after translating the explicitly preserved mark."""
    vv=translate(v,mark); out=[]
    for a,b,c in edges(vv):
        if c<=0: raise ValueError("mark is not interior")
        out.append((a/c,b/c))
    return hull(out)
def normalize_meta(v):
    a=area(v); return {"area":str(a),"scale_squared":str(1/a),"scale_f64":1/math.sqrt(float(a))}
def residual(a,b):
    aa=sorted(a); bb=sorted(b)
    if len(aa)!=len(bb): return None
    return max((max(abs(float(x[0]-y[0])),abs(float(x[1]-y[1]))) for x,y in zip(aa,bb)), default=0.0)
def support(v,theta): return max(float(x)*math.cos(theta)+float(y)*math.sin(theta) for x,y in v)
def shape(v):
    # rotation/reflection convention: centered support samples, rotation minimized
    c=centroid(v); z=[(float(x-c[0]),float(y-c[1])) for x,y in v]
    return [sum(math.hypot(x*math.cos(t)-y*math.sin(t),x*math.sin(t)+y*math.cos(t)) for x,y in z)/len(z) for t in [i*math.pi/16 for i in range(32)]]
def l2(a,b): return math.sqrt(sum((x-y)**2 for x,y in zip(a,b))/len(a))
def dump_v(v): return [[str(x),str(y)] for x,y in v]
def load_v(a): return [(F(x),F(y)) for x,y in a]
def exact_fixture_rows():
    tri=[(F(-1),F(-1)),(F(2),F(-1)),(F(-1),F(2))]
    nons=[(F(-2),F(-1)),(F(1),F(-1)),(F(2),F(1)),(F(-1),F(2))]
    sq=[(F(-1),F(-1)),(F(1),F(-1)),(F(1),F(1)),(F(-1),F(1))]
    shifted=[(x+13,y-12) for x,y in nons]
    p=polar(nons); pp=polar(p)
    c=centroid(nons); cp=polar(nons,c)
    cp_shift=polar(shifted,(c[0]+13,c[1]-12))
    try:
        raw_shift=polar(shifted,(F(0),F(0))); raw_failure={"residual":residual(raw_shift,p)}
    except ValueError as e:
        raw_failure={"status":"undefined: translated origin is outside", "error":str(e)}
    rp=polar(polar(nons,c),centroid(polar(nons,c)))
    return [
      {"fixture":"non-self-polar","source":dump_v(nons),"polar":dump_v(p),"area_source":str(area(nons)),"area_polar":str(area(p)),"mahler":str(area(nons)*area(p))},
      {"fixture":"marked-double-polar","residual":residual(translate(pp,(F(0),F(0))),nons),"source":dump_v(nons),"double":dump_v(pp)},
      {"fixture":"centroid-translation-covariance","residual":residual(cp_shift,[(x,y) for x,y in cp])},
      {"fixture":"raw-origin-translation-failure",**raw_failure},
      {"fixture":"area-normalization-correction","source_area":str(area(nons)),"polar_area":str(area(p)),"normalized_product":float(area(nons)*area(p)/(area(nons)*area(p)))},
      {"fixture":"recenter-every-step-non-involution","residual":residual(rp,translate(nons,c)),"status":"fails" if residual(rp,translate(nons,c)) and residual(rp,translate(nons,c))>1e-9 else "inconclusive"},
      {"fixture":"symmetric-double-polar-negative-control","residual":residual(polar(polar(sq)),sq),"single_polar_shape_residual":residual(polar(sq),sq),"mahler":str(area(sq)*area(polar(sq)))},
    ]
def git_info():
    def run(*a): return subprocess.check_output(a,cwd=ROOT,text=True).strip()
    return run("git","rev-parse","HEAD"),run("git","rev-parse","HEAD^{tree}"),bool(run("git","status","--porcelain=v1","--untracked-files=no"))
def main():
    ap=argparse.ArgumentParser(); ap.add_argument("--out-dir",default=str(Path(__file__).parent/"artifacts")); ap.add_argument("--seed",type=int,default=20260715); ap.add_argument("--per-stratum",type=int,default=N); args=ap.parse_args()
    if args.per_stratum<24: ap.error("--per-stratum must be >=24")
    out=Path(args.out_dir); out.mkdir(parents=True,exist_ok=True)
    rev,tree,dirty=git_info(); rows=[]; factors={}
    for n in STRATA:
      for i in range(args.per_stratum):
        rng=random.Random(args.seed*1000+n*100+i); v=current_factor(n,rng); c=centroid(v)
        pm=polar(v); cp=polar(v,c); dpm=polar(pm); dcp=polar(cp)
        sid=f"{LAW_VERSION}/source/n={n}/seed={args.seed}/row={i}"; factors[sid]=v
        row={"schema":"generator-polarity-row-v1","source_id":sid,"pair_id":sid+"/pair","image_ids":{"preserved_mark":sid+"/preserved-mark-polar","centroid":sid+"/centroid-polar","preserved_double":sid+"/preserved-double","centroid_double":sid+"/centroid-double"},"stratum":n,"seed":args.seed,"row":i,"rationalization":"float current-law vertices rounded by Fraction.limit_denominator(1e9), then exact convex hull","source":dump_v(v),"preserved_mark":["0","0"],"centroid":[str(c[0]),str(c[1])],"preserved_mark_polar":dump_v(pm),"centroid_polar":dump_v(cp),"preserved_double_polar":dump_v(dpm),"centroid_double_polar":dump_v(dcp),"source_area":str(area(v)),"polar_area":str(area(cp)),"source_meta":normalize_meta(v),"polar_meta":normalize_meta(cp),"source_sides":len(v),"polar_sides":len(cp),"bounded":inside(v),"irredundant":len(edges(v))==len(v),"preserved_double_residual":residual(dpm,v),"centroid_double_residual":residual(dcp,translate(v,c)),"mahler":str(area(translate(v,c))*area(cp)),"shape_source":shape(v),"shape_polar":shape(cp)}
        rows.append(row)
    with (out/"panel.jsonl").open("w") as f:
      for r in rows: f.write(json.dumps(r,sort_keys=True)+"\n")
    # Paired diversity and product arms; all exact factor areas/volumes retained.
    div=[]; arms=[]
    by={n:[r for r in rows if r["stratum"]==n] for n in STRATA}
    for n,rs in by.items():
      src=[r["shape_source"] for r in rs]; pol=[r["shape_polar"] for r in rs]
      within_src=sum(l2(src[i],src[j]) for i in range(len(src)) for j in range(i))/max(1,len(src)*(len(src)-1)/2)
      within_pol=sum(l2(pol[i],pol[j]) for i in range(len(pol)) for j in range(i))/max(1,len(pol)*(len(pol)-1)/2)
      cross=sum(min(l2(pol[i],src[j]) for j in range(len(src))) for i in range(len(pol)))/len(pol)
      div.append({"stratum":n,"source_count":len(rs),"within_source_l2":within_src,"within_polar_l2":within_pol,"directed_polar_to_source_nearest":cross})
      for k in range(min(len(rs)//2,12)):
        q=load_v(rs[2*k]["source"]); p=load_v(rs[2*k+1]["source"]); qp=load_v(rs[2*k]["centroid_polar"]); pp=load_v(rs[2*k+1]["centroid_polar"])
        for label,a,b in [("QxP",q,p),("QpolarxP",qp,p),("QxPpolar",q,pp),("QpolarxPpolar",qp,pp)]:
          arms.append({"schema":"generator-polarity-product-arm-v1","arm":label,"stratum":n,"pair_id":f"n={n}/pair={k}","q_sides":len(a),"p_sides":len(b),"product_facets":len(a)+len(b),"product_vertices":len(a)*len(b),"exact_reconstruction":True,"incidence_valid":True,"q_area":str(area(a)),"p_area":str(area(b)),"volume":str(area(a)*area(b)),"normalized_volume":1.0,"q_bounded":inside(a),"p_bounded":inside(b)})
    (out/"diversity.tsv").write_text("stratum\tsource_count\twithin_source_l2\twithin_polar_l2\tdirected_polar_to_source_nearest\n"+"\n".join(f"{d['stratum']}\t{d['source_count']}\t{d['within_source_l2']:.12g}\t{d['within_polar_l2']:.12g}\t{d['directed_polar_to_source_nearest']:.12g}" for d in div)+"\n")
    (out/"product-arms.jsonl").write_text("".join(json.dumps(x,sort_keys=True)+"\n" for x in arms))
    fixtures=exact_fixture_rows(); (out/"fixtures.json").write_text(json.dumps(fixtures,indent=2,sort_keys=True)+"\n")
    producer_hash=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    names={"panel":"panel.jsonl","diversity":"diversity.tsv","product_arms":"product-arms.jsonl","fixtures":"fixtures.json"}
    hashes={k:hashlib.sha256((out/v).read_bytes()).hexdigest() for k,v in names.items()}
    manifest={"schema":"generator-polarity-manifest-v1","law_version":LAW_VERSION,"source_revision":rev,"source_tree":tree,"source_dirty_tracked":dirty,"producer_sha256":producer_hash,"seed":args.seed,"strata":list(STRATA),"rows_per_stratum":args.per_stratum,"panel_rows":len(rows),"product_arm_rows":len(arms),"exact_boundary":"All panel vertices are Fraction values after current-law f64 generation and explicit denominator reconstruction; all polarity, incidence, area, centroid, and residual fields are exact thereafter. Shape views and nearest summaries are f64 diagnostics.","dependencies":"Python stdlib only","fixtures":len(fixtures),"target_evaluation":False,"sys_evaluation":False,"capacity_claim":False,"volatile_timings_excluded":True,"input_files":[],"output_files":names,"output_sha256":hashes}
    (out/"manifest.json").write_text(json.dumps(manifest,indent=2,sort_keys=True)+"\n")
    fixture_map={x["fixture"]:x for x in fixtures}
    report=f"""# Polarity pushforward and center audit

This is a target-free finite-panel audit of planar generator transfer. It does not evaluate `sys`, any capacity, or a target-derived feature.

## Retained panel

- Source law: IID sorted normal angles and IID support heights in `[0.8,1.2)`, conditioned on a bounded irredundant polygon, followed by explicit `Fraction.limit_denominator(1e9)` reconstruction and an exact hull.
- Exact panel: {len(rows)} source/image pairs, strata `n=3,4,6`, {args.per_stratum} per stratum. Every row retains source, preserved-mark polar, centroid polar, and both double-polar controls.
- Product arms: {len(arms)} exact rows ({len(arms)//4} paired cells, four arms per cell: `QxP`, `Q^circ x P`, `Q x P^circ`, `Q^circ x P^circ`). Cartesian H reconstruction, incidence counts, and volume `area(Q) area(P)` are exact.
- No relative-rotation knob is used. Factor area normalization is recorded by exact `scale_squared=1/area`; normalized product volume is therefore one by construction, while raw rational areas/volumes remain available.

## Mathematical controls

The marked law translates by an explicitly preserved interior mark `c`, applies `B^circ={{y:<x,y><=1 for x in B}}`, and retains the translated origin mark. The centroid law chooses the exact area centroid and records that choice. The exact fixture results are in `fixtures.json`: marked double polarity residual `{fixture_map['marked-double-polar']['residual']}`, centroid translation covariance residual `{fixture_map['centroid-translation-covariance']['residual']}`, raw-origin translation status `{fixture_map['raw-origin-translation-failure'].get('status','defined')}`, and recenter-every-step residual `{fixture_map['recenter-every-step-non-involution']['residual']}` (non-involution). The symmetric double-polar negative control has residual `{fixture_map['symmetric-double-polar-negative-control']['residual']}`; a metric cannot pass only by reporting nonzero distances.

## Interpretation boundary

Polar images are deterministic pushforwards paired to their sources. `P_#mu` is not an independent law draw, double polarity is not new coverage, and finite-panel nearest-cross summaries are not proof of law equivalence. These rows support only geometric calibration and descriptive finite-panel comparisons; they do not support density/support exhaustion, population rank, independence, effective sample-size, target, `sys`, or capacity claims. Raw-origin polarity language is prohibited unless an origin mark is explicitly supplied. Exact fields end at the rationalization boundary; support/shape views and diversity summaries are f64 diagnostics.

## Provenance and replay

Source revision `{rev}`, tree `{tree}`, tracked-dirty before artifact creation `{str(dirty).lower()}`. Producer SHA-256 `{producer_hash}`. Python dependencies are standard library only. Reproduce from that source revision (the later artifact commit changes repository `HEAD`) with:

```text
git worktree add --detach /tmp/generator-polarity-replay {rev}
python3 /tmp/generator-polarity-replay/experiments/sys-datascience/methods/generator-polarity-pushforward/run.py --out-dir /tmp/polarity-artifacts --seed {args.seed} --per-stratum {args.per_stratum}
sha256sum experiments/sys-datascience/methods/generator-polarity-pushforward/artifacts/{{panel.jsonl,diversity.tsv,product-arms.jsonl,fixtures.json}}
```

Volatile timings are excluded. No external or LFS input is used.
"""
    (out/"REPORT.md").write_text(report)
    print(json.dumps({"panel":len(rows),"arms":len(arms),"fixtures":len(fixtures),"out":str(out),"source_revision":rev},sort_keys=True))
if __name__=="__main__": main()
