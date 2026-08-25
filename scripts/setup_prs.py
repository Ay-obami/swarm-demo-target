#!/usr/bin/env python3
"""Seed every demo PR defined in prs.json.

For each entry: branch off main, apply the seeded bug (exact one-time string
replacement), commit, push the branch, and open a REAL GitHub pull request
(via gh) so Swarm CI can fetch refs/pull/N/head like any normal PR.

Also writes OPEN_PRS.md (copyable title/description blocks) and closes any
previously open PRs when the catalog is rebuilt.

Usage: scripts/setup_prs.py [--remote ORIGIN] [--repo OWNER/NAME]
                            [--no-push] [--no-open-prs]
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def sh(args: list[str], **kw) -> subprocess.CompletedProcess:
    return subprocess.run(args, cwd=ROOT, text=True,
                          capture_output=True, **kw)


def git(*args: str, check=True) -> str:
    r = sh(["git", *args])
    if check and r.returncode != 0:
        sys.exit(f"git {' '.join(args)} failed:\n{r.stderr}")
    return r.stdout.strip()


def gh(*args: str) -> str:
    r = sh(["gh", *args])
    if r.returncode != 0:
        raise RuntimeError(f"gh {' '.join(args)} failed:\n{r.stderr}")
    return r.stdout.strip()


def close_stale_prs(repo: str) -> None:
    try:
        out = gh("pr", "list", "--repo", repo, "--state", "open",
                 "--json", "number", "--limit", "100")
        nums = [p["number"] for p in json.loads(out or "[]")]
    except Exception as exc:                      # noqa: BLE001
        print(f"  ! could not list open PRs ({exc}); skipping cleanup")
        return
    for n in nums:
        try:
            gh("pr", "close", str(n), "--repo", repo,
               "--comment", "Catalog rebuilt — superseded by the reseeded set.")
            print(f"  closed stale PR #{n}")
            time.sleep(0.4)
        except Exception as exc:                  # noqa: BLE001
            print(f"  ! could not close #{n}: {exc}")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--remote", default="origin")
    ap.add_argument("--repo", default="Ay-obami/swarm-demo-target")
    ap.add_argument("--no-push", action="store_true")
    ap.add_argument("--no-open-prs", action="store_true")
    args = ap.parse_args()

    catalog = json.loads((ROOT / "prs.json").read_text())
    repo_git = catalog["repo_git"]
    entries = catalog["prs"]

    # Sanity: every find must occur exactly once in the pristine main file.
    git("checkout", "-q", "main")
    problems = []
    for e in entries:
        text = (ROOT / e["file"]).read_text()
        hits = text.count(e["find"])
        if hits != 1:
            problems.append(f"{e['label']}: find occurs {hits}× in {e['file']}")
    if problems:
        sys.exit("catalog sanity failed:\n  " + "\n  ".join(problems))

    if not args.no_push:
        git("push", "-q", args.remote, "main")
        close_stale_prs(args.repo)

    results: list[dict] = []
    for i, e in enumerate(entries):
        label, branch = e["label"], e["branch"]
        file_path = ROOT / e["file"]
        original = file_path.read_text()

        git("checkout", "-q", "-B", branch, "main")
        buggy = original.replace(e["find"], e["replace"], 1)
        assert buggy != original, f"{label}: replacement did not apply"
        file_path.write_text(buggy)
        git("add", e["file"])
        git("commit", "-q",
            "-m", f"PR-{label}: {e['title']}\n\nSeeded bug for the Phoenix CI demo.\n"
                  "Acceptance tests in tests/ pin the correct behavior and fail here.")
        print(f"[{i+1}/{len(entries)}] {branch} seeded")

        if not args.no_push:
            git("push", "-q", args.remote, branch)
            time.sleep(0.3)

        pr_url = ""
        if not args.no_open_prs and not args.no_push:
            try:
                pr_url = gh("pr", "create",
                            "--repo", args.repo,
                            "--base", "main",
                            "--head", branch,
                            "--title", f"PR-{label}: {e['title']}",
                            "--body", e["description"]).strip().splitlines()[-1]
                print(f"    opened {pr_url}")
                time.sleep(0.6)
            except Exception as exc:              # noqa: BLE001
                print(f"    ! gh pr create failed: {exc}")

        results.append({**{k: e[k] for k in ("label", "slug", "title", "description")},
                        "branch": branch, "pr_url": pr_url})
        file_path.write_text(original)            # restore working tree

    git("checkout", "-q", "main")

    lines = ["# Open demo PRs\n",
             "Copy the **Bug** block into the Phoenix CI dashboard together with",
             "the PR link (or just use the dashboard quick-pick).\n"]
    for r in results:
        link = r["pr_url"] or f"(branch `{r['branch']}`)"
        lines += [f"\n## PR-{r['label']} · {r['title']}\n",
                  f"- Link: {link}",
                  f"- Branch: `{r['branch']}`\n",
                  "**Bug description (copy me):**",
                  "```text",
                  r["description"],
                  "```\n"]
    (ROOT / "OPEN_PRS.md").write_text("\n".join(lines))
    print(f"\nwrote OPEN_PRS.md with {len(results)} PRs")


if __name__ == "__main__":
    main()
