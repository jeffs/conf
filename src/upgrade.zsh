#!/usr/bin/env -S zsh -euo pipefail
#
# TODO
#
# - [ ] Upgrade apps from installers: Docker, Firefox, Slack, Steam, VPN
# - [ ] Update Docker images
# - [ ] Build `on-file-click.app`

# Build with cargo, but run the binaries directly; see rebase.zsh.
cd ~/conf/prj
cargo build --quiet --release --package upgrade --package rebase
target/release/upgrade "$@"
target/release/rebase "$@"
