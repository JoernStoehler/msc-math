from __future__ import annotations
import json, math, shutil, tempfile
from pathlib import Path
import unittest
from validate_packet import EXPECTED, validate, check_no_target
from analyze import EVALUATOR_IDENTITY

def _minimal(tmp: Path):
    root = Path(__file__).parent / "artifacts" / "transfer-v1"
    for name in ("source.jsonl", "features.jsonl", "selection.jsonl", "manifest.json"):
        shutil.copyfile(root / name, tmp / name)

def _target_rows(tmp: Path, sys_value=0.1):
    source={r["candidate_id"]:r for r in (json.loads(x) for x in (tmp/"source.jsonl").read_text().splitlines())}
    manifest=json.loads((tmp/"manifest.json").read_text()); out=[]
    for line in (tmp/"selection.jsonl").read_text().splitlines():
        pick=json.loads(line); src=source[pick["candidate_id"]]
        capacity = math.sqrt(2.0 * src["volume"] * sys_value)
        out.append({"schema":"alternative-source-transfer-target-v1","candidate_id":pick["candidate_id"],"logical_cell":pick["logical_cell"],"bucket":pick["bucket"],"selection_memberships":pick["memberships"],"geometry_fingerprint":pick["geometry_fingerprint"],"source_sha256":manifest["source_sha256"],"feature_sha256":manifest["feature_sha256"],"selection_sha256":manifest["selection_sha256"],**EVALUATOR_IDENTITY,"volume":src["volume"],"capacity":capacity,"sys":sys_value})
    return out

class PacketTests(unittest.TestCase):
  def test_source_feature_fingerprint_mutation_rejected_after_hash_rewrite(self):
    with tempfile.TemporaryDirectory() as d:
      p=Path(d); _minimal(p); lines=(p/"features.jsonl").read_text().splitlines(); row=json.loads(lines[0]); row["source_geometry_fingerprint"]="wrong"; lines[0]=json.dumps(row); (p/"features.jsonl").write_text("\n".join(lines)+"\n"); m=json.loads((p/"manifest.json").read_text()); m["feature_sha256"]=__import__("hashlib").sha256((p/"features.jsonl").read_bytes()).hexdigest(); (p/"manifest.json").write_text(json.dumps(m))
      with self.assertRaisesRegex(ValueError,"feature hash"): validate(p)

  def test_selection_source_mutation_rejected_after_hash_rewrite(self):
    with tempfile.TemporaryDirectory() as d:
      p=Path(d); _minimal(p); lines=(p/"selection.jsonl").read_text().splitlines(); row=json.loads(lines[0]); row["geometry_fingerprint"]="wrong"; lines[0]=json.dumps(row); (p/"selection.jsonl").write_text("\n".join(lines)+"\n"); m=json.loads((p/"manifest.json").read_text()); m["selection_sha256"]=__import__("hashlib").sha256((p/"selection.jsonl").read_bytes()).hexdigest(); (p/"manifest.json").write_text(json.dumps(m))
      with self.assertRaisesRegex(ValueError,"selection hash"): validate(p)

  def test_immutable_artifact_hashes_reject_rewritten_manifest(self):
    for name, key in (("source.jsonl", "source_sha256"), ("features.jsonl", "feature_sha256"), ("selection.jsonl", "selection_sha256")):
      with self.subTest(name=name), tempfile.TemporaryDirectory() as d:
        p=Path(d); _minimal(p)
        lines=(p/name).read_text().splitlines(); row=json.loads(lines[0]); row["candidate_id"] += "-mutated"; lines[0]=json.dumps(row); (p/name).write_text("\n".join(lines)+"\n")
        m=json.loads((p/"manifest.json").read_text()); m[key]=__import__("hashlib").sha256((p/name).read_bytes()).hexdigest(); (p/"manifest.json").write_text(json.dumps(m))
        with self.assertRaisesRegex(ValueError, f"{key.split('_')[0]} hash"): validate(p)

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
      with self.assertRaisesRegex(ValueError,"selection hash"): validate(tmp_path)

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
      feature_lines=(tmp_path/"features.jsonl").read_text().splitlines(); first=json.loads(feature_lines[0]); feature_lines[0]=json.dumps({"candidate_id":first["candidate_id"],"sys":0.2}); (tmp_path/"features.jsonl").write_text("\n".join(feature_lines)+"\n")
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

  def test_boolean_target_numeric_rejected(self):
    with tempfile.TemporaryDirectory() as d:
      p=Path(d); _minimal(p); target=_target_rows(p); target[0]["sys"]=True
      path=p/"targets.jsonl"; path.write_text("\n".join(json.dumps(x) for x in target)+"\n"); import subprocess
      out=subprocess.run(["python3",str(Path(__file__).parent/"analyze.py"),str(p),"--targets",str(path)],capture_output=True,text=True)
      self.assertNotEqual(out.returncode,0); self.assertIn("malformed",out.stderr)

  def test_target_row_order_does_not_change_statistics(self):
    with tempfile.TemporaryDirectory() as d:
      p=Path(d); _minimal(p); target=_target_rows(p)
      original=p/"targets.jsonl"; reversed_path=p/"targets-reversed.jsonl"
      original.write_text("\n".join(json.dumps(x) for x in target)+"\n"); reversed_path.write_text("\n".join(json.dumps(x) for x in reversed(target))+"\n"); import subprocess
      a=subprocess.run(["python3",str(Path(__file__).parent/"analyze.py"),str(p),"--targets",str(original)],capture_output=True,text=True); b=subprocess.run(["python3",str(Path(__file__).parent/"analyze.py"),str(p),"--targets",str(reversed_path)],capture_output=True,text=True)
      self.assertEqual(a.returncode,0,a.stderr); self.assertEqual(b.returncode,0,b.stderr)
      self.assertEqual(json.loads(a.stdout)["selectors"],json.loads(b.stdout)["selectors"])

  def test_overlap_summary_uses_both_memberships(self):
    root=Path(__file__).parent/"artifacts"/"transfer-v1"
    counts={bucket:0 for bucket in ("4x6","6x6")}
    for line in (root/"selection.jsonl").read_text().splitlines():
      row=json.loads(line)
      if "rho" in row["memberships"] and "ridge" in row["memberships"]: counts[row["bucket"]]+=1
    self.assertEqual(counts,{"4x6":3,"6x6":2})

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
        if "control" in pick["memberships"]: row["sys"]=1e-12
        elif "rho" in pick["memberships"]: row["sys"]=1.0 if row["bucket"]=="4x6" else 1e-12
        else: row["sys"]=0.05
        row["capacity"] = math.sqrt(2.0 * row["volume"] * row["sys"])
      target_path=p/"targets.jsonl"; target_path.write_text("\n".join(json.dumps(x) for x in target)+"\n"); import subprocess
      out=subprocess.run(["python3",str(Path(__file__).parent/"analyze.py"),str(p),"--targets",str(target_path)],capture_output=True,text=True); self.assertEqual(out.returncode,0,out.stderr); result=json.loads(out.stdout); self.assertAlmostEqual(result["selectors"]["rho"]["equal_bucket_effect"],.5); self.assertIn(result["selectors"]["rho"]["classification"],{"ambiguous","strong_transfer"})

if __name__ == "__main__": unittest.main()
