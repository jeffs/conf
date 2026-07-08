#!/usr/bin/env -S zsh -euo pipefail
#
# TODO
#
# - [ ] Upgrade apps from installers: Docker, Firefox, Slack, Steam, VPN
# - [ ] Update Docker images
# - [ ] Build `on-file-click.app`

cd ~/conf/prj
cargo run -p upgrade "$@"
cargo run -p rebase "$@"
