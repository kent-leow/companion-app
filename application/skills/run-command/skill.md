---
name: run-command
version: "1.0.0"
description: "Execute a shell command and return stdout/stderr"
triggers:
  - "run"
  - "execute"
  - "shell"
  - "terminal"
  - "npm"
  - "git status"
  - "build"
  - "test"
  - "install"
parameters:
  - name: command
    type: string
    required: true
    description: "Shell command to execute"
  - name: cwd
    type: string
    required: false
    description: "Working directory (default: session cwd)"
commands:
  - name: exec
    template: |
      cd "{cwd}" && {command} 2>&1 | head -100
    timeout: 30
---

# run-command

Execute shell commands. Return stdout/stderr truncated to 100 lines.

## Safety
Never execute commands that delete data, modify system config, or expose secrets unless explicitly confirmed.
