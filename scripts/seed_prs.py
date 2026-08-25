#!/usr/bin/env python3
"""Seed every demo PR defined in prs.json (branches + real GitHub PRs)."""
from __future__ import annotations
import argparse, json, subprocess, sys, time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def sh(args):
    return subprocess.run(args, cwd=ROOT, text=True, capture_output=True)


def git(*args):
    r = sh(["git", *args])
    if r.returncode != 0:
        raise RuntimeError(f"git {' '.join(args)} failed:\n{r.stderr}")
    return r.stdout.strip()


def gh(*args):
    r = sh(["gh", *args])
    if r.returncode != 0:
        raise RuntimeError(f"gh {' '.join(args)} failed:\n{r.stderr}")
    return r.stdout.strip()


def close_stale_prs(repo):
    try:
        out = gh("pr", "list", "--repo", repo, "--state", "open",
                 "--json", "number", "--limit", "100")
        nums = [p["number"] for p in json.loads(out or "[]")]
    except Exception as exc:
        print(f"  ! could not list open PRs ({exc})")
        return
    for n in nums:
        try:
            gh("pr", "close", str(n), "--repo", repo,
               "--comment", "Catalog rebuilt.")
            print(f"  closed stale PR #{n}")
            time.sleep(0.4)
        except Exception as exc:
            print(f"  ! could not close #{n}: {exc}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--remote", default="origin")
    ap.add_argument("--repo", default="Ay-obami/swarm-demo-target")
    ap.add_argument("--no-push", action="store_true")
    ap.add_argument("--no-open-prs", action="store_true")
    args = ap.parse_args()

    catalog = json.loads((ROOT / "prs.json").read_text())
    entries = catalog["prs"]

    git("checkout", "-qf", "main")
    git("reset", "-q", "--hard", args.remote + "/main")

    problems = []
    for e in entries:
        hits = (ROOT / e["file"]).read_text().count(e["find"])
        if hits != 1:
            problems.append(f"{e['label']}: find occurs {hits}x in {e['file']}")
    if problems:
        sys.exit("catalog sanity failed:\n  " + "\n  ".join(problems))

    if not args.no_push:
        git("push", "-qf", args.remote, "main")
        close_stale_prs(args.repo)

    results = []
    for i, e in enumerate(entries):
        label, branch = e["label"], e["branch"]
        file_path = ROOT / e["file"]

        git("checkout", "-qf", "-B", branch, "main")
        original = file_path.read_text()

        buggy = original.replace(e["find"], e["replace"], 1)
        assert buggy != original, f"{label}: replacement did not apply"
        file_path.write_text(buggy)
        sh(["git", "add", e["file"]])
        sh(["git", "commit", "-q", "-m",
            f"PR-{label}: {e['title']}"])
        print(f"[{i+1}/{len(entries)}] {branch} seeded")

        pr_url = ""
        if not args.no_push:
            push = sh(["git", "push", "-qf", args.remote, branch])
            if push.returncode != 0:
                time.sleep(1.5)
                push = sh(["git", "push", "-qf", args.remote, branch])
            if push.returncode != 0:
                print(f"    ! push failed for {branch}: {push.stderr.strip()[:160]}")
            if not args.no_open_prs:
                try:
                    out = gh("pr", "create", "--repo", args.repo,
                             "--base", "main", "--head", branch,
                             "--title", f"PR-{label}: {e['title']}",
                             "--body", e["description"])
                    pr_url = out.strip().splitlines()[-1]
                    print(f"    opened {pr_url}")
                    time.sleep(0.5)
                except Exception as exc:
                    print(f"    ! gh pr create failed: {exc}")

        results.append({**{k: e[k] for k in ("label", "slug", "title", "description")},
                        "branch": branch, "pr_url": pr_url})

        git("checkout", "-qf", "main")
        file_path.write_text(original)

    md = ["# Open demo PRs", "",
          "Copy each Bug block into the Phoenix CI dashboard quick-pick."]
    for r in results:
        link = r["pr_url"] or f"(branch `{r['branch']}`)"
        md += ["", f"## PR-{r['label']} - {r['title']}",
               f"- Link: {link}", f"- Branch: `{r['branch']}`", "", "```text",
               r["description"], "```"]
    (ROOT / "OPEN_PRS.md").write_text("\n".join(md))
    print(f"wrote OPEN_PRS.md ({len(results)} PRs)")


if __name__ == "__main__":
    main()
