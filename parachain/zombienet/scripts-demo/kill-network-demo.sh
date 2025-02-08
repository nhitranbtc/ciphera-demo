#!/usr/bin/env bash

# This script kills all processes started by launch-network-demo.sh

echo "Killing all processes with names containing 'ciphera-c' or 'polkadot'..."

pkill -f ciphera-c
pkill -f polkadot

echo "All specified processes killed."

echo "Killing all processes started by launch-network-demo.sh..."

pkill -f polkadot-alice.sh
pkill -f polkadot-bob.sh
pkill -f ciphera-collator-01.sh

echo "All processes killed."

rm -rf $CIPHERA_DEMO_LOG

lsof -i -P -n | grep LISTEN
