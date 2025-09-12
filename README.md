# Environment configuration for macOS

## To Do
* Automatically configure Notion not to steal [Cmd+Shift+K][1]
* Enable reversal of `conf/bin/init.zsh` and `conf/bin/install-*.zsh`.
  - Should I be using nix?
* Consolidate Linux, macOS, Docker, and Windows support
  - Rewrite as much as possible in Rust
  - Avoid Homebrew etc. where possible
* Automatically install font(s), and configure desktop apps (such as IDEs):
  - https://www.jetbrains.com/lp/mono/
* Move scripts from `bin` to `src`, symlink them from `bin`, and add `bin` to `PATH`, rather than creating `~/usr/bin`.
* Complete automatic installation of `prj/*` in `src/init.zsh`
  - Copy `cl` to `app/cl.app/Contents/MacOS/cl`
* On install/upgrade, log versions before and after.  For example, Helix SHAs:
  ```sh
  $ hx --version
  helix 25.07.1 (001efa80)
  $ ~/conf/src/install-helix.nu
  $ hx --version
  helix 25.07.1 (d0218f7e)
  ```
* General purpose config specialization
  - Conditional config. Something better than commenting out parts of config files.
  - Cascading config. Helix has [some support](https://docs.helix-editor.com/configuration.html) for this, but that's totally different from the Nushell support. Wezterm probably has some way to have different windows use different config, probably via Lua magic. Define the various forms of config specialization of different machines (e.g., laptops), profiles (home vs. work), and projects, with sane rules for how they should interact, and implement them (in whole or in part) for each application.

[1]: https://forum.figma.com/t/keyboard-shortcut-are-overriden-by-notion-running-on-background/59521/7
