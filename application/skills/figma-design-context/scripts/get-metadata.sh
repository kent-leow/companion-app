#!/bin/bash
set -euo pipefail

FILE_KEY=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --file-key) FILE_KEY="$2"; shift 2 ;;
    *) shift ;;
  esac
done

if [ -z "$FILE_KEY" ]; then
  echo "ERROR: --file-key required" >&2
  exit 1
fi

if [ -z "${FIGMA_TOKEN:-}" ]; then
  echo "ERROR: FIGMA_TOKEN not set" >&2
  exit 1
fi

curl -s -H "X-Figma-Token: $FIGMA_TOKEN" \
  "https://api.figma.com/v1/files/${FILE_KEY}?depth=1" \
  | jq '{name: .name, lastModified: .lastModified, version: .version, pages: [.document.children[] | {id: .id, name: .name, type: .type}]}'
