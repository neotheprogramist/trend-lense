#!/bin/bash

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Check if identity parameter is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <dfx_identity_profile>"
    echo "Example: $0 default"
    exit 1
fi

DFX_IDENTITY="$1" 

dfx identity use "$DFX_IDENTITY"

dfx stop
dfx start --background --clean
# Source other scripts using the absolute path and pass the identity
. "$SCRIPT_DIR/deploy_proxy_local.sh" "$DFX_IDENTITY"
dfx deploy
. "$SCRIPT_DIR/init_proxy_local.sh" "$DFX_IDENTITY"
. "$SCRIPT_DIR/init_backend_local.sh" "$DFX_IDENTITY"
. "$SCRIPT_DIR/setup_backend_time_periods.sh" "$DFX_IDENTITY" "BTC-EUR" "Okx Coinbase" "24h"