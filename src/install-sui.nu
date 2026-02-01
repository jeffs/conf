# #!/usr/bin/env nu
#
# Installs the Sui Move toolchain from scratch.
#
# NOTE: This script is meant to be used interactively.
 
# Install `suiup`. This downloads a single binary to `~/.local/bin/suiup`.
# <https://docs.sui.io/guides/developer/getting-started/sui-install>
http get `https://raw.githubusercontent.com/Mystenlabs/suiup/main/install.sh` | sh

# Install `sui`. This downloads `~/.local/bin/sui` and configures
# `~/.sui/sui_config/client.yaml`.
# 
# Alternatively, `brew install sui`; but Mysten recommends using `suiup`.
# <https://docs.sui.io/guides/developer/getting-started/sui-install>
suiup install sui@testnet

# Install the LSP server.
# <https://docs.sui.io/references/ide/move>
suiup install move-analyzer

# Install the formatter.
npm install -g @mysten/prettier-plugin-move

