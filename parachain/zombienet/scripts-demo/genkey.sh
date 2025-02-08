#!/bin/bash
set -euo pipefail

# Source the variable declarations from variables.sh
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/variables.sh"

## Generate keystore

##===Clear data/db
rm -r $ZOMBIENET_TMP/2111
rm -r $CIPHERA_DEMO_ROOTDIR/alice/data/chains/rococo_local_testnet/db
rm -r $CIPHERA_DEMO_ROOTDIR/bob/data/chains/rococo_local_testnet/db
rm -r $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/chains/local_testnet/db
rm -r $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/polkadot/chains/rococo_local_testnet/db


## Generate Keystore

cp -r $ZOMBIENET_TMP/alice/keystore $CIPHERA_DEMO_ROOTDIR/alice/data/chains/rococo_local_testnet/
cp -r $ZOMBIENET_TMP/bob/keystore $CIPHERA_DEMO_ROOTDIR/bob/data/chains/rococo_local_testnet/

### Ciphera-collator1 keystore
cp -r $ZOMBIENET_TMP/2111 $CIPHERA_DEMO_ROOTDIR/
cp -r $CIPHERA_DEMO_1/2111/ciphera-collator1/keystore $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/chains/local_testnet/
