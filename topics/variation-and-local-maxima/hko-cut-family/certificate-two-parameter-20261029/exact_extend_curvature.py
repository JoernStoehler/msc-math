from common import *
start=time.monotonic()
base=set(map(tuple,load('base-exclusions.txt')))
original=set(map(tuple,load('proposed-exclusions.txt')))
receipt=json.loads((out/'curvature-exact-summary.json').read_text())
frontier=set(map(tuple,receipt['rejected']))
assert base|frontier==original and not base&frontier
raw=list(map(int,(out/'graph.txt').read_text().split()));assert len(raw)==400 and all(x in [0,1] for x in raw)
graph=[raw[20*i:20*i+20] for i in range(20)]
perms=[]
for ref in range(2):
 for sw in range(2):
  for rot in range(5):
   p=[]
   for x in range(20):
    b=x//10;f=(x%10)//5;i=x%5
    if ref:i=4-i
    if sw:f=1-f
    p.append(b*10+f*5+(i+rot)%5)
   perms.append((ref,p))
def canonical(word):
 best=None
 for ref,p in perms:
  v=[p[x] for x in word]
  if ref:v.reverse()
  j=v.index(min(v));v=tuple(v[j:]+v[:j])
  if best is None or v<best:best=v
 return best
accepted=set();proposed=set();checked=original.copy();rounds=[];rejected=set()
for rr in range(1,3):
 candidates=set()
 for word in frontier:
  unused=set(range(20))-set(word)
  for shift in range(len(word)):
   v=word[shift:]+word[:shift]
   if any(not graph[a][b] for a,b in zip(v,v[1:])):continue
   for j in unused:
    if graph[v[-1]][j]:candidates.add(canonical(v+(j,)))
    if graph[j][v[0]]:candidates.add(canonical((j,)+v))
 candidates-=checked
 print('extension round',rr,'new proposals',len(candidates),flush=True)
 passed=set();failed=set()
 for k,word in enumerate(sorted(candidates)):
  C,H=CH(word);Z=C.right_kernel_matrix()
  (passed if positive(Z*H*Z.transpose()) else failed).add(word)
  if k%1000==0:print('extension',rr,k,'/',len(candidates),'passed',len(passed),'elapsed',round(time.monotonic()-start,2),flush=True)
 accepted|=passed;proposed|=candidates;rejected|=failed;checked|=candidates
 rounds.append({'round':rr,'proposed':len(candidates),'accepted':len(passed),'rejected':len(failed)})
 frontier=failed
for f,words in [('extension-proposals.txt',proposed),('extension-exclusions.txt',accepted),('exclusions.txt',base|accepted)]:
 (out/f).write_text(''.join(str(len(w))+' '+' '.join(map(str,w))+'\n' for w in sorted(words)))
summary={'parameter':body_label,'rounds':rounds,'proposed':len(proposed),'checked':len(proposed),'accepted':len(accepted),'rejected_count':len(rejected),'rejected':sorted(rejected),'total_accepted_exclusions':len(base|accepted),'seconds':time.monotonic()-start}
(out/'curvature-extension-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print(json.dumps({k:v for k,v in summary.items() if k!='rejected'},indent=2),flush=True)
