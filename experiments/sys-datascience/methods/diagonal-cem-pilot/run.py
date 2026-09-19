"""One serial adaptive run; launch under the common flock and timeout180."""
import hashlib,json,os,subprocess,time
from pathlib import Path
P=Path(__file__).resolve().parent
ROOT=P.parents[3]
A=P/'artifacts-supported-v2'
TARGET=ROOT/'.git/codex/ds-first-wave/readiness/target/release'
ENV=dict(os.environ,RAYON_NUM_THREADS='2',OMP_NUM_THREADS='2',OPENBLAS_NUM_THREADS='2')
SEEDS=[202609140301,202609140302,202609140303]


def write(path, obj):
    with path.open('x') as f: json.dump(obj,f,indent=2);f.write('\n')


def rust(req):
    t=time.monotonic()
    child=subprocess.run([str(TARGET/'diagonal-cem-pilot')],input=json.dumps(req),text=True,capture_output=True,env=ENV,check=True,timeout=60)
    return json.loads(child.stdout),(time.monotonic()-t)*1000


def batch(seed,arm,generation,distribution):
    name=f'{seed}-{arm}-{generation}'
    req={'mode':'generate','seed':seed,'arm':arm,'generation':generation,'distribution':distribution}
    write(A/f'{name}-request.json',req)
    generated,cost=rust(req)
    generated['process_wall_ms']=cost
    write(A/f'{name}-generation.json',generated)
    if not generated['complete']: raise RuntimeError(f'{name}: construction cap exhausted; no update')
    ip=A/f'{name}-inputs.jsonl';op=A/f'{name}-results.jsonl'
    with ip.open('x') as f:
        for row in generated['rows']: f.write(json.dumps(row)+'\n')
    t=time.monotonic()
    subprocess.run(['python3',str(ROOT/'experiments/sys-datascience/methods/current-body-evaluator/evaluate.py'),
                    '--input',str(ip),'--output',str(op),'--binary',str(TARGET/'current-body-evaluator'),
                    '--build-receipt',str(ROOT/'experiments/sys-datascience/methods/current-body-evaluator/controls/build-receipt.json'),
                    '--timeout-seconds','5'],check=True,env=ENV)
    results=[json.loads(x) for x in op.read_text().splitlines()]
    write(A/f'{name}-cost.json',{'generation_process_ms':cost,'evaluation_process_ms':(time.monotonic()-t)*1000})
    assert len(results)==64 and [r['id'] for r in results]==[r['id'] for r in generated['rows']]
    if any(r['status']!='ok' for r in results): raise RuntimeError(f'{name}: target failure; no update')
    return [{'id':r['id'],'coordinates':r['input']['coordinates'],'sys':r['sys']} for r in results]


def update(seed,generation,population,previous):
    req={'mode':'update','population':population,'previous':previous}
    state,cost=rust(req)
    write(A/f'{seed}-update-{generation}.json',{'request':req,'result':state,'process_wall_ms':cost})
    return state['next']


def main():
    paths=[P/'Cargo.toml',P/'Cargo.lock',P/'src/main.rs',P/'run.py',P/'analyze.py',
           * (P.parent/'equal-budget-product-search/src').glob('*.rs'),P.parent/'equal-budget-product-search/Cargo.toml',
           ROOT/'crates/symplectic/src/geom/polygon.rs',TARGET/'diagonal-cem-pilot',TARGET/'current-body-evaluator',P.parent/'current-body-evaluator/src/main.rs',P.parent/'current-body-evaluator/evaluate.py',P.parent/'current-body-evaluator/controls/build-receipt.json', *sorted((ROOT/'crates/symplectic/src/algorithms/capacity_4d').rglob('*.rs'))]
    frozen={'schema':'diagonal-cem-pilot-supported-v2','seeds':SEEDS,'population':64,'elites':16,'generations_including_initial':4,
            'smoothing':.5,'variance_floor_fraction':.05,'construction_cap_per_generation':640,
            'shared_initialization':64,'charged_per_arm':256,'charged_total_cap':1536,'unique_evaluation_cap':1344,
            'per_input_seconds':5,'outer_wall_seconds':240, 'prior_exposure':'v1 stopped after 256 requests; admission mechanically aligned to unchanged evaluator; seeds and constants retained','no_update_on_incomplete_generation':True,
            'sha256':{str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths},
            'head':subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()}
    write(A/'frozen-config.json',frozen)
    start=time.monotonic()
    try:
        for seed in SEEDS:
            initial=batch(seed,'shared',0,None)
            distribution=update(seed,0,initial,None)
            for gen in (1,2,3):
                batch(seed,'iid',gen,None)
                population=batch(seed,'cem',gen,distribution)
                if gen<3: distribution=update(seed,gen,population,distribution)
        status={'complete':True}
    except Exception as exc:
        status={'complete':False,'error':repr(exc)}
    status['wall_ms']=(time.monotonic()-start)*1000
    write(A/'run-status.json',status)
    print(json.dumps(status))
    if not status["complete"]: raise SystemExit(1)


if __name__=='__main__':main()
