#!/usr/bin/env python3
"""Paired-GL invariant cross moments: bounded historical product tail test.
Requires NumPy. No capacity/volume routine, producer or source mutation.
"""
import argparse,csv,gzip,hashlib,json,math,time
from pathlib import Path
import numpy as np
SOURCE_SHA='66bf82010e92e0f26b0df226f4e6c0eef05d21eb22a0967c7f669530f6545736'
WIDTH_SHA='dcffead16ab5ad0fcdf6ae55d78375bea5b6ee7274877a33846cf4c3c2a7d57f'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def primal(a):
    a=a[np.argsort(np.arctan2(a[:,1],a[:,0]))]
    return np.array([np.linalg.solve(np.stack([a[i],a[(i+1)%len(a)]]),np.ones(2)) for i in range(len(a))])
def moments(p,q):
    p=p-p.mean(axis=0);q=q-q.mean(axis=0);z=p@q.T;s=np.mean(z*z)
    assert s>0 and np.isfinite(s)
    return np.array([np.mean(z**k)/s**(k/2) for k in [3,4,5]])
def rank(x):
    order=np.argsort(x,kind='stable');out=np.empty(len(x),float)
    for start in range(len(x)):
        if start and x[order[start]]==x[order[start-1]]:continue
        end=start+1
        while end<len(x) and x[order[end]]==x[order[start]]:end+=1
        out[order[start:end]]=(start+end-1)/2
    return out

def corr(x,y):
    if len(x)<3 or np.std(x)<1e-15 or np.std(y)<1e-15:return None
    return float(np.corrcoef(rank(x),rank(y))[0,1])
def basic(y,p):
    return dict(rows=len(y),log_r2=float(1-sum((p-y)**2)/sum((y-y.mean())**2)),sys_mae=float(np.mean(abs(np.exp(p)-np.exp(y)))),median_predicted_over_target=float(np.median(np.exp(p-y))))
def score(y,p,b):
    tail=[];rp=[];ry=[];by=[]
    for bucket in sorted(set(b)):
        ii=np.flatnonzero(b==bucket);n=math.ceil(.1*len(ii));tt=ii[np.argsort(y[ii])[-n:]];selected=ii[np.argsort(p[ii])[-n:]]
        tail.extend(tt);rp.extend(rank(p[tt])/n);ry.extend(rank(y[tt])/n)
        by.append(dict(bucket=bucket,**basic(y[ii],p[ii]),tail_order_spearman=corr(y[tt],p[tt]),top_decile_recall=len(set(tt)&set(selected))/n))
    return dict(all=basic(y,p),upper_decile=basic(y[np.array(tail)],p[np.array(tail)]),upper_decile_within_bucket_rank_correlation=corr(np.array(rp),np.array(ry)),mean_top_decile_recall=float(np.mean([r['top_decile_recall'] for r in by])),by_bucket=by)
def controls():
    angles=np.linspace(0,np.pi/10,101);aa=2*np.pi*np.arange(5)/5;p=np.c_[np.cos(aa),np.sin(aa)];err=0.;profileerr=0.;inverr=0.
    A=np.array([[2.,.7],[-.3,.9]])
    for theta in angles:
        q=np.c_[np.cos(aa+theta),np.sin(aa+theta)];m=moments(p,q);t=2*np.sqrt(2)*abs(m[2]);err=max(err,abs(t-abs(np.cos(5*theta))))
        d=math.acos(min(1.,t))/5;s0=(5+2*np.sqrt(5))/10;profileerr=max(profileerr,abs(s0/np.cos(d)**2-s0/np.cos(theta)**2))
        inverr=max(inverr,float(np.max(abs(m-moments(p@A.T+[2.,-4.],q@np.linalg.inv(A)+[-8.,3.])))))
    assert err<1e-12 and profileerr<1e-12 and inverr<1e-12
    square=np.array([[-1.,-1.],[1.,-1.],[1.,1.],[-1.,1.]])
    assert max(abs(moments(square,p)[[0,2]]))<1e-12
    return dict(angles=len(angles),max_regular_pentagon_alignment_error=err,max_known_profile_reconstruction_error=profileerr,max_translation_paired_GL_error=inverr,odd_symmetric_control='PASS')
