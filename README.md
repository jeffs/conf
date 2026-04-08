# Environment configuration for macOS

This is my dotfiles repo for macOS. In theory, you should be able to reproduce my environment; but since I'm probably the only person who ever runs this from scratch, I would be shocked if everything went smoothly. If you try it and hit any issues (or, miraculously, do *not* hit any issues), do let me know.

First, clone this repo into your home directory:

```sh
git clone https://github.com/jeffs/conf ~/conf
```

Then, install dotfiles and basic dependencies (such as Homebrew):

```sh
~/conf/src/init.zsh
```

Next, run programs to generate shell config and install some things from source:

```sh
cd ~/conf/prj
cargo run -p jeff-login # Maps etc/platform/macos.toml to var/env.{json,sh}
cargo run -p sync       # Installs stuff from my source, per etc/sync.toml
````

Now, cross your fingers, and hold them that way for an amount of time proportionate to how much you care about this working on the first try. Once you uncross them, start a fresh shell in a new terminal window.

Finally, you may want to configure your terminal to start something other than its default shell. Startup files are included for Zsh, Nushell, and Xonsh. As of this writing, Xonsh is my daily driver.
