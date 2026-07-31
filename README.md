# Environment configuration for macOS

This is my dotfiles repo for macOS. Some useful commands to get started:

```sh
git clone https://github.com/jeffs/conf ~/conf
cd ~/conf/prj
cargo run -p mkenv  # Maps etc/platform/macos.toml to var/env.{json,sh}
cargo run -p rebase # Installs stuff from my source, per etc/rebase.toml
````

There's also an initialization script (`src/init.zsh`), but it's bound to hit errors when you run it, because it presumes that various tools from the other `src/install-*` scripts have already run. Eventually, I hope to replace these scripts with a single, deterministic installer that initializes a fresh its default settings.
