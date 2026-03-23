from xonsh.built_ins import XSH

if XSH.env and XSH.env.get("XONSH_INTERACTIVE"):
    # Importe these into every interactive shell.
    from pathlib import Path
    from subprocess import call, run
    import json

    # This import also saves from accidentally running `/usr/bin/pl`.
    import polars as pl

    # AFAICS, Pyright hints cannot be suppressed per line. (`# pyright:
    # ignore[reportUnusedImport]` would suppress a warning or error, but not
    # hints.) Ruff is also displeased by unused imports, but can be silenced
    # using `# noqa: F401`.
    #
    # Every time I write more than ~100 lines of Python, I'm reminded why I
    # prefer Rust.
    def ignore_unused(*_):
        pass

    ignore_unused(Path, call, json, run)
    del ignore_unused

    # Polars is an avid truncater of strings, even when the terminal would handily
    # accommodate wider tables.
    pl.Config.set_fmt_str_lengths(100)

del XSH


def setup():
    from pathlib import Path
    from typing import cast
    import json
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
                    k: (v if isinstance(v, str) else os.pathsep.join(v))
                    for k, v in env_json.items()
                }
            )
        except Exception as e:
            print(f"error: env: {e}", file=sys.stderr)

    # `history transfer` has a known bug.
    # env["XONSH_HISTORY_BACKEND"] = "sqlite"

    sys.path.insert(0, str(Path("~/conf/etc/xonsh/modules").expanduser()))

    if env.get("XONSH_INTERACTIVE"):
        import subprocess
        import webbrowser

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

        def capture_text(command, args=()):
            try:
                output = subprocess.run(
                    [*command, *args],
                    capture_output=True,
                    text=True,
                )
            except Exception as e:
                return None, str(e), 1
            return output.stdout, output.stderr, output.returncode

        def mc(arg):
            dir = Path(arg).expanduser()
            dir.mkdir(parents=True, exist_ok=True)
            os.chdir(dir)

        def alias_cg():
            match capture_text(["git", "rev-parse", "--show-toplevel"]):
                case [stdout, _, 0]:
                    os.chdir(Path(stdout.rstrip()))
                case other:
                    return other

        def alias_mc(args):
            try:
                [arg] = args
            except ValueError as e:
                return None, f"mc: {e}", 2
            return mc(arg)

        def alias_f(args):
            match capture_text([path_jump], args):
                case [stdout, _, 0] if stdout.startswith("http:") or stdout.startswith(
                    "https:"
                ):
                    return 0 if webbrowser.open(stdout) else 1
                case [stdout, _, 0]:
                    mc(Path(stdout.rstrip()))
                case other:
                    return other

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
            match capture_text(command):
                case [stdout, _, 0]:
                    return stdout.rstrip() or None

        def alias_y():
            with tempfile.NamedTemporaryFile(prefix="yazi-cwd") as tmp:
                subprocess.run([path_yazi, "--cwd-file", tmp.name])
                s = Path(tmp.name).read_text()
            if s:
                os.chdir(s)

        # Fun fact: `...` doesn't work as an alias, because it's valid Python.
        for i in range(1, 10):
            command = f"cd {'/'.join(['..'] * i)}"
            aliases["." * (i + 1)] = command  # .., as in parent
            aliases["u" * i] = command  # u, as in up

        del aliases["ls"]

        aliases["c"] = "cd"
        aliases["cg"] = alias_cg
        aliases["f"] = alias_f
        aliases["mc"] = alias_mc
        aliases["y"] = alias_y

        prompt_fields = cast(dict, env["PROMPT_FIELDS"])
        prompt_fields["curr_branch"] = curr_branch
        env_name = prompt_fields["env_name"]
        prompt_fields["env_name"] = lambda: s if (s := env_name().rstrip()) else None

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
