#!/usr/bin/env bash
# Charge HTTP — wrk (prioritaire) ou hey.
# Usage: ./benchmarks/scripts/load_test.sh <URL> [label]
# Exemple: ./benchmarks/scripts/load_test.sh http://127.0.0.1:3000/users ruest

set -euo pipefail

URL="${1:?URL required, e.g. http://127.0.0.1:3000/users}"
LABEL="${2:-bench}"
THREADS="${THREADS:-4}"
CONNECTIONS="${CONNECTIONS:-100}"
DURATION="${DURATION:-30s}"

echo "=== Load test: $LABEL ==="
echo "URL=$URL threads=$THREADS connections=$CONNECTIONS duration=$DURATION"
echo ""

if command -v wrk >/dev/null 2>&1; then
  wrk -t"$THREADS" -c"$CONNECTIONS" -d"$DURATION" --latency "$URL"
elif command -v hey >/dev/null 2>&1; then
  # hey uses -z duration like 30s
  hey -z "$DURATION" -c "$CONNECTIONS" "$URL"
else
  echo "Install wrk (brew install wrk) or hey for load testing." >&2
  exit 1
fi
