import sympy as s
J=s.Matrix([[0,0,-1,0],[0,0,0,-1],[1,0,0,0],[0,1,0,0]])
a={k:s.Matrix(v) for k,v in zip('ABCDE',[(2,3,-4,-3),(0,1,2,-1),(-4,-3,0,-1),(4,-1,1,1),(-4,2,-4,-4)])}
def detret(word):
    w=[a[x] for x in word];k=len(w)
    E=s.Matrix.vstack(w[0].T,w[1].T);base=s.Matrix.hstack(*E.nullspace()); P=s.eye(4)
    for i in range(k):
        along=w[(i+1)%k];nxt=w[(i+2)%k];R=2*J*along
        f=s.eye(4)-R*nxt.T/(nxt.dot(R));P=f*P
    ret=base.gauss_jordan_solve(P*base)[0]
    return (s.eye(2)-ret).det()
for word in ['ABCDE','ACDE','ABACDE','ABABCDE','ABABACDE']:
 print(word,detret(word))
