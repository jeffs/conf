from typing import cast


def setup():
    from pathlib import Path
    import os
    import tempfile

    from xonsh.built_ins import XSH

    aliases = cast(dict[str, object], XSH.aliases)
    env = cast(dict[str, object], XSH.env)

    if "JEFF_LOGIN_DONE" not in env:
        env["JEFF_LOGIN_DONE"] = True

        import json
        import sys

        try:
            env_json = json.loads(Path("~/conf/var/env.json").expanduser().read_text())
            env.update(env_json)
            os.environ.update(
                {
                    k: (v if isinstance(v, str) else ":".join(v))
                    for k, v in env_json.items()
                }
            )
        except Exception as e:
            print(f"error: env: {e}", file=sys.stderr)

    if env.get("XONSH_INTERACTIVE"):
        import subprocess

        path_jj = Path("~/.cargo/bin/jj").expanduser()
        path_jump = Path("~/conf/prj/target/release/jump").expanduser()
        path_yazi = Path("/opt/homebrew/bin/yazi")

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
                    env={k: v for k, v in env.items() if type(v) is str},
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

        def alias_y(args):
            with tempfile.NamedTemporaryFile(prefix="yazi-cwd") as tmp:
                subprocess.run([path_yazi, "--cwd-file", tmp.name])
                s = Path(tmp.name).read_text()
            if s:
                os.chdir(s)

        # Fun fact: `...` doesn't work as an alias, because it's valid Python.
        for i in range(1, 10):
            aliases["." * (i + 1)] = f"cd {'/'.join(['..'] * i)}"

        del aliases["ls"]

        aliases["c"] = "cd"
        aliases["f"] = alias_f
        aliases["mc"] = alias_mc
        aliases["y"] = alias_y

        env["SHELL_TYPE"] = "prompt_toolkit"
        env["TITLE"] = "{cwd}"
        cast(dict, env["PROMPT_FIELDS"])["curr_branch"] = curr_branch
        env["XONSH_SHOW_TRACEBACK"] = False
        env["PROMPT"] = (
            "{YELLOW}{env_name}"
            "{RESET}{BOLD_BLUE} {cwd}{branch_color}{curr_branch: {}}"
            "{RESET} {RED}{last_return_code_if_nonzero:[{BOLD_INTENSE_RED}{}{RED}] }"
            "{RESET}{BOLD_BLUE}{prompt_end}"
            "{RESET} "
        )

        # Color styles are from pygments:
        #
        # ```py
        # import pygments.styles
        # sorted(pygments.styles.get_all_styles())
        # ```
        env["XONSH_COLOR_STYLE"] = "one-dark"
        style_overrides = env["XONSH_STYLE_OVERRIDES"]
        style_overrides["completion-menu"] = "bg:ansiblack ansiwhite"
        style_overrides["completion-menu.completion"] = "bg:ansiblack ansiwhite"
        style_overrides["completion-menu.completion.current"] = (
            "bg:ansibrightblack ansiwhite"
        )


setup()
del setup
