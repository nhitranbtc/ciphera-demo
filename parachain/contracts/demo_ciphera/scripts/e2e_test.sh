#!/usr/bin/env bash

set -eu

ROOTDIR=$(git rev-parse --show-toplevel)

cd $ROOTDIR

cd contracts/demo_ciphera/tests && pop test contract --e2e