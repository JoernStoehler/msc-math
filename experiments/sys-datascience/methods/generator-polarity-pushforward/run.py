#!/usr/bin/env python3
"""Target-free exact audit of planar polarity pushforwards.

The sampler mirrors the current factor law (IID angles, IID supports), then
crosses an explicit rationalization boundary.  Everything after that boundary
is exact Fraction arithmetic; scale factors, support samples, distances, and
residuals are retained as f64 diagnostics.
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
SEED_COUNT = 3

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
def float_candidate_valid(angs, hs, vertices):
    gaps=[angs[(i+1)%len(angs)]-angs[i] + (2*math.pi if i+1==len(angs) else 0.0) for i in range(len(angs))]
    if max(gaps) >= math.pi - 1e-12:
        return False
    normals=[(math.cos(t),math.sin(t)) for t in angs]
    for x,y in vertices:
        if any(a*x+b*y > h+1e-9 for (a,b),h in zip(normals,hs)):
            return False
    return all(any(abs(a*x+b*y-h) <= 1e-8 for x,y in vertices) for a,b,h in [(a,b,h) for (a,b),h in zip(normals,hs)])

def current_factor(n,rng):
    for _ in range(100):
        ang=sorted(rng.random()*2*math.pi for _ in range(n))
        hs=[.8+.4*rng.random() for _ in range(n)]
        raw=intersections(ang,hs)
        if not float_candidate_valid(ang,hs,raw):
            continue
        v=rationalize(raw)
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
    """Area-normalized centered support samples on a fixed 64-direction grid."""
    c=centroid(v); scale=math.sqrt(float(area(v)))
    z=[((float(x)-float(c[0]))/scale,(float(y)-float(c[1]))/scale) for x,y in v]
    return [max(x*math.cos(t)+y*math.sin(t) for x,y in z) for t in [2*math.pi*i/64 for i in range(64)]]
def l2(a,b): return math.sqrt(sum((x-y)**2 for x,y in zip(a,b))/len(a))
def shape_distance(a,b):
    """Support-shape L2 modulo 64-grid cyclic rotation and reflection."""
    n=len(a); best=float("inf")
    for reflected in (a,list(reversed(a))):
        for k in range(n):
            shifted=reflected[k:]+reflected[:k]
            best=min(best,l2(shifted,b))
    return best
def dump_v(v): return [[str(x),str(y)] for x,y in v]
def load_v(a): return [(F(x),F(y)) for x,y in a]
def product_exact(qv,pv):
    """Construct and verify the exact Cartesian H/incidence representation."""
    vertices=[(x,y,u,w) for x,y in qv for u,w in pv]
    facets=[(a,b,F(0),F(0),c) for a,b,c in edges(qv)] + [(F(0),F(0),a,b,c) for a,b,c in edges(pv)]
    incidence=[]
    all_inside=True
    for v in vertices:
        row=[]
        for a,b,c,d,e in facets:
            value=a*v[0]+b*v[1]+c*v[2]+d*v[3]
            all_inside &= value <= e
            row.append(value == e)
        incidence.append(row)
    facet_counts=[sum(row[j] for row in incidence) for j in range(len(facets))]
    vertex_counts=[sum(row) for row in incidence]
    expected_facets=[2*len(pv)]*len(qv)+[2*len(qv)]*len(pv)
    valid=(all_inside and len(vertices)==len(qv)*len(pv)
           and len(facets)==len(qv)+len(pv)
           and facet_counts==expected_facets
           and all(count==4 for count in vertex_counts))
    return {"vertices":vertices,"facets":facets,"incidence":incidence,"valid":valid,"facet_counts":facet_counts,"vertex_counts":vertex_counts}
def validate_product_arm_linkage(arms, panel):
    """Fail closed unless each four-arm cell uses exactly its paired images."""
    rows={row["source_id"]:row for row in panel}
    groups={}
    for arm in arms: groups.setdefault(arm["pair_id"],{})[arm["arm"]]=arm
    for pair_id,group in groups.items():
        if set(group)!={"QxP","QpolarxP","QxPpolar","QpolarxPpolar"}: raise RuntimeError(f"incomplete product arm linkage: {pair_id}")
        base=group["QxP"]
        q_source=rows.get(base["q_source_id"]); p_source=rows.get(base["p_source_id"])
        if q_source is None or p_source is None: raise RuntimeError(f"unknown product source ID: {pair_id}")
        expected={"QxP":(None,None),"QpolarxP":(q_source["image_ids"]["centroid"],None),"QxPpolar":(None,p_source["image_ids"]["centroid"]),"QpolarxPpolar":(q_source["image_ids"]["centroid"],p_source["image_ids"]["centroid"])}
        for label,arm in group.items():
            if arm["q_source_id"]!=base["q_source_id"] or arm["p_source_id"]!=base["p_source_id"]: raise RuntimeError(f"source linkage mismatch: {pair_id}")
            if (arm["q_polar_image_id"],arm["p_polar_image_id"])!=expected[label]: raise RuntimeError(f"polar image linkage mismatch: {pair_id}/{label}")
def exact_fixture_rows():
    tri=[(F(-1),F(-1)),(F(2),F(-1)),(F(-1),F(2))]
    nons=[(F(-2),F(-1)),(F(1),F(-1)),(F(2),F(1)),(F(-1),F(2))]
    sq=[(F(-1),F(-1)),(F(1),F(-1)),(F(1),F(1)),(F(-1),F(1))]
    shifted=[(x+F(1,4),y+F(1,5)) for x,y in nons]
    p=polar(nons); pp=polar(p)
    c=centroid(nons); cp=polar(nons,c)
    cp_shift=polar(shifted,(c[0]+F(1,4),c[1]+F(1,5)))
    raw_shift=polar(shifted,(F(0),F(0)))
    raw_failure={"status":"defined: raw origin remains interior", "raw_vs_centroid_residual":residual(raw_shift,cp_shift)}
    rp=polar(polar(nons,c),centroid(polar(nons,c)))
    scaled=[(2*x,2*y) for x,y in nons]
    translated=[(x+7,y-3) for x,y in nons]
    rotated=[(-y,x) for x,y in nons]
    reflected=[(-x,y) for x,y in nons]
    return [
      {"fixture":"non-self-polar","source":dump_v(nons),"polar":dump_v(p),"area_source":str(area(nons)),"area_polar":str(area(p)),"mahler":str(area(nons)*area(p))},
      {"fixture":"marked-double-polar","residual":residual(translate(pp,(F(0),F(0))),nons),"source":dump_v(nons),"double":dump_v(pp)},
      {"fixture":"centroid-translation-covariance","residual":residual(cp_shift,[(x,y) for x,y in cp])},
      {"fixture":"raw-origin-translation-failure",**raw_failure},
      {"fixture":"area-normalization-correction","source_area":str(area(nons)),"polar_area":str(area(p)),"source_scale_squared":str(1/area(nons)),"polar_scale_squared":str(1/area(p)),"normalized_source_area":str(area(nons)*(1/area(nons))),"normalized_polar_area":str(area(p)*(1/area(p))),"normalized_product":str((area(nons)*(1/area(nons)))*(area(p)*(1/area(p))))},
      {"fixture":"recenter-every-step-non-involution","residual":residual(rp,translate(nons,c)),"status":"fails" if residual(rp,translate(nons,c)) and residual(rp,translate(nons,c))>1e-9 else "inconclusive"},
      {"fixture":"symmetric-double-polar-negative-control","residual":residual(polar(polar(sq)),sq),"single_polar_shape_residual":residual(polar(sq),sq),"mahler":str(area(sq)*area(polar(sq)))},
      {"fixture":"support-metric-invariance-controls","scale":shape_distance(shape(nons),shape(scaled)),"translation":shape_distance(shape(nons),shape(translated)),"rotation_90":shape_distance(shape(nons),shape(rotated)),"reflection":shape_distance(shape(nons),shape(reflected)),"distinct_triangle":shape_distance(shape(nons),shape(tri))},
    ]
def git_info():
    def run(*a): return subprocess.check_output(a,cwd=ROOT,text=True).strip()
    return run("git","rev-parse","HEAD"),run("git","rev-parse","HEAD^{tree}"),bool(run("git","status","--porcelain=v1","--untracked-files=no"))
def main():
    ap=argparse.ArgumentParser(); ap.add_argument("--out-dir",default=str(Path(__file__).parent/"artifacts")); ap.add_argument("--seed",type=int,default=20260715); ap.add_argument("--per-stratum",type=int,default=N); args=ap.parse_args()
    if args.per_stratum<24 or args.per_stratum % SEED_COUNT: ap.error("--per-stratum must be >=24 and divisible by 3")
    out=Path(args.out_dir); out.mkdir(parents=True,exist_ok=True)
    rev,tree,dirty=git_info(); rows=[]; factors={}; generation_failures=[]
    seeds=[args.seed+i for i in range(SEED_COUNT)]; per_seed=args.per_stratum//SEED_COUNT
    for n in STRATA:
      for seed in seeds:
       for i in range(per_seed):
        rng=random.Random(seed*1000+n*100+i)
        try: v=current_factor(n,rng)
        except RuntimeError:
            generation_failures.append({"stratum":n,"seed":seed,"row":i,"reason":"bounded current-law draw exhausted"}); continue
        c=centroid(v)
        pm=polar(v); cp=polar(v,c); dpm=polar(pm); dcp=polar(cp)
        sid=f"{LAW_VERSION}/source/n={n}/seed={seed}/row={i}"; factors[sid]=v
        row={"schema":"generator-polarity-row-v1","source_id":sid,"pair_id":sid+"/pair","source_pair_id":f"generator-polarity/source/n={n}/seed={seed}/pair={i//2}","image_ids":{"preserved_mark":sid+"/preserved-mark-polar","centroid":sid+"/centroid-polar","preserved_double":sid+"/preserved-double","centroid_double":sid+"/centroid-double"},"stratum":n,"seed":seed,"row":i,"rationalization":"Float current-law vertices pass cyclic-gap/halfspace/all-facets-active checks. Vertices then cross a Fraction.limit_denominator(1e9) reconstruction and exact convex hull; rational vertices/facets/incidence/areas/centroids/polars/Mahler fields are exact after this boundary. scale_f64, support samples/distances, and stored residual numbers are f64 diagnostics.","source":dump_v(v),"preserved_mark":["0","0"],"centroid":[str(c[0]),str(c[1])],"preserved_mark_polar":dump_v(pm),"centroid_polar":dump_v(cp),"preserved_double_polar":dump_v(dpm),"centroid_double_polar":dump_v(dcp),"source_area":str(area(v)),"polar_area":str(area(cp)),"source_meta":normalize_meta(v),"polar_meta":normalize_meta(cp),"source_sides":len(v),"polar_sides":len(cp),"bounded":inside(v),"irredundant":len(edges(v))==len(v),"preserved_double_residual":residual(dpm,v),"centroid_double_residual":residual(dcp,translate(v,c)),"mahler":str(area(translate(v,c))*area(cp)),"shape_source":shape(v),"shape_polar":shape(cp)}
        rows.append(row)
    with (out/"panel.jsonl").open("w") as f:
      for r in rows: f.write(json.dumps(r,sort_keys=True)+"\n")
    # Paired diversity and product arms; all exact factor areas/volumes retained.
    div=[]; arms=[]
    by={n:[r for r in rows if r["stratum"]==n] for n in STRATA}
    for n,rs in by.items():
      src=[r["shape_source"] for r in rs]; pol=[r["shape_polar"] for r in rs]
      within_src=sum(shape_distance(src[i],src[j]) for i in range(len(src)) for j in range(i))/max(1,len(src)*(len(src)-1)/2)
      within_pol=sum(shape_distance(pol[i],pol[j]) for i in range(len(pol)) for j in range(i))/max(1,len(pol)*(len(pol)-1)/2)
      included=[]; leave_pair_out=[]; paired_wins=0
      for i in range(len(pol)):
        distances=[shape_distance(pol[i],src[j]) for j in range(len(src))]
        best=min(range(len(src)),key=lambda j: distances[j]); included.append(distances[best])
        paired=2*(i//2)
        if best in (paired,paired+1): paired_wins += 1
        leave=[distances[j] for j in range(len(src)) if j not in (paired,paired+1)]
        leave_pair_out.append(min(leave))
      cross=sum(included)/len(included); cross_leave=sum(leave_pair_out)/len(leave_pair_out)
      div.append({"stratum":n,"source_count":len(rs),"within_source_l2":within_src,"within_polar_l2":within_pol,"directed_polar_to_source_nearest_paired_included":cross,"directed_polar_to_source_nearest_leave_pair_out":cross_leave,"paired_source_wins":paired_wins})
      for k in range(min(len(rs)//2,12)):
        q=load_v(rs[2*k]["source"]); p=load_v(rs[2*k+1]["source"]); qp=load_v(rs[2*k]["centroid_polar"]); pp=load_v(rs[2*k+1]["centroid_polar"])
        pair_seed=rs[2*k]["seed"]
        arm_specs=[("QxP",q,p,None,None),("QpolarxP",qp,p,rs[2*k]["image_ids"]["centroid"],None),("QxPpolar",q,pp,None,rs[2*k+1]["image_ids"]["centroid"]),("QpolarxPpolar",qp,pp,rs[2*k]["image_ids"]["centroid"],rs[2*k+1]["image_ids"]["centroid"])]
        for label,a,b,q_polar_image_id,p_polar_image_id in arm_specs:
          product=product_exact(a,b)
          arms.append({"schema":"generator-polarity-product-arm-v1","arm":label,"stratum":n,"seed":pair_seed,"pair_id":f"n={n}/seed={pair_seed}/pair={k}","q_source_id":rs[2*k]["source_id"],"p_source_id":rs[2*k+1]["source_id"],"q_polar_image_id":q_polar_image_id,"p_polar_image_id":p_polar_image_id,"q_sides":len(a),"p_sides":len(b),"product_facets":len(product["facets"]),"product_vertices":len(product["vertices"]),"facet_incidence_counts":product["facet_counts"],"vertex_incidence_counts":product["vertex_counts"],"exact_reconstruction":product["valid"],"incidence_valid":product["valid"],"q_area":str(area(a)),"p_area":str(area(b)),"q_scale_squared":str(1/area(a)),"p_scale_squared":str(1/area(b)),"volume":str(area(a)*area(b)),"normalized_volume":"1","q_bounded":inside(a),"p_bounded":inside(b)})
    validate_product_arm_linkage(arms, rows)
    (out/"diversity.tsv").write_text("stratum\tsource_count\twithin_source_l2\twithin_polar_l2\tdirected_polar_to_source_nearest_paired_included\tdirected_polar_to_source_nearest_leave_pair_out\tpaired_source_wins\n"+"\n".join(f"{d['stratum']}\t{d['source_count']}\t{d['within_source_l2']:.12g}\t{d['within_polar_l2']:.12g}\t{d['directed_polar_to_source_nearest_paired_included']:.12g}\t{d['directed_polar_to_source_nearest_leave_pair_out']:.12g}\t{d['paired_source_wins']}" for d in div)+"\n")
    (out/"product-arms.jsonl").write_text("".join(json.dumps(x,sort_keys=True)+"\n" for x in arms))
    fixtures=exact_fixture_rows(); (out/"fixtures.json").write_text(json.dumps(fixtures,indent=2,sort_keys=True)+"\n")
    producer_hash=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    names={"panel":"panel.jsonl","diversity":"diversity.tsv","product_arms":"product-arms.jsonl","fixtures":"fixtures.json"}
    hashes={k:hashlib.sha256((out/v).read_bytes()).hexdigest() for k,v in names.items()}
    seed_counts={str(seed):sum(r["seed"]==seed for r in rows) for seed in seeds}
    seed_stratum_counts={str(seed):{str(n):sum(r["seed"]==seed and r["stratum"]==n for r in rows) for n in STRATA} for seed in seeds}
    manifest={"schema":"generator-polarity-manifest-v1","law_version":LAW_VERSION,"source_revision":rev,"source_tree":tree,"source_dirty_tracked":dirty,"producer_sha256":producer_hash,"seed":args.seed,"seeds":seeds,"seed_counts":seed_counts,"seed_stratum_counts":seed_stratum_counts,"generation_failures":generation_failures,"strata":list(STRATA),"rows_per_stratum":args.per_stratum,"panel_rows":len(rows),"product_arm_rows":len(arms),"exact_boundary":"Current-law f64 candidates first pass cyclic-gap, original-halfspace, and all-facets-active checks. After Fraction.limit_denominator(1e9) reconstruction and exact hull, rational vertices/facets/incidence/areas/centroids/polars/Mahler are exact. scale_f64, support samples/distances, and stored residual numbers are f64 diagnostics.","dependencies":"Python stdlib only","fixtures":len(fixtures),"target_evaluation":False,"sys_evaluation":False,"capacity_claim":False,"volatile_timings_excluded":True,"input_files":[],"output_files":names,"output_sha256":hashes}
    (out/"manifest.json").write_text(json.dumps(manifest,indent=2,sort_keys=True)+"\n")
    fixture_map={x["fixture"]:x for x in fixtures}
    report=f"""# Polarity pushforward and center audit

