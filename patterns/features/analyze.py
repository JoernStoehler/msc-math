#!/usr/bin/env python3
"""Cheap retained-data covariance/ridge falsifiers; no capacity evaluations.
Needs NumPy; --figure additionally needs Matplotlib. Writes only --out.
"""
from __future__ import annotations
import argparse,collections,csv,gzip,hashlib,json,math
from pathlib import Path
import numpy as np

def rows(path):
    op=gzip.open if path.suffix=='.gz' else open
    with op(path,'rt') as f:return [json.loads(l) for l in f]
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
def rank(x):
    order=np.argsort(x,kind='stable'); out=np.empty(len(x),float)
    for start in range(len(x)):
        if start and x[order[start]]==x[order[start-1]]:continue
        end=start+1
        while end<len(x) and x[order[end]]==x[order[start]]:end+=1
        out[order[start:end]]=(start+end-1)/2
    return out

def corr(x,y):
    if len(x)<3 or np.std(x)==0 or np.std(y)==0:return None
    return float(np.corrcoef(rank(np.asarray(x)),rank(np.asarray(y)))[0,1])
def metrics(y,p):
    e=p-y
    return dict(rows=len(y),log_r2=float(1-np.sum(e*e)/np.sum((y-y.mean())**2)),log_rmse=float(np.sqrt(np.mean(e*e))),sys_mae=float(np.mean(abs(np.exp(y)-np.exp(p)))),median_prediction_over_target=float(np.median(np.exp(e))),log_bias=float(e.mean()))
def fit(X,y,train,test):
    return X[test]@np.linalg.lstsq(X[train],y[train],rcond=None)[0]
