#!/bin/bash
set -euo pipefail

# Source the variable declarations from variables.sh
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/variables.sh"

echo "CIPHERA_BIN: $CIPHERA_BIN"

## rococo-local-plain.json

$CIPHERA_BIN build-spec --disable-default-bootnode > $CIPHERA_DEMO_ROOTDIR/cfg/2111-rococo-local-plain.json

## rococo-local-plain.json
$CIPHERA_BIN build-spec --chain $CFG/2111-rococo-local-plain.json --disable-default-bootnode  --raw > $CIPHERA_DEMO_ROOTDIR/cfg/2111-rococo-local.json