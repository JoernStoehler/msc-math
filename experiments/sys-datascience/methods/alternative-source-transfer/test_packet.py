from __future__ import annotations
import json, tempfile
from pathlib import Path
import unittest
from validate_packet import EXPECTED, validate, check_no_target

def _minimal(tmp: Path):
    source=[]; feature=[]; selected_by_id={}
    for bucket in ("4x6", "6x6"):
        for i in range(3200):
            cid=f"{bucket}-{i}"
            source.append({"schema":"alternative-source-transfer-source-v1","candidate_id":cid,"logical_cell":cid,"identity_scope":"alternative-source-transfer-v1","law":"factorial-both","bucket":bucket,"row_index":i,"attempt":0,"geometry_fingerprint":cid,"volume":1.0})
            feature.append({"schema":"alternative-source-transfer-feature-v1","candidate_id":cid,"logical_cell":cid,"identity_scope":"alternative-source-transfer-v1","law":"factorial-both","bucket":bucket,"row_index":i,"attempt":0,"vertex_covariance_status":"eligible","vertex_covariance_rho":float(i+1),"ridge_symp_area_sum_over_volume_sqrt":float(i+1),"ridge_symp_area_max_share":0.5,"ridge_symp_area_ordering_failure_count":0,"source_geometry_fingerprint":cid})
        ridge_offset = 13 if bucket == "4x6" else 14
        for arm, offset in (("rho", 0), ("ridge", ridge_offset), ("control", 32)):
            for i in range(16):
                cid=f"{bucket}-{offset+i}"
                selected_by_id.setdefault(cid,{"candidate_id":cid,"logical_cell":cid,"bucket":bucket,"row_index":offset+i,"attempt":0,"memberships":[],"geometry_fingerprint":cid})["memberships"].append(arm)
    selected=list(selected_by_id.values())
    for name, data in (("source.jsonl",source),("features.jsonl",feature),("selection.jsonl",selected)):
        (tmp/name).write_text("\n".join(json.dumps(x) for x in data)+"\n")
    import hashlib
    h=lambda n: hashlib.sha256((tmp/n).read_bytes()).hexdigest()
    manifest={"schema":"alternative-source-transfer-manifest-v1","source_sha256":h("source.jsonl"),"feature_sha256":h("features.jsonl"),"selection_sha256":h("selection.jsonl"),"target_free":True,"identity_scope":EXPECTED["identity_scope"],"master_seed":EXPECTED["master_seed"],"control_seed":EXPECTED["control_seed"],"law":EXPECTED["law"],"buckets":EXPECTED["buckets"],"row_target_per_bucket":EXPECTED["row_target_per_bucket"],"row_cap_per_bucket":EXPECTED["row_cap_per_bucket"],"attempt_cap":EXPECTED["attempt_cap"],"source_count":6400,"feature_count":6400,"selection_count":91,"unique_target_rows":91,"arm_overlap_rows":5,"clean_commit":EXPECTED["clean_commit"],"lock_hash":EXPECTED["lock_hash"],"membership_counts":{b:{a:16 for a in ("rho","ridge","control")} for b in ("4x6","6x6")}}
    (tmp/"manifest.json").write_text(json.dumps(manifest))

def _target_rows(tmp: Path, sys_value=0.1):
    source={r["candidate_id"]:r for r in (json.loads(x) for x in (tmp/"source.jsonl").read_text().splitlines())}
    manifest=json.loads((tmp/"manifest.json").read_text()); out=[]
    for line in (tmp/"selection.jsonl").read_text().splitlines():
        pick=json.loads(line); src=source[pick["candidate_id"]]
        out.append({"schema":"alternative-source-transfer-target-v1","candidate_id":pick["candidate_id"],"logical_cell":pick["logical_cell"],"bucket":pick["bucket"],"selection_memberships":pick["memberships"],"geometry_fingerprint":pick["geometry_fingerprint"],"source_sha256":manifest["source_sha256"],"feature_sha256":manifest["feature_sha256"],"selection_sha256":manifest["selection_sha256"],"evaluator_source":"fake","evaluator_build":"fake","volume":src["volume"],"capacity":1.0,"sys":sys_value})
    return out

