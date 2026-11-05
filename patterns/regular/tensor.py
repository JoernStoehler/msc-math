from scan import *
from scipy.spatial import ConvexHull

def test(n,m):
    wn=1+math.cos(math.pi/n); wm=1+math.cos(math.pi/m)
    vx=np.exp(1j*math.pi*np.arange(2*n)/n)/wn
    vy=np.exp(1j*math.pi*np.arange(2*m)/m)/wm
    real=lambda z:np.array([z.real,z.imag])
    pts=np.array([.5*np.outer(real(x),real(y)).ravel() for x in vx for y in vy])
    hull=ConvexHull(pts)
    # normalize facet linear function so hull support is1
    normals=hull.equations[:,:4]/(-hull.equations[:,4:])
    best=(0.,None,None,None)
    for ids,w,x in closures(n):
        for jds,v,y in closures(m):
            for ox in [(0,1,2),(0,2,1)]:
                for oy in itertools.permutations(range(3)):
                    t=.5*(np.outer(real(x[ox[0]]),real(y[oy[0]]))-np.outer(real(x[ox[2]]),real(y[oy[1]])))
                    gauges=normals@t.ravel(); k=int(np.argmax(gauges)); g=float(gauges[k])
                    if g>best[0]:best=(g,(ids,jds,ox,oy),normals[k].reshape(2,2).tolist(),t.tolist())
    return dict(n=n,m=m,tensor_gauge=best[0],witness=best[1],separating_matrix=best[2],six_tensor=best[3],facets=len(normals))
if __name__=='__main__':
    rows=[]
    for n,m in [(5,5),(5,7),(7,7),(5,9),(7,9),(9,9),(5,11),(7,11),(9,11),(11,11)]:
        r=test(n,m);rows.append(r);print(n,m,r['tensor_gauge'],flush=True)
    Path(__file__).with_name('tensor.json').write_text(json.dumps(rows,indent=2)+'\n')
