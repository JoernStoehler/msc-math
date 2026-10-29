#include <algorithm>
#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <set>
#include <vector>
using namespace std;
set<vector<int>> exclusions,leaves;uint32_t adj[20];vector<int>w;uint64_t prefixes=0,pruned=0,cycles=0,unknown=0;int maxlen=0;
vector<int> canonical(const vector<int>& word){vector<int> best;for(int refl=0;refl<2;refl++)for(int sw=0;sw<2;sw++)for(int rot=0;rot<5;rot++){vector<int> v;for(int x:word){int block=x/10,fac=x%10,f=fac/5,i=fac%5;if(refl)i=4-i;i=(i+rot)%5;if(sw)f=1-f;v.push_back(block*10+f*5+i);}if(refl)reverse(v.begin(),v.end());rotate(v.begin(),min_element(v.begin(),v.end()),v.end());if(best.empty()||v<best)best=v;}return best;}
void load(const char*name,set<vector<int>>&dst){ifstream f(name);int n;while(f>>n){vector<int>v(n);for(int&i:v)f>>i;if(!f){cerr<<"truncated input";exit(2);}if(canonical(v)!=v){cerr<<"noncanonical input";exit(2);}dst.insert(v);}}
void dfs(int first,int last,uint32_t used){prefixes++;auto key=canonical(w);if(exclusions.count(key)){pruned++;return;}if(w.size()>=2&&(adj[last]&(1u<<first))){cycles++;maxlen=max(maxlen,int(w.size()));if(!leaves.count(key))unknown++;}uint32_t next=adj[last]&~used&(~((1u<<(first+1))-1));while(next){int j=__builtin_ctz(next);next&=next-1;w.push_back(j);dfs(first,j,used|(1u<<j));w.pop_back();}}
int main(){load("exclusions.txt",exclusions);load("leaves.txt",leaves);ifstream f("graph-0.3.txt");for(int i=0;i<20;i++)for(int j=0;j<20;j++){int x;f>>x;if(x)adj[i]|=1u<<j;}if(!f)return 2;for(int i=0;i<20;i++){w={i};dfs(i,i,1u<<i);}cout<<"{\"verified\":"<<(unknown==0?"true":"false")<<",\"prefixes\":"<<prefixes<<",\"curvature_prunes\":"<<pruned<<",\"cycles\":"<<cycles<<",\"maximum_word_length\":"<<maxlen<<",\"unknown_cycles\":"<<unknown<<",\"certified_exclusion_representatives\":"<<exclusions.size()<<",\"checked_leaf_representatives\":"<<leaves.size()<<"}\n";return unknown?1:0;}