def key(r):return tuple(tuple(v) for v in r['dual_vertices_rational'])
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--repo',type=Path,default=Path(__file__).resolve().parents[2]);ap.add_argument('--out',type=Path,required=True);ap.add_argument('--figure',action='store_true');ap.add_argument('--source-cache',type=Path,default=Path.home()/'.cache/msc-math/artifacts/polytope-datasets/f7bc6be841e9d30741d5bf7ec8d4f0c0c74ec22745d3325b792e64cec333ca96/files');args=ap.parse_args()
    root=args.repo; out=args.out;out.mkdir(parents=True,exist_ok=True)
    emp=root/'topics/empirical-studies';cp=emp/'sys-datascience/methods/canonical-vertex-covariance/artifacts/current/per_polytope.jsonl';tp=emp/'retrospective-revalidation/geometry-full/polytope-table.jsonl.gz'
    assert sha(cp)=='88cba83c48996309d6f4f050adb44413d449acb873d3222c515040087a7c8301'
    assert sha(tp)=='466068ace1248c0d2ab3bb55bfa8269bf8597ad475ede6bbd3739d87500ddd45'
    cov=sorted(rows(cp),key=lambda r:r['poly_id']);table={r['poly_id']:r for r in rows(tp)};assert len(cov)==14334 and len(table)==14336
    for r in cov:assert r['sys']==table[r['poly_id']]['sys']
    y=np.log([r['sys'] for r in cov]); R=np.log([table[r['poly_id']]['ridge_symp_area_sum_over_volume_sqrt'] for r in cov]);rho=np.log([r['rho'] for r in cov]);b=np.array([r['bucket'] for r in cov]);ones=np.ones(len(y));names=sorted(set(b));tail=np.zeros(len(y),bool)
    for group in names:
        idx=np.flatnonzero(b==group);tail[idx[np.argsort(y[idx])[-math.ceil(.1*len(idx)):]]]=True
    models={'log_R_linear':np.c_[ones,R],'log_R_quadratic':np.c_[ones,R,R*R],'log_rho_linear':np.c_[ones,rho],'log_rho_quadratic':np.c_[ones,rho,rho*rho],'both_quadratic':np.c_[ones,R,rho,R*R,R*rho,rho*rho]}
    all_scores={};preds={};by_group=[]
    for name,X in models.items():
        pred=np.empty(len(y))
        for group in names:
            test=b==group; pred[test]=fit(X,y,~test,test)
            by_group.append(dict(model=name,bucket=group,**metrics(y[test],pred[test])))
        preds[name]=pred
        # Matched deterministic within-bucket holdout uses hash bits and no target.
        test=np.array([int(r['poly_id'][:8],16)%5==0 for r in cov]);p=fit(X,y,~test,test)
        score={'leave_one_bucket_out':metrics(y,pred),'within_bucket_hash_holdout':metrics(y[test],p),'upper_decile_leave_one_bucket_out':metrics(y[tail],pred[tail]),'coefficients_full_data':np.linalg.lstsq(X,y,rcond=None)[0].tolist()}
        score['leave_one_bucket_out_generic']=metrics(y[np.char.startswith(b,'random:')],pred[np.char.startswith(b,'random:')]);score['leave_one_bucket_out_product']=metrics(y[np.char.startswith(b,'random_product:')],pred[np.char.startswith(b,'random_product:')]);all_scores[name]=score
    conditional=[]
    for fraction in [1,.5,.2,.1,.05,.02]:
        xp=[];zp=[];yp=[];each=[]
        for group in names:
            ii=np.flatnonzero(b==group);ii=ii[np.argsort(y[ii])[-math.ceil(fraction*len(ii)):]]
            xp.extend(rank(R[ii])/len(ii));zp.extend(rank(rho[ii])/len(ii));yp.extend(rank(y[ii])/len(ii))
            each.append(dict(bucket=group,rows=len(ii),ridge_spearman=corr(R[ii],y[ii]),rho_spearman=corr(rho[ii],y[ii])))
        conditional.append(dict(retained_top_fraction=fraction,rows=len(yp),pooled_within_bucket_rank_correlation_ridge=corr(xp,yp),pooled_within_bucket_rank_correlation_rho=corr(zp,yp),by_bucket=each))
    # Broad-transfer falsifier: fit to generic only, evaluate products, and reverse.
    family_transfer=[]
    generic=np.char.startswith(b,'random:')
    for name,X in models.items():
        for train,label in [(generic,'generic_to_product'),(~generic,'product_to_generic')]:
            p=fit(X,y,train,~train);family_transfer.append(dict(model=name,transfer=label,**metrics(y[~train],p)))
    # Audit the stored word representation; these are returned words, not completeness certificates.
    src={};source_hashes={}
    for fname,expected in [('random.jsonl','a21ac62ba5c9496ef631d3cce74e8b663764516b76e9d4725f1e517d8dd55f9f'),('random-product.jsonl','66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736')]:
        p=args.source_cache/fname;source_hashes[fname]=sha(p);assert source_hashes[fname]==expected
        for r in rows(p):src[key(r)]=r
    pp=args.source_cache/'shared-cache.jsonl';source_hashes[pp.name]=sha(pp);orbits=collections.defaultdict(list);joined=set()
    for r in rows(pp):
        kk=key(r)
        if kk not in src:continue
        s=src[kk];joined.add(s['name']);group=f"product:{s['k']}x{s['m']}" if 'k' in s else f"generic:F{s['facet_count']}"
        ls=[len(sig['perm']) for sig in r['sigmas']]
        orbits[group].append(dict(source_name=s['name'],sys=s['sys'],returned_words=len(ls),min_returned_word_length=min(ls),max_returned_word_length=max(ls),product_bounces=s.get('bounces'),capacity_matches=s['capacity']==r['capacity']))
    assert len(joined)==14336
    orbit_summary=[]
    for group,rr in sorted(orbits.items()):
        top=sorted(rr,key=lambda r:r['sys'])[-math.ceil(.1*len(rr)):]
        orbit_summary.append(dict(bucket=group,rows=len(rr),length_counts=dict(collections.Counter(r['min_returned_word_length'] for r in rr)),top_decile_length_counts=dict(collections.Counter(r['min_returned_word_length'] for r in top)),returned_word_count_counts=dict(collections.Counter(r['returned_words'] for r in rr)),capacity_match_count=sum(r['capacity_matches'] for r in rr),two_bounce_six_facet_words=sum(r['product_bounces']==2 and r['min_returned_word_length']==6 for r in rr)))
    # Exact-known pentagon-profile obstruction to covariance-only prediction.
    theta=np.linspace(0,np.pi/10,101);pent=(1+np.cos(np.pi/5))**4/(2*(2.5*np.sin(2*np.pi/5))**2)/np.cos(theta)**2
    assert abs(pent[-1]-(3+math.sqrt(5))/5)<1e-14
    # Independently compute all vertex moments of total degree <=4 along the family.
    powers=[(a,b,c,d) for a in range(5) for b in range(5-a) for c in range(5-a-b) for d in range(5-a-b-c)]
    assert len(powers)==70
    aa=2*np.pi*np.arange(5)/5; q=np.c_[np.cos(aa),np.sin(aa)]; baseline=None;moment_error=0.;fifth_error=0.;profile_error=0.
    for angle,value in zip(theta,pent):
        pp=np.c_[np.cos(aa+angle),np.sin(aa+angle)];vv=np.array([list(qi)+list(pj) for qi in q for pj in pp]);mm=np.array([np.mean(np.prod(vv**np.array(e),axis=1)) for e in powers])
        if baseline is None:baseline=mm
        moment_error=max(moment_error,float(np.max(abs(mm-baseline))))
        m5=np.mean((pp[:,0]+1j*pp[:,1])**5);fifth_error=max(fifth_error,float(abs(m5-np.exp(5j*angle))))
        T=abs(float(m5.real));dist=math.acos(min(1.,T))/5
        reconstructed=float(pent[0])/math.cos(dist)**2;profile_error=max(profile_error,abs(reconstructed-float(value)))
    moment_audit=dict(measure='Uniform distinct vertices of circumradius-one regular pentagon products',total_degree_at_most=4,monomials=70,angles=101,max_absolute_moment_drift=moment_error,max_fifth_complex_harmonic_error=fifth_error,max_capacity_profile_reconstruction_error=profile_error)
    report=dict(moment_audit=moment_audit,status='Post-target retained-data falsification; no fresh capacity/volume evaluations.',rows=14334,excluded_covariance_rows=['random_F8_s3_45','random_F12_s2_38'],target='Historical numerical sys, exactly joined; current target corroboration is separately owned by retrospective-revalidation.',models=all_scores,conditional_upper_tail=conditional,family_transfer=family_transfer,word_audit=orbit_summary,pentagon_covariance_obstruction={'rho':1,'sys_min':float(pent.min()),'sys_max':float(pent.max()),'relative_variation':float(pent.max()/pent.min()-1),'absolute_gap':float(pent.max()-pent.min())},inputs={str(cp.relative_to(root)):sha(cp),str(tp.relative_to(root)):sha(tp),**source_hashes},model_protocol='OLS on log(sys); predictors log(R), log(rho), squares and product; leave exact generator facet/polygon-pair bucket out. No model or feature selection on new targets.',method_choice='Compared flexible many-feature fitting (would obscure explanation), new capacity production (unneeded), and simple two-invariant transfer/falsification (chosen). Raw pooled R2 overweights low-ratio degeneration; therefore tail errors and exact blind family are mandatory controls.')
    (out/'results.json').write_text(json.dumps(report,indent=2)+'\n')
    with (out/'by-bucket.csv').open('w') as f:
        w=csv.DictWriter(f,fieldnames=list(by_group[0]),lineterminator="\n");w.writeheader();w.writerows(by_group)
    with (out/'predictions.csv').open('w') as f:
        fields=['poly_id','bucket','sys','ridge_sum','rho','upper_decile']+list(preds);w=csv.writer(f);w.writerow(fields)
        for i,r in enumerate(cov):w.writerow([r['poly_id'],r['bucket'],r['sys'],math.exp(R[i]),r['rho'],bool(tail[i])]+[math.exp(preds[k][i]) for k in preds])
    if args.figure:
        import matplotlib;matplotlib.use('Agg');import matplotlib.pyplot as plt
        fig,axs=plt.subplots(1,3,figsize=(15,4.5),layout='constrained')
        for mask,label,color in [(generic,'Generic bodies','#bc6b35'),(~generic,'Polygon products','#246a9c')]:
            axs[0].scatter(np.exp(rho[mask]),np.exp(y[mask]),s=3,alpha=.22,color=color,label=label,rasterized=True)
        axs[0].set(xscale='log',xlabel='Vertex covariance symplectic eccentricity ρ',ylabel='Historical numerical systolic ratio',title='Strong bulk association');axs[0].legend(markerscale=3,fontsize=8)
        fs=[z['retained_top_fraction']*100 for z in conditional]
        axs[1].plot(fs,[z['pooled_within_bucket_rank_correlation_ridge'] for z in conditional],'-o',label='Ridge sum R');axs[1].plot(fs,[z['pooled_within_bucket_rank_correlation_rho'] for z in conditional],'-o',label='Covariance ρ');axs[1].set(xscale='log',xlabel='Highest ratios retained in each bucket (%)',ylabel='Within-selected-bucket rank correlation',title='Upper-tail ordering weakens');axs[1].invert_xaxis();axs[1].set_ylim(-1,.1);axs[1].legend(fontsize=8)
        axs[2].plot(theta*180/np.pi,pent,color='#7b3294',lw=2);axs[2].axhline(1,color='gray',ls='--');axs[2].set(xlabel='Distance d from pentagon angle grid (degrees)',ylabel='Exact systolic ratio',title='ρ = 1 along the entire curve');axs[2].text(.04,.94,'Regular pentagon products\nKnown capacity formula',transform=axs[2].transAxes,va='top',fontsize=9)
        fig.savefig(out/'covariance-patterns.png',dpi=170);fig.savefig(out/'covariance-patterns.pdf');plt.close(fig)
    print(json.dumps({name:{k:round(v['log_r2'],5) for k,v in z.items() if isinstance(v,dict) and 'log_r2' in v} for name,z in all_scores.items()},indent=2));print('CONDITIONAL',[(z['retained_top_fraction'],z['pooled_within_bucket_rank_correlation_ridge'],z['pooled_within_bucket_rank_correlation_rho']) for z in conditional])
if __name__=='__main__':main()
