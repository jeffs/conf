def setup():
    from pathlib import Path

    from xonsh.built_ins import XSH

    if "JEFF_LOGIN_DONE" not in XSH.env:
        XSH.env["JEFF_LOGIN_DONE"] = True

        import json
        import sys

        try:
            env_json = Path("~/conf/var/env.json").expanduser().read_text()
            XSH.env.update(json.loads(env_json))
        except Exception as e:
            print(f"error: env: {e}", file=sys.stderr)

    if XSH.env.get("XONSH_INTERACTIVE"):
        import os
        import subprocess

        path_jj = Path("~/.cargo/bin/jj").expanduser()
        path_jump = Path("~/conf/prj/target/release/jump").expanduser()

        def mc(arg):
            dir = Path(arg).expanduser()
            dir.mkdir(parents=True, exist_ok=True)
            os.chdir(dir)

        def alias_mc(args):
            try:
                [arg] = args
            except ValueError as e:
                return None, f"mc: {e}", 2
            return mc(arg)

        def alias_f(args):
            try:
                output = subprocess.run(
                    [path_jump, *args],
                    capture_output=True,
                    text=True,
                    env={k: v for k, v in XSH.env.items() if type(v) is str},
                )
                print(output)
            except Exception as e:
                return (None, str(e), 1)
            if output.returncode == 0:
                return mc(output.stdout)

        def curr_branch():
            """TODO: Move to Rust"""
            command = [
                path_jj,
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
                return (None, str(e), 1)
            if output.returncode == 0:
                return output.stdout.rstrip()

        # Fun fact: `...` doesn't work as an alias, because it's valid Python.
        for i in range(1, 10):
            XSH.aliases["." * i] = f"cd {'/'.join(['..'] * i)}"

        XSH.aliases["c"] = "cd"
        XSH.aliases["f"] = alias_f
        XSH.aliases["mc"] = alias_mc

        XSH.env["TITLE"] = "{cwd}"
        XSH.env["PROMPT_FIELDS"]["curr_branch"] = curr_branch
        XSH.env["XONSH_SHOW_TRACEBACK"] = False
        XSH.env["PROMPT"] = (
            "{YELLOW}{env_name}"
            "{RESET}{BOLD_BLUE} {cwd}{branch_color}{curr_branch: {}}"
            "{RESET} {RED}{last_return_code_if_nonzero:[{BOLD_INTENSE_RED}{}{RED}] }"
            "{RESET}{BOLD_BLUE}{prompt_end}"
            "{RESET} "
        )


setup()
del setup
