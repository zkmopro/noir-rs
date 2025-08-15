#!/usr/bin/env sh

set -e
circuit=${1:-recursive}

# Execute the circuit to generate a witness
nargo execute --package $circuit ${circuit}_witness
