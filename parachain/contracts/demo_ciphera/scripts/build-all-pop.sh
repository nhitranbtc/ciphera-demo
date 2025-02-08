#!/usr/bin/env bash


set -eu

ROOTDIR=$(git rev-parse --show-toplevel)

cd $ROOTDIR/parachain/contracts/demo_ciphera/

pop build --release

cd access_control && pop build --release
