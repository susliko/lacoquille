# Live Progress Log

> Append-only. Newest entries at the bottom.

## Round 1 — 2026-06-14

### Starting: ISSUE-001 + ISSUE-002

Both are critical, isolated, and can run in parallel:

- **ISSUE-001** (Practice hub blank) is a frontend CSS-scoping bug. Pure style fix, no data or backend work.
- **ISSUE-002** (Shadowing stuck on loading) needs the missing API endpoints or a graceful error state. Decision will be made by the planner sub-agent after recon.

Plan: dispatch scout for both, then planner, then two parallel workers, then reviewer, then land PRs.
