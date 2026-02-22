# This file runs for login shells only. Weirdly, it runs *after* config.nu.
#
# # TODO
# 
# * Correct groff config, rather than workin around ^H in MANPAGER
#   - The `col` command is deprecated. (Weird tab behavior, spotty support on musl.)
#   - The relevant functionality _should_ be done automatically by bat once
#   <https://github.com/sharkdp/bat/pull/3517> ships.
#   - See also:
#    + <https://github.com/sharkdp/bat/issues/652#issuecomment-528998521>
#    + <https://unix.stackexchange.com/questions/15855/how-to-dump-a-man-page>

load-env {
  PROMPT_COMMAND: {||
    let t = date now
    $"(ansi green)($t | format date '%-I:%M %p')(ansi reset)"
  }
  PROMPT_COMMAND_RIGHT: {||
    $"(ansi green_dimmed)(date now | format date '%b %-d')(ansi reset)"
  }

  EDITOR: hx
  LESS: '-FRX -j5'
  MANPAGER: 'col -b | bat -pl man'

  PATH: ([
      ~/usr/bin
      ~/conf/bin
      ~/.local/bin
      ~/.cargo/bin
  ] | path expand)

  FZF_DEFAULT_OPTS_FILE: ('~/conf/etc/fzf' | path expand)
  RIPGREP_CONFIG_PATH: ('~/conf/etc/ripgreprc' | path expand)
  COPILOT_CUSTOM_INSTRUCTIONS_DIRS: ('~/conf/etc/copilot/instructions.md' | path expand)

  HOMEBREW_NO_ENV_HINTS: true
  RUSTC_WRAPPER: '/opt/homebrew/bin/sccache'

  # Let Claude access language servers. There's some confusion online about
  # whether the var name is singular or plural.
  ENABLE_LSP_TOOL: 1
  ENABLE_LSP_TOOLS: 1

  # My own little home-grown tools.
  GRIT_TRUNKS: 'dev,main,master'
  JUMP_PREFIXES: ('~/conf/etc/jump' | path expand)
  JUMP_HOME: ('~' | path expand)

  # Forks.
  HELIX_RUNTIME: ('~/usr/src/helix/runtime' | path expand)
}

# FNM is a version manager for Node.js.
/opt/homebrew/bin/fnm env --json | from json | load-env
$env.PATH ++= [($env.FNM_MULTISHELL_PATH | path join 'bin')]

$env.PATH ++= [
    ~/go/bin
    /usr/local/go/bin
    /opt/homebrew/bin
    /usr/local/bin
    /usr/bin
    /bin
    /usr/sbin
    /sbin
    /Library/Developer/CommandLineTools/usr/bin
] | path expand

if $nu.is-interactive {
  source 'ls-colors.nu'

  # TODO: Install iTerm2 and imgcat automatically; see:
  #  <https://iterm2.com/documentation-images.html>
  let image = "~/big/img/fun/dont-panic.jpg" | path expand
  if ($image | path exists) {
    match $env.TERM_PROGRAM {
      'WezTerm' => {^wezterm imgcat $image},
      'iTerm.app' if not (which 'imgcat' | is-empty) => {^imgcat $image},
      _ => {}
    }
  }
}
