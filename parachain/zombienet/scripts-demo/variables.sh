ROOTDIR=$(git rev-parse --show-toplevel)
echo "ROOTDIR: $ROOTDIR"

POLKADOT_BIN="${POLKADOT_BIN:-$ROOTDIR/parachain/binaries/polkadot-stable2407/polkadot}"
CIPHERA_BIN="${CIPHERA_BIN:-$ROOTDIR/parachain/target/release/ciphera-collator}"

ZOMBIENET_TMP=$ROOTDIR/parachain/zombienet/zombienet-demo ## Custom
CIPHERA_DEMO_N="ciphera-demo" ## Update: 01, 02, ...


CIPHERA_DEMO_ROOTDIR="$ROOTDIR/parachain/zombienet/$CIPHERA_DEMO_N"
CIPHERA_DEMO_LOG="$ROOTDIR/parachain/zombienet/$CIPHERA_DEMO_N/log"
