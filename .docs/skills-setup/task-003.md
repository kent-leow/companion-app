# Task 003 — Create figma-design-context skill

## Goal
Add `figma-design-context` skill with 19 associated shell scripts for Figma REST API extraction.

## Prerequisites
- None

## Decisions
- Option C: `## Commands` has token preflight; `## Prompt` has full workflow referencing scripts
- Script paths updated to `application/skills/figma-design-context/scripts/<name>.sh`

## Tasks
- [x] skill: Create `application/skills/figma-design-context/skill.md`
  - `# figma-design-context` heading
  - Description line: extract Figma design context via REST API
  - `## Commands`: FIGMA_TOKEN check (`echo "FIGMA_TOKEN: $([ -n "$FIGMA_TOKEN" ] && echo OK || echo MISSING)"` with `{query}` passthrough)
  - `## Prompt`: Full content from `.docs/skills-setup/figma-design-context/SKILL.md` with ALL script paths rewritten from `.github/skills/figma-design-context/scripts/` → `application/skills/figma-design-context/scripts/`
- [x] scripts: Copy all 19 scripts from `.docs/skills-setup/figma-design-context/scripts/` → `application/skills/figma-design-context/scripts/`
- [x] scripts: Ensure all scripts are executable (`chmod +x`)
- [x] test: Verify skill loads — `cargo test` in `application/`

## Done When
- `application/skills/figma-design-context/skill.md` exists and parses correctly
- `application/skills/figma-design-context/scripts/` contains 19 executable `.sh` files
- All script path references in skill.md point to `application/skills/figma-design-context/scripts/`
- `cargo test` passes
