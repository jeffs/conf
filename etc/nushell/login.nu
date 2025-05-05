# This file runs for login shells only. Weirdly, it runs *after* config.nu.
#
# TODO
# 
# * Fix MANPAGER properly; see:
#   <https://github.com/sharkdp/bat/issues/652#issuecomment-528998521>

load-env {
  "EDITOR": "hx",
  "LESS": "-FRX -j5",
  "MANPAGER": "bat -pl man --pager='less -FrX -j5'",
  "PATH": ([
      ~/usr/bin
      ~/conf/bin
      ~/.local/bin
      ~/.cargo/bin
      ~/go/bin
      /usr/local/go/bin
      /opt/homebrew/opt/libpq/bin
      /opt/homebrew/opt/openjdk/bin
      /opt/homebrew/bin
      /usr/local/bin
      /usr/bin
      /bin
      /usr/sbin
      /sbin
      /Library/Developer/CommandLineTools/usr/bin
      "/Applications/Visual Studio Code.app/Contents/Resources/app/bin",
  ] | each {path expand}),
  "RIPGREP_CONFIG_PATH": ('~/conf/etc/ripgreprc' | path expand),
  "JUMP_PREFIXES": ('~/conf/etc/jump' | path expand),
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
