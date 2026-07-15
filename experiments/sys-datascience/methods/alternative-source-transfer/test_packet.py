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

if __name__ == "__main__": unittest.main()