def main():
    ap=argparse.ArgumentParser();ap.add_argument('--source',type=Path,required=True);ap.add_argument('--width',type=Path,required=True);ap.add_argument('--out',type=Path,required=True);args=ap.parse_args();start=time.perf_counter();args.out.mkdir(parents=True,exist_ok=True)
    assert sha(args.source)==SOURCE_SHA and sha(args.width)==WIDTH_SHA
    with gzip.open(args.width,'rt') as f:width={r['name']:r for r in csv.DictReader(f)}
    rows=[];invarerr=0.;A=np.array([[2.,.7],[-.3,.9]])
    for i,line in enumerate(args.source.open()):
        r=json.loads(line);w=width.pop(r['name']);assert float(w['sys'])==r['sys'];assert (int(w['k']),int(w['m']))==(r['k'],r['m'])
        a=np.array(r['dual_vertices']);p=primal(a[:r['k'],:2]);q=primal(a[r['k']:,2:]);m=moments(p,q)
        if i%320==0:invarerr=max(invarerr,float(np.max(abs(m-moments(p@A.T+[2.,-4.],q@np.linalg.inv(A)+[-8.,3.])))))
        rows.append(dict(name=r['name'],bucket=f"{r['k']}x{r['m']}",sys=r['sys'],width_ceiling=float(w['sys2']),asymmetry=float(w['asym_p'])*float(w['asym_q']),cross3=float(m[0]),cross4=float(m[1]),cross5=float(m[2])))
    assert not width and len(rows)==10240 and len(set(r['name'] for r in rows))==10240 and invarerr<1e-9
    y=np.log([r['sys'] for r in rows]);b=np.array([r['bucket'] for r in rows]);w=np.log([r['width_ceiling'] for r in rows]);a=np.log([r['asymmetry'] for r in rows]);m3=np.abs([r['cross3'] for r in rows]);m5=np.abs([r['cross5'] for r in rows]);one=np.ones(len(rows))
    base=np.c_[one,w,w*w,a]
    models=dict(width_calibration=np.c_[one,w,w*w],width_asymmetry=base,width_asymmetry_cross3=np.c_[base,m3],width_asymmetry_cross5=np.c_[base,m5],width_asymmetry_cross3_cross5=np.c_[base,m3,m5])
    result={};predictions={}
    split=np.array([int(hashlib.sha256(r['name'].encode()).hexdigest()[:8],16)%5==0 for r in rows])
    for name,X in models.items():
        pred=np.empty(len(y));coefs={}
        for bucket in sorted(set(b)):
            test=b==bucket;coef=np.linalg.lstsq(X[~test],y[~test],rcond=None)[0];pred[test]=X[test]@coef;coefs[bucket]=coef.tolist()
        hp=X[split]@np.linalg.lstsq(X[~split],y[~split],rcond=None)[0];predictions[name]=pred
        result[name]=dict(leave_one_bucket_out=score(y,pred,b),name_hash_holdout=score(y[split],hp,b[split]),held_out_coefficients=coefs)
    result['unfitted_width_ceiling']=dict(leave_one_bucket_out=score(y,w,b))
    with (args.out/'features.csv').open('w') as f:
        writer=csv.DictWriter(f,fieldnames=list(rows[0]),lineterminator='\n');writer.writeheader();writer.writerows(rows)
    best=predictions['width_asymmetry_cross3_cross5'];bb=predictions['width_asymmetry'];extremes=[]
    for bucket in sorted(set(b)):
        ii=np.flatnonzero(b==bucket);target=ii[np.argmax(y[ii])];picked=ii[np.argmax(best[ii])]
        extremes.append(dict(bucket=bucket,actual_best=rows[target],model_first=rows[picked],model_first_actual_percentile=float(np.mean(y[ii]<=y[picked])),baseline_first_actual_percentile=float(np.mean(y[ii]<=y[ii[np.argmax(bb[ii])]]))))
    report=dict(source_sha256=SOURCE_SHA,width_features_sha256=WIDTH_SHA,rows=len(rows),target='Historical numerical systolic ratio, not certified; no capacity/volume recomputation',features='Uniform distinct centered factor vertices; m_k=E[(X dot Y)^k]/E[(X dot Y)^2]^(k/2). Predictors use abs(m3),abs(m5).',protocol='Five predeclared low-complexity OLS designs, no tuning: exact side-count bucket holdout and independent name-hash holdout; top-decile order and recall within each bucket.',controls=controls(),retained_paired_GL_max_absolute_error=invarerr,models=result,extremes=extremes,elapsed_seconds=time.perf_counter()-start)
    (args.out/'results.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({name:{'bulk_r2':v['leave_one_bucket_out']['all']['log_r2'],'tail_r2':v['leave_one_bucket_out']['upper_decile']['log_r2'],'tail_order':v['leave_one_bucket_out']['upper_decile_within_bucket_rank_correlation'],'recall':v['leave_one_bucket_out']['mean_top_decile_recall']} for name,v in result.items()},indent=2));print('CONTROLS',report['controls'],'SECONDS',report['elapsed_seconds'])
if __name__=='__main__':main()
