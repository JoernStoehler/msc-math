#!/usr/bin/env python3
"""Frozen inscribed affine-pentagon feature. NumPy only; --figure needs Matplotlib.
No capacity producer or fitted parameter. Only --out is written.
"""
import argparse,csv,gzip,hashlib,json,math,time
from pathlib import Path
import numpy as np
SOURCE_SHA='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
WIDTH_SHA='dcffead16ab5ad0fcdf6ae55d78375bea5b6ee7274877a33846cf4c3c2a7d57f'
P5=np.c_[np.cos(2*np.pi*np.arange(5)/5),np.sin(2*np.pi*np.arange(5)/5)]
N5=np.c_[np.cos(np.pi/5+2*np.pi*np.arange(5)/5),np.sin(np.pi/5+2*np.pi*np.arange(5)/5)]
H=math.cos(math.pi/5);W5=1+H;AREA5=2.5*math.sin(2*math.pi/5);HKO=(3+math.sqrt(5))/5

def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def polygon(a):
    a=a[np.argsort(np.arctan2(a[:,1],a[:,0]))]
    return np.array([np.linalg.solve(a[[i,(i+1)%len(a)]],np.ones(2)) for i in range(len(a))])
def area(v):return abs(float(np.sum(v[:,0]*np.roll(v[:,1],-1)-v[:,1]*np.roll(v[:,0],-1))))/2

def core(a):
    v=polygon(a);m=v.mean(axis=0);z=v-m
    # SVD computes the positive square root without squaring condition numbers.
    _,s,vt=np.linalg.svd(z,full_matrices=False)
    B=(vt.T*(s*np.sqrt(2/len(v))))@vt
    y=np.linalg.solve(B,z.T).T
    phases=np.mod(np.arctan2(y[:,1],y[:,0]),2*np.pi/5)
    height=1-a@m;assert height.min()>0
    candidates=[]
    for i,theta in enumerate(phases):
        R=np.array([[math.cos(theta),-math.sin(theta)],[math.sin(theta),math.cos(theta)]])
        G0=B@R;support=np.max((a@G0)@P5.T,axis=1);assert support.min()>0
        scale=float(np.min(height/support));assert scale>0
        candidates.append((scale,float(theta),i,G0))
    best=max(x[0] for x in candidates)
    scale,theta,index,G0=min((x for x in candidates if x[0]>=best*(1-1e-12)),key=lambda x:(x[1],x[2]))
    G=scale*G0;verts=m+P5@G.T
    violation=float(np.max(a@verts.T-1))
    area_original=area(v);area_core=abs(float(np.linalg.det(G)))*AREA5
    assert area_original>0 and area_core>0
    assert violation<1e-8 and area_core<=area_original*(1+1e-8)
    return dict(G=G.tolist(),center=m.tolist(),scale=scale,phase=theta,anchor_index=index,candidate_count=len(candidates),area_original=area_original,area_core=area_core,coverage=area_core/area_original,maximum_halfplane_violation=violation)

def lower(p,q):
    G=np.array(p['G']);T=np.array(q['G']);M=np.linalg.inv(G)@np.linalg.inv(T).T
    return W5**2/np.max(abs(N5@M@N5.T))

def quantile(x):
    a=np.array(x,dtype=float)
    return dict(zip(['min','q10','median','q90','max'],map(float,np.quantile(a,[0,.1,.5,.9,1]))))
def corr(x,y):
    def rank(a):
        order=np.argsort(a,kind='stable');r=np.empty(len(a));i=0
        while i<len(a):
            j=i+1
            while j<len(a) and a[order[j]]==a[order[i]]:j+=1
            r[order[i:j]]=(i+j-1)/2;i=j
        return r
    if len(x)<3 or np.std(x)==0 or np.std(y)==0:return None
    return float(np.corrcoef(rank(np.array(x)),rank(np.array(y)))[0,1])
