#!/usr/bin/env -S zsh -euo pipefail
#
# Install the Session Manager plugin on macOS
# https://docs.aws.amazon.com/systems-manager/latest/userguide/install-plugin-macos-overview.html
#
# The Session Manager is a prerequisite for executing interactive shells in
# back-end containers via `aws ecs execute-command`.

readonly wd=~/var/install-session-manager
 
mkdir -p "$wd" ~/pkg ~/usr/bin
cd "$wd"

if [ "$(uname -m)" = arm64 ]; then
    readonly arch='_arm64'
else
    readonly arch=''
fi

curl -O "https://s3.amazonaws.com/session-manager-downloads/plugin/latest/mac$arch/sessionmanager-bundle.zip"
unzip sessionmanager-bundle.zip

./sessionmanager-bundle/install \
    -i ~/pkg/sessionmanagerplugin \
    -b ~/usr/bin/session-manager-plugin
