#!/usr/bin/env bash

# This script starts a standalone node without relaychain network locally, with the parachain runtime

set -eo pipefail

# Source the variable declarations from variables.sh
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/variables.sh"

# Check if required binaries exist
if [ ! -f "$POLKADOT_BIN" ]; then
  echo "polkadot binary not found in binaries/polkadot-stable2407/, build failed?"
  exit 1
fi

if [ ! -f "$CIPHERA_BIN" ]; then
  echo "No ciphera-collator found, exiting ..."
  exit 1
fi

if ! "$CIPHERA_BIN" --version &> /dev/null; then
  echo "Cannot execute $CIPHERA_BIN, wrong executable?"
  exit 1
fi

# Ensure the log directory exists
if [ ! -d "$CIPHERA_DEMO_LOG" ]; then
  mkdir -p "$CIPHERA_DEMO_LOG"
fi

function print_divider() {
  echo "------------------------------------------------------------"
}

function cleanup() {
  echo "Cleaning up background processes..."
  pkill -P $$
}

trap cleanup EXIT

# Ensure the script is executable
chmod +x ./ciphera-collator-01.sh

./clear_db.sh

print_divider
echo "Starting ciphera-relay01 in standalone mode ..."
nohup ./polkadot-alice.sh > "$CIPHERA_DEMO_LOG/polkadot-alice.log" 2>&1 &

print_divider
echo "Starting ciphera-relay02 in standalone mode ..."
nohup ./polkadot-bob.sh > "$CIPHERA_DEMO_LOG/polkadot-bob.log" 2>&1 &

print_divider
echo "Starting ciphera-collator in standalone mode ..."
nohup ./ciphera-collator-01.sh > "$CIPHERA_DEMO_LOG/ciphera-collator-01.log" 2>&1 &

# List nohup processes to verify they are running
print_divider
echo "Listing nohup processes..."
ps -ef | grep nohup

wait
print_divider
