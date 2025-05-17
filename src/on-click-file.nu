#!/usr/bin/env nu

# TODO: Replace `hx` with `$env.EDITOR`, if set.
def main [file: string] {
  source ~/.config/nushell/login.nu
  ^wezterm cli spawn --new-window --cwd (pwd) -- hx $file
}
 
