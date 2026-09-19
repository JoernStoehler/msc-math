"""Independent integer determinant replay of retained segment polynomials.

Uses Python integers and the Leibniz determinant, not numpy or Pfaffians.
This verifies finite evidence, not the universal pairing-volume conjecture.
"""
from __future__ import annotations
import itertools
import json
from pathlib import Path
import time

ROOT=Path(__file__).parent
PERMS=[(p,(-1)**sum(p[i]>p[j] for i in range(4) for j in range(i+1,4))) for p in itertools.permutations(range(4))]


def determinant(rows):
    return sum(s*rows[0][p[0]]*rows[1][p[1]]*rows[2][p[2]]*rows[3][p[3]] for p,s in PERMS)


def omega(a,b):
    return a[0]*b[2]+a[1]*b[3]-a[2]*b[0]-a[3]*b[1]


def phi(rows):
    return sum(abs(omega(a,b)) for a,b in itertools.combinations(rows,2))


def integer(rows):
    assert all(x==int(x) for row in rows for x in row)
    return [[int(x) for x in row] for row in rows]


def volume(rows):
    return 16*sum(abs(determinant(q)) for q in itertools.combinations(rows,4))


def main():
    start=time.perf_counter()
    packet=json.loads((ROOT/"segment_paths.json").read_text())
    bases={b["body_id"]:integer(b["generators"]) for b in packet["bases"]}
    checked=0
    certified=0
    for row in packet["paths"]:
        v=bases[row["base_id"]];u,w=integer([row["u"],row["w"]])
        co=[volume(v),16*sum(abs(determinant([*q,u])) for q in itertools.combinations(v,3)),
            16*sum(abs(determinant([*q,w])) for q in itertools.combinations(v,3)),
            16*sum(abs(determinant([*q,u,w])) for q in itertools.combinations(v,2))]
        ph=[phi(v),sum(abs(omega(x,u)) for x in v),sum(abs(omega(x,w)) for x in v),abs(omega(u,w))]
        assert co==row["coefficients"]["volume"]
        assert ph==row["coefficients"]["phi"]
        a,b,c,d=co;x,y,z,k=ph
        # F=4Phi^2-V = constant + linear terms +
        # 4(yt-zs)^2 + (16yz+8xk-d)ts +
        # 8yk t^2s + 8zk ts^2 + 4k^2t^2s^2.
        positive=all(q>=0 for q in (4*x*x-a,8*x*y-b,8*x*z-c,16*y*z+8*x*k-d))
        assert positive==row["all_nonnegative_parameters_certificate"]
        certified+=positive;checked+=1
    # Published source uses [0,v] rather than [-v,v].
    sc=json.loads((ROOT/"scout.json").read_text())
    source=[r for r in sc["controls"] if r["source"].startswith("Skorupinski")]
    source_volumes=[volume(integer(r["generators"]))//16 for r in source]
    assert source_volumes==[14,30,26,56]
    ad=json.loads((ROOT/"adversarial.json").read_text())
    exact_checked=0;degenerate=0
    # Independent determinant/omega verification for every integer sample.
    for group in ad["integer_populations"]:
        for v,stored_phi,stored_psi,slack in zip(group["generators"],group["phi"],group["psi"],group["exact_candidate_slack"]):
            p=phi(v);psi=volume(v)//16
            assert p==stored_phi and psi==stored_psi and p*p-4*psi==slack
            assert slack>=0
            exact_checked+=1;degenerate+=(psi==0)
    result={"status":"passed","path_polynomials_checked_with_integer_determinants":checked,
            "all_parameter_sufficient_certificates_verified":certified,
            "integer_samples_checked_independently":exact_checked,"rank_deficient_integer_controls":degenerate,
            "source_volumes_original_convention":source_volumes,
            "universal_conjecture_proved":False,"wall_seconds":time.perf_counter()-start}
    (ROOT/"validation.json").write_text(json.dumps(result,indent=2)+"\n")
    print(json.dumps(result,indent=2))


if __name__=="__main__":main()
