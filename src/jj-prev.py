#!/usr/bin/env python3
"""Print the change ID of the most recent working-copy change that differs from @.

Changes jj has since abandoned, such as an empty working copy left behind by
`jj edit`, are skipped.

Usage: jj edit "$(jj-prev.py)"
"""

import subprocess
import sys


def jj(*args: str) -> str:
    return subprocess.run(
        ["jj", *args], capture_output=True, text=True, check=True
    ).stdout


def change_id(revset: str) -> str | None:
    try:
        return jj("log", "-r", revset, "--no-graph", "-T", "change_id") or None
    except subprocess.CalledProcessError:
        return None


def previous_working_copy() -> str | None:
    current = change_id("@")
    for op in jj("op", "log", "--no-graph", "-T", 'id ++ "\\n"').split():
        cid = change_id(f"at_operation({op}, @)")
        if cid and cid != current and change_id(cid):
            return cid
    return None


if __name__ == "__main__":
    cid = previous_working_copy()
    if cid is None:
        sys.exit("no previous working copy found")
    print(cid)
