from pathlib import Path
from xonsh.built_ins import XSH
import sys

sys.path.insert(0, str(Path("~/conf/etc/xonsh/modules").expanduser()))

from setup import setup

setup()

# Import a few items into every interactive shell for convenience.
if XSH.env and XSH.env.get("XONSH_INTERACTIVE"):
    from pathlib import Path
    import json
    import os
    import sys

    # Let the linter know these imports are not “unused.”
    _ = Path, json, os, sys
    del _

del XSH, setup
