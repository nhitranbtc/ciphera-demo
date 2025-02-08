#!/usr/bin/env bash

set -eu

cargo contract build --manifest-path access_control/Cargo.toml

cargo contract build