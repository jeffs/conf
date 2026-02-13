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
alias n = dirs next
alias p = dirs prev

alias d = describe
alias o = open
alias x = explore
alias xp = x --peek
alias jobs = job list
alias now = date now

alias glow = /opt/homebrew/bin/glow --pager --width=(tput cols)
alias rust = evcxr # -q

# Don't accidentally run `R` on case-insensitive filesystems like macOS.
alias r = error make { msg: "Did you mean R?" }

alias lc = loccount
alias fz = fzf --preview='bat -p --color=always {}'
alias ef = fzf --bind 'enter:become(hx {})'
alias mat = bat -pl man
alias w = wezterm
alias z = zellij

# My neuromuscular memory doesn't map cleanly to Jujutsu. I used Fig in anger,
# both metaphorically and literally, but it never really sank in for me. This is
# the first time in 25+ years (since RCS) I haven't had `ci` and `co` commands.
alias br = jj log -r '@ | bookmarks() | trunk()'
alias di = jj diff
alias st = jj status
alias gl = jj log

alias j = jj
alias jb = jj bookmark
alias jbc = jj bookmark create
alias jbd = jj bookmark delete
alias jbm = jj bookmark move
alias jbs = jj bookmark set
alias jbt = jj bookmark track
alias jd = jj describe
alias jdm = jj describe --message
alias je = jj edit
alias jg = jj git
alias jgf = jj git fetch # see also `alias up`
alias jgi = jj git init
alias jgp = jj git push
alias jl = jj log
alias jlr = jj log -r
alias jn = jj new
alias jnm = jj new --message

def --wrapped jbct [...rest, --revision (-r): string] {
  if ($revision == null) {
      jj bookmark create ...$rest
  } else {
      jj bookmark create ...$rest --revision $revision
  }
    jj bookmark track ...$rest
}

def --wrapped jbctp [...rest, --revision (-r): string] {
  jbct ...$rest --revision $revision
  jj git push --bookmark ...$rest
}

# This is the closest thing I could come up with to `git log --first-parent`.
def glog [spec: string = 'trunk()::@'] {
  jj log -r $"first_ancestors\(heads\((($spec)))) & ($spec)"
}

# TODO: Fix path expansion so `lg ~/Downloads` works.
#  Is this a known issue with wrapped commands? Seems like a Nushell bug.
# TODO: Patch Nushell, so we don't have to:
#  - special case empty $rest
#  - redeclare every flag we want to support
#  See also: <https://github.com/nushell/nushell/issues/12592>
def --wrapped lg [...rest, --all (-a)] {
  if ($rest | is-empty) {
    ls --all=$all
  } else {
    ls --all=$all ...$rest
  } | grid -cis '  '
}

def --env c [path: string = ~] { cd $path; l }
def --env cf [] { c (fzf --walker=dir,follow,hidden) }
def --env cg [] { c (grit root) }
def --env mc [path] { mkdir $path; c $path }

# Jump command wrapper, to (1) work around the shortcoming of jump not
# understanding relative dates, and (2) open a browser when the resolved target
# is a URL.
#
# Named `f` as in `follow`, instead of `j` for `jump`, because `j` is for `jj`.
def --env f [target] {
  let found = if ($target == 'cy' or $target == 'y') {
    # TODO: Move this to Rust, so it can (for example) be called from Helix.
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

# TODO: Default to pedantic, but allow override by local Cargo.toml file.
def clippy [] { cargo clippy --all-targets --workspace }

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
