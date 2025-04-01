# Environment configuration for macOS

Other stuff you should do after running the install scripts:

* Tell Notion not to steal Cmd+Shift+K [1][]

## TODO
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

[1]: https://forum.figma.com/t/keyboard-shortcut-are-overriden-by-notion-running-on-background/59521/7
