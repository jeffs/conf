#!/usr/bin/env nu

# TODO: Replace `hx` with `$env.EDITOR`, if set.
def main [file: string] {
  source ~/.config/nushell/login.nu
  ^wezterm cli split-pane -- hx $file
}
 
