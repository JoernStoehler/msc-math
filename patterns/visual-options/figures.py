#!/usr/bin/env python3
"""Three reader-facing figure choices from retained sources; no capacity search.
Requires an existing NumPy/SciPy/Matplotlib environment. Writes --out only.
"""
from __future__ import annotations
import argparse,csv,gzip,hashlib,json,math
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

SOURCE='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
COV='88cba83c48996309d6f4f050adb44413d449acb873d3222c515040087a7c8301'
TABLE='466068ace1248c0d2ab3bb55bfa8269bf8597ad475ede6bbd3739d87500ddd45'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def readlines(p):
    with (gzip.open if p.suffix=='.gz' else open)(p,'rt') as f:return [json.loads(l) for l in f]
def dump(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def primal(a):
    a=a[np.argsort(np.arctan2(a[:,1],a[:,0]))]
    v=np.array([np.linalg.solve(np.stack([a[i],a[(i+1)%len(a)]]),np.ones(2)) for i in range(len(a))])
    return v[ConvexHull(v).vertices]
def save(fig,out,name):
    fig.savefig(out/(name+'.png'),dpi=180);fig.savefig(out/(name+'.pdf'));plt.close(fig)
def key(r):return tuple(tuple(v) for v in r['dual_vertices_rational'])
def reconstruct(raw,stored):
    # Fixed-word stationarity and the repository's beta-to-orbit convention.
    # This recovers a candidate; no support enumeration/global search is run.
    a=np.array(raw['dual_vertices']);sig=stored['sigmas'][0];perm=sig['perm'];action=sig['action'];u=a[perm];n=len(u)
    assert abs(action/raw['capacity']-1)<1e-10
    om=u[:,:2]@u[:,2:].T-u[:,2:]@u[:,:2].T;H=np.triu(om,1);H+=H.T
    C=np.vstack([u.T,np.ones(n)]);K=np.block([[H,C.T],[C,np.zeros((5,5))]])
    rhs=np.r_[np.zeros(n),np.zeros(4),1];sol=np.linalg.lstsq(K,rhs,rcond=None)[0];beta=sol[:n]
    J=np.block([[np.zeros((2,2)),-np.eye(2)],[np.eye(2),np.zeros((2,2))]])
    disp=np.vstack([np.zeros(4),np.cumsum((2*action*beta)[:,None]*(u@J.T),axis=0)])
    base=np.linalg.lstsq(u,1-np.einsum('ij,ij->i',u,disp[:-1]),rcond=None)[0];pts=disp+base
    Q=.5*beta@H@beta
    geometric_action=.5*np.sum(pts[:-1,:2]*pts[1:,2:]-pts[:-1,2:]*pts[1:,:2])
    checks=dict(min_beta=float(min(beta)),normalization_residual=float(abs(sum(beta)-1)),closure_residual=float(np.linalg.norm(disp[-1])),max_halfspace_violation=float(np.max(a@pts.T-1)),max_active_facet_residual=float(np.max(abs(np.einsum('ij,ij->i',u,pts[:-1])-1))),relative_action_error=float(abs(2*action*Q-1)),relative_geometric_action_error=float(abs(geometric_action/action-1)),kkt_residual=float(np.max(abs(K@sol-rhs))))
    assert min(beta)>0 and max(v for k,v in checks.items() if k!='min_beta')<1e-10
    return dict(sigma_zero_based=perm,beta=beta.tolist(),stored_action=action,breakpoints=pts.tolist(),checks=checks)
def A(root,cache,out):
    with gzip.open(root/'patterns/product-shape/features.csv.gz','rt') as f: feats=list(csv.DictReader(f))
    selected=[]
    for k,m in [(5,5),(6,6)]:
        rr=sorted([r for r in feats if (int(r['k']),int(r['m']))==(k,m)],key=lambda r:(float(r['sys']),r['name']))
        selected.extend([(rr[-1],'Sample leader'),(rr[len(rr)//2],'Median control')])
    names={r['name'] for r,_ in selected};source={r['name']:r for r in readlines(cache/'random-product.jsonl') if r['name'] in names};keys={key(r):r['name'] for r in source.values()};words={}
    for stored in readlines(cache/'shared-cache.jsonl'):
        if key(stored) in keys:words[keys[key(stored)]]=stored
    fig,axes=plt.subplots(2,4,figsize=(12.5,7.6),layout='constrained');details=[]
    for col,(feat,label) in enumerate(selected):
        raw=source[feat['name']];stored=words[raw['name']];orbit=reconstruct(raw,stored);pts=np.array(orbit['breakpoints']);a=np.array(raw['dual_vertices']);k=raw['k'];factors=[primal(a[:k,:2]),primal(a[k:,2:])]
        details.append(dict(name=raw['name'],selection=label,bucket=f'{k}×{raw["m"]}',historical_sys=raw['sys'],producer_bounces=raw['bounces'],orbit=orbit,p_vertices=factors[0].tolist(),q_vertices=factors[1].tolist()))
        for row,v in enumerate(factors):
            ax=axes[row,col];center=v.mean(0);scale=np.sqrt(ConvexHull(v).volume);vv=(v-center)/scale;op=(pts[:,row*2:row*2+2]-center)/scale
            vv=np.vstack([vv,vv[0]]);ax.fill(*vv.T,color='#2877ad' if row==0 else '#c96b19',alpha=.12);ax.plot(*vv.T,color='#2877ad' if row==0 else '#c96b19',lw=1.9)
            ax.plot(*op.T,color='#6e2f72',lw=1.8)
            for i,(start,end) in enumerate(zip(op[:-1],op[1:])):
                if np.linalg.norm(end-start)>1e-9:
                    middle=.4*start+.6*end;before=.6*start+.4*end;ax.annotate('',xy=middle,xytext=before,arrowprops=dict(arrowstyle='-|>',color='#6e2f72',lw=1.5))
            ax.scatter(*op[:-1].T,s=12,color='#6e2f72',zorder=3)
            # Facet labels refer to the displayed word; panel zoom differs.
            local=a[:k,:2] if row==0 else a[k:,2:]
            for fi,normal in enumerate(local):
                active=v[np.abs(v@normal-1)<1e-7];mid=(active.mean(0)-center)/scale;mid+=.075*normal/np.linalg.norm(normal)
                ax.text(*mid,str(fi+1),ha='center',va='center',fontsize=8,color='#555555')
            bound=max(1.0,float(np.max(abs((v-center)/scale)))*1.16)
            ax.set_aspect('equal');ax.set_xlim(-bound,bound);ax.set_ylim(-bound,bound);ax.set_xticks([]);ax.set_yticks([])
            if row==0:ax.set_title(f'{k}×{raw["m"]}: {label}\nratio {raw["sys"]:.4f}',fontsize=11)
            if col==0:ax.set_ylabel('P: q projection' if row==0 else 'Q: p projection',fontsize=11)
            if row==1:
                word=' → '.join(('q' if i<k else 'p')+str(i+1 if i<k else i-k+1) for i in orbit['sigma_zero_based'])
                ax.set_xlabel(f'{raw["name"]}\n{word}',fontsize=8)
    fig.suptitle('A · What do the largest sampled products look like?\nPurple: one reconstructed stored minimizing candidate; each factor area 1, panel zooms differ',fontsize=13)
    save(fig,out,'A-shapes-orbits');dump(out/'A-shapes-orbits.json',details)
    return details

def B(root,out):
    # P5 uses facet unit normals n_i, support cos(pi/5), circumradius one.
    # G=I, H=Rtheta diag(t,1/t), so M=G^-1 H^-T=Rtheta diag(1/t,t).
    t=np.r_[np.linspace(1,1.1,101),np.linspace(1.105,2,180)];theta=np.linspace(0,2*np.pi/5,721);ang=2*np.pi*np.arange(5)/5;n=np.c_[np.cos(ang),np.sin(ang)];w=1+np.cos(np.pi/5);S=2.5*np.sin(2*np.pi/5)
    cap=np.empty((len(t),len(theta)))
    for i,stretch in enumerate(t):
        aa=np.outer(n[:,0],n[:,0])/stretch+np.outer(n[:,1],n[:,1])*stretch
        bb=np.outer(n[:,1],n[:,0])/stretch-np.outer(n[:,0],n[:,1])*stretch
        denominator=np.max(abs(aa[:,:,None]*np.cos(theta)+bb[:,:,None]*np.sin(theta)),axis=(0,1))
        cap[i]=w*w/denominator
    sys=cap*cap/(2*S*S)
    assert abs(sys[0,180]-(3+math.sqrt(5))/5)<1e-13
    assert np.max(sys)<(3+math.sqrt(5))/5+1e-12
    # Separate known rotation profile check across full angle grid.
    d=abs((theta+np.pi/10)%(np.pi/5)-np.pi/10);expected=w*w/np.cos(d)
    assert np.max(abs(cap[0]-expected))<1e-13
    fig,axes=plt.subplots(1,2,figsize=(12,5.4),layout='constrained')
    for ax,z,cmap,title,zoom in [(axes[0],cap,'cividis','Capacity: stretch 1–2',False),(axes[1],sys,'viridis','Ratio: zoom near regular shape',True)]:
        keep=t<=1.1+1e-12 if zoom else np.ones(len(t),bool)
        im=ax.pcolormesh(theta*180/np.pi,t[keep],z[keep],shading='auto',cmap=cmap,rasterized=True);fig.colorbar(im,ax=ax,label='Systolic ratio' if zoom else 'Capacity')
        ax.contour(theta*180/np.pi,t[keep],sys[keep],levels=[1],colors=['#e2534a'],linewidths=1.5)
        ax.set(xlabel='Relative angle θ (degrees)',ylabel='Area-preserving stretch t',title=title);ax.set_xticks([0,18,36,54,72])
    axes[1].text(.02,.97,'Red contour: ratio = 1',transform=axes[1].transAxes,va='top',color='white',fontsize=10)
    fig.suptitle('B · The above-one region in an exactly solved shape family\nP₅ × Rθ diag(t, 1/t)P₅; known affine-pentagon theorem, no new capacity search',fontsize=14)
    save(fig,out,'B-stretch-angle');np.savez_compressed(out/'B-stretch-angle.npz',stretch=t,theta_radians=theta,capacity=cap,systolic_ratio=sys)
    return dict(rows=len(t),columns=len(theta),max_plotted_sys=float(sys.max()),rotation_profile_max_error=float(np.max(abs(cap[0]-expected))),family='P5 × Rtheta diag(t,1/t)P5; t in [1,2], theta in [0,2pi/5]',formula='c=(1+cos(pi/5))^2/max_ij|n_i^T Rtheta diag(1/t,t)n_j|; sys=c^2/(2 area(P5)^2)')

def metric(y,p):
    return dict(rows=len(y),sys_mae=float(np.mean(abs(y-p))),median_signed_error=float(np.median(p-y)),log_r2=float(1-np.sum((np.log(p)-np.log(y))**2)/np.sum((np.log(y)-np.mean(np.log(y)))**2)))
def C(root,out):
    cp=root/'topics/empirical-studies/sys-datascience/methods/canonical-vertex-covariance/artifacts/current/per_polytope.jsonl';tp=root/'topics/empirical-studies/retrospective-revalidation/geometry-full/polytope-table.jsonl.gz'
    assert sha(cp)==COV and sha(tp)==TABLE
    cov=sorted(readlines(cp),key=lambda r:r['poly_id']);table={r['poly_id']:r for r in readlines(tp)}
    assert len(cov)==14334 and len(table)==14336
    assert all(r['sys']==table[r['poly_id']]['sys'] for r in cov)
    y=np.array([r['sys'] for r in cov]);R=np.log([table[r['poly_id']]['ridge_symp_area_sum_over_volume_sqrt'] for r in cov]);rho=np.log([r['rho'] for r in cov]);b=np.array([r['bucket'] for r in cov]);test=np.array([int(r['poly_id'][:8],16)%5==0 for r in cov]);names=sorted(set(b));X=np.c_[np.ones(len(y)),R,rho,R*R,R*rho,rho*rho]
    pred=np.full(len(y),np.nan);tail=np.zeros(len(y),bool);stats=[];coeffs={}
    for group in names:
        train=(b==group)&~test;hold=(b==group)&test;coef=np.linalg.lstsq(X[train],np.log(y[train]),rcond=None)[0];pred[hold]=np.exp(X[hold]@coef);coeffs[group]=coef.tolist()
        ii=np.flatnonzero(hold);tail[ii[np.argsort(y[ii])[-math.ceil(.1*len(ii)):]]]=True
        stats.append(dict(bucket=group,train_rows=int(sum(train)),heldout=metric(y[hold],pred[hold]),heldout_top_decile=metric(y[tail&(b==group)],pred[tail&(b==group)])))
    allstat=dict(heldout=metric(y[test],pred[test]),heldout_top_decile=metric(y[tail],pred[tail]))
    # Three exact product buckets show family-specific errors without a pooled score.
    groups=['random_product:5x5','random_product:5x6','random_product:6x6']
    assert set(groups)<=set(names),names
    fig,axes=plt.subplots(2,3,figsize=(12.5,8),layout='constrained')
    for col,group in enumerate(groups):
        mask=(b==group)&test;ordinary=mask&~tail;top=mask&tail;ax=axes[0,col]
        ax.scatter(y[ordinary],pred[ordinary],s=13,alpha=.5,color='#7f9bb0',label='Other held-out rows',rasterized=True);ax.scatter(y[top],pred[top],s=35,color='#d15d38',label='Held-out top decile',zorder=3)
        lim=[min(y[mask].min(),pred[mask].min())*.95,max(y[mask].max(),pred[mask].max())*1.03];ax.plot(lim,lim,color='gray',ls='--',lw=1);ax.set(xlim=lim,ylim=lim,title=group.split(':')[1].replace('x','×')+' products',xlabel='Observed historical ratio',ylabel='Predicted ratio');ax.set_aspect('equal')
        if col==0:ax.legend(fontsize=8)
        ax=axes[1,col];ax.set_ylim(-.25,.21);ax.scatter(y[ordinary],pred[ordinary]-y[ordinary],s=13,alpha=.5,color='#7f9bb0',rasterized=True);ax.scatter(y[top],pred[top]-y[top],s=35,color='#d15d38',zorder=3);ax.axhline(0,color='gray',ls='--',lw=1)
        stat=next(s for s in stats if s['bucket']==group);ss=stat['heldout_top_decile'];ax.set(xlabel='Observed historical ratio',ylabel='Prediction − observed',title=f'Top {ss["rows"]} held-out rows: MAE {ss["sys_mae"]:.3f}')
    fig.suptitle('C · Does the predictor follow the best bodies within each family?\nQuadratic log-ratio model of ridge sum + covariance; separate fit and hash holdout in each bucket',fontsize=13)
    save(fig,out,'C-within-family-errors')
    with gzip.GzipFile(filename=str(out/'C-heldout-predictions.csv.gz'),mode='wb',mtime=0) as z:
        import io
        f=io.TextIOWrapper(z,encoding='utf8',newline='');w=csv.writer(f,lineterminator='\n');w.writerow(['poly_id','bucket','historical_sys','predicted_sys','heldout_top_decile'])
        for i in np.flatnonzero(test):w.writerow([cov[i]['poly_id'],b[i],y[i],pred[i],bool(tail[i])])
        f.flush()
    report=dict(protocol='Each exact generator facet/polygon-pair bucket fitted separately: train hash(poly_id[:8])%5 != 0, test == 0. OLS on log(sys), predictors 1,log R,log rho,(log R)^2,log R log rho,(log rho)^2. Top decile selected only among heldout rows separately per bucket. Model fixed from existing combined model; new evaluation split/family training, no new capacities.',rows=len(y),summary=allstat,by_bucket=stats,coefficients=coeffs,excluded=['random_F8_s3_45','random_F12_s2_38'])
    dump(out/'C-within-family-errors.json',report);return report

def main():
    ap=argparse.ArgumentParser();ap.add_argument('--repo',type=Path,required=True);ap.add_argument('--cache',type=Path,default=Path.home()/'.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files');ap.add_argument('--out',type=Path,required=True);args=ap.parse_args();args.out.mkdir(parents=True,exist_ok=True)
    assert sha(args.cache/'random-product.jsonl')==SOURCE
    a=A(args.repo,args.cache,args.out);b=B(args.repo,args.out);c=C(args.repo,args.out)
    paths=[args.repo/'patterns/product-shape/features.csv.gz',args.repo/'patterns/features/analyze.py',args.repo/'topics/products/pentagon-affine-products/research.tex',args.repo/'crates/symplectic/src/algorithms/hk2017/orbit_recovery.rs',args.cache/'random-product.jsonl',args.cache/'shared-cache.jsonl',args.repo/'topics/empirical-studies/sys-datascience/methods/canonical-vertex-covariance/artifacts/current/per_polytope.jsonl',args.repo/'topics/empirical-studies/retrospective-revalidation/geometry-full/polytope-table.jsonl.gz']
    dump(args.out/'verification.json',dict(status='PASS numerical checks; visual inspection recorded separately',producer_sha256=sha(Path(__file__)),source_sha256={str(p):sha(p) for p in paths},A_recovered_orbits=[dict(name=r['name'],**r['orbit']['checks']) for r in a],B=b,C=c['summary'],output_sha256={p.name:sha(p) for p in args.out.iterdir() if p.suffix in ['.png','.pdf','.npz','.gz','.json'] and p.name!='verification.json'}))
    print(json.dumps(dict(A='four positive fixed-word orbits recovered and checked',B=b,C=c['summary']),indent=2))
if __name__=='__main__':main()
