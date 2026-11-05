from bank import *
bank.extend([[int(x) for x in w] for w in words]);calls=0;allrec=[];t0=time.monotonic()
def obj(p):
 global calls
 calls+=1
 if not (.75<p[0]<.85 and .24<p[1]<.31):return 1e3
 r=scan(*p);allrec.append(r)
 if calls%30==0:print(calls,r,flush=True)
 return -r['rho_upper']
res=minimize(obj,[.7946515685511027,.2793463229418876],method='Nelder-Mead',options={'maxiter':90,'maxfev':180,'xatol':1e-9,'fatol':1e-10,'initial_simplex':np.array([[.7946515685511027,.2793463229418876],[.7947515685511027,.2793463229418876],[.7946515685511027,.2794463229418876]])})
rats=[]
for a,t in [(159/200,69/200),(1591/2000,69/200),(795/1000,691/2000),(159/200,346/1000)]:rats.append(scan(a,t*np.cos(np.pi/5)))
best=max(allrec,key=lambda r:r['rho_upper']);print('best',best,'rational_candidates',rats,flush=True)
(out/'refined-bank.json').write_text(json.dumps(dict(best=best,rational_candidates=rats,seconds=time.monotonic()-t0,calls=calls,status=str(res),scope='floating feasible-word upper bounds; no complete capacity search'),indent=2))
