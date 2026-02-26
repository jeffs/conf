# Environment file generator

I use multiple shells, and want them all to be able to load settings from a common source; so, I declare them in Rust, then serialize them to JSON (for Nushell) and POSIX (for Bash, Zsh, et al).

# Notes

`nu` is a symlink so I can switch between cargo and brew installed versions.

Nushell loads its own config (including `login.nu`) from an OS-specific directory by default. On macOS, it's a mixed-case, space-laden path that's hard to remember. Setting `XDG_CONFIG_HOME` here is a must, as `~/conf/src/install-dotfiles.zsh` symlinks to `~/.config` regardless of platform.

`ENABLE_LSP_TOOL(S)` let Claude access language servers. There's some confusion online about whether the var name is singular or plural.

`GRIT` and `JUMP` are my own little home-grown tools. Setting `HELIX_RUNTIME` is necessary because my Helix is a fork.

# TODO

Correct groff config, rather than working around `^H` in `MANPAGER`.

- `col` is deprecated: weird tab behavior, poor support on musl.
- The relevant functionality _should_ be provided by `bat` once <https://github.com/sharkdp/bat/pull/3517> ships.
- See also:
  + <https://github.com/sharkdp/bat/issues/652#issuecomment-528998521>
  + <https://unix.stackexchange.com/questions/15855/how-to-dump-a-man-page>

Get imgcat working in Zellij. See also:

- <https://github.com/zellij-org/zellij/issues/2158>
- <https://github.com/saitoha/libsixel>

