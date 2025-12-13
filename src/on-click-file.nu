#!/usr/bin/env nu

def main [file: string] {
  source ~/.config/nushell/login.nu
  let editor = ($env.EDITOR? | default "hx")
  ^wezterm cli spawn --new-window --cwd (pwd) -- $editor $file
}
 
