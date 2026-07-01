#!/bin/bash
set -euo pipefail

FILE_KEY=""
NODE_ID=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --file-key) FILE_KEY="$2"; shift 2 ;;
    --node-id) NODE_ID="$2"; shift 2 ;;
    *) shift ;;
  esac
done

if [ -z "$FILE_KEY" ] || [ -z "$NODE_ID" ]; then
  echo "ERROR: --file-key and --node-id required" >&2
  exit 1
fi

if [ -z "${FIGMA_TOKEN:-}" ]; then
  echo "ERROR: FIGMA_TOKEN not set" >&2
  exit 1
fi

curl -s -H "X-Figma-Token: $FIGMA_TOKEN" \
  "https://api.figma.com/v1/files/${FILE_KEY}/nodes?ids=${NODE_ID}" \
  | jq '.nodes | to_entries[0].value.document | {
    id: .id, name: .name, type: .type,
    size: {width: .absoluteBoundingBox.width, height: .absoluteBoundingBox.height},
    fills: .fills, strokes: .strokes,
    children: [.children[]? | {id: .id, name: .name, type: .type}]
  }'
