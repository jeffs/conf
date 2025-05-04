# config.nu
#
# Installed by:
# version = "0.104.0"
#
# This file is used to override default Nushell settings, define
# (or import) custom commands, or run any other startup tasks.
# See https://www.nushell.sh/book/configuration.html
#
# This file is loaded after env.nu and before login.nu
#
# You can open this file in your default editor using:
# config nu
#
# See `help config nu` for more options
#
# You can remove these comments if you want or leave
# them for future reference.
#
# TODO
# * Fix MANPAGER properly; see:
#   <https://github.com/sharkdp/bat/issues/652#issuecomment-528998521>
# * Set environment variables only for login shells. (Does Nushell have that concept?)
# * Set up profiles; see $nu.user-autoload-dirs:
#   <https://www.nushell.sh/book/configuration.html#configuration-overview>

load-env {
  "EDITOR": "hx",
  "LESS": "-FRX -j5",
  "MANPAGER": "bat -pl man --pager='less -FrX -j5'",
  "PATH": [
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
  ],
  "RIPGREP_CONFIG_PATH": ('~/conf/etc/ripgreprc' | path expand)
}

# alias c = cd

alias e = hx
alias g = git
alias l = ls

# TODO: How do I forward the patterns with their original type, one_of<glob,
#  string>? Right now, `l *` tries to call `ls '*'`, with the asterisk being a
#  literal string rather than a glob.
def l [...patterns] {
  if ($patterns | is-empty) {
  ls              | grid -cis '  '
  } else {
  ls ...$patterns | grid -cis '  '
  }
}

def --env c [path: string = ~] {
  cd $path
  l
}

alias u = c ..

alias tree = eza -T --group-directories-first
alias t = tree --git-ignore

alias w = wezterm
alias z = zellij

alias br = git br
alias ci = git ci
alias co = git co
alias di = git di
alias st = git st

alias pull = git pull
alias push = git push
def yolo [] {
  git commit -a --amend --no-edit --no-verify
  git push -f
}

alias d = describe
alias x = explore

alias xp = x --peek

def --env mc [path] { mkdir $path; cd $path }

def --env j [target] {
  let path = match $target {
    y | cy => { (date now) - 1day | format date '~/log/%Y/%m/%d' | path expand },
    _ => { jump $target },
  }
  mc $path
}
