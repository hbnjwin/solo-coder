#!/usr/bin/env python3
"""Git init, commit, branch and push for 20 batch2 projects"""
import os, subprocess

BASE = r"d:\work\github\solo-coder\projects\trae-solo"
REMOTE = "https://github.com/hbnjwin/solo-coder.git"

PROJECTS = [
    "workflow-state-machine", "approval-routing", "query-def-safety",
    "report-validator", "deal-content-parser", "approval-delegation",
    "audit-trail", "concurrent-approval", "approval-form-validation",
    "workflow-designer", "master-detail-sync", "permission-guard",
    "notification-badge", "offline-queue", "api-contract",
    "workflow-escalation", "export-retry", "approval-csv-export",
    "config-validator", "approval-metrics",
]

def run(cmd, cwd):
    r = subprocess.run(cmd, cwd=cwd, shell=True, capture_output=True, text=True)
    return r

for abbr in PROJECTS:
    d = f"{BASE}/{abbr}"
    if not os.path.isdir(d):
        print(f"  SKIP {abbr}: directory not found")
        continue

    # Check if already a git repo
    git_dir = os.path.join(d, ".git")
    if os.path.isdir(git_dir):
        print(f"  SKIP {abbr}: already a git repo")
        continue

    # git init
    r = run("git init", d)
    if r.returncode != 0:
        print(f"  FAIL git init {abbr}: {r.stderr.strip()}")
        continue

    # git add all (excluding target/node_modules via .gitignore)
    run("git add -A", d)

    # git commit
    r = run('git commit -m "initial: single-file starter with bugs/TODOs for trae-solo"', d)
    if r.returncode != 0:
        print(f"  FAIL commit {abbr}: {r.stderr.strip()[:100]}")
        continue

    # rename branch to match abbr
    r = run(f"git branch -M {abbr}", d)
    if r.returncode != 0:
        print(f"  FAIL rename branch {abbr}: {r.stderr.strip()}")
        continue

    # add remote
    run("git remote remove origin", d)  # ignore error if no remote
    r = run(f"git remote add origin {REMOTE}", d)
    if r.returncode != 0:
        print(f"  FAIL add remote {abbr}: {r.stderr.strip()}")
        continue

    # push
    r = run(f"git push -u origin {abbr}", d)
    if r.returncode != 0:
        print(f"  FAIL push {abbr}: {r.stderr.strip()[:200]}")
        continue

    print(f"  OK {abbr}")

print("\nDone!")
