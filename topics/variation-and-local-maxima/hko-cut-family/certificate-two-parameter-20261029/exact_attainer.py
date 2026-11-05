from common import *
candidates=[]
for word in [[0,1,6,7,3,12,9],[0,6,5,2,19,11,8,4]]:
 C,H=CH(word);n=len(word)
 M=block_matrix(K,[[H,C.transpose()],[C,zero_matrix(K,5,5)]])
 assert M.det()!=0
 sol=M.solve_right(vector(K,[0]*n+[0,0,0,0,1]));b=vector(K,sol[:n]);q=b*H*b/2
 assert all(emb(v)>0 for v in b) and C*b==vector(K,[0,0,0,0,1])
 candidates.append({'word':word,'positive_exact_weights':list(map(str,b)),'Q':str(q),'Q_numeric':float(emb(q))})
best=max(candidates,key=lambda v:emb(K(v['Q'])))
summary={'parameter':body_label,'candidates':candidates,**best,'status':'Positive feasible objective only; global maximum and ratio require separate complete leaf, coverage and geometry receipts.'}
(out/'exact-attainer.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
