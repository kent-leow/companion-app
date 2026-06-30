# Spike Plan

Generate a time-boxed technical investigation document to reduce uncertainty before implementation.

## Trigger

Keywords: "spike", "technical spike", "research spike"

## Principle

Spikes produce knowledge, not shippable code.

## Phases

### Phase 1 — Ingest
- Read plan.md
- Identify uncertainty signals ("TBD", unclear dependencies, new tech)

### Phase 2 — Explore
- Search codebase for analogous patterns
- Identify reusable components

### Phase 3 — Research
- External research on unfamiliar technologies
- Evaluate alternatives with pros/cons

### Phase 4 — Synthesize
- Create `spike.md` with confidence and complexity scoring
- Sections: Goals, Risks, Approach, Findings, Recommendations

### Phase 5 — Executive Summary
- Create `spike-report.md` for non-technical stakeholders
- Business language, Mermaid diagrams, no code

## Output

Save to `.docs/<feature>/`:
- `spike.md` — technical findings
- `spike-report.md` — executive summary

## Constraints

- No code implementation
- Goals must have evidence, not assumptions
- Time-box capped at 5 days
- All web research must be cited
