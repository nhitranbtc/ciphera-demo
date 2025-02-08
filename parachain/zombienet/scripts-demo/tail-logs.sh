#!/usr/bin/env bash

# This script starts a standalone node without relaychain network locally, with the parachain runtime

set -eo pipefail

# Source the variable declarations from variables.sh
SCRIPT_DIR="$(dirname "$0")"
source "$SCRIPT_DIR/variables.sh"


LOG_DIR="$CIPHERA_DEMO_LOG"


if [ ! -d "$LOG_DIR" ]; then
  echo "Log directory $LOG_DIR does not exist."
  exit 1
fi

# Function to tail logs with color
tail_with_color() {
  local color=$1
  local file=$2
  tail -f "$file" | sed "s/^/${color}/; s/$/${NC}/" &
}

# Trap SIGINT (Ctrl+C) to kill all background tail processes
trap 'kill $(jobs -p)' SIGINT

echo "Tailing log files in $LOG_DIR..."

tail -f "$LOG_DIR/polkadot-alice.log" "$LOG_DIR/polkadot-bob.log" "$LOG_DIR/ciphera-collator-01.log"

wait
