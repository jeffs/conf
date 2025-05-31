# This file runs for login shells only. Weirdly, it runs *after* config.nu.
#
# # TODO
# 
# * Correct groff config, rather than workin around ^H in MANPAGER; see:
#   <https://github.com/sharkdp/bat/issues/652#issuecomment-528998521>

load-env {
  EDITOR: hx
  LESS: '-FRX -j5'
  HOMEBREW_NO_ENV_HINTS: true
  JUMP_PREFIXES: ('~/conf/etc/jump' | path expand)
  MANPAGER: 'bat -pl man --color=always | sd "_\x08|\x08\w" "" | less'
  PATH: ([
      ~/usr/bin
      ~/conf/bin
      ~/.local/bin
      ~/.cargo/bin
      ~/.local/state/fnm_multishells/45162_1747348875574/bin
      ~/go/bin
      /opt/homebrew/bin
      /usr/local/bin
      /usr/bin
      /bin
      /usr/sbin
      /sbin
      /Library/Developer/CommandLineTools/usr/bin
  ] | path expand)
  RIPGREP_CONFIG_PATH: ('~/conf/etc/ripgreprc' | path expand)

  # FNM is a version manager for Node.js.
  FNM_MULTISHELL_PATH: ("~/.local/state/fnm_multishells/45162_1747348875574" | path expand)
  FNM_VERSION_FILE_STRATEGY: "local"
  FNM_DIR: ("~/.local/share/fnm" | path expand)
  FNM_LOGLEVEL: "info"
  FNM_NODE_DIST_MIRROR: "https://nodejs.org/dist"
  FNM_COREPACK_ENABLED: "false"
  FNM_RESOLVE_ENGINES: "true"
  FNM_ARCH: "arm64"
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
