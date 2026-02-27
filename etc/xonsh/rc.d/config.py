def setup():

    from xonsh.built_ins import XSH

    if XSH.env.get("XONSH_INTERACTIVE"):
        XSH.aliases["c"] = "cd"

    if "JEFF_LOGIN_DONE" in XSH.env:
        return

    from pathlib import Path
    import json
    import subprocess
    import sys

    XSH.env["JEFF_LOGIN_DONE"] = True
    XSH.env["TITLE"] = "{cwd}"

    try:
        XSH.env.update(json.loads(Path("~/conf/var/env.json").expanduser().read_text()))
    except Exception as e:
        print(f"error: env: {e}", file=sys.stderr)

    jj = Path("~/.cargo/bin/jj").expanduser()

    def curr_branch():
        command = [
            jj,
            "log",
            "-r",
            "heads(::@ & bookmarks())",
            "--no-graph",
            "-T",
            "bookmarks ++ ' '",
        ]
        try:
            output = subprocess.run(command, capture_output=True, text=True)
        except Exception as e:
            print(e)
            return None
        if output.returncode == 0:
            return output.stdout.rstrip()

    XSH.env["PROMPT_FIELDS"]["curr_branch"] = curr_branch


setup()
del setup
