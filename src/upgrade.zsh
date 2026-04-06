#!/usr/bin/env -S zsh -euo pipefail
#
# TODO
#
# - [ ] Upgrade apps from installers: Docker, Firefox, Slack, Steam
# - [ ] Update Docker images
# - [ ] Build `on-file-click.app`
# - [ ] Consolidate with `cargo run -p sync`

cd ~/conf/prj
cargo run -p upgrade
