## TODO

* Automatically configure Notion not to steal [Cmd+Shift+K][1].
* Enable reversal of `conf/bin/init.zsh` and `conf/bin/install-*.zsh`; e.g., `uninstall`, `uninit`.
* Consolidate Linux, macOS, Docker, and Windows support.
  - Rewrite as much as possible in Rust.
  - { 2026-03-10: This is largely done now. See `prj/platform`, and `etc/platform/macos.toml`. }
* Automatically install font(s), and configure desktop apps (such as IDEs):
  - <https://www.jetbrains.com/lp/mono/>
* Complete automatic installation of `prj/*` in `src/init.zsh`.
  - Ditto updates when I run `prj/upgrade`.
* On install/upgrade, log versions before and after.  For example, Helix SHAs:
  ```sh
  $ hx --version
  helix 25.07.1 (001efa80)
  $ ~/conf/src/install-helix.nu
  $ hx --version
  helix 25.07.1 (d0218f7e)
  ```
* General purpose config specialization
  - [x] Conditional config. Something better than commenting out parts of config files.
    { 2026-03-10: Mostly done. See `prj/jeff-login`. }
  - Cascading config. Helix has [some support](https://docs.helix-editor.com/configuration.html) for this, but that's totally different from the Nushell support. Wezterm probably has some way to have different windows use different config, probably via Lua magic. Define the various forms of config specialization of different machines (e.g., laptops), profiles (home vs. work), and projects, with sane rules for how they should interact, and implement them (in whole or in part) for each application.
* Distinguish project and "home" directories, especially on machines where `~` is reserved for a particular project.
* Distinguish `pkg` (where packages are downloaded) from `opt` (where their runtimes are installed).
  - My forks and own code are mostly in `~/usr/src` now; see `prj/sync`. But this still applies to third-party packages.
* Link aliases (in the `bin` folder) to a `libexec` (or something akin to homebrew's `cellar`) instead of p`rj/target/release`. The Cargo target directory gets huge, mostly to help speed up development, with the expectation that you'll `cargo install` when you just want the binary; but installation would put the raw `jeff-alias` in my PATH, and introduces the possibility of conflicts with upstream crates


[1]: https://forum.figma.com/t/keyboard-shortcut-are-overriden-by-notion-running-on-background/59521/7
