# This file runs for login shells only. Weirdly, it runs *after* config.nu.

# Environment variables are of two ilks:
# 
# 1. Specific to Nushell, like PROMPT_COMMAND.
# 2. Not specific to Nushell, like MANPAGER.
#   + These may *also* be used by Nushell, like EDITOR.
#
# Only ilk 1 belongs in this file. The other ilk should be defined by
# the calling process; see `~/conf/prj/nu-login/src/main.rs`.
load-env {
  PROMPT_COMMAND: {||
    let t = date now
    $"(ansi green)($t | format date '%-I:%M %p')(ansi reset)"
  }
  PROMPT_COMMAND_RIGHT: {||
    $"(ansi green_dimmed)(date now | format date '%b %-d')(ansi reset)"
  }
}

# FNM is a version manager for Node.js.
#
# TODO: Move this to nu-login, which will need to parse the JSON.
/opt/homebrew/bin/fnm env --json | from json | load-env
$env.PATH ++= [($env.FNM_MULTISHELL_PATH | path join 'bin')]

if $nu.is-interactive {
  source 'ls-colors.nu'
}
