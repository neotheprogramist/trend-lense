#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if [ $# -ne 5 ]; then
    echo "Usage: $0 <dfx_identity> <pair> <exchanges> <start_timestamp> <end_timestamp>"
    echo "Example: $0 default BTC-EUR \"Okx Coinbase\" 1680000000 1680100000"
    exit 1
fi

DFX_IDENTITY="$1"
PAIR="$2"
IFS=' ' read -r -a EXCHANGES <<< "$3"
START_TIMESTAMP="$4"
END_TIMESTAMP="$5"

dfx identity use "$DFX_IDENTITY"

BACKEND_CANISTER=$(dfx canister id trendlens_backend)

if [ -z "$BACKEND_CANISTER" ]; then
    echo "Error: Could not find trendlens_backend canister ID"
    exit 1
fi

echo "Setting up backend for pair $PAIR with exchanges: ${EXCHANGES[*]}"

for EXCHANGE in "${EXCHANGES[@]}"; do
    echo "Initializing pair for $EXCHANGE..."
    dfx canister call "$BACKEND_CANISTER" initialize_pair "(\"$PAIR\", variant { $EXCHANGE })"
    
    echo "Refreshing instruments for $EXCHANGE..."
    dfx canister call "$BACKEND_CANISTER" refresh_instruments "(variant { $EXCHANGE }, variant { Spot })"
done


for EXCHANGE in "${EXCHANGES[@]}"; do
    echo "Initializing volume store..."
    dfx canister call "$BACKEND_CANISTER" initialize_volume_store "(variant { $EXCHANGE }, \"$PAIR\", $START_TIMESTAMP)"
    echo "Pulling data for $EXCHANGE..."
    dfx canister call "$BACKEND_CANISTER" pull_volumes "(variant { $EXCHANGE }, \"$PAIR\", $END_TIMESTAMP)"
    dfx canister call "$BACKEND_CANISTER" pull_candles "(\"$PAIR\", variant { $EXCHANGE }, $START_TIMESTAMP, $END_TIMESTAMP)"
done

echo "Backend setup completed"

