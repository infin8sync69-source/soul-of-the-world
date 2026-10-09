#!/usr/bin/env python3
"""Documentation checks for Soul of the World.

Checks (all must pass):
  1. relative links resolve, including #anchors into markdown files
  2. every document under docs/ is reachable from docs/README.md
  3. every document under docs/ has a Status line near the top
  4. referenced IDs (REQ, OQ, T, R, L, ADR, SPEC) are defined
  5. ADR files have the required fields and sections
  6. test vector files parse as JSON
  7. no obvious secret patterns or machine-specific paths

Run from anywhere:  python3 scripts/check_docs.py
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
errors = []


def err(msg):
    errors.append(msg)


def md_files():
    return sorted(p for p in ROOT.rglob("*.md") if ".git" not in p.parts)


def slug(heading):
    s = heading.strip().lower()
    s = re.sub(r"[`*_]", "", s)
    s = re.sub(r"[^\w\- ]", "", s, flags=re.UNICODE)
    return s.replace(" ", "-")


def anchors(path):
    out = set()
    in_code = False
    for line in path.read_text(encoding="utf-8").splitlines():
        if line.startswith("```"):
            in_code = not in_code
            continue
        if not in_code:
            m = re.match(r"^#{1,6}\s+(.*)", line)
            if m:
                out.add(slug(m.group(1)))
    return out


LINK = re.compile(r"\[[^\]]*\]\(([^)\s]+)\)")


def links(path):
    text = re.sub(r"```.*?```", "", path.read_text(encoding="utf-8"), flags=re.S)
    return LINK.findall(text)


# 1. links
graph = {}
for f in md_files():
    graph[f] = set()
    for target in links(f):
        if re.match(r"^(https?:|mailto:)", target):
            continue
        path_part, _, frag = target.partition("#")
        dest = f if path_part == "" else (f.parent / path_part).resolve()
        if not dest.exists():
            err(f"{f.relative_to(ROOT)}: broken link -> {target}")
            continue
        if dest.suffix == ".md":
            graph[f].add(dest)
            if frag and slug(frag) not in anchors(dest) and frag not in anchors(dest):
                err(f"{f.relative_to(ROOT)}: missing anchor -> {target}")

# 2. reachability from docs/README.md and 3. status lines
index = ROOT / "docs" / "README.md"
seen, stack = set(), [index]
while stack:
    cur = stack.pop()
    if cur in seen:
        continue
    seen.add(cur)
    stack.extend(graph.get(cur, ()))
for f in md_files():
    rel = f.relative_to(ROOT)
    if rel.parts[0] == "docs":
        if f not in seen:
            err(f"{rel}: not reachable from docs/README.md")
        head = "\n".join(f.read_text(encoding="utf-8").splitlines()[:14])
        if f != index and not re.search(r"(^|\n)\s*-?\s*Status:", head):
            err(f"{rel}: missing Status line")

# 4. ID definitions and references
def table_ids(relpath, pattern):
    p = ROOT / relpath
    return set(re.findall(pattern, p.read_text(encoding="utf-8"))) if p.exists() else set()

defs = {
    "REQ": table_ids("docs/requirements.md", r"\|\s*(REQ-[A-Z]{2}-\d\d)\s*\|"),
    "OQ": table_ids("docs/open-questions.md", r"\|\s*(OQ-\d\d)\s*\|"),
    "T": table_ids("docs/security/threat-model.md", r"\|\s*(T-\d\d)\s*\|"),
    "R": table_ids("docs/risks.md", r"\|\s*(R-\d\d)\s*\|"),
    "L": table_ids("docs/references/lessons-learned.md", r"\|\s*(L-\d\d)\s*\|"),
    "SPEC": table_ids("docs/specs/README.md", r"(SPEC-\d{3})"),
    "ADR": {"ADR-" + p.name[:4] for p in (ROOT / "docs" / "decisions").glob("[0-9][0-9][0-9][0-9]-*.md")},
}
ref_patterns = {
    "OQ": r"\bOQ-\d\d\b",
    "T": r"(?<![A-Za-z-])T-\d\d\b",
    "R": r"(?<![A-Za-z-])R-\d\d\b",
    "L": r"(?<![A-Za-z-])L-\d\d\b",
    "SPEC": r"\bSPEC-\d{3}\b",
    "ADR": r"\bADR-\d{4}\b",
}
for f in md_files():
    text = f.read_text(encoding="utf-8")
    rel = f.relative_to(ROOT)
    for m in re.finditer(r"\b(REQ-[A-Z]{2}-)(\d\d)(?:\.\.(\d\d))?", text):
        lo = int(m.group(2))
        hi = int(m.group(3)) if m.group(3) else lo
        for n in range(lo, hi + 1):
            rid = f"{m.group(1)}{n:02d}"
            if rid not in defs["REQ"]:
                err(f"{rel}: undefined {rid}")
    for kind, pat in ref_patterns.items():
        for rid in sorted(set(re.findall(pat, text))):
            if rid not in defs[kind]:
                err(f"{rel}: undefined {rid}")

# 5. ADR format
for p in sorted((ROOT / "docs" / "decisions").glob("[0-9][0-9][0-9][0-9]-*.md")):
    t = p.read_text(encoding="utf-8")
    for need in ("- Status:", "- Date:", "## Context", "## Decision", "## Consequences"):
        if need not in t:
            err(f"{p.relative_to(ROOT)}: ADR missing '{need}'")

# 6. vectors
for p in sorted((ROOT / "spec" / "test-vectors").glob("*.json")):
    try:
        json.loads(p.read_text(encoding="utf-8"))
    except Exception as e:  # noqa: BLE001
        err(f"{p.relative_to(ROOT)}: invalid JSON ({e})")

# 7. secrets and machine paths
SECRET = [
    r"postgres(?:ql)?://[A-Za-z0-9_]+:[^@\s<>]{6,}@",
    r"AIza[0-9A-Za-z_\-]{30,}",
    r"-----BEGIN [A-Z ]*PRIVATE KEY-----",
    r"\bnvapi-[A-Za-z0-9_\-]{20,}",
    r"\bsk-[A-Za-z0-9]{20,}",
    r"\beyJhbGciOi[A-Za-z0-9_\-\.]{20,}",
    r"CLUSTER_SECRET=[0-9a-fA-F]{20,}",
]
for f in sorted(p for p in ROOT.rglob("*") if p.is_file() and ".git" not in p.parts):
    if f.suffix in (".png", ".jpg", ".zip", ".gz"):
        continue
    try:
        text = f.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        continue
    rel = f.relative_to(ROOT)
    for pat in SECRET:
        if re.search(pat, text):
            err(f"{rel}: possible secret matching /{pat}/")
    if f.suffix == ".md" and re.search(r"(/Users/[A-Za-z0-9_.-]+/|C:\\Users\\)", text):
        err(f"{rel}: machine-specific path")

if errors:
    print(f"{len(errors)} problem(s):")
    for e in errors:
        print("  -", e)
    sys.exit(1)
print(f"ok: {len(md_files())} markdown files checked")
