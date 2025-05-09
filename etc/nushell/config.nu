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

# TODO
#
# * Automatically load project-specific config, here and in `login.nu`; see:
#   - [$nu.user-autoload-dirs](https://www.nushell.sh/book/configuration.html#configuration-overview)
#   - [source](https://www.nushell.sh/commands/docs/source.html)
# * Add support for exit hooks
#   - See <https://www.nushell.sh/book/hooks.html#basic-hooks>
#   - Or, as a hack, launch a subshell, and print a message after it exits

$env.config.show_banner = false

source 'command/fc-list.nu'
source 'command/tree.nu'

alias e = hx
alias g = git
alias l = ls

# TODO: How do I forward the patterns with their original type, one_of<glob,
#  string>? Right now, `l *` tries to call `ls '*'`, with the asterisk being a
#  literal string rather than a glob.
def lg [...patterns] {
  if ($patterns | is-empty) {
    ls | grid -cis '  '
  } else {
    ls ...$patterns | grid -cis '  '
  }
}

def --env c [path: string = ~] {
  cd $path
  l
}

alias u = c ..
alias uu = c ...

alias w = wezterm
alias z = zellij

alias br = git br
alias ci = git ci
alias co = git co
alias di = git di
alias st = git st
alias si = since

alias mat = bat -pl man

alias glog = git glog
alias pull = git pull
alias push = git push
def yolo [] {
  git commit -a --amend --no-edit --no-verify
  git push -f
}

alias rust = evcxr -q
alias r = rust

alias d = describe
alias o = open

alias fg = job unfreeze
alias jobs = job list

alias x = explore
alias xp = x --peek

def --env mc [path] { mkdir $path; c $path }

def --env j [target] {
  let path = match $target {
    y | cy => { (date now) - 1day | format date '~/log/%Y/%m/%d' | path expand },
    _ => { jump $target },
  }
  mc $path
}

alias cl = c (jump cl)

def clippy [...args: string] {
  if ($args| is-empty) {
    cargo clippy --tests
    cargo clippy -- -W clippy::pedantic
  } else {
    cargo clippy --tests ...$args
    cargo clippy ...$args -- -W clippy::pedantic
  }
}

# TODO: Accept an optional list of languages, rather than a scalar.
def hx-health [lang?: string] {
  let table = (
    hx --health |
    lines |
    skip 8 |
    str join "\n" |
    str replace --regex 'Language serv\S*' 'Servers' |
    str replace 'Debug adapter' 'Adapter' |
    detect columns --guess
  )
  if $lang == null {
    $table
  } else {
    $table | where Language =~ $lang
  }
}

def imgcat [...args: string] {
  let args = $args | each {path expand}
  match $env.TERM_PROGRAM {
    'WezTerm' => {^wezterm imgcat ...$args},
    _ => {^imgcat ...$args},
  }
}

# # Example
#
#   git diff --numstat dev... | numstat
#
# # TODO
#
# * Make this a `from` subcommand.
# * How do I document custom commands in a way that integrates with F1?
def from-numstat [] {
  detect columns -n | where column0 =~ '\d' | rename '+' '-' name |
  update '+' {into int} |
  update '-' {into int} |
  upsert delta {|r| ($r | get +) - ($r | get -)} | move delta --before name
}
