---
name: figma-design-context
version: "1.0.0"
description: "Extract Figma design context via REST API: layout, typography, colors, UI flow, components"
triggers:
  - "figma"
  - "figma.com"
  - "design"
  - "design spec"
  - "UI spec"
  - "mockup"
  - "prototype"
parameters:
  - name: query
    type: string
    required: true
    description: "Figma URL or file key with optional node-id"
auth:
  - env: FIGMA_TOKEN
    description: "Figma personal access token"
commands:
  - name: preflight
    template: |
      echo "FIGMA_TOKEN: $([ -n "$FIGMA_TOKEN" ] && echo OK || echo MISSING)"
    timeout: 5
  - name: get-metadata
    template: |
      bash skills/figma-design-context/scripts/get-metadata.sh --file-key {fileKey}
    timeout: 15
  - name: get-screenshot
    template: |
      bash skills/figma-design-context/scripts/get-screenshot.sh --file-key {fileKey} --node-id {nodeId} --scale 2
    timeout: 30
  - name: get-design-context
    template: |
      bash skills/figma-design-context/scripts/get-design-context.sh --file-key {fileKey} --node-id {nodeId}
    timeout: 30
---

# figma-design-context

Extract Figma design context via REST API.

## URL Parsing

`https://www.figma.com/design/AbCdEfGhIj/My-Project?node-id=2313-102848`
- `fileKey` = `AbCdEfGhIj` (segment after `/design/`)
- `nodeId` = `2313:102848` (query param `node-id`, convert `-` to `:`)

## Workflow

1. Parse Figma URL → extract fileKey + nodeId
2. [TOOL:figma-design-context] with `get-metadata`
3. [TOOL:figma-design-context] with `get-screenshot`
4. [TOOL:figma-design-context] with `get-design-context`
5. [RESPOND] with structured design context
