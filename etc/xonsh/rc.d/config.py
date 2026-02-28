import json


def setup():
    from pathlib import Path
    from typing import cast
    import os
    import sys
    import tempfile

    from xonsh.built_ins import XSH

    aliases = cast(dict[str, object], XSH.aliases)
    env = cast(dict[str, object], XSH.env)

    if "JEFF_LOGIN_DONE" not in env:
        env["JEFF_LOGIN_DONE"] = True

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

    sys.path.insert(0, str(Path("~/conf/etc/xonsh/modules").expanduser()))

    import jeff

    if env.get("XONSH_INTERACTIVE"):
        import subprocess
        import webbrowser
        from pygments import highlight
        from pygments.lexers import PythonLexer
        from pygments.formatters import TerminalTrueColorFormatter

        # To list available values:
        #
        # ```py
        # import pygments.styles
        # sorted(pygments.styles.get_all_styles())
        # ```
        COLOR_STYLE = "one-dark"

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
            except Exception as e:
                return None, str(e), 1
            if output.returncode != 0:
                return output.stdout, output.stderr, output.returncode
            if output.stdout.startswith("http:") or output.stdout.startswith("https:"):
                return 0 if webbrowser.open(output.stdout) else 1
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
                return None, str(e), 1
            if output.returncode == 0:
                return output.stdout.rstrip()

        def alias_y():
            with tempfile.NamedTemporaryFile(prefix="yazi-cwd") as tmp:
                subprocess.run([path_yazi, "--cwd-file", tmp.name])
                s = Path(tmp.name).read_text()
            if s:
                os.chdir(s)

        def alias_ls(args):
            """TODO: Parse flags"""
            df = jeff.ls(*args)
            hl = highlight(
                repr(df),
                PythonLexer(),
                TerminalTrueColorFormatter(style=COLOR_STYLE),
            )
            return hl

        # Fun fact: `...` doesn't work as an alias, because it's valid Python.
        for i in range(1, 10):
            aliases["." * (i + 1)] = f"cd {'/'.join(['..'] * i)}"

        del aliases["ls"]

        aliases["c"] = "cd"
        aliases["f"] = alias_f
        aliases["mc"] = alias_mc
        aliases["y"] = alias_y

        prompt_fields = cast(dict, env["PROMPT_FIELDS"])
        prompt_fields["curr_branch"] = curr_branch
        env_name = prompt_fields["env_name"]
        prompt_fields["env_name"] = lambda: s if (s := env_name()) else None

        env["SHELL_TYPE"] = "prompt_toolkit"
        env["TITLE"] = "{cwd}"
        env["XONSH_SHOW_TRACEBACK"] = False
        env["PROMPT"] = (
            "{YELLOW}{env_name:{} }"
            "{BOLD_BLUE}{cwd} "
            "{branch_color}{curr_branch:{} }"
            "{RED}{last_return_code_if_nonzero:[{BOLD_INTENSE_RED}{}{RED}] }"
            "{BOLD_BLUE}{prompt_end} "
            "{RESET}"
        )

        # There's an actual kill ring. After ctrl+y, use alt+y to rotate.
        env["XONSH_COPY_ON_DELETE"] = True  # ctrl+k: yes, keep it on the kill ring
        env["XONSH_USE_SYSTEM_CLIPBOARD"] = False  # ctrk+y: no, don't hide the ring

        env["XONSH_COLOR_STYLE"] = COLOR_STYLE
        style_overrides = cast(dict[str, str], env["XONSH_STYLE_OVERRIDES"])
        style_overrides["completion-menu"] = "bg:ansiblack ansiwhite"
        style_overrides["completion-menu.completion"] = "bg:ansiblack ansiwhite"
        style_overrides["completion-menu.completion.current"] = (
            "bg:ansibrightblack ansiwhite"
        )


setup()
del setup
