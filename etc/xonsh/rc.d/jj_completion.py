"""Dynamic jj completion.

jj has no native xonsh completion, but any clap-based CLI built with
`CompleteEnv` emits its own candidates when invoked as::

    COMPLETE=<shell> _CLAP_COMPLETE_INDEX=<word-index> jj -- <words so far>

The protocol is shell-agnostic; asking for the `zsh` flavor gets
`value:description` lines, which feed RichCompletion descriptions. This covers
subcommands, flags, and bookmarks/revisions after `-r` — not the interior of
revset expressions.
"""


def setup():
    import os
    import subprocess

    from xonsh.completers.completer import add_one_completer
    from xonsh.completers.tools import RichCompletion, contextual_command_completer
    from xonsh.parsers.completion_context import CommandContext

    @contextual_command_completer
    def complete_jj(ctx: CommandContext):
        if not ctx.args or ctx.args[0].value != "jj":
            return None

        # Reconstruct the word list with the in-progress word (sans quotes)
        # spliced back into its position.
        words = [arg.value for arg in ctx.args]
        words.insert(ctx.arg_index, ctx.prefix)

        env = dict(os.environ)
        env.update(
            COMPLETE="zsh",
            _CLAP_COMPLETE_INDEX=str(ctx.arg_index),
            _CLAP_IFS="\n",
        )
        try:
            out = subprocess.run(
                ["jj", "--", *words],
                env=env,
                capture_output=True,
                text=True,
                timeout=5,
            ).stdout
        except (OSError, subprocess.TimeoutExpired):
            return None

        completions = set()
        for line in out.splitlines():
            value, _, description = line.partition(":")
            if value:
                completions.add(
                    RichCompletion(
                        value,
                        description=description,
                        append_space=not value.endswith(("/", "=")),
                    )
                )
        # Empty result falls through to xonsh's other completers (e.g. paths).
        return completions or None

    add_one_completer("jj", complete_jj, "start")


setup()
del setup
