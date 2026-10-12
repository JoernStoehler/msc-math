# /// script
# dependencies = ["sympy"]
# ///
"""Exact uniform P* facet-split slice, including arbitrarily small cut fractions."""
import itertools,json,math
from pathlib import Path
import sympy as S
root=Path(__file__).resolve().parent
packet=json.loads((root.parent/'vertex-cones/witnesses.json').read_text())
A=S.Matrix(packet['normal_rows']);O=S.Matrix([[0,0,1,0],[0,0,0,1],[-1,0,0,0],[0,-1,0,0]])
K=S.Matrix(packet['q_vertices']);T=S.Matrix(packet['p_vertices'])
r,c,tau,eta,x,p,u=S.symbols('r c tau eta x p u')
delta=3*r/4;D=S.Matrix([[0,-2*c,1,0]])
word=[0,10,6,4,8,2,1,9];B=A.col_join(A[0,:]+tau*D)
b=S.Matrix([r-delta,delta,S.Rational(1,4),S.Rational(1,4)-r/2,S.Rational(1,8),7*r/4,S.Rational(1,4)-9*r/4,S.Rational(1,8)])
old=[i for i in word if i!=10];closure=A[old,:].T.col_join(S.ones(1,len(old)))
pivots=[0,1,2,3,4]
correction=S.zeros(len(old),1)
sol=closure[:,pivots].inv()*(-delta*D.T.col_join(S.zeros(1,1)))
for k,z in zip(pivots,sol):correction[k]=S.factor(z)
jet=S.Matrix([0 if i==10 else correction[old.index(i)] for i in word]);beta=b+tau*jet
assert all(S.expand(z)==0 for z in B[word,:].T*beta)
assert S.expand(sum(beta))==1
Q=S.expand(sum(beta[i]*beta[j]*(B[word[i],:]*O*B[word[j],:].T)[0] for i,j in itertools.combinations(range(len(word)),2)))
assert S.expand(Q-(S.Rational(1,2)+delta*(S.Rational(3,4)-c-3*r)*tau-S.Rational(21,64)*r*r*(4*c+3)*tau*tau))==0
# All coordinates are multiaffine on the rectangular r,c,tau box. Positive-base
# coordinates are positive on its corners; coordinates proportional to r have
# a strictly positive quotient (r is positive for genuine cuts).
for z in beta:
 if S.expand(z.subs(r,0))==0:
  quotient=S.cancel(z/r)
  assert all(quotient.subs({c:cc,tau:tt})>0 for cc,tt in itertools.product([S.Rational(-1,4),S.Rational(3,4)],[0,S.Rational(1,100)]))
 else:
  assert all(z.subs({r:rr,c:cc,tau:tt})>0 for rr,cc,tt in itertools.product([0,S.Rational(1,12)],[S.Rational(-1,4),S.Rational(3,4)],[0,S.Rational(1,100)]))
# The adaptive rule has r<=1/12 and eta-3r>0 on each genuine-cut interval.
# Hence the linear Q coefficient is positive; the negative quadratic term is bounded.
assert S.Rational(1,2)-S.Rational(21,64)*S.Rational(1,12)**2*6*S.Rational(1,100)**2>0
# Cap-height formula stays below the q factor's top-to-middle distance1/2.
assert S.Rational(1,100)/(2-2*S.Rational(3,4)/100)<S.Rational(1,2)
# P* q width near its top is5/6+2h/3, derived from the retained vertices.
assert list(K[0,:])==[S.Rational(-5,12),S.Rational(1,2)]
assert list(K[1,:])==[S.Rational(5,12),S.Rational(1,2)]
assert list(K[2,:])==[S.Rational(7,12),0]
assert list(T[0,:])==[S.Rational(3,4),0]
assert list(T[1,:])==[0,S.Rational(1,2)]
assert list(T[2,:])==[S.Rational(-1,4),0]
assert list(T[3,:])==[0,S.Rational(-1,2)]
I1p=S.factor(S.integrate((x-c)*(1-S.Rational(4,3)*x),(x,c,S.Rational(3,4))))
I2p=S.factor(S.integrate((x-c)**2*(1-S.Rational(4,3)*x),(x,c,S.Rational(3,4))))
I1n=S.factor(S.integrate((x-c)*(1+4*x),(x,c,0))+S.integrate((x-c)*(1-S.Rational(4,3)*x),(x,0,S.Rational(3,4))))
I2n=S.factor(S.integrate((x-c)**2*(1+4*x),(x,c,0))+S.integrate((x-c)**2*(1-S.Rational(4,3)*x),(x,0,S.Rational(3,4))))

def bernstein(poly):
 po=S.Poly(S.expand(poly),p,u);n,m=po.degree(p),po.degree(u);d=po.as_dict();out=[]
 for i,j in itertools.product(range(n+1),range(m+1)):
  z=sum(a*S.Rational(math.comb(i,k),math.comb(n,k))*S.Rational(math.comb(j,l),math.comb(m,l)) for(k,l),a in d.items() if k<=i and l<=j)
  out.append(z)
 return n,m,out
records=[]
for lo,hi,rr,I1,I2,pref in [(0,S.Rational(1,2),eta/6,I1p,I2p,tau*eta**2),(S.Rational(1,2),S.Rational(3,4),S.Rational(1,12),I1p,I2p,tau),(S.Rational(3,4),1,S.Rational(1,12),I1n,I2n,tau)]:
 cc=S.Rational(3,4)-eta;QQ=S.factor(Q.subs({r:rr,c:cc}));den=2-2*cc*tau
 ii1=I1.subs(c,cc);ii2=I2.subs(c,cc)
 cleared=S.factor(8*(den**2/S.Integer(2)-S.Rational(5,6)*tau*den*ii1-S.Rational(1,3)*tau**2*ii2)*QQ**2-den**2)
 reduced=S.cancel(cleared/pref)
 assert S.denom(reduced).free_symbols==set()
 normalized=S.expand(reduced.subs({eta:lo+(hi-lo)*p,tau:u/100}))
 n,m,coeff=bernstein(normalized);assert all(z>0 for z in coeff)
 records.append({'eta_interval':[str(lo),str(hi)],'r':str(rr),'factor_removed':str(pref),'degree':[n,m],'minimum_bernstein_coefficient':str(min(coeff)),'bernstein_coefficients':[str(z) for z in coeff]})
out={'status':'PASS uniform exact facet-split slice','domain':'0<tau<=1/100, -1/4<c<3/4','new_normal':'(0,2-2*c*tau,tau,0)','word':word,'beta':[str(z) for z in beta],'Q':str(S.factor(Q)),'I1_positive_c':str(I1p),'I2_positive_c':str(I2p),'I1_negative_c':str(I1n),'I2_negative_c':str(I2n),'records':records}
(root/'certificate.json').write_text(json.dumps(out,indent=2)+'\n')
print('PASS: exact feasible positive eight-label section across the full cut-fraction interval.')
print('PASS: exact cap volume and three strictly positive Bernstein boxes, tau<=1/100.')
print('Consequence: every genuine split in this slice has rho<1, uniformly even as c approaches either cut-fraction endpoint.')
