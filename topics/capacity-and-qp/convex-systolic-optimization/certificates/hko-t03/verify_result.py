exec(open('exact_certify.py').read().split('start=time.monotonic();')[0])
import json
read=lambda name:json.loads((out/name).read_text())
cur=read('curvature-exact-summary.json');geo=read('exact-geometry-summary.json');cov=read('coverage-summary.json');leaf=read('exact-refined-leaves-summary.json');att=read('exact-attainer.json')
assert cur['checked']==77927 and cur['failed']==[]
assert geo['vertices']==75 and geo['edges']==120 and geo['all_twenty_facets_genuine'] and geo['discovery_graph_matches_exact']
assert cov['verified'] and cov['unknown_cycles']==0 and cov['checked_leaf_representatives']==2326 and cov['certified_exclusion_representatives']==77927
assert leaf['checked_leaf_words']==2326 and leaf['counts']['singular_consistent']==1030
assert leaf['counts']['singular_below_target']==984 and leaf['counts']['singular_coarse_excluded']==21 and leaf['counts']['singular_vertex_excluded']==25 and leaf['counts']['singular_feasible']==0
q=K(leaf['best_Q']);v=K(geo['volume']);wq=K(att['Q'])
assert q==wq==QQ(205)/1019*u
assert v==K(att['volume']), 'Geometry and attainer volume receipts disagree'
assert 1/(2*q)==K(att['capacity_if_upper_bound_passes']), 'Capacity receipts disagree'
assert all(emb(K(b))>0 for b in att['positive_exact_weights'])
ratio=1/(8*v*q*q);base=(3+s)/5
assert ratio==K(att['ratio_if_upper_bound_passes']), 'Ratio receipts disagree'
assert emb(ratio-base)>0
assert (u/5)/q==QQ(1019)/1025
result={'verified_finite_predicates':True,'parameter':'3/10','facets':20,'capacity':'1019/(410*tau)','capacity_relative_to_HKO':'1019/1025','Qmax':str(q),'volume':str(v),'systolic_ratio':str(ratio),'systolic_ratio_numeric':float(emb(ratio)),'improvement_over_HKO_numeric':float(emb(ratio-base)),'mathematical_coverage':'COVERAGE-PROOF.md; independently audit published geometric simple-minimizer premise, cyclic curvature inheritance, graph enumeration and singular-family feasibility.'}
(out/'verification-summary.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
