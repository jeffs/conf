# config.nu
#
# TODO
#
# * Automatically load project-specific config, here and in `login.nu`; see:
#   - [`$nu.user-autoload-dirs`](https://www.nushell.sh/book/configuration.html#configuration-overview)
#   - [source](https://www.nushell.sh/commands/docs/source.html)
# * Add support for exit hooks
#   - See <https://www.nushell.sh/book/hooks.html#basic-hooks>
#   - Or, as a hack, launch a subshell, and print a message after it exits
# * Get aliases to autocomplete correctly
#   - This is a [known issue](https://www.nushell.sh/cookbook/external_completers.html#alias-completions)
#   - I don't understand the suggested workaround. There's no `$spans` variable
#     in `git-completions.nu`.
# * Auto-complete `since` with Git refs

$env.config.show_banner = false

use std/dirs

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

alias br = git branch
alias ci = git commit
alias co = git checkout
alias di = git diff
alias st = git status
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