class PacketTests(unittest.TestCase):
  def test_source_feature_fingerprint_mutation_rejected_after_hash_rewrite(self):
    with tempfile.TemporaryDirectory() as d:
      p=Path(d); _minimal(p); lines=(p/"features.jsonl").read_text().splitlines(); row=json.loads(lines[0]); row["source_geometry_fingerprint"]="wrong"; lines[0]=json.dumps(row); (p/"features.jsonl").write_text("\n".join(lines)+"\n"); m=json.loads((p/"manifest.json").read_text()); m["feature_sha256"]=__import__("hashlib").sha256((p/"features.jsonl").read_bytes()).hexdigest(); (p/"manifest.json").write_text(json.dumps(m))
      with self.assertRaisesRegex(ValueError,"feature/source"): validate(p)

  def test_selection_source_mutation_rejected_after_hash_rewrite(self):
    with tempfile.TemporaryDirectory() as d:
      p=Path(d); _minimal(p); lines=(p/"selection.jsonl").read_text().splitlines(); row=json.loads(lines[0]); row["geometry_fingerprint"]="wrong"; lines[0]=json.dumps(row); (p/"selection.jsonl").write_text("\n".join(lines)+"\n"); m=json.loads((p/"manifest.json").read_text()); m["selection_sha256"]=__import__("hashlib").sha256((p/"selection.jsonl").read_bytes()).hexdigest(); (p/"manifest.json").write_text(json.dumps(m))
      with self.assertRaisesRegex(ValueError,"selection/source"): validate(p)

  def test_manifest_constant_mutation_rejected(self):
    with tempfile.TemporaryDirectory() as d:
      p=Path(d); _minimal(p); m=json.loads((p/"manifest.json").read_text()); m["master_seed"]+=1; (p/"manifest.json").write_text(json.dumps(m))
      with self.assertRaisesRegex(ValueError,"manifest master_seed"): validate(p)

  def test_selector_equivalence_against_frozen_artifact(self):
    root=Path(__file__).parent/"artifacts"/"transfer-v1"
    if not root.exists(): self.skipTest("production artifact not present")
    features=[json.loads(x) for x in (root/"features.jsonl").read_text().splitlines()]
    selected=[json.loads(x) for x in (root/"selection.jsonl").read_text().splitlines()]
    def key(row, field): return (row[field], row["candidate_id"])
    expected={"rho":set(),"ridge":set(),"control":set()}
    for bucket in ("4x6","6x6"):
      eligible=[r for r in features if r["bucket"]==bucket and r["vertex_covariance_status"]=="eligible"]
      rho=sorted(eligible,key=lambda r:key(r,"vertex_covariance_rho"))[:16]
      stage=sorted(eligible,key=lambda r:key(r,"ridge_symp_area_sum_over_volume_sqrt"))[:32]
      ridge=sorted(stage,key=lambda r:key(r,"ridge_symp_area_max_share"))[:16]
      arm={r["candidate_id"] for r in rho}|{r["candidate_id"] for r in ridge}
      expected["rho"]|={r["candidate_id"] for r in rho}; expected["ridge"]|={r["candidate_id"] for r in ridge}
    got={arm:{r["candidate_id"] for r in selected if arm in r["memberships"]} for arm in expected}
    self.assertEqual(got["rho"],expected["rho"]); self.assertEqual(got["ridge"],expected["ridge"])
    self.assertEqual(len(got["control"]),32); self.assertTrue(got["control"].isdisjoint(got["rho"]|got["ridge"]))

  def test_uniqueness_rejected(self):
    with tempfile.TemporaryDirectory() as d:
      tmp_path=Path(d); _minimal(tmp_path)
      rows=(tmp_path/"selection.jsonl").read_text().splitlines(); duplicate=json.loads(rows[1]); duplicate["geometry_fingerprint"]=json.loads(rows[0])["geometry_fingerprint"]; rows[1]=json.dumps(duplicate); (tmp_path/"selection.jsonl").write_text("\n".join(rows)+"\n")
      m=json.loads((tmp_path/"manifest.json").read_text()); m["selection_sha256"]=__import__("hashlib").sha256((tmp_path/"selection.jsonl").read_bytes()).hexdigest(); (tmp_path/"manifest.json").write_text(json.dumps(m))
      with self.assertRaisesRegex(ValueError,"duplicate geometry"): validate(tmp_path)

  def test_incomplete_source_rejected(self):
    with tempfile.TemporaryDirectory() as d:
      tmp_path=Path(d); _minimal(tmp_path)
      lines=(tmp_path/"source.jsonl").read_text().splitlines(); (tmp_path/"source.jsonl").write_text("\n".join(lines[:-1])+"\n")
      m=json.loads((tmp_path/"manifest.json").read_text()); m["source_sha256"]=__import__("hashlib").sha256((tmp_path/"source.jsonl").read_bytes()).hexdigest(); (tmp_path/"manifest.json").write_text(json.dumps(m))
      with self.assertRaisesRegex(ValueError,"incomplete frozen"): validate(tmp_path)

  def test_validation_only_has_no_target_path(self):
    with tempfile.TemporaryDirectory() as d:
      tmp_path=Path(d); _minimal(tmp_path)
      import subprocess
      out=subprocess.run(["python3",str(Path(__file__).parent/"validate_packet.py"),str(tmp_path),"--validate-only"],capture_output=True,text=True)
      self.assertEqual(out.returncode,0); self.assertIn("validation-only",out.stdout); self.assertNotIn("sys",out.stdout)

  def test_target_free_leakage_rejected(self):
    with tempfile.TemporaryDirectory() as d:
      tmp_path=Path(d); _minimal(tmp_path)
      feature_lines=(tmp_path/"features.jsonl").read_text().splitlines(); feature_lines[0]=json.dumps({"candidate_id":"4x6-0","sys":0.2}); (tmp_path/"features.jsonl").write_text("\n".join(feature_lines)+"\n")
      m=json.loads((tmp_path/"manifest.json").read_text()); m["feature_sha256"]=__import__("hashlib").sha256((tmp_path/"features.jsonl").read_bytes()).hexdigest(); (tmp_path/"manifest.json").write_text(json.dumps(m))
      with self.assertRaisesRegex(ValueError, "target leakage"): validate(tmp_path)

  def test_manifest_corruption_rejected(self):
    with tempfile.TemporaryDirectory() as d:
      tmp_path=Path(d); _minimal(tmp_path)
      m=json.loads((tmp_path/"manifest.json").read_text()); m["feature_sha256"]="bad"; (tmp_path/"manifest.json").write_text(json.dumps(m))
      with self.assertRaisesRegex(ValueError, "feature hash"): validate(tmp_path)

  def test_partial_target_rejected(self):
    with tempfile.TemporaryDirectory() as d:
      tmp_path=Path(d); _minimal(tmp_path)
      import subprocess
      p=tmp_path/"targets.jsonl"; p.write_text(json.dumps({"candidate_id":"missing","sys":0.1})+"\n")
      out=subprocess.run(["python3",str(Path(__file__).parent/"analyze.py"),str(tmp_path),"--targets",str(p)],capture_output=True,text=True)
      self.assertNotEqual(out.returncode, 0)

  def test_nonfinite_target_rejected(self):
    with tempfile.TemporaryDirectory() as d:
      tmp_path=Path(d); _minimal(tmp_path)
      targets=tmp_path/"targets.jsonl"; target_data=_target_rows(tmp_path, float("nan")); targets.write_text("\n".join(json.dumps(r) for r in target_data)+"\n")
      import subprocess
      out=subprocess.run(["python3",str(Path(__file__).parent/"analyze.py"),str(tmp_path),"--targets",str(targets)],capture_output=True,text=True)
      self.assertNotEqual(out.returncode,0); self.assertIn("finite",out.stderr)

  def test_duplicate_target_id_and_identity_mismatch_rejected(self):
    with tempfile.TemporaryDirectory() as d:
      p=Path(d); _minimal(p); target=_target_rows(p); target[-1]=dict(target[0]); path=p/"targets.jsonl"; path.write_text("\n".join(json.dumps(x) for x in target)+"\n"); import subprocess
      out=subprocess.run(["python3",str(Path(__file__).parent/"analyze.py"),str(p),"--targets",str(path)],capture_output=True,text=True); self.assertNotEqual(out.returncode,0)
      target=_target_rows(p); target[0]["geometry_fingerprint"]="wrong"; path.write_text("\n".join(json.dumps(x) for x in target)+"\n"); out=subprocess.run(["python3",str(Path(__file__).parent/"analyze.py"),str(p),"--targets",str(path)],capture_output=True,text=True); self.assertNotEqual(out.returncode,0)

  def test_evaluator_refuses_altered_manifest_before_target(self):
    root=Path(__file__).parent/"artifacts"/"transfer-v1"; binary=Path(__file__).parents[4]/"target/debug/sys-datascience-alternative-source-transfer-evaluator"
    if not binary.exists(): self.skipTest("evaluator test binary not built")
    with tempfile.TemporaryDirectory() as d:
      p=Path(d)
      for name in ("source.jsonl","features.jsonl","selection.jsonl"): (p/name).symlink_to(root/name)
      m=json.loads((root/"manifest.json").read_text()); m["selection_sha256"]="altered"; (p/"manifest.json").write_text(json.dumps(m)); target=p/"target.jsonl"; import subprocess
      out=subprocess.run([str(binary),"evaluate",str(p),str(target)],capture_output=True,text=True); self.assertNotEqual(out.returncode,0); self.assertFalse(target.exists())

  def test_equal_bucket_estimand_and_gate_classification_fixtures(self):
    with tempfile.TemporaryDirectory() as d:
      p=Path(d); _minimal(p); target=_target_rows(p)
      for row in target:
        pick=next(x for x in (json.loads(y) for y in (p/"selection.jsonl").read_text().splitlines()) if x["candidate_id"]==row["candidate_id"])
        if "control" in pick["memberships"]: row["sys"]=0.0
        elif "rho" in pick["memberships"]: row["sys"]=1.0 if row["bucket"]=="4x6" else 0.0
        else: row["sys"]=0.05
      target_path=p/"targets.jsonl"; target_path.write_text("\n".join(json.dumps(x) for x in target)+"\n"); import subprocess
      out=subprocess.run(["python3",str(Path(__file__).parent/"analyze.py"),str(p),"--targets",str(target_path)],capture_output=True,text=True); self.assertEqual(out.returncode,0,out.stderr); result=json.loads(out.stdout); self.assertAlmostEqual(result["selectors"]["rho"]["equal_bucket_effect"],.5); self.assertIn(result["selectors"]["rho"]["classification"],{"ambiguous","strong_transfer"})

if __name__ == "__main__": unittest.main()
