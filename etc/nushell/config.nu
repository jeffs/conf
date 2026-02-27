# config.nu
#
# # TODO
#
# Add support for exit hooks
# - See <https://www.nushell.sh/book/hooks.html#basic-hooks>
# - Or, as a hack, launch a subshell, and print a message after it exits
# 
# Get aliases to autocomplete correctly
# - This is a [known issue](https://www.nushell.sh/cookbook/external_completers.html#alias-completions)
# - I don't understand the suggested workaround. There's no `$spans` variable in `git-completions.nu`.
# 
# Auto-complete `grit since` with Git refs.
# 
# Append to the history file on Enter, but read it only on startup.
#
# Get imgcat working in Zellij. It should run `wezterm imgcat` in WezTerm
# wth without Zellij, `imgcat` in other terminals without Zellij, and
# [apparently](https://github.com/zellij-org/zellij/issues/2158) `sixel` in
# Zellij

# Source login script from interactive shells for two reason:
#
# 1. Zellij `default_shell` ignores arguments like `--login`. (We could work
#    around this using a wrapper script, but this is just as easy.)
# 2. Nushell sources `login.nu` *after* `config.nu`; so, by dfeault, anything
#    that needs a variable (such as an external command lookup via `PATH`) works
#    differently in the top level login shell than in subshells. Doing login
#    initialization in `config.nu` directly wouldn't help us for noninteractive
#    login shells, so we keep that logic in `login.nu`.
if $env.JEFF_LOGIN_DONE? == null {
  source ~/conf/etc/nushell/login.nu

  load-env {
    PROMPT_COMMAND: {||
      let t = date now
      $"(ansi green)($t | format date '%-I:%M %p')(ansi reset)"
    }
    PROMPT_COMMAND_RIGHT: {|| ''}
  }
}

$env.config.show_banner = false
$env.config.history.file_format = 'sqlite'

use std/dirs

# Commented out because I almost never use this. It lists fonts.
# source 'command/fc-list.nu'

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

# Nushell builtins. I usually prefer symlinks and small scripts to shell
# aliases, so that they'll work across shells; but it wouldn't do any good to
# call these in a subprocess.
alias l = ls
alias g = dirs goto
alias n = dirs next
alias p = dirs prev
alias d = describe
alias o = open
alias x = explore
alias xp = x --peek
alias jobs = job list
alias now = date now

# TODO: Plain `job unfreeze` sometimes mistakenly thinks there's no job running,
# so you have to give it the job ID explicitly.
alias fg = job unfreeze

# Don't accidentally run `R` on case-insensitive filesystems like macOS.
# Like shell builtins, I can't really work around this one with a symlink.
alias r = error make { msg: "Did you mean R?" }

def --env c [path: string = ~] { cd $path; l }
def --env cf [] { c (fzf --walker=dir,follow,hidden) }
def --env cg [] { c (grit root) }
def --env mc [path] { mkdir $path; c $path }

# `ls`, and put results in a grid.
#
# TODO
#
# Fix path expansion so `lg ~/Downloads` works. Is this a known issue with
# wrapped commands? Seems like a Nushell bug.
# 
# Patch Nushell, so you don't have to:
# - special case empty $rest
# - redeclare every flag you want to support
# See also: <https://github.com/nushell/nushell/issues/12592>
def --wrapped lg [...rest, --all (-a)] {
  if ($rest | is-empty) {
    ls --all=$all
  } else {
    ls --all=$all ...$rest
  } | grid -cis '  '
}

# Jump command wrapper, to:
#
# 1. Work around the shortcoming of jump not understanding relative dates
# 2. open a browser when the resolved target is a URL.
# 3. cd to the output directory
#
# (1) and (2) could be done in Rust, but (3) requires shell support.
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

# File manager; see:
# 
# - <https://yazi-rs.github.io/features>.
# - <https://yazi-rs.github.io/docs/quick-start>
def --env y [...args] {
	let tmp = (mktemp -t "yazi-cwd.XXXXXX")
	yazi ...$args --cwd-file $tmp
	let cwd = (open $tmp)
	if $cwd != "" and $cwd != $env.PWD {
		cd $cwd
	}
	rm -fp $tmp
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

# ------------------
# TODO: Move to Rust
# ------------------

def --wrapped jbct [...rest, --revision (-r): string] {
  if ($revision == null) {
      jj bookmark create ...$rest
  } else {
      jj bookmark create ...$rest --revision $revision
  }
  jj bookmark track ...$rest
  jj
}

def --wrapped jbctp [...rest, --revision (-r): string] {
  jbct ...$rest --revision $revision
  jj git push --bookmark ...$rest
}
