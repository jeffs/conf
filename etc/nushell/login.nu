# This file runs for login shells only. Weirdly, it runs *after* config.nu.

# Environment variables are of two ilks:
# 
# 1. Specific to Nushell, like PROMPT_COMMAND.
# 2. Not specific to Nushell, like EDITOR or MANPAGER.
#
# Only ilk 1 belongs in this file. The other ilk should be defined by
# the calling process. See also `~/conf/prj/jeff-login`.
load-env {
  PROMPT_COMMAND: {||
    let t = date now
    $"(ansi green)($t | format date '%-I:%M %p')(ansi reset)"
  }
  PROMPT_COMMAND_RIGHT: {|| ''}
}

# FNM is a version manager for Node.js.
#
# TODO: Move this to jeff-login, which will need to parse the JSON.
/opt/homebrew/bin/fnm env --json | from json | load-env
$env.PATH ++= [($env.FNM_MULTISHELL_PATH | path join 'bin')]
