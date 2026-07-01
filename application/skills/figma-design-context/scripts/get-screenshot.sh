#!/bin/bash
set -euo pipefail

FILE_KEY=""
NODE_ID=""
SCALE=2

while [[ $# -gt 0 ]]; do
  case "$1" in
    --file-key) FILE_KEY="$2"; shift 2 ;;
    --node-id) NODE_ID="$2"; shift 2 ;;
    --scale) SCALE="$2"; shift 2 ;;
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

RESPONSE=$(curl -s -H "X-Figma-Token: $FIGMA_TOKEN" \
  "https://api.figma.com/v1/images/${FILE_KEY}?ids=${NODE_ID}&scale=${SCALE}&format=png")

IMAGE_URL=$(echo "$RESPONSE" | jq -r ".images[\"${NODE_ID}\"]")
echo "IMAGE_URL: $IMAGE_URL"
