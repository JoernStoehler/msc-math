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
    try: mod.reconstruct_body("vertex_touch",mod.cube_normals()+[["1/4"]*4])
    except mod.ReconstructionError as exc: assert "nonfacet" in str(exc)
    else: raise AssertionError("vertex-touching redundant inequality accepted")
    try: mod.reconstruct_body("unbounded",mod.unbounded_counterexample_normals())
    except mod.ReconstructionError as exc: assert "unbounded recession witness" in str(exc)
    else: raise AssertionError("full-dimensional irredundant unbounded presentation accepted")

def test_packet_controls_and_cross_f() -> None:
    report=mod.run_packet(ROOT/"experiments/sys-datascience/methods/generator-orientation-smoke/artifacts/panel-2-per-bucket/rows.jsonl",ROOT/"experiments/sys-datascience/methods/generator-exact-feature-augmenter/artifacts/full-panels/tangential-replay/smoke-rows.jsonl")
    assert all(report["controls"].values()),report["controls"]
    cases={row["case"]:row for row in report["comparisons"]}
    assert cases["facet_birth_truncated_cube"]["left_facets"]!=cases["facet_birth_truncated_cube"]["right_facets"]
    axis=cases["adversarial_narrow_feature"]["direct_sampled"]["axis_only"]["linf"]
    fine=cases["adversarial_narrow_feature"]["direct_sampled"]["primitive_level_3"]["linf"]
    assert axis <= .51*fine and fine>1e-4
    translated_scaled=cases["translation_positive_scale"]["direct_sampled"]["primitive_level_3"]
    assert translated_scaled["linf"]<1e-12 and translated_scaled["l2"]<1e-12
    objective_fixture=cases["retained_exact_feature_witness"]
    for group in ("u2_finite_bank","so4_finite_bank"):
        values=objective_fixture[group]
        assert values["linf_winning_index"]!=values["l2_winning_index"]
        assert values["linf_min"] < .615 and values["l2_min"] > 0

def test_staged_dependency_rejects_clean_guard() -> None:
    with tempfile.TemporaryDirectory() as directory:
        repo=Path(directory)
        subprocess.run(["git","init","-q"],cwd=repo,check=True)
        subprocess.run(["git","config","user.email","test@example.invalid"],cwd=repo,check=True)
        subprocess.run(["git","config","user.name","Test"],cwd=repo,check=True)
        dependency=repo/"tracked-dependency.txt"; dependency.write_text("base\n")
        subprocess.run(["git","add","tracked-dependency.txt"],cwd=repo,check=True)
        subprocess.run(["git","commit","-qm","base"],cwd=repo,check=True)
        dependency.write_text("staged edit\n")
        subprocess.run(["git","add","tracked-dependency.txt"],cwd=repo,check=True)
        state=mod.git_state(repo)
        assert state["revision"] != "unknown" and state["tree"] != "unknown"
        assert state["tracked_clean_before_generation"] is False
        assert state["tracked_status_porcelain"] == ["M  tracked-dependency.txt"]
        try: mod.require_tracked_clean(state)
        except RuntimeError as exc: assert "not clean" in str(exc)
        else: raise AssertionError("--require-tracked-clean precondition accepted a staged tracked edit")

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
    test_reconstruction_boundary(); test_packet_controls_and_cross_f(); test_staged_dependency_rejects_clean_guard(); test_cli_replay(); print("variable-facet body-distance tests: PASS")
