# Define a "doctor" command

Docker Desktop replaced my `~/.zprofile` symlink with a new file containing only an alteration of PATH, discarding everything else. My shell was OK because `../../etc/zshrc` also sources `../../etc/zprofile`, but this was confusing.

The result of this ticket should be a script to check for problems like this, and potentially fix them.

- Check symlinked dotfiles.
- Check Markdown for broken paths, in and out of hyperlinks.
- Check shell scripts for broken `source` paths, and Python for broken imports.
- Lint Python and Rust. Run tests.
- Switch most scripts to Rust.
- Check for modifications to dotfiles.
  + Docker just crapped in my zprofile.
  + It's the same thing they previous overwrote my zshrc with.

A lot of this isn't conf-specific. For instance, Markdown checks would be useful in `~/file/tbd`.

## See also

- [Define self-contained environments](~/file/tbd/15-init-env.md)
