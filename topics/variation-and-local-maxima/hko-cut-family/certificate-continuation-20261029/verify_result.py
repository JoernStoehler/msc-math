from common import *
import hashlib
read=lambda f:json.loads((out/f).read_text())
cur=read('curvature-exact-summary.json');geo=read('exact-geometry-summary.json');cov=read('coverage-summary.json');leaf=read('exact-leaves-summary.json');att=read('exact-attainer.json')
for r in [cur,geo,leaf,att]:assert r['parameter']==str(t)
proposed=set(map(tuple,load('proposed-exclusions.txt')));baseaccepted=set(map(tuple,load('base-exclusions.txt')));rejected=set(map(tuple,cur['rejected']))
accepted=set(map(tuple,load('exclusions.txt')))
ext=read('curvature-extension-summary.json');extaccepted=set(map(tuple,load('extension-exclusions.txt')));extproposed=set(map(tuple,load('extension-proposals.txt')));extrejected=set(map(tuple,ext['rejected']))
assert ext['parameter']==str(t) and extaccepted|extrejected==extproposed and not extaccepted&extrejected
assert ext['accepted']==len(extaccepted) and ext['rejected_count']==len(extrejected) and ext['proposed']==ext['checked']==len(extproposed)
assert accepted==baseaccepted|extaccepted and ext['total_accepted_exclusions']==len(accepted)
assert not baseaccepted.intersection(rejected) and baseaccepted.union(rejected)==proposed
assert cur['proposed']==len(proposed) and cur['checked']==len(baseaccepted) and cur['rejected_count']==len(rejected) and cur['failed']==[]
assert geo['vertices']==len(geo['vertex_active_sets'])==75 and geo['edges']==120 and geo['all_twenty_facets_genuine'] and geo['all_vertices_simple']
assert set().union(*map(set,geo['vertex_active_sets']))==set(range(20)) and all(len(s)==4 for s in geo['vertex_active_sets'])
assert K(geo['coarse_scale'])==1+2*t/5 and all(emb(K(x['support'])-K(geo['coarse_scale']))<=0 for x in geo['support_bounds_on_H'])
assert cov['mode']=='verification' and cov['verified'] and cov['unknown_cycles']==0
assert cov['certified_exclusion_representatives']==len(accepted)
nleaves=len(load('leaves.txt'))
assert cov['checked_leaf_representatives']==leaf['checked_leaf_words']==nleaves
assert leaf['curvature_exclusion_input_count_separately_verified']==len(accepted)
for f,h in leaf['input_sha256'].items():assert hashlib.sha256((out/f).read_bytes()).hexdigest()==h
counts=leaf['counts']
assert counts['singular']==counts['singular_consistent']+counts['singular_inconsistent']
assert counts['singular_consistent']==sum(counts[k] for k in ['singular_coarse_excluded','singular_vertex_excluded','singular_feasible','singular_below_target'])
assert nleaves==sum(counts[k] for k in ['inconsistent','singular_inconsistent','negative','nonnegative','singular_coarse_excluded','singular_vertex_excluded','singular_below_target'])
q=K(leaf['best_Q']);v=K(geo['volume']);wq=K(att['Q'])
assert q==wq
for rec in att['candidates']+[att]:
 C,H=CH(rec['word']);b=vector(K,list(map(K,rec['positive_exact_weights'])))
 assert C*b==vector(K,[0,0,0,0,1]) and all(emb(x)>0 for x in b)
 assert b*H*b/2==K(rec['Q']) and emb(K(rec['Q'])-q)<=0
C,H=CH(leaf['word']);b=vector(K,list(map(K,leaf['nonnegative_weights'])))
assert C*b==vector(K,[0,0,0,0,1]) and all(emb(x)>=0 for x in b) and b*H*b/2==q
ratio=1/(8*v*q*q);base=(3+s)/5
oldq=QQ(205)/1019*u;oldv=QQ(300748771)/52178240-QQ(99664289)/260891200*u*u;oldratio=1/(8*oldv*oldq**2)
assert emb(ratio-oldratio)>0 and emb(oldratio-base)>0
result={'verified_finite_predicates':True,'parameter':str(t),'facets':20,'Qmax':str(q),'capacity':str(1/(2*q)),'capacity_relative_to_HKO':str((u/5)/q),'volume':str(v),'systolic_ratio':str(ratio),'systolic_ratio_numeric':float(emb(ratio)),'improvement_over_HKO_numeric':float(emb(ratio-base)),'improvement_over_t03_numeric':float(emb(ratio-oldratio)),'old_t03_ratio_numeric':float(emb(oldratio)),'accepted_curvature_exclusions':len(accepted),'discarded_old_exclusions':len(rejected),'leaf_words':nleaves,'mathematical_coverage':'COVERAGE-PROOF.md; published geometric simple-minimizer theorem and Lawrence formula are separate proof premises; audit and fresh-copy replay are separately recorded.'}
(out/'verification-summary.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
