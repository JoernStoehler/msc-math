#!/usr/bin/env python3
"""Stage all thirteen regenerated assets, build and compare the selected thesis.

Run from the repository root. Retained plot receipts own generation/input
checks; this script checks the publication-copy and source-to-PDF arrows.
It writes only a new temporary directory and its named report/PDF/log below.
"""
from pathlib import Path
import concurrent.futures
import datetime
import hashlib
import json
import re
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[3]
PACKET = Path(__file__).resolve().parent
FIGURES = PACKET / 'figure-reproduction'
MAPPING = {
    'association/association-generic.pdf': 'association-generic.pdf',
    'association/association-products.pdf': 'association-products.pdf',
    'conditional/conditional-ranks.pdf': 'conditional-ridge-correlation.pdf',
    'hko-calibration/recovery-by-source-distance.pdf': 'hko-recovery-by-source-distance.pdf',
    'derivative/derivative-and-kkt-scale.pdf': 'derivative-and-kkt-scale.pdf',
    'rotation/lagrangian_products_5x5.png': 'rotation-profile.png',
}
for name in ['foundations/characteristic-normalization.pdf', 'foundations/facet-polarity.pdf',
             'upper-bound-mechanisms.pdf', 'symmetry-extension.pdf',
             'flow-graph/flow-graph-f6-tube-sequence.pdf',
             'visualization/viz-hypercube-ridges.png', 'visualization/viz-hko-pentagon-min-orbit.png']:
    MAPPING['structural/outputs/' + name] = name

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def run(command, **kwargs):
    return subprocess.run(command, check=True, text=True, capture_output=True, **kwargs)

started = utc()
t0 = time.monotonic()
manifest = json.loads((PACKET / 'candidate-manifest.json').read_text())
source_hashes = manifest['source_files_sha256']
for name, digest in source_hashes.items():
    assert sha(ROOT / name) == digest, f'source changed: {name}'
reference = ROOT / manifest['pdf']['path']
assert sha(reference) == manifest['pdf']['sha256']
work = Path(tempfile.mkdtemp(prefix='msc-regenerated-figures-pdf-'))
for name in source_hashes:
    target = work / name
    target.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(ROOT / name, target)
assets = []
for fresh, selected in MAPPING.items():
    source = FIGURES / fresh
    target = work / 'thesis' / 'figures' / selected
    assets.append({'regenerated': str(source.relative_to(ROOT)),
                   'selected': 'thesis/figures/' + selected,
                   'regenerated_sha256': sha(source),
                   'selected_sha256': sha(target)})
    shutil.copy2(source, target)
build_start = time.monotonic()
result = run(['sh', 'thesis/build.sh'], cwd=work)
build_seconds = time.monotonic() - build_start
(PACKET / 'regenerated-assets-build-stdout.log').write_text(result.stdout + result.stderr)
log = (work / 'thesis/build/main.log').read_text(errors='replace')
assert not re.search(r'Overfull \\[hv]box|undefined references|undefined citations|Citation .* undefined|Reference .* undefined', log)
rebuilt = work / 'thesis/build/main.pdf'
shutil.copy2(rebuilt, PACKET / 'regenerated-assets.pdf')
shutil.copy2(work / 'thesis/build/main.log', PACKET / 'regenerated-assets-build.log')
texts = {}
def render(label, pdf):
    directory = work / label
    directory.mkdir()
    text = run(['pdftotext', '-layout', str(pdf), '-']).stdout
    run(['pdftoppm', '-r', '96', '-png', str(pdf), str(directory / 'page')])
    return text, {p.name: sha(p) for p in sorted(directory.glob('page-*.png'))}
with concurrent.futures.ThreadPoolExecutor(max_workers=2) as executor:
    jobs = {label: executor.submit(render, label, pdf) for label, pdf in [('candidate', reference), ('regenerated', rebuilt)]}
    rendered = {label: job.result() for label, job in jobs.items()}
old_text, old_pages = rendered['candidate']
new_text, new_pages = rendered['regenerated']
assert len(old_pages) == len(new_pages) == manifest['pdf']['pages']
different = [p for p in old_pages if old_pages[p] != new_pages[p]]
report = {
    'started_at': started, 'completed_at': utc(), 'elapsed_seconds': time.monotonic() - t0,
    'build_seconds': build_seconds, 'scratch_directory': str(work),
    'scope': 'Same installed TeX environment; fresh empty source copy with all 13 figure inputs replaced by freshly regenerated outputs from named retained inputs. No new environment setup, R2 download or raw capacity-table rerun.',
    'source_tree_sha256': manifest['source_tree_sha256'],
    'candidate_pdf_sha256': manifest['pdf']['sha256'], 'regenerated_pdf_sha256': sha(rebuilt),
    'candidate_pages': len(old_pages), 'regenerated_pages': len(new_pages),
    'all_13_selected_figures_replaced': len(assets) == 13, 'assets': assets,
    'build_exit_code': 0, 'overfull_boxes': 0, 'undefined_references_or_citations': 0,
    'layout_text_identical': old_text == new_text, 'raster_dpi': 96,
    'all_page_rasters_identical': not different, 'different_raster_pages': different,
    'candidate_page_png_sha256': old_pages, 'regenerated_page_png_sha256': new_pages,
    'tool_versions': {name: run(cmd).stdout + run(cmd).stderr for name, cmd in {
        'latexmk': ['latexmk', '-v'], 'pdftoppm': ['pdftoppm', '-v']}.items()},
}
(PACKET / 'regenerated-assets-rebuild.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps({k: report[k] for k in ['completed_at', 'elapsed_seconds', 'build_seconds', 'regenerated_pdf_sha256', 'candidate_pages', 'layout_text_identical', 'all_page_rasters_identical', 'different_raster_pages']}, indent=2))
