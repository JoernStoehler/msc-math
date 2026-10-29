exec(open('exact_certify.py').read().split('start=time.monotonic();')[0])
from itertools import combinations
start=time.monotonic();vertices={};graph=matrix(ZZ,20,20)
for ids in combinations(range(20),4):
 M=N.matrix_from_rows(ids)
 if M.det()==0:continue
 v=M.solve_right(vector(K,[1]*4));slacks=vector(K,[1]*20)-N*v
 if any(emb(x)<0 for x in slacks):continue
 active=tuple(i for i,x in enumerate(slacks) if x==0)
 assert len(active)==4
 vertices[active]=v
 for i in active:
  for j in active:
   if i!=j and emb(O[i,j])>=0:graph[i,j]=1
assert set().union(*(set(ids) for ids in vertices))==set(range(20))
print('exact vertices',len(vertices),'edges',sum(graph.list()),'seconds',time.monotonic()-start,flush=True)
# Compare with discovery graph: it must retain every exact transition edge.
lines=open('graph-0.3.txt').read().split();old=matrix(ZZ,20,20,list(map(int,lines)))
assert all(graph[i,j]<=old[i,j] for i in range(20) for j in range(20))
assert graph==old
# Exact simple-polytope volume by the vertex tangent-cone formula (dimension4).
# Check two generic linear functionals independently; no geometric rounding.
volumes=[]
for ell in [vector(K,[1,2,4,8]),vector(K,[1,3,9,27])]:
 total=K(0)
 for ids,v in vertices.items():
  M=N.matrix_from_rows(ids);det=M.det();detabs=det if emb(det)>0 else -det
  slopes=ell*M.inverse()
  assert all(z!=0 for z in slopes)
  total+=(ell*v)**4/(24*detabs*prod(slopes))
 assert emb(total)>0;volumes.append(total)
assert volumes[0]==volumes[1]
V=volumes[0]
print('volume',V,'numeric',float(emb(V)),flush=True)
(out/'exact-geometry-summary.json').write_text(json.dumps({'parameter':str(t),'vertices':len(vertices),'edges':int(sum(graph.list())),'discovery_graph_matches_exact':True,'volume':str(V),'volume_numeric':float(emb(V)),'all_twenty_facets_genuine':True,'volume_methods':'Lawrence simple-polytope formula; two exact generic linear functionals','seconds':time.monotonic()-start},indent=2)+'\n')
