#!/usr/bin/env bash

# Config
HOST="${1:-127.0.0.1}"
PORT="${2:-6379}"
NUM_CLIENTS=50
TEMP_DIR=$(mktemp -d)

trap 'rm -rf "$TEMP_DIR"' EXIT

echo "=========================================="
echo " Running KV Store Concurrency Stress Test "
echo " Target: $HOST:$PORT | Clients: $NUM_CLIENTS"
echo "=========================================="

if ! nc -z "$HOST" "$PORT" >/dev/null 2>&1; then
    echo "Error: KV store server is not runiing on $HOST:$PORT"
    exit 1
fi

echo "1. Spawning $NUM_CLIENTS parallel client workers..."

for i in $(seq 1 "$NUM_CLIENTS"); do
    (
        printf "SET key_%s value_%s\nGET key_%s\n" "$i" "$i" "$i" | nc -w 1 "$HOST" "$PORT" > "$TEMP_DIR/out_$i.txt"
    ) &
done

wait

echo "2. Verifying thread safety and data integrity..."

PASSED=0
FAILED=0

for i in $(seq 1 "$NUM_CLIENTS"); do
    OUT_FILE="$TEMP_DIR/out_$i.txt"
    EXPECTED="value_$i"

    if grep -q "$EXPECTED" "$OUT_FILE"; then
        ((PASSED++))
    else
        echo "  [FAIL] Client $i failed. Expected '$EXPECTED', got:"
        cat "$OUT_FILE"
        ((FAILED++))
    fi
done


echo "=========================================="
echo " Results: $PASSED Passed | $FAILED Failed"
echo "=========================================="

if [ "$FAILED" -eq 0 ]; then
    echo "SUCCESS: Zero data races or thread deadlocks detected!"
    exit 0
else
    echo "FAILURE: Concurrency errors detected."
    exit 1
fi
