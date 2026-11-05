import itertools, math, json, time
from pathlib import Path
import numpy as np

def closures(n):
    h=math.cos(math.pi/n)
    a=np.exp(2j*math.pi*np.arange(n)/n)/h
    out=[]
    for ids in itertools.combinations(range(n),3):
        z=a[list(ids)]
        mat=np.vstack([z.real,z.imag,np.ones(3)])
        if abs(np.linalg.det(mat))<1e-10: continue
        w=np.linalg.solve(mat,[0,0,1])
        if min(w)>1e-10:
            out.append((ids,w,z*w))
    return out

def scan(n,m):
    ca,cb=closures(n),closures(m)
    ha,hb=math.cos(math.pi/n),math.cos(math.pi/m)
    wa=1+ha if n%2 else 2*ha
    wb=1+hb if m%2 else 2*hb
    sides=math.lcm(n,2,m)
    # lcm of n,m,2 is both width polygon side-count lcm.
    rho=.5/(wa*wb)
    coeff=[]; labels=[]
    for ia,(ids,w,x) in enumerate(ca):
        for ib,(jds,v,y) in enumerate(cb):
            for orderx in [(0,1,2),(0,2,1)]:
                for ordery in itertools.permutations(range(3)):
                    z=.5*(x[orderx[0]]*y[ordery[0]].conjugate()-x[orderx[2]]*y[ordery[1]].conjugate())
                    coeff.append(z); labels.append((ids,jds,orderx,ordery))
    z=np.array(coeff)
    if len(z):
        # regular H4 polygon edge normals are (2k+1)*pi/sides.
        delta=(np.angle(z)-math.pi/sides+math.pi/sides)%(2*math.pi/sides)-math.pi/sides
        gauges=np.abs(z)/rho*np.cos(delta)/math.cos(math.pi/sides)
        k=int(np.argmax(gauges)); radial=float(max(abs(z))/rho)
        gauge=float(gauges[k]); witness=labels[k]
    else: radial=0.; gauge=0.; witness=None
    # capacity ratio at maximum angular switch if H6 contained.
    area=lambda k: .5*k*math.sin(2*math.pi/k)
    upper=(wa*wb/math.cos(math.pi/sides))**2/(2*area(n)*area(m))
    return dict(n=n,m=m,closures_n=len(ca),closures_m=len(cb),six_coefficients=len(z),four_hull_sides=sides,max_six_gauge=gauge,max_six_radial_ratio=radial,incircle_radius_ratio=math.cos(math.pi/sides),four_block_max_sys=upper,max_gauge_witness=witness)

if __name__=='__main__':
    t=time.monotonic(); rows=[scan(n,m) for n in range(3,13) for m in range(n,13)]
    out=dict(contract='Floating-point exhaustive support and twelve-order enumeration; not exact certification',rows=rows,seconds=time.monotonic()-t)
    Path(__file__).with_name('scan.json').write_text(json.dumps(out,indent=2)+'\n')
    for r in rows:
        print(r['n'],r['m'],'gauge',round(r['max_six_gauge'],9),'radial',round(r['max_six_radial_ratio'],9),'incircle',round(r['incircle_radius_ratio'],9),'maxsys',round(r['four_block_max_sys'],9))
