# Bail early on recursive login shells. `config-nu` also sources this file,
# which Nushell then sources again *after* `config.nu`, so this check saves us
# from redefining the environment in interactive login shells.
if $env.JEFF_LOGIN_DONE? == null {
  $env.JEFF_LOGIN_DONE = 1

  if ('~/conf/var/env.json' | path exists) {
    open ~/conf/var/env.json | load-env
  }

  # FNM is a version manager for Node.js.
  /opt/homebrew/bin/fnm env --json | from json | load-env
  $env.PATH = $env.PATH | prepend $env.FNM_MULTISHELL_PATH
}
