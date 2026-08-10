"""Completion for `jump` and its `f` alias.

`jump --complete WORD...` prints one candidate per line for the last of the
words typed so far: a target name for the first word, and a path under that
target for the second. A candidate that names a directory ends in `/`.
"""


def setup():
    import subprocess
    from pathlib import Path

    from xonsh.completers.completer import add_one_completer
    from xonsh.completers.tools import RichCompletion, contextual_command_completer
    from xonsh.parsers.completion_context import CommandContext

    path_jump = Path("~/conf/prj/target/release/jump").expanduser()
    commands = {"f", "jump"}

    @contextual_command_completer
    def complete_jump(ctx: CommandContext):
        if ctx.arg_index == 0 or not ctx.args or ctx.args[0].value not in commands:
            return None

        # Reconstruct the argument list, dropping the command name and
        # splicing the in-progress word (sans quotes) into its position.
        words = [arg.value for arg in ctx.args[1:]]
        words.insert(ctx.arg_index - 1, ctx.prefix)

        try:
            out = subprocess.run(
                [path_jump, "--complete", *words],
                capture_output=True,
                text=True,
                timeout=5,
            ).stdout
        except (OSError, subprocess.TimeoutExpired):
            return None

        # Empty result falls through to xonsh's other completers (e.g. paths).
        return {
            RichCompletion(line, append_space=not line.endswith("/"))
            for line in out.splitlines()
        } or None

    add_one_completer("jump", complete_jump, "start")


setup()
del setup
