from common import *
start=time.monotonic();exc=load('proposed-exclusions.txt');accepted=[];rejected=[]
for k,word in enumerate(exc):
 C,H=CH(word);Z=C.right_kernel_matrix();B=Z*H*Z.transpose()
 (accepted if positive(B) else rejected).append(word)
 if k%1000==0:print('curvature',k,'/',len(exc),'elapsed',round(time.monotonic()-start,2),'rejected',len(rejected),flush=True)
(out/'base-exclusions.txt').write_text(''.join(str(len(w))+' '+' '.join(map(str,w))+'\n' for w in accepted))
(out/'curvature-exact-summary.json').write_text(json.dumps({'parameter':body_label,'proposed':len(exc),'checked':len(accepted),'rejected_count':len(rejected),'rejected':rejected,'failed':[],'seconds':time.monotonic()-start},indent=2)+'\n')
print('accepted',len(accepted),'rejected',len(rejected),'seconds',time.monotonic()-start,flush=True)
