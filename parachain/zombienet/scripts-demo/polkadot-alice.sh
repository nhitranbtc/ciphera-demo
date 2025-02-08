#!/bin/bash
set -euo pipefail

# Source the variable declarations from variables.sh
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/variables.sh"

echo "POLKADOT_BIN: $POLKADOT_BIN"

## Bootnode:
## rococo-local.json:   "bootNodes": [ "/ip4/127.0.0.1/tcp/62379/ws/p2p/12D3KooWQCkBm1BYtkHpocxCwMgR8yjitEeHGx8spzcDLGt2gkBm"],
## Alice node listen port: 62379;  --listen-addr /ip4/0.0.0.0/tcp/62379/ws

$POLKADOT_BIN --chain $CIPHERA_DEMO_ROOTDIR/alice/cfg/rococo-local.json --name alice --rpc-cors all --unsafe-rpc-external --rpc-methods unsafe --no-mdns --node-key 2bd806c97f0e00af1a1fc3328fa763a9269723c8db8fac4f93af71db186d6e90 --no-telemetry --prometheus-external --validator --insecure-validator-i-know-what-i-do --prometheus-port 58407 --rpc-port 51072 --listen-addr /ip4/0.0.0.0/tcp/62379/ws --base-path $CIPHERA_DEMO_ROOTDIR/alice/data