def metrics(rr):
    out={'n':len(rr)}
    if not rr:return out
    for tag,field in [('core','L'),('center','prediction'),('width','width')]:
        d=np.array([r[field]-r['capacity'] for r in rr]);re=d/np.array([r['capacity'] for r in rr])
        sf=field+'_sys' if field!='L' else 'core_sys'
        ds=np.array([r[sf]-r['sys'] for r in rr]);rs=ds/np.array([r['sys'] for r in rr])
        out[tag]=dict(capacity_signed_relative=quantile(re),capacity_absolute_relative=quantile(abs(re)),capacity_MAE=float(np.mean(abs(d))),sys_signed_error=quantile(ds),sys_absolute_relative=quantile(abs(rs)),sys_MAE=float(np.mean(abs(ds))),spearman_sys=corr([r[sf] for r in rr],[r['sys'] for r in rr]))
    improved=[abs(r['prediction']-r['capacity'])<abs(r['width']-r['capacity'])-1e-12*max(1,r['capacity']) for r in rr]
    out['center_beats_width_capacity_count']=sum(improved)
    out['center_over_width_capacity_MAE']=out['center']['capacity_MAE']/out['width']['capacity_MAE'] if out['width']['capacity_MAE'] else None
    out['coverage_product']=quantile([r['coverage_product'] for r in rr]);out['multiplicative_bracket_radius']=quantile([r['bracket_radius'] for r in rr])
    out['worst_center_relative_row']=max(rr,key=lambda r:abs(r['prediction']/r['capacity']-1))['name']
    return out

