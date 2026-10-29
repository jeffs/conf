#!/usr/bin/env python3

from pathlib import Path
from subprocess import run
from sys import stderr

splash = Path("~/big/img/fun/dont-panic.jpg").expanduser()
try:
    run(["/opt/homebrew/bin/chafa", "-f", "kitty", splash])
except FileNotFoundError as e:
    print(f"warning: can't show image: {e}", file=stderr)
