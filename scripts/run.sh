#!/usr/bin/env bash
set -euo pipefail

BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_FILE="${CONFIG_FILE:-$BASE_DIR/config.ini}"
OUTPUT_DIR="${OUTPUT_DIR:-$BASE_DIR/output}"
SEND_QUERY_JSON="${SEND_QUERY_JSON:-0}"

mkdir -p "$OUTPUT_DIR"

ARGS=(
  --config "$CONFIG_FILE"
  --output "$OUTPUT_DIR"
)

if [[ "$SEND_QUERY_JSON" == "1" ]]; then
  ARGS+=(--send-query-json)
fi

exec "$BASE_DIR/data-import-sql" "${ARGS[@]}" "$@"
