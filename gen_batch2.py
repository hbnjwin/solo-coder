#!/usr/bin/env python3
"""Generate 20 trae-solo batch2 projects"""
import os, subprocess, sys

BASE = r"d:\work\github\solo-coder\projects\trae-solo"
REMOTE = "https://github.com/hbnjwin/solo-coder.git"
GI = "/target\n/dist\n/node_modules\n"

def w(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

def run(cmd, cwd):
    return subprocess.run(cmd, cwd=cwd, shell=True, capture_output=True, text=True)

# Read project definitions from stdin
projects = eval(sys.stdin.read())
print(f"Generating {len(projects)} projects...")

for proj in projects:
    abbr = proj["abbr"]
    files = proj["files"]
    dir_path = f"{BASE}/{abbr}"
    
    for filepath, content in files.items():
        w(f"{dir_path}/{filepath}", content)
    
    print(f"  Written {abbr}: {len(files)} files")

print("Done writing files")
