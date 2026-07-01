---
name: read-file
version: "1.0.0"
description: "Read a local file's content (text, code, config, markdown)"
triggers:
  - "read file"
  - "show file"
  - "cat"
  - "open file"
  - "@"
parameters:
  - name: path
    type: string
    required: true
    description: "Absolute or relative file path"
  - name: lines
    type: number
    required: false
    description: "Max lines to read (default: 200)"
commands:
  - name: read
    template: |
      if [ -f "{path}" ]; then
        head -n ${lines:-200} "{path}"
      else
        echo "[ERROR] File not found: {path}"
      fi
    timeout: 5
---

# read-file

Read and return file contents. Used when user references a file with @path or asks to look at code.
