exec(open('exact_certify.py').read().split('start=time.monotonic();')[0])
word=[0,1,6,7,3,12,9];C,H=CH(word);n=len(word)
M=block_matrix(K,[[H,C.transpose()],[C,zero_matrix(K,5,5)]])
sol=M.solve_right(vector(K,[0]*n+[0,0,0,0,1]));b=vector(K,sol[:n]);q=b*H*b/2
assert all(emb(v)>0 for v in b) and C*b==vector(K,[0,0,0,0,1])
assert q==QQ(205)/1019*u
V=QQ(300748771)/52178240-QQ(99664289)/260891200*u*u
rho=1/(8*V*q*q);base=(3+s)/5
assert emb(rho-base)>0
summary={'parameter':'3/10','word':word,'positive_exact_weights':list(map(str,b)),
 'Q':str(q),'capacity_if_upper_bound_passes':str(1/(2*q)),
 'volume':str(V),'ratio_if_upper_bound_passes':str(rho),
 'ratio_numeric':float(emb(rho)),'improvement_numeric':float(emb(rho-base)),
 'status':'Exact feasible attainer and exact volume; global capacity upperQ bound remains separate.'}
(out/'exact-attainer.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
