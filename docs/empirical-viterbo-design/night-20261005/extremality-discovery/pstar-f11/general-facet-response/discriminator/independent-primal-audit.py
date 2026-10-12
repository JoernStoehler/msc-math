# /// script
# dependencies = ["sympy"]
# ///
"""Read-only positive certificate audit using independently derived slot rows."""
import hashlib,json,runpy
from pathlib import Path
import sympy as S
here=Path(__file__).resolve().parent
ctx=runpy.run_path(str(here/'independent-slot-audit.py'))
repo,root,A,rows,bank=[ctx[k] for k in ['repo','root','A','rows','bank']]
r,delta=ctx['r'],ctx['d'];v,s=ctx['v'],ctx['s'];base=ctx['base']
path=here/'certificate.json';raw=path.read_bytes();package=json.loads(raw);candidate=package['primal']
assert package['D']==['1','-1','0','1'] and S.Rational(package['L0'])==S.Rational(625,497664)
assert S.Rational(package['facet0_positive_fraction'])==S.Rational(25,432)
external=json.loads((here/'formulas.json').read_text())
for key in external:
    if key in ['status','schemas']:continue
    assert package['formulas'][key]==external[key]
for supplied,embedded in zip(external['schemas'],package['formulas']['schemas']):
    for key in supplied:assert supplied[key]==embedded[key]
    assert embedded['closure_pivot_labels']==[embedded['word'][i] for i in embedded['closure_pivot_positions']]
assert candidate==json.loads((here/'primal-exchange-proposal.json').read_text())
for source,metadata in package['sources'].items():
    content=(repo/source).read_bytes()
    assert len(content)==metadata['bytes'] and hashlib.sha256(content).hexdigest()==metadata['sha256']
old=json.loads((repo/'experiments/hko-local-maximum/non-hko/certificate/certificate_base.json').read_text())
total=S.Rational(0);weighted=S.Rational(0);balanced=S.zeros(40,1);contact_count=0
for point in candidate['participants']:
    coefficient=S.Rational(point['lambda']);assert coefficient>0
    total+=coefficient
    if point['kind']=='old':
        index=point['index'];gradient=S.Matrix(old['gradient_rows'][index]);ell=S.Rational(0)
        assert min(S.Matrix(old['sections'][index]['beta']))>0
    else:
        contact_count+=1
        independent=next(z for z in rows if z['schema']==point['schema'] and z['slot']==point['slot'])
        assert independent['touching']
        substitution={delta:S.Rational(point['delta'])}
        if point['r'] is not None:substitution[r]=S.Rational(point['r'])
        gradient=S.Matrix([S.sympify(z,locals={'r':r,'delta':delta}).subs(substitution) for z in independent['gradient']])
        ell=S.sympify(independent['ell'],locals={'r':r,'delta':delta}).subs(substitution)
        schema=next(z for z in bank if z['id']==point['schema'])
        beta=[S.sympify(z,locals={'v':v,'s':s,'r':r}).subs(base).subs(substitution) for z in schema['beta']]
        word=independent['word'];weights=S.Matrix([substitution[delta] if lab==10 else beta[schema['word'].index(lab)]-(substitution[delta] if lab==0 else 0) for lab in word])
        assert min(weights)>0
        B=A.col_join(A[0,:])[word,:];C=B.T.col_join(S.ones(1,len(word)))
        assert C*weights==S.Matrix([0,0,0,0,1])
        oldC=A[schema['word'],:].T.col_join(S.ones(1,len(schema['word'])))
        assert oldC[:,schema['closure_pivot_positions']].det()!=0
        H=S.zeros(len(word));om=ctx['om']
        for i in range(len(word)):
            for j in range(i+1,len(word)):H[i,j]=H[j,i]=om(B[i,:],B[j,:])
        assert (weights.T*H*weights)[0]==1
        mu=S.Matrix([S.sympify(z,locals={'r':r,'delta':delta}).subs(substitution) for z in independent['mu']]+[1])
        assert H*weights==C.T*mu
    assert gradient==S.Matrix([S.Rational(z) for z in point['G']])
    assert ell==S.Rational(point['ell'])
    balanced+=coefficient*gradient;weighted+=coefficient*ell
assert total==1 and balanced==S.zeros(40,1)
cap=S.Rational(625,497664);gap=weighted-cap
assert gap>0 and weighted==S.Rational(candidate['response']) and gap==S.Rational(candidate['margin'])
assert len(candidate['participants'])==21 and contact_count==6
print('PASS 21 strictly positive rational coefficients; all21 source gradients/responses and full40 balance+normalization.')
print('PASS all6 contact points: strict weight positivity, exact closure/Q/stationarity and regular old closure minor.')
print('response:',weighted,'; positive response-minus-cap gap:',gap)
print('certificate SHA256:',hashlib.sha256(raw).hexdigest())
