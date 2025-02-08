#!/bin/bash
set -euo pipefail

# Source the variable declarations from variables.sh
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/variables.sh"

echo "POLKADOT_BIN: $POLKADOT_BIN"

## Bootnode:
## rococo-local.json:   "bootNodes": [ "/ip4/127.0.0.1/tcp/62379/ws/p2p/12D3KooWQCkBm1BYtkHpocxCwMgR8yjitEeHGx8spzcDLGt2gkBm"],
## Alice node listen port: 62379;  --listen-addr /ip4/0.0.0.0/tcp/62379/ws
## Bob --bootnode: --bootnodes /ip4/127.0.0.1/tcp/62379/ws/p2p/12D3KooWQCkBm1BYtkHpocxCwMgR8yjitEeHGx8spzcDLGt2gkBm

$POLKADOT_BIN --chain $CIPHERA_DEMO_ROOTDIR/bob/cfg/rococo-local.json --name bob --rpc-cors all --unsafe-rpc-external --rpc-methods unsafe --no-mdns --node-key 81b637d8fcd2c6da6359e6963113a1170de795e4b725b84d1e0b4cfd9ec58ce9 --no-telemetry --prometheus-external --validator --insecure-validator-i-know-what-i-do --bootnodes /ip4/127.0.0.1/tcp/62379/ws/p2p/12D3KooWQCkBm1BYtkHpocxCwMgR8yjitEeHGx8spzcDLGt2gkBm --prometheus-port 58410 --rpc-port 51076 --listen-addr /ip4/0.0.0.0/tcp/58408/ws --base-path $CIPHERA_DEMO_ROOTDIR/bob/data