This is a target-free finite-panel audit of planar generator transfer. It does not evaluate `sys`, any capacity, or a target-derived feature.

## Retained panel

- Source law: IID sorted normal angles and IID support heights in `[0.8,1.2)`, with max cyclic angular gap `< pi`, every intersection satisfying every original halfspace, and every input line active before explicit `Fraction.limit_denominator(1e9)` reconstruction and an exact hull.
- Exact panel: {len(rows)} source/image pairs, strata `n=3,4,6`, {args.per_stratum} per stratum across seeds `{','.join(map(str,seeds))}`. Aggregate seed counts are `{seed_counts}` and seed-by-stratum counts are `{seed_stratum_counts}`; bounded-generation failures are `{len(generation_failures)}`. Every row retains source, preserved-mark polar, centroid polar, and both double-polar controls.
- Product arms: {len(arms)} exact rows ({len(arms)//4} paired cells, four arms per cell: `QxP`, `Q^circ x P`, `Q x P^circ`, `Q^circ x P^circ`). Cartesian H reconstruction, incidence counts, and volume `area(Q) area(P)` are exact; every row carries source IDs and polar image IDs where used.
- No relative-rotation knob is used. Shape views are centered, area-normalized support samples on a 64-direction grid; distances minimize cyclic shifts and reflection. Diversity reports paired-source-included and leave-pair-out directed polar-to-source nearest views separately; the included view is not leave-pair-out and neither is an independence estimate. Factor area normalization is recorded by exact `scale_squared=1/area`; normalized product volume is the exact string `1`, while raw rational areas/volumes remain available.

## Mathematical controls

The marked law translates by an explicitly preserved interior mark `c`, applies `B^circ={{y:<x,y><=1 for x in B}}`, and retains the translated origin mark. The centroid law chooses the exact area centroid and records that choice. The exact fixture results are in `fixtures.json`: marked double polarity residual `{fixture_map['marked-double-polar']['residual']}`, centroid translation covariance residual `{fixture_map['centroid-translation-covariance']['residual']}`, raw-origin translation status `{fixture_map['raw-origin-translation-failure'].get('status','defined')}` with raw-vs-centroid residual `{fixture_map['raw-origin-translation-failure'].get('raw_vs_centroid_residual')}`, and recenter-every-step residual `{fixture_map['recenter-every-step-non-involution']['residual']}` (non-involution). The symmetric double-polar negative control has residual `{fixture_map['symmetric-double-polar-negative-control']['residual']}`. Synthetic support-metric controls are in `fixtures.json`; scale, translation, 90-degree rotation, and reflection should be zero while the distinct-triangle control should be positive.

## Interpretation boundary

Polar images are deterministic pushforwards paired to their sources. `P_#mu` is not an independent law draw, double polarity is not new coverage, and finite-panel nearest-cross summaries are not proof of law equivalence. These rows support only geometric calibration and descriptive finite-panel comparisons; they do not support density/support exhaustion, population rank, independence, effective sample-size, target, `sys`, or capacity claims. Raw-origin polarity language is prohibited unless an origin mark is explicitly supplied. Rational vertices/facets/incidence/areas/centroids/polars/Mahler fields are exact after the rationalization boundary. `scale_f64`, support samples, distances, and stored residual numbers are f64 diagnostics, even when derived from exact equalities.

## Provenance and replay

Source revision `{rev}`, tree `{tree}`, tracked-dirty before artifact creation `{str(dirty).lower()}`. Producer SHA-256 `{producer_hash}`. Python dependencies are standard library only. Reproduce from that source revision (the later artifact commit changes repository `HEAD`) with:

```text
git worktree add --detach /tmp/generator-polarity-replay {rev}
python3 /tmp/generator-polarity-replay/experiments/sys-datascience/methods/generator-polarity-pushforward/run.py --out-dir /tmp/polarity-artifacts --seed {args.seed} --per-stratum {args.per_stratum}
sha256sum /tmp/polarity-artifacts/{{panel.jsonl,diversity.tsv,product-arms.jsonl,fixtures.json}}
```

Volatile timings are excluded. No external or LFS input is used.
"""
    (out/"REPORT.md").write_text(report)
    print(json.dumps({"panel":len(rows),"arms":len(arms),"fixtures":len(fixtures),"out":str(out),"source_revision":rev},sort_keys=True))
if __name__=="__main__": main()
