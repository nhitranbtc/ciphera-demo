#!/bin/bash
set -euo pipefail

# Source the variable declarations from variables.sh
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/variables.sh"

## Bootnode:
## rococo-local.json:   "bootNodes": [ "/ip4/127.0.0.1/tcp/62379/ws/p2p/12D3KooWQCkBm1BYtkHpocxCwMgR8yjitEeHGx8spzcDLGt2gkBm"],
## Alice node listen port: 62379;  --listen-addr /ip4/0.0.0.0/tcp/62379/ws
## Bob node peer with Alice node: --bootnodes /ip4/127.0.0.1/tcp/62379/ws/p2p/12D3KooWQCkBm1BYtkHpocxCwMgR8yjitEeHGx8spzcDLGt2gkBm
## Collator node: If collator can't peer with relay node, we can start collator node with --bootnoodes

$CIPHERA_BIN --name ciphera-collator1 --node-key 1678111c0477f9dccd521d0c2e8fd2bb8c3436f5ab6beb8f6cc5381353d234b8 --chain $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/cfg/rococo-local-2111.json --base-path $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data --listen-addr /ip4/0.0.0.0/tcp/58411/ws --prometheus-external --rpc-cors all --unsafe-rpc-external --rpc-methods unsafe --prometheus-port 58413 --rpc-port 9944 --collator -- --chain $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/cfg/rococo-local.json --execution wasm --port 58465 --rpc-port 58466
