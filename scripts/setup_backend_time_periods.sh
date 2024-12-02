#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

if [ $# -ne 4 ]; then
    echo "Usage: $0 <dfx_identity> <pair> <exchanges> <time_period>"
    echo "Example: $0 default BTC-EUR \"Okx Coinbase\" 24h"
    echo "Time periods: 12h, 1d, 2d, 7d, 30d"
    exit 1
fi

DFX_IDENTITY="$1"
PAIR="$2"
EXCHANGES="$3"
PERIOD="$4"

case $PERIOD in
    "12h")
        SECONDS_AGO=$((12 * 3600))
        ;;
    "1d" | "24h")
        SECONDS_AGO=$((24 * 3600))
        ;;
    "2d" | "48h")
        SECONDS_AGO=$((2 * 24 * 3600))
        ;;
    "7d")
        SECONDS_AGO=$((7 * 24 * 3600))
        ;;
    "30d")
        SECONDS_AGO=$((30 * 24 * 3600))
        ;;
    *)
        echo "Invalid time period. Use: 12h, 1d, 2d, 7d, or 30d"
        exit 1
        ;;
esac

END_TS=$(date +%s)
START_TS=$((END_TS - SECONDS_AGO))

echo "Setting up backend for last $PERIOD..."
echo "Start time: $(date -d @$START_TS)"
echo "End time: $(date -d @$END_TS)"

"$SCRIPT_DIR/setup_backend.sh" "$DFX_IDENTITY" "$PAIR" "$EXCHANGES" "$START_TS" "$END_TS" 