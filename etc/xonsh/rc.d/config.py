def setup():
    from pathlib import Path
    import json
    import subprocess
    import sys

    from xonsh.built_ins import XSH

    if XSH.env.get("XONSH_INTERACTIVE"):
        XSH.aliases["c"] = "cd"

        XSH.env["PROMPT"] = (
            "{YELLOW}{env_name}"
            "{RESET}{BOLD_BLUE} {cwd}{branch_color}{curr_branch: {}}"
            "{RESET} {RED}{last_return_code_if_nonzero:[{BOLD_INTENSE_RED}{}{RED}] }"
            "{RESET}{BOLD_BLUE}{prompt_end}"
            "{RESET} "
        )

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

        XSH.env["TITLE"] = "{cwd}"
        XSH.env["PROMPT_FIELDS"]["curr_branch"] = curr_branch

    if "JEFF_LOGIN_DONE" not in XSH.env:
        XSH.env["JEFF_LOGIN_DONE"] = True

        try:
            env_json = Path("~/conf/var/env.json").expanduser().read_text()
            XSH.env.update(json.loads(env_json))
        except Exception as e:
            print(f"error: env: {e}", file=sys.stderr)


setup()
del setup
