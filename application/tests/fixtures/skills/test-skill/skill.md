---
name: test-skill
version: "1.0.0"
description: "A test skill for unit testing"
triggers:
  - "test"
  - "check"
  - "verify"
parameters:
  - name: input
    type: string
    required: true
    description: "Test input value"
  - name: count
    type: number
    required: false
    description: "Optional count"
commands:
  - name: run
    template: |
      echo "Running test with {input}"
    timeout: 5
---

# test-skill

A test skill used in unit tests.
