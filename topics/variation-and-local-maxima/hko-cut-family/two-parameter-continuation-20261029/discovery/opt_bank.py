from bank import *
bank.extend([[int(x) for x in w] for w in words])
last=[];calls=0;t0=time.monotonic()
def obj(p):
 global calls
 calls+=1
 try:r=scan(*p)
 except (ValueError,np.linalg.LinAlgError):return 1e3
 if calls%50==0: print(calls,'secs',time.monotonic()-t0,'candidate',r,flush=True)
 last.append(r)
 return -r['rho_upper']
res=differential_evolution(obj,[(.72,.85),(.18,.33)],popsize=8,maxiter=25,tol=1e-6,polish=False,seed=324)
best=scan(*res.x);print('BEST',best,flush=True)
(out/'optimized-bank.json').write_text(json.dumps(dict(best=best,calls=calls,seconds=time.monotonic()-t0,method='floating feasible-word bank upper-bound optimization; not global capacity'),indent=2))