def control():
    ans=[]
    for t in np.linspace(0,np.pi/10,9):
        R=np.array([[math.cos(t),-math.sin(t)],[math.sin(t),math.cos(t)]])
        p=core(N5/H);q=core((N5/H)@R.T);L=lower(p,q);truth=W5**2/math.cos(t);ratio=L*truth/(2*AREA5**2)
        ans.append(dict(theta=float(t),L=float(L),known_capacity=truth,predicted_sys=ratio,known_sys=truth**2/(2*AREA5**2),coverage_product=p['coverage']*q['coverage'],capacity_relative_error=L/truth-1))
    assert max(abs(r['capacity_relative_error']) for r in ans)<1e-12
    assert abs(ans[-1]['predicted_sys']-HKO)<1e-12 and ans[-1]['predicted_sys']>1
    return dict(known_reference_only=True,hko=ans[-1],regular_rotation_controls=ans)

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--width',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);ap.add_argument('--figure',action='store_true');args=ap.parse_args()
    started=time.perf_counter();assert sha(args.source)==SOURCE_SHA and sha(args.width)==WIDTH_SHA
    args.out.mkdir(parents=True,exist_ok=True)
    # Phase 1: geometry only, never passes capacities/ratios/bounces to feature code.
    geometries=[]
    for line in args.source.open():
        r=json.loads(line);a=np.array(r['dual_vertices']);k=r['k'];p=core(a[:k,:2]);q=core(a[k:,2:]);L=float(lower(p,q));V=p['area_original']*q['area_original']
        own_sys=L*L/(2*p['area_core']*q['area_core']);assert own_sys<=HKO+1e-9
        geometries.append(dict(name=r['name'],k=k,m=r['m'],bucket=f"{k}x{r['m']}",p=p,q=q,L=L,geometric_volume=V,coverage_product=p['coverage']*q['coverage'],core_own_sys=own_sys))
    assert len(geometries)==10240 and len(set(r['name'] for r in geometries))==10240
    geometry_seconds=time.perf_counter()-started
    # Phase 2: evaluator joins frozen features to historical labels and baseline.
    labels={r['name']:r for r in map(json.loads,args.source.open())}
    with gzip.open(args.width,'rt') as f:width={r['name']:r for r in csv.DictReader(f)}
    rows=[];violations=[]
    for r in geometries:
        s=labels[r['name']];w=width[r['name']];assert s['sys']==float(w['sys']) and s['capacity']==float(w['capacity'])
        assert int(w['k'])==r['k'] and int(w['m'])==r['m']
        assert abs(r['geometric_volume']/s['volume']-1)<1e-8
        L=r['L'];W=float(w['c2']);pred=math.sqrt(L*W);V=s['volume'];C=s['capacity']
        role='calibration' if int(hashlib.sha256(r['name'].encode()).hexdigest()[:8],16)%5==0 else 'diagnostic'
        r.update(capacity=C,sys=s['sys'],volume=V,width=W,prediction=pred,core_sys=L*L/(2*V),width_sys=W*W/(2*V),prediction_sys=L*W/(2*V),bracket_radius=math.sqrt(W/L),role=role)
        if L>C*(1+1e-8) or C>W*(1+1e-8):violations.append(dict(name=r['name'],L_over_c=L/C,c_over_width=C/W))
        rows.append(r)
    assert not violations,violations
    summary=dict(status='PASS geometry/benchmark checks; scientific route decided in report',method='Frozen inscribed affine regular pentagon capacity lower feature L; parameter-free point sqrt(L*width); retrospective labels, no fitted constants',source_sha256=SOURCE_SHA,width_sha256=WIDTH_SHA,rows=len(rows),geometry_seconds=geometry_seconds,elapsed_seconds=time.perf_counter()-started,label_limits='Historical numerical capacities/sys/volumes; no new certified sys intervals; computed cores/baseline binary64',partition='SHA256(name) first8hex modulo5:0 calibration,else diagnostic; no fitting in either subset',all=metrics(rows),diagnostic=metrics([r for r in rows if r['role']=='diagnostic']),calibration=metrics([r for r in rows if r['role']=='calibration']),buckets={},HKO_control=control(),bound_violations=violations,maximum_halfplane_violation=max(max(r['p']['maximum_halfplane_violation'],r['q']['maximum_halfplane_violation']) for r in rows))
    for bucket in sorted(set(r['bucket'] for r in rows)):
        rr=[r for r in rows if r['bucket']==bucket];top=sorted(rr,key=lambda r:(-r['sys'],r['name']))[:math.ceil(.02*len(rr))]
        summary['buckets'][bucket]=dict(all=metrics(rr),diagnostic=metrics([r for r in rr if r['role']=='diagnostic']),top2=metrics(top),diagnostic_top2=metrics([r for r in top if r['role']=='diagnostic']),top2_names=[r['name'] for r in top],worst_top2_row=max(top,key=lambda r:abs(r['prediction']/r['capacity']-1)))
    with gzip.GzipFile(filename=str(args.out/'rows.jsonl.gz'),mode='wb',mtime=0) as f:
        for r in rows:f.write((json.dumps(r,separators=(',',':'))+'\n').encode())
    (args.out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    if args.figure:plot(rows,summary,args.out)
    for b in ['5x5','5x6','6x6']:
        d=summary['buckets'][b]['top2'];print(b,'center median/90/max relerr',d['center']['capacity_absolute_relative'],'width',d['width']['capacity_absolute_relative'],'MAE ratio',d['center_over_width_capacity_MAE'],'improved',d['center_beats_width_capacity_count'])
    print('HKO',summary['HKO_control']['hko']['predicted_sys'],'seconds',summary['elapsed_seconds'])

def plot(rows,summary,out):
    import matplotlib;matplotlib.use('Agg')
    import matplotlib.pyplot as plt
    fig,ax=plt.subplots(1,3,figsize=(13,4.1),layout='constrained')
    for j,b in enumerate(['5x5','5x6','6x6']):
        rr=[r for r in rows if r['bucket']==b];top=set(summary['buckets'][b]['top2_names']);a=ax[j]
        a.scatter([r['sys'] for r in rr],[r['prediction_sys'] for r in rr],s=7,alpha=.25,color='#266b88',label='all retained')
        tt=[r for r in rr if r['name'] in top];a.scatter([r['sys'] for r in tt],[r['prediction_sys'] for r in tt],s=24,color='#d46d28',label='top2% by true label')
        a.plot([0,1.1],[0,1.1],'--',lw=1,color='#444444');a.scatter([HKO],[summary['HKO_control']['hko']['predicted_sys']],marker='*',s=130,color='#903866',label='separate HKO')
        a.set(xlabel='Historical numerical systolic ratio',ylabel='Frozen core–width center prediction',title=b,xlim=(0,1.08),ylim=(0,1.08));a.grid(alpha=.15)
    ax[0].legend(fontsize=8);fig.suptitle('A geometry-aware pentagon core: exact on HKO, retrospective random-tail test');fig.savefig(out/'core-prediction.png',dpi=160);fig.savefig(out/'core-prediction.pdf');plt.close(fig)
if __name__=='__main__':main()
