#!/usr/bin/env -S zsh -euo pipefail
#
# https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html

readonly target_dir=~/pkg/awscliv2
mkdir -p $target_dir

cd $(mktemp -d)
readonly tmp=$(pwd)

cat >choices.xml <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <array>
    <dict>
      <key>choiceAttribute</key>
      <string>customLocation</string>
      <key>attributeSetting</key>
      <string>$target_dir</string>
      <key>choiceIdentifier</key>
      <string>default</string>
    </dict>
  </array>
</plist>
EOF

curl -O https://awscli.amazonaws.com/AWSCLIV2.pkg
installer -pkg AWSCLIV2.pkg \
    -target CurrentUserHomeDirectory \
    -applyChoiceChangesXML choices.xml

mkdir -p ~/usr/bin
ln -s $target_dir/aws-cli/aws ~/usr/bin
ln -s $target_dir/aws-cli/aws_completer ~/usr/bin

echo See /var/log/install.log for debug logs.

which aws
aws --version

cd -
rm -rf $tmp
