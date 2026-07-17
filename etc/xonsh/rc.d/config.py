from xonsh.built_ins import XSH

if XSH.env and XSH.env.get("XONSH_INTERACTIVE"):
    # Import a few items into every interactive shell, for convenience. Guess
    # that makes them import-ant.
    from pathlib import Path
    from subprocess import call, run
    import json

    # AFAICS, Pyright hints cannot be suppressed per line. (`# pyright:
    # ignore[reportUnusedImport]` would suppress a warning or error, but not
    # hints.) Ruff is also displeased by unused imports, but can be silenced
    # using `noqa: F401` comments.
    #
    # Every time I write more than ~100 lines of Python, I'm reminded why I
    # prefer Rust.
    def ignore_unused(*_):
        pass

    ignore_unused(Path, call, json, run)
    del ignore_unused

del XSH


def setup():
    from pathlib import Path
    from typing import cast
    import datetime as dt
    import json
    import os
    import subprocess
    import sys

    from xonsh.built_ins import XSH

    aliases = cast(dict[str, object], XSH.aliases)
    env = cast(dict[str, object], XSH.env)

    if "JEFF_LOGIN_DONE" not in env:
        env["JEFF_LOGIN_DONE"] = True

        try:
            env_json = json.loads(Path("~/conf/var/env.json").expanduser().read_text())
            env.update(env_json)

            def _format(v):
                if isinstance(v, str):
                    return v
                if isinstance(v, list):
                    return os.pathsep.join(v)
                if isinstance(v, bool):
                    return "true" if v else "false"
                return str(v)

            os.environ.update({k: _format(v) for k, v in env_json.items()})
        except Exception as e:
            print(f"error: env: {e}", file=sys.stderr)

        # FNM is a version manager for Node.js.
        try:
            fnm = "/opt/homebrew/bin/fnm"
            fnm_env = json.loads(
                subprocess.run(
                    [fnm, "env", "--json"],
                    capture_output=True,
                    text=True,
                    check=True,
                ).stdout
            )
            env.update(fnm_env)
            os.environ.update(fnm_env)
            fnm_bin = f"{fnm_env['FNM_MULTISHELL_PATH']}/bin"
            cast(list, env["PATH"]).insert(0, fnm_bin)
            os.environ["PATH"] = os.pathsep.join([fnm_bin, os.environ["PATH"]])

            if "NVMRC" in os.environ:
                version = Path(os.environ["NVMRC"]).read_text()
                command = [fnm, "--log-level=quiet", "use", version]
                subprocess.run(command, check=True)

        except Exception as e:
            print(f"error: fnm: {e}", file=sys.stderr)

    # `history transfer` has a known bug.
    # env["XONSH_HISTORY_BACKEND"] = "sqlite"

    sys.path.insert(0, str(Path("~/conf/etc/xonsh/modules").expanduser()))

    if env.get("XONSH_INTERACTIVE"):
        import tempfile
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

        def alias_ec(args):
            """
            Spawn Claude in a new (empty) directory.

            Removes the directory if it is empty when Claude exits. Does NOT
            remove the conversation history or project memory.
            """
            claude = Path("~/.local/bin/claude").expanduser()
            parent = Path("~/var/tmp/claude").expanduser()
            parent.mkdir(parents=True, exist_ok=True)
            temp = tempfile.mkdtemp("", "", parent)
            returncode = subprocess.run([claude, *args], cwd=temp).returncode
            if next(Path(temp).iterdir(), None) is None:
                os.rmdir(temp)
            return returncode

        def alias_f(args):
            args = tuple(args)

            # Jump doesn't yet support date arithmetic, so do it here instead.
            if args == ("y",):
                yesterday = dt.datetime.now() - dt.timedelta(days=1)
                target = Path.home() / "file/log" / yesterday.strftime("%Y/%m/%d")
                return mc(target)

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

        # Fun fact: `...` doesn't work as an alias, because it's valid Python.
        for i in range(1, 10):
            command = f"cd {'/'.join(['..'] * i)}"
            aliases["." * (i + 1)] = command  # .., as in parent
            aliases["u" * i] = command  # u, as in up

        del aliases["ls"]

        aliases["-"] = ("cd", "-")
        aliases["c"] = "cd"
        aliases["cam"] = "cd ~/git/camelot"
        aliases["camb"] = "cd ~/git/camelot-b"
        aliases["cg"] = alias_cg
        aliases["ec"] = alias_ec
        aliases["f"] = alias_f
        aliases["mc"] = alias_mc

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

        env["FUZZY_PATH_COMPLETION"] = False
        env["SUBSEQUENCE_PATH_COMPLETION"] = False

        # There's an actual kill ring. After ctrl+y, use alt+y to rotate.
        env["XONSH_COPY_ON_DELETE"] = True  # ctrl+k: yes, keep it on the kill ring
        env["XONSH_USE_SYSTEM_CLIPBOARD"] = False  # ctrk+y: no, don't hide the ring

        env["XONSH_COLOR_STYLE"] = COLOR_STYLE

        # Don't set the background color when listing (block or character) device files.
        ls_colors = cast(dict[str, tuple[str]], env["LS_COLORS"])
        ls_colors["bd"] = ("INTENSE_YELLOW",)
        ls_colors["cd"] = ("INTENSE_YELLOW",)

        style_overrides = cast(dict[str, str], env["XONSH_STYLE_OVERRIDES"])
        style_overrides["completion-menu"] = "bg:ansiblack ansiwhite"
        style_overrides["completion-menu.completion"] = "bg:ansiblack ansiwhite"
        style_overrides["completion-menu.completion.current"] = (
            "bg:ansibrightblack ansiwhite"
        )


setup()
del setup
