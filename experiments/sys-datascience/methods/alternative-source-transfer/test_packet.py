from __future__ import annotations
import json, tempfile
from pathlib import Path
import unittest
from validate_packet import validate, check_no_target

def _minimal(tmp: Path):
    source=[]; feature=[]; selected=[]
    for bucket in ("4x6", "6x6"):
        for i in range(3200):
            cid=f"{bucket}-{i}"
            source.append({"candidate_id":cid,"logical_cell":cid,"geometry_fingerprint":cid,"bucket":bucket})
            feature.append({"candidate_id":cid})
        for arm, offset in (("rho", 0), ("ridge", 16), ("control", 32)):
            for i in range(16):
                cid=f"{bucket}-{offset+i}"
                selected.append({"candidate_id":cid,"bucket":bucket,"memberships":[arm],"geometry_fingerprint":cid})
    for name, data in (("source.jsonl",source),("features.jsonl",feature),("selection.jsonl",selected)):
        (tmp/name).write_text("\n".join(json.dumps(x) for x in data)+"\n")
    import hashlib
    h=lambda n: hashlib.sha256((tmp/n).read_bytes()).hexdigest()
    (tmp/"manifest.json").write_text(json.dumps({"schema":"alternative-source-transfer-manifest-v1","source_sha256":h("source.jsonl"),"feature_sha256":h("features.jsonl"),"selection_sha256":h("selection.jsonl"),"target_free":True}))

class PacketTests(unittest.TestCase):
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
      out=subprocess.run(["python3","analyze.py",str(tmp_path),"--targets",str(p)],capture_output=True,text=True)
      self.assertNotEqual(out.returncode, 0)

  def test_nonfinite_target_rejected(self):
    with tempfile.TemporaryDirectory() as d:
      tmp_path=Path(d); _minimal(tmp_path)
      targets=tmp_path/"targets.jsonl"; targets.write_text("\n".join(json.dumps({"candidate_id":r["candidate_id"],"sys":float("nan")}) for r in (json.loads(x) for x in (tmp_path/"selection.jsonl").read_text().splitlines()))+"\n")
      import subprocess
      out=subprocess.run(["python3",str(Path(__file__).parent/"analyze.py"),str(tmp_path),"--targets",str(targets)],capture_output=True,text=True)
      self.assertNotEqual(out.returncode,0); self.assertIn("finite",out.stderr)

if __name__ == "__main__": unittest.main()
