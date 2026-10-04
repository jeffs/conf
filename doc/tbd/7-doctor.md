# Define a "doctor" command

I just noticed that Docker Desktop replaced my `~/.zprofile` symlink with a new file containing only an alteration of PATH, discarding everything else. (My shell was OK because `../../etc/zshrc` also sources `../../etc/zprofile`.)

The result of this ticket should be a script to check for problems like this, and potentially fix them.

See also `~/file/tbd/15-init-env.md`.
