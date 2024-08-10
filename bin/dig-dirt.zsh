#!/usr/bin/env -S zsh -euo pipefail

fd -HIg .git |tr '\n' '\0' |xargs -0 ~/conf/libexec/dig-dirt.zsh
