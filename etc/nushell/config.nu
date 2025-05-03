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

# TODO: Set environment variables only for login shells. (Does Nushell have that concept?)
load-env {
  "EDITOR": "hx",
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
  ]
}

alias c = cd
alias e = hx
alias l = ls
alias u = cd ..

alias ci = git commit
alias co = git checkout
alias di = git di
alias st = git st

alias pull = git pull
alias push = git push
