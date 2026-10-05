#!/usr/bin/env -S zsh -euo pipefail
#
# # TODO
#
# This script does not yet:
#
# - [ ] Upgrade apps from installers: Docker, Firefox, Slack, Steam, VPN, Zoom
#     + Zoom updates often hits an "Error" that has to be dismissed, but the
#       update seems to work anyway.
# - [ ] Update Docker images
# - [ ] Build `on-file-click.app`

# Build with cargo, but run the binaries directly; see rebase.zsh.
cd ~/conf/prj
cargo build --quiet --release --package upgrade --package rebase
target/release/upgrade "$@"
target/release/rebase "$@"
