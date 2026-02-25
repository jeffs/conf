# Login shell spawner

The point of doing this in Rust is that I use multiple shells, and want them all to inherit common settings. (It's currently hard-coded to launch Nushell, but I intend to use it for Bash and Zsh, and possibly more exotic shells like Xonsh.) There doesn't appear to be any portable format for environment variables, such that mutliple shells could load the same file. The closest is .env, but sourcing a `.env` file from a POSIX shell doesn't export the variables; so you end up needing a wrapper that uses something like `dotenvy`, at which point you might as well use the wrapper's own language to declare the variables.

I would welcome any well-informed guidance on this. I think most people don't bump their heads on this issue because they don't use multiple shells, or don't significantly customize their process environments, or both.

In the meantime, I gave up and started exec'ing my login shells from Rust. This works well, but requires the Rust build chain be installed before I can get my shell working right on a new machine. Other options don't totally avoid the problem, because not all systems (especially Windows) provide the same shells or interpreters by default. Moreover, scripting languages like Python are just slow enough on startup that I don't want to impose them on every login shell.

# Notes

`nu` is a symlink so I can switch between cargo and brew installed versions.

Nushell loads its own config (including `login.nu`) from an OS-specific directory by default. On macOS, it's a mixed-case, space-laden path that's hard to remember. Setting `XDG_CONFIG_HOME` here is a must, as `~/conf/src/install-dotfiles.zsh` symlinks to `~/.config` regardless of platform.

`ENABLE_LSP_TOOL(S)` let Claude access language servers. There's some confusion online about whether the var name is singular or plural.

`GRIT` and `JUMP` are my own little home-grown tools. Setting `HELIX_RUNTIME` is necessary because my Helix is a fork.

# TODO

Support shell path provided through pre-existing env var, so terminals can dictate what shell to start.

`exec` on Unix only; spawn on Windows.

Correct groff config, rather than working around `^H` in `MANPAGER`.

- `col` is deprecated: weird tab behavior, poor support on musl.
- The relevant functionality _should_ be provided by `bat` once <https://github.com/sharkdp/bat/pull/3517> ships.
- See also:
  + <https://github.com/sharkdp/bat/issues/652#issuecomment-528998521>
  + <https://unix.stackexchange.com/questions/15855/how-to-dump-a-man-page>

Get imgcat working in Zellij. See also:

- <https://github.com/zellij-org/zellij/issues/2158>
- <https://github.com/saitoha/libsixel>

