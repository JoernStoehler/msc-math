"""Exact triangle closure vertices and all winning six-word orders."""
import itertools,json,time
from pathlib import Path
import exact_core as E
ROOT=Path(__file__).parent
start=time.time()
def vertices(offset):
    out=[]
    for ids in itertools.combinations(range(offset,offset+5),3):
        axes=range(2) if offset==0 else range(2,4)
        M=[[E.A[i][r] for i in ids] for r in axes]+[[E.ONE]*3]
        try:w=E.mv(E.inv(M),[E.ZERO,E.ZERO,E.ONE])
        except ArithmeticError:continue
        if all(E.sign(x)>0 for x in w):out.append((ids,w))
    assert len(out)==5
    return out
Q,P=vertices(0),vertices(5)
win=[];maxval=E.ZERO
for qi,(qs,qw) in enumerate(Q):
 for pi,(ps,pw) in enumerate(P):
    labels=qs+ps;weights=dict(zip(labels,[w/2 for w in qw+pw]));first=min(labels)
    for tail in itertools.permutations([x for x in labels if x!=first]):
        sig=(first,)+tail;val=E.qeval(E.A,sig,[weights[i] for i in sig])
        assert E.sign(E.Q0-val)>=0
        if val==E.Q0:
            code=sum((x+1)<<(4*i) for i,x in enumerate(sig))
            win.append([qi,pi,code,list(sig)])
out={'status':'exact six-word primitive enumeration; full-order catalogue is separate',
 'triangles_q':[[list(s),[x.coefficients() for x in w]] for s,w in Q],
 'triangles_p':[[list(s),[x.coefficients() for x in w]] for s,w in P],
 'winning_six_words':win,'checked_six_words':3000,'seconds':time.time()-start}
(ROOT/'primitives.json').write_text(json.dumps(out,indent=2)+'\n')
with (ROOT/'primitives.txt').open('w') as f:
 f.write(str(len(win))+'\n')
 for q,p,code,s in win:f.write(f'{q} {p} {code}\n')
 for s,w in Q+P:f.write(str(sum(1<<x for x in s))+'\n')
print({'winners':len(win),'seconds':out['seconds']},flush=True)
