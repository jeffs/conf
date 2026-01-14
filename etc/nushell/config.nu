# config.nu
#
# # TODO
#
# - Add support for exit hooks
#   + See <https://www.nushell.sh/book/hooks.html#basic-hooks>
#   + Or, as a hack, launch a subshell, and print a message after it exits
# - Get aliases to autocomplete correctly
#   + This is a [known issue](https://www.nushell.sh/cookbook/external_completers.html#alias-completions)
#   + I don't understand the suggested workaround. There's no `$spans` variable
#     in `git-completions.nu`.
# - Auto-complete `grit since` with Git refs
# - TODO: Append to the history file on Enter, but read it only on startup

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

# Fun fact: There's now the exact opposite bug to this:
# <https://github.com/nushell/nushell/issues/7790>
#
# TODO: File a bug report.
# $env.config.history.sync_on_enter = false

alias e = edit # From rust-kart.
alias l = ls
# alias y = yazi # File manager; see <https://yazi-rs.github.io/features>.

# pushd = dirs add
alias g = dirs goto
alias n = dirs next
alias p = dirs prev

alias d = describe
alias o = open
alias x = explore
alias xp = x --peek
alias jobs = job list

alias glow = /opt/homebrew/bin/glow --pager --width=(tput cols)
alias rust = evcxr # -q

# Don't accidentally run `R` on case-insensitive filesystems like macOS.
alias r = error make { msg: "Did you mean R?" }

alias lc = loccount
alias f = fzf --preview='bat -p --color=always {}'
alias ef = f --bind 'enter:become(hx {})'
alias mat = bat -pl man
alias w = wezterm
alias z = zellij

alias br = git branch
alias ci = git commit
alias co = git checkout
alias di = git diff
alias st = git status
alias gl = git-branches # from rust-kart

alias pull = git pull
alias push = git push

alias si = grit si
alias sj = grit -v si
alias up = grit up

def lg [...patterns] {
  # TODO: How do I type the patterns one_of<glob, string>? Right now, a pattern
  #  like `*` is treated as a string instead of a glob.
  if ($patterns | is-empty) {
    ls
  } else {
    ls ...$patterns
  } | grid -cis '  '
}

def --env c [path: string = ~] { cd $path; l }
def --env cf [] { c (fzf --walker=dir,follow,hidden) }
def --env cg [] { c (grit root) }
def --env ct [] { c (grit trunk) }
def --env mc [path] { mkdir $path; c $path }

# TODO: Move this to Rust, so it can (for example) be called from Helix.
def --env j [target] {
  let found = if ($target == 'cy' or $target == 'y') {
    (date now) - 1day | format date '~/file/log/%Y/%m/%d' | path expand
  } else {
    ^jump $target
  }
  if ($found =~ '^https?://') {
    start $found
  } else {
    mc $found
  }
}

alias cl = mc (jump l)
alias cy = j y

# TODO: Default to pedantic, but allow override by local Cargo.toml file.
def clippy [] { cargo clippy --all-targets --tests --workspace }

def yolo [] { git commit -a --amend --no-edit --no-verify; git push --force-with-lease }

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

def imgcat [...args: string] {
  let args = $args | each {path expand}
  match $env.TERM_PROGRAM {
    'WezTerm' => {^wezterm imgcat ...$args},
    _ => {^imgcat ...$args},
  }
}

# Recognize Obsidian (data)base files.
def "from base" [] { from yaml }

# Extract YAML front matter from Obsidian notes.
#
# NOTE: Claude also uses YAML front matter; for example, in a user-level skill
#  (`~/.claude/skills/*/SKILL.md`):
#
#  ```yaml
#  name: refresh-rust-expert
#  description: Update the rust-expert agent with the latest Rust idioms, features, and best practices from official sources. Run this periodically to keep Rust advice current.
#  allowed-tools: WebSearch, WebFetch, Read, Write, Glob
#  user-invocable: true
#  ```
#
# def "from md" [] {
#    md-front | from yaml
# }

def "from brash" [] {
  gzip -d | from json
}

# Example
#
#   git diff --numstat dev... | numstat
def "from numstat" [] {
  detect columns -n | where column0 =~ '\d' | rename '+' '-' name |
  update '+' {into int} |
  update '-' {into int} |
  upsert delta {|r| ($r | get +) - ($r | get -)} | move delta --before name
}

# <https://yazi-rs.github.io/docs/quick-start>
def --env y [...args] {
	let tmp = (mktemp -t "yazi-cwd.XXXXXX")
	yazi ...$args --cwd-file $tmp
	let cwd = (open $tmp)
	if $cwd != "" and $cwd != $env.PWD {
		cd $cwd
	}
	rm -fp $tmp
}
