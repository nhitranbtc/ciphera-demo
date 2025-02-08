#!/bin/bash
set -euo pipefail

# Source the variable declarations from variables.sh
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/variables.sh"

echo "POLKADOT_BIN: $POLKADOT_BIN"

## rococo-local-plain.json

$POLKADOT_BIN build-spec --chain rococo-local --disable-default-bootnode > $CIPHERA_DEMO_ROOTDIR/cfg/rococo-local-plain.json

## rococo-local.json (raw)
$POLKADOT_BIN build-spec --chain $CIPHERA_DEMO_ROOTDIR/cfg/rococo-local-plain.json --disable-default-bootnode  --raw > $CIPHERA_DEMO_ROOTDIR/cfg/rococo-local.json
