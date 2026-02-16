#!/usr/bin/env python3
"""Generate a PDF-number → LaTeX-label mapping from the .aux file.

Usage:
    python label-map.py              # after latexmk has run
    python label-map.py --build      # run latexmk -f first, then generate

Reads build/main.aux for \newlabel entries, cross-references with .tex
sources to extract a short quote of each environment's opening text.

Output: build/label-map.txt (human-readable, tab-separated).
"""

import argparse
import re
import subprocess
import sys
from pathlib import Path

THESIS_DIR = Path(__file__).parent
AUX_FILE = THESIS_DIR / "build" / "main.aux"
OUTPUT_FILE = THESIS_DIR / "build" / "label-map.txt"

# Prefixes that indicate theorem-like environments we care about
LABEL_PREFIXES = ("def:", "rem:", "lem:", "thm:", "prop:", "claim:")

# Pattern: \newlabel{label}{{number}{page}{title}{anchor}{}}
NEWLABEL_RE = re.compile(
    r"\\newlabel\{([^}]+)\}\{\{([^}]*)\}\{([^}]*)\}\{([^}]*)\}\{[^}]*\}\{[^}]*\}\}"
    r"|"
    r"\\newlabel\{([^}]+)\}\{\{([^}]*)\}\{([^}]*)\}\{([^}]*)\}\{[^}]*\}\{\}\}"
)

# Simpler fallback: just grab the fields we need
NEWLABEL_SIMPLE_RE = re.compile(
    r"\\newlabel\{([^}]+)\}\{\{([^}]+)\}\{(\d+)\}\{([^}]*)\}"
)


def parse_aux(aux_path: Path) -> list[dict]:
    """Parse .aux file for theorem-like \newlabel entries."""
    entries = []
    for line in aux_path.read_text().splitlines():
        m = NEWLABEL_SIMPLE_RE.search(line)
        if not m:
            continue
        label, number, page, title = m.group(1), m.group(2), m.group(3), m.group(4)
        if not any(label.startswith(p) for p in LABEL_PREFIXES):
            continue
        # Clean up title: remove \cite{...}, extra braces
        title = re.sub(r"\\cite\s*\{[^}]*\}", "", title)
        title = title.replace("{", "").replace("}", "").strip()
        entries.append({
            "label": label,
            "number": number,
            "page": page,
            "title": title,
        })
    return entries


def find_quote(label: str, tex_files: list[Path]) -> str:
    """Find the first content line after \\label{<label>} in the .tex sources."""
    label_pat = re.compile(r"\\label\{" + re.escape(label) + r"\}")
    for tex_path in tex_files:
        lines = tex_path.read_text().splitlines()
        for i, line in enumerate(lines):
            if label_pat.search(line):
                # Scan forward for first non-comment, non-blank, non-marker content line
                for j in range(i + 1, min(i + 15, len(lines))):
                    candidate = lines[j].strip()
                    if not candidate:
                        continue
                    if candidate.startswith("%"):
                        continue
                    if candidate.startswith("\\begin{"):
                        continue
                    # Strip leading LaTeX commands that aren't content
                    if candidate.startswith("\\[") or candidate.startswith("\\]"):
                        continue
                    # Truncate to ~80 chars
                    if len(candidate) > 80:
                        candidate = candidate[:77] + "..."
                    return candidate
    return ""


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", action="store_true", help="Run latexmk -f first")
    args = parser.parse_args()

    if args.build:
        print("Running latexmk -f ...", file=sys.stderr)
        subprocess.run(["latexmk", "-f"], cwd=THESIS_DIR, capture_output=True)

    if not AUX_FILE.exists():
        print(f"Error: {AUX_FILE} not found. Run latexmk first.", file=sys.stderr)
        sys.exit(1)

    # Collect all .tex files
    tex_files = sorted(THESIS_DIR.glob("*.tex"))

    entries = parse_aux(AUX_FILE)

    # Build output
    lines = []
    lines.append(f"# Label map generated from {AUX_FILE.name}")
    lines.append(f"# {len(entries)} theorem-like environments")
    lines.append(f"#")
    lines.append(f"# NUM\tPG\tLABEL\tTITLE\tQUOTE")
    for e in entries:
        quote = find_quote(e["label"], tex_files)
        lines.append(f"{e['number']}\t{e['page']}\t{e['label']}\t{e['title']}\t{quote}")

    output = "\n".join(lines) + "\n"
    OUTPUT_FILE.write_text(output)
    # Also print to stdout
    print(output, end="")


if __name__ == "__main__":
    main()
