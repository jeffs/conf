# This file runs for login shells only. Weirdly, it runs *after* config.nu.
#
# # TODO
# 
# * Correct groff config, rather than workin around ^H in MANPAGER; see:
#   <https://github.com/sharkdp/bat/issues/652#issuecomment-528998521>
# * Update StackExchange to suggest -P 'col -b | bat -pl man':
#   <https://unix.stackexchange.com/questions/15855/how-to-dump-a-man-page>

load-env {
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

  HOMEBREW_NO_ENV_HINTS: true
  RUSTC_WRAPPER: '/opt/homebrew/bin/sccache'

  # My own little home-grown tools.
  GRIT_TRUNKS: 'dev,main,master'
  JUMP_PREFIXES: ('~/conf/etc/jump' | path expand)
  JUMP_HOME: ('~' | path expand)
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
