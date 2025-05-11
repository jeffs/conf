# This file runs for login shells only. Weirdly, it runs *after* config.nu.
#
# TODO
# 
# * Work around ^H in bat/man output
#   see <https://github.com/sharkdp/bat/issues/652#issuecomment-528998521>

load-env {
  EDITOR: hx
  LESS: '-FRX -j5'
  HOMEBREW_NO_ENV_HINTS: true
  JUMP_PREFIXES: ('~/conf/etc/jump' | path expand)
  PATH: ([
      ~/usr/bin
      ~/conf/bin
      ~/.local/bin
      ~/.cargo/bin
      /opt/homebrew/bin
      /usr/local/bin
      /usr/bin
      /bin
      /usr/sbin
      /sbin
      /Library/Developer/CommandLineTools/usr/bin
  ] | path expand)
  RIPGREP_CONFIG_PATH: ('~/conf/etc/ripgreprc' | path expand)
}

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
