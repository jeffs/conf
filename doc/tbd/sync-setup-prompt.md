I just added a tool called `sync` to my `~/conf` repo. It's a Rust CLI that manages my source-installed packages. I need you to do the one-time setup on this machine to get it working.

## Steps

1. **Pull conf.** In `~/conf`, run `jj git fetch` and make sure you're on the latest main that includes `etc/src.toml` and `prj/sync/`.

2. **Build sync.** Run `cargo install --path ~/conf/prj/sync`.

3. **Run `sync status`** and show me the output. This is read-only and safe. It will tell us which repos exist, which are missing, and which need jj init.

4. **For each repo that exists but fails with "no jj repo"**, `cd` into that repo's directory and run `jj git init --colocate`. Then follow any hints jj prints (e.g. `jj bookmark track main --remote=origin`).

5. **For each fork repo that has jj but is missing an `upstream` remote** (check with `jj git remote list`), add it. The upstream URLs are in `~/conf/etc/src.toml` — read that file to find them.

6. **Check for path mismatches.** Most repos are expected at `~/usr/src/<name>/`. If a repo on this machine lives elsewhere, move it to `~/usr/src/<name>/` and leave a symlink at the old path. The only exception is `conf`, which stays at `~/conf`.

7. **Run `sync status` again** and show me the final output so I can see what's clean vs. still needs attention.

Don't run `sync update`, `sync clone`, or any builds — just the read-only setup. Use `jj` not `git` for all VCS operations.
