# Investigate

Structured protocol for debugging and root cause analysis.

## Trigger

Keywords: "investigate", "root cause", "debug", "why is", "what happened"

## Principle

Every claim must cite a file, log line, code path, or terminal output — no assumptions without evidence.

## Phases

### 1. Understand
- Parse the symptom, timeframe, location, and impact
- Ask clarifying questions if needed

### 2. Plan
- Develop testable hypotheses
- Create task list of checks

### 3. Gather Evidence
- Search code, read files, execute safe diagnostics
- Document findings with citations

### 4. Root Cause
- Synthesize findings into explicit conclusions
- Identify contributing factors

### 5. Solutions
- Propose immediate mitigation
- Propose permanent fix
- Note effort/risk for each

### 6. Write Outputs
Save to `.docs/investigations/`:
- `report.md` — summary, evidence table, root cause, timeline, solutions
- `flowchart.mmd` — Mermaid diagram: trigger → components → root cause

### 7. Summarize
- Deliver verdict with recommendations
