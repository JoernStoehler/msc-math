#include <algorithm>
#include <array>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <vector>
using namespace std;
uint64_t encode(const vector<int>& v){
  int k=min_element(v.begin(),v.end())-v.begin(); uint64_t c=0;
  for(int j=0;j<(int)v.size();++j)c|=uint64_t(v[(k+j)%v.size()]+1)<<(4*j);
  return c;
}
int main(int argc,char**argv){
 if(argc!=3)return 2;
 ifstream in(argv[1]);int n;in>>n;
 set<uint64_t>wins;int q,p;uint64_t code;
 for(int i=0;i<n;++i){in>>q>>p>>code;wins.insert(code);}
 array<int,10> tri;for(auto &t:tri)in>>t;
 array<int,10>word;for(int i=0;i<10;++i)word[i]=i;
 map<uint32_t,vector<pair<int,int>>> cache;
 set<uint64_t> charts;int full=0,positive=0;
 do{
  ++full;uint32_t graph=0;
  for(q=0;q<5;++q)for(p=0;p<5;++p){
   int mask=tri[q]|tri[5+p];vector<int>sub;
   for(int x:word)if(mask&(1<<x))sub.push_back(x);
   if(wins.count(encode(sub)))graph|=uint32_t(1)<<(5*q+p);
  }
  if(!graph)continue;++positive;
  auto pos=cache.find(graph);
  if(pos==cache.end()){
   vector<pair<int,int>> bic;
   for(int r=1;r<32;++r){
    int cols=31;for(int i=0;i<5;++i)if(r&(1<<i))cols&=(graph>>(5*i))&31;
    if(!cols)continue;
    int rr=0;for(int i=0;i<5;++i)if((((graph>>(5*i))&31)&cols)==cols)rr|=1<<i;
    if(rr==r)bic.emplace_back(r,cols);
   }
   pos=cache.emplace(graph,bic).first;
  }
  for(auto [r,c]:pos->second){
   int mask=0;for(int i=0;i<5;++i){if(r&(1<<i))mask|=tri[i];if(c&(1<<i))mask|=tri[5+i];}
   vector<int>sub;for(int x:word)if(mask&(1<<x))sub.push_back(x);
   charts.insert(encode(sub)|(uint64_t(r)<<(40))|(uint64_t(c)<<(45)));
  }
 }while(next_permutation(word.begin()+1,word.end()));
 ofstream out(argv[2]);out<<"{\n\"full_cyclic_orders\":"<<full<<",\n\"orders_with_maximizers\":"<<positive<<",\n\"distinct_active_graphs\":"<<cache.size()<<",\n\"charts\":[\n";
 bool comma=false;
 for(uint64_t k:charts){
  if(comma)out<<",\n";comma=true;
  int r=(k>>40)&31,c=(k>>45)&31;
  out<<"{\"q_vertices_mask\":"<<r<<",\"p_vertices_mask\":"<<c<<",\"sigma\":[";
  uint64_t v=k&((uint64_t(1)<<40)-1);bool cc=false;
  while(v){if(cc)out<<",";cc=true;out<<int(v&15)-1;v>>=4;}
  out<<"]}";
 }
 out<<"\n]}\n";
 cout<<"orders "<<full<<" positive "<<positive<<" graphs "<<cache.size()<<" charts "<<charts.size()<<"\n";
}
