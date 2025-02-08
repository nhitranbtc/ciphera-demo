#!/bin/bash
set -euo pipefail

# Source the variable declarations from variables.sh
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/variables.sh"

mkdir -p $ROOTDIR/parachain/zombienet/$CIPHERA_DEMO_N
CIPHERA_DEMO_ROOTDIR="$ROOTDIR/parachain/zombienet/$CIPHERA_DEMO_N"

## cfg
mkdir -p $CIPHERA_DEMO_ROOTDIR/cfg

## alice

mkdir -p $CIPHERA_DEMO_ROOTDIR/alice/cfg
mkdir -p $CIPHERA_DEMO_ROOTDIR/alice/data/chains/rococo_local_testnet/db
mkdir -p $CIPHERA_DEMO_ROOTDIR/alice/data/chains/rococo_local_testnet/keystore
mkdir -p $CIPHERA_DEMO_ROOTDIR/alice/data/chains/rococo_local_testnet/network
mkdir -p $CIPHERA_DEMO_ROOTDIR/alice/relay-data

## bob

mkdir -p $CIPHERA_DEMO_ROOTDIR/bob/cfg
mkdir -p $CIPHERA_DEMO_ROOTDIR/bob/data/chains/rococo_local_testnet/db
mkdir -p $CIPHERA_DEMO_ROOTDIR/bob/data/chains/rococo_local_testnet/keystore
mkdir -p $CIPHERA_DEMO_ROOTDIR/bob/data/chains/rococo_local_testnet/network
mkdir -p $CIPHERA_DEMO_ROOTDIR/bob/relay-data

## ciphera-collator1

mkdir -p $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/cfg

mkdir -p $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/chains/local_testnet/db
mkdir -p $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/chains/local_testnet/keystore
mkdir -p $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/chains/local_testnet/network

mkdir -p $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/polkadot/chains/rococo_local_testnet/db
mkdir -p $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/polkadot/chains/rococo_local_testnet/keystore
mkdir -p $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/polkadot/chains/rococo_local_testnet/network

mkdir -p $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/relay-data


## Generate chain spec
cp -r $ZOMBIENET_TMP/ciphera-collator1/cfg $CIPHERA_DEMO_ROOTDIR/

### Alice, Bod relay chain spec

cp $CIPHERA_DEMO_ROOTDIR/cfg/rococo-local.json $CIPHERA_DEMO_ROOTDIR/alice/cfg/
cp $CIPHERA_DEMO_ROOTDIR/cfg/rococo-local.json $CIPHERA_DEMO_ROOTDIR/bob/cfg/

### ciphera-collator1
cp -r $CIPHERA_DEMO_ROOTDIR/cfg $CIPHERA_DEMO_ROOTDIR/ciphera-collator1

## Generate Keystore

cp -r $ZOMBIENET_TMP/alice/keystore $CIPHERA_DEMO_ROOTDIR/alice/data/chains/rococo_local_testnet/

cp -r $ZOMBIENET_TMP/bob/keystore $CIPHERA_DEMO_ROOTDIR/bob/data/chains/rococo_local_testnet/

### Ciphera-collator1 keystore
cp -r $ZOMBIENET_TMP/2111 $CIPHERA_DEMO_ROOTDIR/

cp -r $CIPHERA_DEMO_ROOTDIR/2111/ciphera-collator1/keystore $CIPHERA_DEMO_ROOTDIR/ciphera-collator1/data/chains/local_testnet/
