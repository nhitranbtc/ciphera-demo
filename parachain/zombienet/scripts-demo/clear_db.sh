#!/bin/bash
set -euo pipefail

# Source the variable declarations from variables.sh
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/variables.sh"

rm -rf $CIPHERA_DEMO_ROOTDIR/alice/data/chains/rococo_local_testnet/db

## bob


rm -rf $CIPHERA_DEMO_ROOTDIR/bob/data/chains/rococo_local_testnet/db


## ciphera-collator1

rm -rf $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/chains/local_testnet/db


rm -rf $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/polkadot/chains/rococo_local_testnet/db
