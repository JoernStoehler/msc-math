#!/usr/bin/env python3
"""Build in a filesystem namespace without the original repository; compare PDF.

Requires Python 3, bubblewrap, the documented TeX toolchain, and Poppler.
Writes the verification report and ignored build evidence beside this script.
"""
import hashlib
import json
import shutil
import subprocess
import tempfile
from pathlib import Path

here = Path(__file__).resolve().parent
root = here.parent.parent
candidate = root / "thesis/candidate"
frozen = here / "evidence/frozen.pdf"
output = here / "verification-output"
output.mkdir(exist_ok=True)

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def run(command, **kwargs):
    return subprocess.run(command, check=True, **kwargs)

manifest = json.loads((here / "dependency-manifest.json").read_text())
assert sha(frozen) == manifest["frozen_pdf_sha256"]
for entry in manifest["files"]:
    assert sha(root / entry["new"]) == entry["new_sha256"], entry["new"]

with tempfile.TemporaryDirectory(prefix="msc-thesis-recovery-") as temporary:
    temp = Path(temporary)
    clean = temp / "candidate"
    shutil.copytree(candidate, clean, ignore=shutil.ignore_patterns("build"))
    sandbox = ["bwrap", "--unshare-net", "--die-with-parent", "--clearenv"]
    for directory in ("/usr", "/bin", "/lib", "/lib64", "/etc", "/var/lib/texmf"):
        sandbox += ["--ro-bind", directory, directory]
    sandbox += ["--proc", "/proc", "--dev", "/dev", "--tmpfs", "/tmp",
                "--bind", str(clean), "/work", "--chdir", "/work",
                "--setenv", "PATH", "/usr/bin:/bin",
                "--setenv", "LANG", "C.UTF-8",
                "/bin/sh", "./build.sh"]
    with (output / "clean-build.log").open("wb") as log:
        run(sandbox, stdout=log, stderr=subprocess.STDOUT)
    built = clean / "build/main.pdf"
    # The namespace has no /workspaces, original .git, or source checkout.
    project_inputs = set()
    for line in (clean / "build/main.fls").read_text().splitlines():
        if line.startswith("INPUT "):
            value = line.removeprefix("INPUT ")
            assert "/workspaces/" not in value and ".git/codex" not in value, value
            if value.startswith(("./", "/work/", "legacy/", "recovered/", "build/")):
                project_inputs.add(value)
    for name in ("main.pdf", "main.log", "main.fls", "main.fdb_latexmk", "main.blg"):
        shutil.copy2(clean / "build" / name, output / name)

    frozen_text = subprocess.check_output(["pdftotext", "-layout", str(frozen), "-"])
    rebuilt_text = subprocess.check_output(["pdftotext", "-layout", str(built), "-"])
    (output / "frozen.txt").write_bytes(frozen_text)
    (output / "rebuilt.txt").write_bytes(rebuilt_text)
    pages = []
    # Compare every rendered page in RGB at 96 dpi, without lossy encoding.
    for pdf, prefix in ((frozen, "frozen"), (built, "rebuilt")):
        run(["pdftoppm", "-r", "96", str(pdf), str(temp / prefix)],
            stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    for reference in sorted(temp.glob("frozen-*.ppm")):
        suffix = reference.name.removeprefix("frozen-")
        regenerated = temp / ("rebuilt-" + suffix)
        pages.append({"page": int(suffix.removesuffix(".ppm")),
                      "reference_sha256": sha(reference),
                      "rebuilt_sha256": sha(regenerated),
                      "identical": reference.read_bytes() == regenerated.read_bytes()})
    assert len(pages) == len(list(temp.glob("rebuilt-*.ppm")))
    log_text = (output / "main.log").read_text()
    notices = [line for line in log_text.splitlines()
               if "Warning" in line or "Overfull" in line or "Underfull" in line]
    report = {
        "reference_pdf_sha256": sha(frozen), "rebuilt_pdf_sha256": sha(built),
        "page_count": len(pages), "pdf_bytes_identical": sha(frozen) == sha(built),
        "layout_text_identical": frozen_text == rebuilt_text,
        "reference_layout_text_sha256": hashlib.sha256(frozen_text).hexdigest(),
        "rebuilt_layout_text_sha256": hashlib.sha256(rebuilt_text).hexdigest(),
        "render_comparison": "RGB PPM, 96 dpi, all pages, byte-for-byte pixel comparison",
        "all_rendered_pages_identical": all(page["identical"] for page in pages),
        "isolation": "bubblewrap: system TeX directories and a temporary candidate copy only; no original checkout mounted; network disabled",
        "recorded_project_inputs": sorted(project_inputs),
        "latex_notices": notices,
        "tool_versions": {
            program: subprocess.check_output(command, stderr=subprocess.STDOUT).decode().strip()
            for program, command in {
                "pdflatex": ["pdflatex", "--version"],
                "latexmk": ["latexmk", "--version"],
                "biber": ["biber", "--version"],
                "pdftoppm": ["pdftoppm", "-v"],
                "bubblewrap": ["bwrap", "--version"],
            }.items()
        },
        "pages": pages,
    }
    (here / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in (
        "page_count", "pdf_bytes_identical", "layout_text_identical",
        "all_rendered_pages_identical", "latex_notices")}))
    assert report["layout_text_identical"], "Text differs; inspect verification-output"
    assert report["all_rendered_pages_identical"], "Rendered pages differ"
