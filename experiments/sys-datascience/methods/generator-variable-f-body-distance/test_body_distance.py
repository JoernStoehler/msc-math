#!/usr/bin/env python3
"""Focused executable checks for the variable-facet body-distance packet."""
import importlib.util
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT=Path(__file__).resolve().parents[4]
MODULE=Path(__file__).with_name("body_distance.py")
spec=importlib.util.spec_from_file_location("body_distance",MODULE)
mod=importlib.util.module_from_spec(spec); assert spec.loader is not None; sys.modules[spec.name]=mod; spec.loader.exec_module(mod)

def test_reconstruction_boundary() -> None:
    cube=mod.reconstruct_body("cube",mod.cube_normals()); assert cube.exact_vertex_count==16
    try: mod.reconstruct_body("redundant",mod.cube_normals()+[mod.cube_normals()[0]])
    except mod.ReconstructionError as exc: assert "redundant" in str(exc)
    else: raise AssertionError("redundant normalized inequality accepted")

def test_packet_controls_and_cross_f() -> None:
    report=mod.run_packet(ROOT/"experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl",ROOT/"experiments/sys-datascience/methods/generator-exact-feature-augmenter/artifacts/full-panels/tangential-replay/smoke-rows.jsonl")
    assert all(report["controls"].values()),report["controls"]
    cases={row["case"]:row for row in report["comparisons"]}
    assert cases["facet_birth_truncated_cube"]["left_facets"]!=cases["facet_birth_truncated_cube"]["right_facets"]
    axis=cases["adversarial_narrow_feature"]["direct_sampled"]["axis_only"]["linf"]
    fine=cases["adversarial_narrow_feature"]["direct_sampled"]["primitive_level_3"]["linf"]
    assert axis <= .51*fine and fine>1e-4

def test_cli_replay() -> None:
    with tempfile.TemporaryDirectory() as first, tempfile.TemporaryDirectory() as second:
        command=["uv","run","--script",str(MODULE),"--out-dir"]
        subprocess.run(command+[first],cwd=ROOT,check=True); subprocess.run(command+[second],cwd=ROOT,check=True)
        a=json.loads((Path(first)/"report.json").read_text()); b=json.loads((Path(second)/"report.json").read_text())
        for report in (a,b):
            report.pop("cost_observations_seconds")
            for row in report["comparisons"]:row.pop("cost_observation_seconds")
        assert a==b,"only timing observations may vary between deterministic replays"

if __name__=="__main__":
    test_reconstruction_boundary(); test_packet_controls_and_cross_f(); test_cli_replay(); print("variable-facet body-distance tests: PASS")
