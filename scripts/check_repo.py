#!/usr/bin/env python3
"""Repository hygiene checks (docs/engineering/handbook.md section 6, lessons L-06, L-07).

Fails on: tracked files over the size limit, binary or archive file types, gitlinks
(submodule pointers), symlinks, and machine-specific home-directory paths in text files.
Run: python3 scripts/check_repo.py
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MAX_BYTES = 512 * 1024
ALLOW_LARGE = {"core/Cargo.lock"}
FORBIDDEN = {".zip", ".gz", ".tgz", ".tar", ".7z", ".rar", ".exe", ".dll", ".so", ".dylib",
             ".a", ".o", ".db", ".sqlite", ".sqlite3", ".apk", ".aab", ".ipa", ".dmg", ".pkg",
             ".msi", ".gguf", ".safetensors", ".onnx", ".bin", ".jar", ".class", ".pem", ".p12",
             ".key", ".keystore", ".jks"}
MACHINE = re.compile(r"(/Users/[A-Za-z0-9_.-]+/|/home/[a-z][a-z0-9_-]*/(?!user/)|C:\\Users\\)")
SELF = {"scripts/check_repo.py", "scripts/check_docs.py"}

out = subprocess.run(["git", "ls-files", "-s"], cwd=ROOT, capture_output=True, text=True, check=True).stdout
errors = []
for line in out.splitlines():
    meta, path = line.split("\t", 1)
    mode = meta.split()[0]
    if mode == "160000":
        errors.append(f"{path}: gitlink (submodule pointer) is not allowed")
        continue
    if mode == "120000":
        errors.append(f"{path}: symlink is not allowed")
        continue
    p = ROOT / path
    if not p.exists():
        continue
    if p.suffix.lower() in FORBIDDEN or path.endswith(".tar.gz"):
        errors.append(f"{path}: file type not allowed in git (use release storage)")
    size = p.stat().st_size
    if size > MAX_BYTES and path not in ALLOW_LARGE:
        errors.append(f"{path}: {size} bytes exceeds {MAX_BYTES}")
    if path in SELF:
        continue
    try:
        text = p.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        errors.append(f"{path}: binary content")
        continue
    if MACHINE.search(text):
        errors.append(f"{path}: machine-specific path")

if errors:
    print(f"{len(errors)} problem(s):")
    for e in errors:
        print("  -", e)
    sys.exit(1)
print(f"ok: {len(out.splitlines())} tracked files checked")
