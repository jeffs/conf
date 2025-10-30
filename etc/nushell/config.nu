# config.nu
#
# # TODO
#
# * Add support for exit hooks
#   - See <https://www.nushell.sh/book/hooks.html#basic-hooks>
#   - Or, as a hack, launch a subshell, and print a message after it exits
# * Get aliases to autocomplete correctly
#   - This is a [known issue](https://www.nushell.sh/cookbook/external_completers.html#alias-completions)
#   - I don't understand the suggested workaround. There's no `$spans` variable
#     in `git-completions.nu`.
# * Auto-complete `grit since` with Git refs

$env.config.show_banner = false

use std/dirs

source 'command/fc-list.nu'
source 'command/tree.nu'

# For reasons beyond my ken, completions sourced from autoload scripts don't
# respect aliases. So, I source them here, in which case they _kinda_ respect
# aliases. This seems unrelated to the known issue mentioned in the TODO above.
#
# Nushell doesn't appear to have any way (other than autoload) to source files
# only if they exist. Maybe `$NU_LIB_DIRS` can be used to search "real" paths,
# falling back to empty dummy files committed to this repository.
use ~/pkg/nu_scripts/custom-completions/git/git-completions.nu *

# TODO: Append to the history file on Enter, but read it only on startup.
# $env.config.history.sync_on_enter = false

alias e = edit # From rust-kart.
alias l = ls

# pushd = dirs add
alias g = dirs goto
alias n = dirs next
alias p = dirs prev

def lg [...patterns] {
  # TODO: How do I type the patterns one_of<glob, string>? Right now, a pattern
  #  like `*` is treated as a string instead of a glob.
  if ($patterns | is-empty) {
    ls
  } else {
    ls ...$patterns
  } | grid -cis '  '
}

def --env c [path: string = ~] {
  cd $path
  l
}

alias w = wezterm
alias z = zellij

alias br = git branch
alias ci = git commit
alias co = git checkout
alias di = git diff
alias st = git status

alias lc = loccount
alias si = grit si
alias sj = grit -v si
alias up = grit up

alias mat = bat -pl man

alias f = fzf --walker-skip=.git,dist,node_modules,target,var --preview='bat -p --color=always {}'
alias fe = f --bind 'enter:become(hx {})'

alias glog = git glog
alias pull = git pull
alias push = git push

def glog [...args: string] {
  if ($args | is-empty) {
    git log --graph --oneline --branches $"(grit trunk).."
  } else {
    git glog ...$args
  }
}

def yolo [] {
  git commit -a --amend --no-edit --no-verify
  git push -f
}

alias rust = evcxr -q

alias d = describe
alias o = open

alias jobs = job list

# Don't accidentally run `R` case-insensitive filesystems like macOS.
alias r = error make { msg: "Did you mean R?" }

alias x = explore
alias xp = x --peek

def --env mc [path] { mkdir $path; c $path }

def --env j [target] {
  let path = match $target {
    y | cy => { (date now) - 1day | format date '~/file/log/%Y/%m/%d' | path expand },
    _ => { jump $target },
  }
  mc $path
}

alias cl = mc (jump cl)

def clippy [...args: string] {
    cargo clippy --all-targets --tests --workspace ...$args -- -W clippy::pedantic
}

# Unfreeze a frozen job.
def fg [id?: int] {
  # TODO: Report the following `job unfreeze` issue: Plain `job unfreeze`
  #  sometimes mistakenly thinks there's no job running, so you have to give it
  #  the job ID explicitly.
  if $id != null {
    return (job unfreeze $id)
  }
  let ids = job list | where type == 'frozen' | get 'id'
  if ($ids | is-empty) {
    # TODO: `error make` seems to return early. Is that its intende behavior?
    error make --unspanned { msg: 'No frozen jobs' }
  }
  job unfreeze ($ids | first)
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
