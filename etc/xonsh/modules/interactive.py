from pathlib import Path
from typing import cast
import datetime as dt
import os
import subprocess
import sys
import tempfile
import webbrowser

from xonsh.aliases import Aliases
from xonsh.built_ins import XSH
from xonsh.environ import Env, EnvPath, LsColors

aliases = cast(Aliases, XSH.aliases)
env = cast(Env, XSH.env)

home = Path.home()
path_jj = home / ".cargo/bin/jj"
path_jump = home / "conf/prj/target/release/jump"
path_ls = home / "conf/bin/ls"


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


def mcls(arg):
    mc(arg)
    subprocess.run(path_ls)


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
    """Wraps the `jump` command."""
    args = tuple(args)

    # Jump doesn't yet support date arithmetic, so do it here instead.
    if args == ("y",):
        yesterday = dt.datetime.now() - dt.timedelta(days=1)
        target = Path.home() / "file/log" / yesterday.strftime("%Y/%m/%d")
        return mcls(target)

    [out, err, status] = (got := capture_text([path_jump], args))
    out = out and out.rstrip()
    if status != 0 or not out:
        return got

    if err:
        print("warning:", err, file=sys.stderr)

    if out.startswith("http:") or out.startswith("https:"):
        return 0 if webbrowser.open(out) else 1

    if len(args) == 1:
        # The user specified only a target name, and `jump` respected it;
        # so, create the output if it doesn't exit. This is  useful for
        # autovivifying daily log directories.
        mcls(out)
    else:
        # The user specified both the target name and a relative path. It's
        # easy to fat-finger the path, so don't create it automatically.
        try:
            os.chdir(out)
        except FileNotFoundError as e:
            return [None, e, 1]

        subprocess.run(path_ls)


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


def configure():
    """Configure a shell for interactive use."""

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
    aliases["/exit"] = "exit"  # Nasty habit from using LLM TUIs.

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
    env["XONSH_COMPLETER_MODE"] = "prefix"
    cast(EnvPath, env["BASH_COMPLETIONS"]).append(
        "/opt/homebrew/etc/bash_completion.d/just"
    )

    # Disabled so I can complete ticket files by slug, rather than number.
    # env["SUBSEQUENCE_PATH_COMPLETION"] = False

    # There's an actual kill ring. After ctrl+y, use alt+y to rotate.
    env["XONSH_COPY_ON_DELETE"] = True  # ctrl+k: yes, keep it on the kill ring
    env["XONSH_USE_SYSTEM_CLIPBOARD"] = False  # ctrk+y: no, don't hide the ring

    # To list available values:
    #
    # ```py
    # import pygments.styles
    # sorted(pygments.styles.get_all_styles())
    # ```
    env["XONSH_COLOR_STYLE"] = "one-dark"

    # Don't set the background color when listing (block or character) device files.
    ls_colors = cast(LsColors, env["LS_COLORS"])
    ls_colors["bd"] = ("INTENSE_YELLOW",)
    ls_colors["cd"] = ("INTENSE_YELLOW",)

    # Xonsh detypes $LS_COLORS only for the processes it spawns itself.
    # Export it too, for the children of `subprocess.run`.
    os.environ["LS_COLORS"] = ls_colors.detype()

    style_overrides = cast(dict[str, str], env["XONSH_STYLE_OVERRIDES"])
    style_overrides["completion-menu"] = "bg:ansiblack ansiwhite"
    style_overrides["completion-menu.completion"] = "bg:ansiblack ansiwhite"
    style_overrides["completion-menu.completion.current"] = (
        "bg:ansibrightblack ansiwhite"
    )
